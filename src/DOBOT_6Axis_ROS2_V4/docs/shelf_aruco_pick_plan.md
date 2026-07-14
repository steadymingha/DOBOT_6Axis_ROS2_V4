# Shelf ArUco pick & place 개편 계획 (2026-07-11)

선반 위치/높이 변경 후 `shelf_pick_place.py`가 선반에 부딪히고 못 집는 문제의
수정 + ArUco 기반 선반 인식 도입 + 2단 선반(총 8박스) 왕복 테스트 계획.

## 0. 증상과 근본 원인

선반 보드 높이가 mesh/world에서 **0.90/1.40 → 0.685/1.30**으로 낮아졌는데
(cr.world 주석, shelf/model.sdf), 코드 상수는 옛 값 그대로였다:

| 위치 | 상수 | 코드 값(구) | 실제 값 |
|---|---|---|---|
| `cr7_pnp/geometry.py` | `SHELF_BOARD_TOPS` (충돌 보드) | 0.40/0.90/1.40/1.90 | 0.40/**0.685**/**1.30**/1.90 |
| `sequences/shelf_pick_place.py` | `SHELF_BOXES` z | 0.97 | **0.755** |

→ 플래너는 **없는 위치(0.90)의 보드**를 피하고, **실제 보드(0.685)는 모델에
없어서** 그대로 관통 경로를 만든다. 박스 목표 z도 21.5 cm 위를 잡으므로 못 집는다.
이런 "world만 바꾸고 상수는 안 바뀜" 사고를 없애는 것이 이번 개편의 핵심:
**선반 pose는 ArUco로 런타임에 읽고, 박스 위치·충돌 보드는 그 pose + 공유 레이아웃
상수에서 유도**한다.

## 1. 목표 (요구사항 매핑)

1. **wirebonder 방식과 동일한 ArUco 인식**: 선반 각 단(tier)에 마커를 붙이고,
   그 단을 집기 전에 한 번 캡처(정지 후 median-read) → 선반 frame 확보 → 박스
   대략 위치 유도. (박스 중심 정밀 탐지는 추후 AI 모델이 이 seam을 대체)
   공중에 떠 있는 `aruco` 박스 모델은 world에서 **삭제**.
2. **tier-2에도 박스 4개** (cr.world).
3. **두 단 8박스 모두 운반 가능**: tier1=0.685, tier2=1.30 유지.
   tier2 박스 중심 1.37 m는 wirebonder 상단 슬롯 pick(박스 중심 1.355, 검증됨)과
   거의 같은 높이라 도달 가능 예상. 안 되면 shelf mesh 재수정(fallback, Blender 작업).
4. **하드코딩 제거 / 공유 상수화**: 레이아웃은 `cr7_pnp/geometry.py` 한 곳,
   태그 배치는 `vision/shelf_vision.py` 한 곳(=model.sdf 미러). dispatcher
   (`main.py`, run_test.sh 경로)에서도 같은 시퀀스 함수 호출.
5. 이 문서.
6. **실물 전환 고려**: 카메라 토픽은 이미 sim/실물 통일. 실물에서는 마커 스티커를
   선반에 붙이고 `shelf_vision.py`의 태그 pose 상수만 실측으로 교체.

## 2. 변경 파일

### 2.1 World / 모델

- `dobot_gazebo/worlds/cr.world`
  - 떠 있는 `aruco` include 삭제.
  - tier-1 박스 4개 이름 변경 `box_l1a-d` → `box_t1a-d` (t=tier; l2 이름이
    wirebonder 매거진과 충돌해 혼동 방지).
  - tier-2: `box_l2d` 삭제, `box_t2a-d` 4개 추가 (tier1과 같은 x, z=1.37).
- `src/blender/shelf/model.sdf`
  - 태그 plate visual 2개 추가 (wirebonder `aruco_G_L` 패턴 그대로):
    - tier1 = AprilTag 36h11 **ID 2**, tier2 = **ID 3** (0/1은 wirebonder 사용 중).
    - 위치(선반 모델 frame): x=-0.45 (박스 행 왼쪽 옆), y=-0.1495(전면 0.5 mm 앞),
      z=보드 상단+0.019 (37.5 mm plate가 보드 앞모서리에 선 placard), roll=π/2
      (로봇 쪽 -y를 향함 → wirebonder와 같은 upright 파이프라인 재사용).
  - `src/blender/shelf/materials/{scripts/aruco.material, textures/april_36h11-2.png, -3.png}`
    신규 (텍스처는 cv2로 생성: 640px = 512 태그 + 64 quiet zone, 검정 태그 30 mm).

### 2.2 공유 상수 (`cr7_pnp/geometry.py`)

- `SHELF_BOARD_TOPS = (0.40, 0.685, 1.30, 1.90)` ← **버그 수정**.
- 신규: `SHELF_WORLD_POSE = (0.8, 0.5, 0.0)` (x, y, yaw; --no-vision 폴백 겸
  vision 타당성 anchor), `SHELF_TIER_TOPS = {1: 0.685, 2: 1.30}`,
  `SHELF_BOX_XS = (-0.0905, +0.0905, -0.2715, +0.2715)` (선반 frame, 집는 순서
  a,b(안쪽)→c,d(바깥쪽) — 기존 순서 유지), `shelf_box_center(tier, i, pose)` 헬퍼.
- 구 `SHELF_WORLD_XY`/`SHELF_BOX_WORLD`/`SHELF_BOX_MODEL` 정리 (유도값으로 대체).
- `update_shelf_collision(pose=None)` (node.py): 라이브 선반 pose(비전 읽기)로
  충돌 보드를 배치할 수 있게 인자 추가 (기본값 = `SHELF_WORLD_POSE`).

### 2.3 Vision

- `vision/wirebonder_vision.py`: `device_pose_in_base()` 등에
  `T_model_tag=` 인자 추가(기본 = wirebonder 상수 → 기존 동작 불변).
- `vision/shelf_vision.py` (신규, 순수 모듈): 선반 태그 정의
  `{tag_id: T_SHELF_TAG}` (model.sdf 미러) + `shelf_pose_in_base()` 래퍼.
- `vision/wirebonder_vision_node.py`: 같은 카메라 스트림에서 선반 태그(2/3)도
  검출해 `/vision/shelf_pose`(PoseStamped, odom = 선반 모델 frame)로 발행.
  두 태그 모두 같은 선반 frame을 추정하므로 토픽 하나로 충분(시퀀스가 median).

### 2.4 시퀀스 (`sequences/shelf_pick_place.py` 개편)

- `SHELF_BOXES` 하드코딩 삭제 → `node.shelf_pose`(캡처 값) ∘ 레이아웃 상수로 유도.
  AI 박스 탐지 모델이 들어올 seam = `box_center_world(node, tier, i)` 한 함수.
- 캡처 단계 추가 (`capture_shelf(node, tier)`): hub → 해당 tier 태그 앞 캡처
  포즈(월드 frame 상수, TUNE) → 2 s 정지 → `/vision/shelf_pose` median-read
  (spread/anchor 게이트, wirebonder `refresh_device_pose`와 동일 구조) → 역재생
  으로 hub 복귀. `--no-vision`이면 `SHELF_WORLD_POSE` 사용.
- 픽 전 `update_shelf_collision(shelf_pose)` — 충돌 보드가 라이브 pose를 따름.
- `pick_place_one_box(node, tier, i)`로 일반화 (기존 hub-and-spoke 로직 유지).
- **되돌려 놓기**(테스트용): pick/place 때 실행한 전 구간의 joint 경로를 박스별로
  기록 → put-back은 검증된 경로의 역재생(포켓 재-pick → rev(선반 pick 경로) →
  release → 전진 재실행으로 hub 복귀). 새 플래닝 없음 = 실패 모드 없음.
  포켓의 박스는 magazine attach 해제 후 집는다 (`detach_box_from_magazine` 신규).

### 2.5 테스트 스크립트 (`sequences/test_shelf_cycle.py` 신규)

```
for tier in (1, 2):
    capture_shelf(tier)                      # ArUco 1회 읽기
    for i in 0..3:  pick_place_one_box(tier, i)   # 선반 → 베이스 포켓 4개
    for i in 3..0:  put_back_one_box(tier, i)     # 포켓 → 선반 (역순: 픽 시점과
                                                  #  동일한 점유 상태 재현)
```
- SPACE 단계 진행(기본) / `--auto` 무정지 / `--no-vision` 지원.

### 2.6 Dispatcher (`main.py`)

- `locate('shelf')` 스텁 → tier별 `capture_shelf` 호출(1회 캐시, tier 바뀌면 재캡처).
- `box_idx` 0..7: tier = 1 + idx//4, i = idx%4. MCS `TargetID.IN → 'shelf'` 매핑 불변.

## 2.7 sim 검증 중 발견/수정된 사항 (구현 노트)

- **선반 높이 복원 (0.685/1.30 → 0.90/1.40)**: 낮춘 tier-1(0.685)은 깊은 저층
  insert에서 **상완(Link2)이 tier-2 보드(1.30)에 걸려** 어떤 통상 IK 브랜치로도
  진입 불가 (43 mm 못 미침, 통과 브랜치는 관절이 크게 감긴 기형 자세뿐).
  AGV를 더 붙이면 이번엔 pregrasp 수직 코리도가 cube(전면 y 0.183)에 막힘.
  → 요구사항 3의 "높이 조절 허용"에 따라 6월 검증 높이(0.90/1.40)로 mesh 복원
  (`blender/shelf/meshes/*.dae`의 board_2/3 노드 z만 수정).
- **CBiRRT 접근 spoke → 직선 코리도**: hub→pregrasp를 tool-down CBiRRT로 잇는
  기존 방식은 저층 목표에서 시간 내 연결 실패. 대체: J6 정렬(방위) → 수직
  (`TRANSIT_TCP_Z` 창: cube 상단 1.01 위, tier-2 보드 하면 아래) → 수평 →
  수직. 전 구간 pre-flight(박스 팬텀 ON) + 충돌 게이트, 실행도 결정론적.
  grasp 방위는 박스 대칭(φ, φ+π) 둘 다 시도.
- **depth QoS**: sim gazebo 플러그인은 depth를 RELIABLE로 발행하는데 노드의
  BEST_EFFORT 구독은 ~97% 드랍 → PnP 폴백(결정론적 cm급 오차, spread 게이트
  통과!)으로 전락. RELIABLE 구독을 병행 추가(실물 best-effort와 공존).
  시퀀스 게이트도 upright 해(qx=qy=0)만 수용하도록 보강.

- **태그 텍스처 180° 규약**: OpenCV `drawMarker`(DICT_APRILTAG_36h11) 출력은
  공식 AprilTag 이미지와 **180° 회전** 관계. 기존 wirebonder 텍스처(공식 svg 유래)에
  맞춰 `R_CV_TO_SDF`가 pin 되어 있으므로, 선반 텍스처도 cv2 생성 후 180° 회전해
  저장했다. 안 맞추면 shelf pose가 yaw 180° 뒤집힘 (sim에서 실측 확인).
- **`plane_scale=1.0` (shelf 전용)**: depth 평면 피팅의 기본 x2 확장은 태그 주변이
  큰 평면(wirebonder 전면)일 때 가정. 선반 placard는 보드 모서리에 서 있어 확장
  영역에 다른 깊이(보드 아래/뒤 박스)가 섞여 normal이 기울고 depth-upright 경로를
  못 탄다 → shelf 솔브만 태그 자체 면적으로 축소.
- **캡처 포즈는 카메라 기하로 유도** (`capture_pose_world`): 카메라(광축=Link6 +x,
  마운트 0.071/−0.007)를 태그 앞 `CAPTURE_STANDOFF`/`CAPTURE_RISE`에 두고 광축이
  태그 중심을 지나게 TCP 포즈를 역산. wirebonder식 base-frame pitch 공식은 이
  주차 방향(베이스 yaw 180°)에서 IK 불능이라 폐기.
- 검증 정확도: tier1 캡처에서 shelf pose (0.799, 0.500, yaw 0.0°) — 참값 대비 ~1 mm.

## 3. 실물 로봇 전환 시 (docs/real_robot_transition.md에 연결)

- 마커: 36h11 ID 2/3, 30 mm 무광 인쇄, 각 단 보드 앞모서리(박스 행 왼쪽)에 부착.
  **인쇄/부착 방향 주의**: 공식 AprilTag 이미지 방향 기준(`R_CV_TO_SDF` pin과 동일).
  sim 텍스처(april_36h11-2/3.png)를 그대로 인쇄하면 방향이 맞는다.
- `shelf_vision.py`의 태그 pose(선반 frame)를 실측으로 교체 — 코드 다른 곳은 불변.
- 캡처 포즈: `capture_pose_world`가 계산하지만, 실물에서 확인 후
  `CAPTURE_JOINTS`에 pin 권장 (wirebonder `CAPTURE_A_JOINTS`와 같은 이유).
- 실물 D405의 depth 품질에 따라 placard 면적(37.5 mm)이 평면 피팅에 부족하면
  placard를 키우거나 `PLANE_RESID_M` 조정.
- hand-eye cal 선행 필수(기존 문서 B 항목과 동일) — `CAM_R_LINK6`/`CAM_T_LINK6`
  (shelf_pick_place.py)도 cal 결과로 교체.

## 3.5 최종 검증 상태 (2026-07-11, sim)

**검증 완료** (로그로 직접 확인):
- ArUco 캡처: tier1/tier2 태그 검출 + shelf pose 해 **~1 mm 정확** (참값 (0.8, 0.5, 0) 대비 (0.799, 0.500, yaw 0.000)).
- **tier-1 4박스 왕복 완주**: 선반→포켓 4회 + 포켓→선반(put-back 역재생) 4회 전부 성공.
- **tier-2 4박스 왕복 완주** (어태처 패치 후 `--tier 2` PASS: 캡처 + 픽 4 +
  put-back 4). AGV는 tier-2 작업 시 선반에 +0.10 m 접근 주차, 외곽 2박스는 대상
  박스 앞 +0.0905 m 박스별 주차 (`TIER2_PARK_DY`/`PARK_AHEAD_X`,
  shelf_pick_place.py).
- dispatcher(main.py) selftest 통과; `IN START` 경로는 시퀀스 함수 공유로 동작 동일.

**dispatcher(`IN START`) 경로 검증 (2026-07-11 저녁)**:
- 증상(첫 상자 운반 중 base 충돌·전방 쏠림)의 원인 3가지를 수정 후,
  `IN START` ×4로 **tier-1 4박스 연속 완료, 어태처 타임아웃 0건, 종료 시 base
  피치 0.001 rad(평평)** 확인:
  1. **IFRA link-attacher gzserver 크래시**: 서비스 스레드에서 CreateJoint/
     RemoveJoint가 물리 루프와 경합 → ~10-20분마다 boost::shared_ptr 널 참조
     SIGABRT (그 전조가 ATTACH/DETACHLINK 무응답). → 조인트 생성/제거를
     **물리 스레드(WorldUpdateBegin) 큐**로 이관 + CreateJoint 널 가드
     (`IFRA_LinkAttacher` 소스 패치, colcon rebuild 필요).
  2. **box_l2c**(런치가 +0.177 포켓에 스폰, 모델 외 물체) → shelf 미션 LOCATE에서
     자동 삭제 (`clear_pocket_stowaway`, sim 전용).
  3. **주차/표류 보정 부재** → 테스트의 주차 로직을 `shelf_pick_place.py`로 이관해
     dispatcher도 공유 (`AGV_PARK`, 실물에선 False로 — AMR이 위치 담당).
  4. (보너스) place가 박스를 포켓 바닥에 5 mm 눌러 넣던 것 수정
     (`PLACE_TCP_ABOVE_HUB` +5 mm) — base를 매 사이클 아래로 재끼던 힘 제거.

**잔여 리스크**:
- tier-1 box 4(우측 외곽)는 브랜치 마진이 얇아 가끔 재시도 필요(재시도 3회 내 대부분 성공).
- Gazebo 장기 세션에서 ATTACHLINK/DETACHLINK 서비스·gzserver가 간헐 행업/사망 →
  `_wait_future` timeout으로 가시화되고, put-back 중 magazine DETACH 실패 시
  박스를 놓고 안전 복귀하도록 가드 추가됨. 행이 보이면 sim 재시작이 정답.
  `kill -USR1 <pid>`로 테스트 스크립트 스택 덤프 가능.
- tier-2 접근 주차(+0.10 m)와 박스별 주차는 실제 AMR의 "대상 앞 정차" 규칙으로
  이관될 파라미터 (MCS 미션 설계에 반영할 것).

**이번 라운드에서 잡은 sim 물리 함정들** (코드 주석에도 기록):
- 정지 박스 위 ATTACHLINK(접촉+고정조인트) → AGV 발사. → **공중(무접촉) attach**로 변경.
- 고정된 박스 주변에서 1 mm 유격 패드 개방 → 토크 싸움 → AGV 발사.
  → **패드를 박스 위로 35 mm 올린 뒤 개방**.
- 포켓 칸막이가 URDF에서 visual 전용 → attach 없인 박스가 팔 진동에 밀려 낙하.
- 팔 반동으로 프리휠 AGV가 사이클당 수 cm 표류 → **폐루프 재주차**(±5 mm)로 해결.
  put-back 재생은 기록 시점 base pose 전제.

## 3.7 물리 접촉 감사 (2026-07-12, tools/world_watchdog.py)

"시퀀스가 성공해도 상자를 쳤는지 알 수 없다"는 공백을 메우기 위해 sim 참값
감시자(`tools/world_watchdog.py`)를 도입: 8박스+로봇 pose를 0.5초 폴링,
`FLOOR`(낙하)/`TIPPED`(전도)/`MOVED`(밀림)/`TILT`(베이스 기울어짐)를 epoch
타임스탬프로 기록 → 시퀀스 로그와 대조하면 가해 동작이 특정됨. **이후 PASS의
정의는 "감시자 무사고 완주"**.

감사로 특정·수정한 뿌리 원인 (시간순):
1. **planar_move 자세 래칫**: 플러그인이 매 틱 각속도 x/y·선속도 z를 0으로
   덮어써 롤/피치/z가 복원 없이 임펄스만 누적(중력 on/off 무관, 픽 1사이클에
   0.1~0.2 rad). → 주차 시 **월드 고정**(`_freeze_base`: 링크어태처로
   mpo↔ground_plane, gravity-off라 접촉 0) + 레벨 텔레포트(`_level_base`).
   베이스 자세 이제 ~1e-6 rad 유지.
2. **rotate_j6 방향 플립**: +90°가 충돌 판정이면 조용히 −90° — pregrasp에 구운
   측면 오프셋(+90 가정)이 반전돼 든 박스가 TCP 반대편 0.28 m에 매달려 이웃
   기둥을 관통(t1c 1.3 m 발사). → 플립 없는 단방향 트위스트(충돌 시 abort).
3. **어태처 조인트 = 자유 힌지**: revolute + 한계 0 + `SetEffortLimit(0,0)` →
   축 유지력 0, 든 박스가 진자처럼 20~90° 스윙하며 마진 잠식. → `fixed`
   조인트로 교체(플러그인 재빌드).

**잔여 (문서화된 한계)** — 제로 마진 행 기하: 조 오프셋 0.14 + 이웃 반폭
0.0405 ≈ 피치 0.181 → 하강/클로즈 시 조 날이 이웃 면과 ~0 mm 간격, 이웃이
2~5 cm 밀림(직립 유지). 밀린 박스를 다음에 레이아웃 좌표로 집으면 ±15 mm 패드
여유 초과 → 그 박스를 넘어뜨릴 수 있음(감사로 재현). 해법 두 가지:
(a) **픽별 AI 박스 탐지**(계획된 `box_center_world` seam) — 밀림을 흡수하고
"넘어진 박스"도 탐지 실패로 걸러짐. (b) 실물 선반의 박스 피치 확대(마진 확보).
실물 참고: 래칫/진자/발사는 sim 전용 아티팩트(실물 AGV 질량·서스펜션, 실물
그리퍼는 강체)지만 **제로 마진 브러시는 실물에도 유효한 설계 이슈**.

## 4. 검증 절차

1. `./kill_sim.sh` → `./run_mpo700_cr7.sh` (real D405 드라이버와 동시 실행 금지).
2. 터미널 A: `/usr/bin/python3 vision/wirebonder_vision_node.py`
   (`ros2 topic echo /vision/shelf_pose` 로 (0.8, 0.5, yaw 0) ± mm 확인)
3. 터미널 B: `/usr/bin/python3 sequences/test_shelf_cycle.py [--auto] [--no-vision]`
   — tier1 4개 왕복 → tier2 4개 왕복, 총 16 pick + 16 place.
4. dispatcher 경로: `./run_test.sh` + `comms/mcs_server.py`에서 `IN START` 반복.

## 5. 튜닝 노브 (sim에서 조정)

| 노브 | 위치 | 값 | 증상 |
|---|---|---|---|
| `CAPTURE_STANDOFF` / `CAPTURE_RISE` | shelf_pick_place | 0.45 / (t1 +0.25, t2 +0.10) | 태그 미검출/캡처 IK 실패 |
| `TRANSIT_TCP_Z` | shelf_pick_place | 1.20 (world) | 접근 수평 캐리가 cube/보드에 걸림 |
| `SHELF_PREGRASP_BACK` | shelf_pick_place | {1: 0.25, 2: 0.30} | pregrasp IK 실패(cube) / 박스가 보드 전면에 걸림 |
| `HUB_TCP` | shelf_pick_place | (0.33, 0, 0.32) | hub IK 실패/충돌 |
| `PLACE_TCP_ABOVE_HUB` | shelf_pick_place | 0.08 | 포켓에 눌러 넣거나 높이서 떨어뜨림 |
| 선반 보드 높이 | shelf mesh DAE (board_2/3 노드 z) | top 0.90 / 1.40 | 저층 insert에서 Link2가 위 보드에 걸림 |
