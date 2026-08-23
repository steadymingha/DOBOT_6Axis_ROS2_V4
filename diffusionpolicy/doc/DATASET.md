# Isaac Sim 선반 Pick-and-Place 전문가 데이터셋 (diffusion policy)

수집 완료: **총 332 에피소드 / 33GB** (`data/isaac_shelf/episodes/episode_0000.npz` ~ `episode_0331.npz`)
— 2층 보드(1.22m) 200개(2026-07-24) + **1층 보드(0.72m) 132개(2026-07-28, ep 200–331)**.
1층 확장의 전 과정은 `doc/LOWBOARD_PROGRESS.md` 참고.

전문가 = 검증된 스크립트 파이프라인(`sequences/shelf_pick_place.py`의 hub-and-spoke:
pre-flight 스포크 + guarded place). 수집기 = `diffusionpolicy/isaac_collect.py`,
시뮬 = `isaac/isaac_sim.py` (Isaac Sim 4.5, `--env fab --headless`).
태그/비전 단계는 사용하지 않음(시뮬 스폰이 정확하므로 `SHELF_WORLD_POSE` 레이아웃 사용).

## 에피소드 정의

**1 에피소드 = 1 rollout = 박스 1개**: 허브에서 출발 → 선반 박스 1개 파지 → 베이스
포켓에 안착 → 허브 복귀. AGV 재파킹·리셋 구간은 녹화 제외.

## npz 키 (에피소드당)

| 키 | shape | 내용 |
|---|---|---|
| `agentview_image` | (T,240,320,3) u8 | canonical 카메라 (AGV **베이스 마운트**, 640×480 렌더 → 리사이즈) |
| `robot0_eye_in_hand_image` | (T,240,320,3) u8 | d405 손목 카메라 (848×480 → 리사이즈) |
| `robot_eef_pose` | (T,7) f32 | eef pose **base_link 프레임** (xyz + quat xyzw, FK) |
| `gripper` | (T,1) f32 | 그리퍼 명령값 |
| `object` | (T,7) f32 | 대상 박스 pose, base_link 프레임 (model_states + TF) |
| `action` | (T,8) f32 | **절대** eef pose(7) + gripper(1) = t+1 시점의 실행 궤적 |
| `meta_*` | – | box 모델명/인덱스, pocket, station, AGV (x,y) — **ep 68+만 존재**. `meta_board_top`(픽 보드 높이, f32)은 **ep 200+만** — 키 없으면 1.22로 간주 |

- 샘플링 `CONTROL_HZ = 10` (시뮬 시간 기준). 에피소드 길이: ep 68+ T=470~705(중앙값 542),
  ep 0–67 T=891~1595(중앙값 1042, 구버전 수집기가 느림).
- obs/action 모두 **BASE 프레임** → AGV 파킹 변화가 정책에 "물체 포즈 변화"로 보임.
- lowdim 학습(eef/gripper/object)과 image 학습이 같은 npz를 읽음.

## 수집 구조 (패스 방식, ep 68+)

1. 리셋: tier-1 박스 10개 전부 선반 스폰 복원, tier-2는 각 50% 확률로 화면 밖 스태시
   (충돌 팬텀도 함께 파킹 — 물리·화면·플래너 일치)
2. tier-1 박스 10개를 전 스테이션에 걸쳐 **랜덤 순서**로 pick-place
   - AGV는 대상 박스가 속한 스테이션 앵커(클러스터 중심 −0.117m, 검증된 파킹 기하)
     ± x 5cm / y 3cm 노이즈로 텔레포트 → 이것이 데이터 다양성의 핵심
   - 포켓은 high-y 끝에서부터 **연속 채움**(성공 횟수 기준, 스킵돼도 구멍 없음)
3. 포켓 4개가 차면 **베이스만 비움**(실린 박스를 스태시로 = "배달"), 선반은 계속 소진
   → 선반 상태 10→1개까지 전 구간이 데이터에 포함됨
4. tier-1 소진 시 패스 종료 → 1로 (선반 재충전)

레이아웃: 선반 티어당 박스 10개(`box_t{1,2}{a-j}`, 피치 0.181m — 그리퍼 개방 침범
~52mm 대비 검증값), 스테이션 = x순 4+4+2. 베이스 포켓 4개.

## 품질 게이트 (모두 통과해야 저장)

1. **pre-flight** (모션 없음): 스포크/서보/복귀 전부 충돌·IK 검증, 실패 시 재파킹 1회 후 스킵
2. **box_in_pocket**: 박스가 목표 포켓 중심 ±6cm(xy)/±5cm(z) 내 안착 확인
3. **shelf_undisturbed**: 나머지 선반 박스 전부(양 티어, 스태시 제외)가 스폰 위치
   ±2cm(xy)/3cm(z) 이내 — 위반 시 에피소드 폐기 + 선반 즉시 리셋 (오염 격리)
4. **QC 스냅샷**: `data/isaac_shelf/snapshots/ep_NNNN_<box>_{carry,place}.png`
   (허브에서 박스 든 순간 / 안착 후) — headless 수집의 육안 검수용

## 수집 결과 지표 (최종)

| 지표 | 값 | 비고 |
|---|---|---|
| 저장 에피소드 | 200 (목표 달성, 21GB) | 마지막 실행에서 132개, ~1.5분/에피소드 |
| 오염 폐기 | 4 | 게이트가 감지·격리 (첫 건: box_t1i +28mm) |
| 박스 스킵 | 18 | pre-flight 거부(모션 없음), 데이터 무영향 |
| settle 경고 | 29 | 궤적 종점 수렴 지연 경고 — 실패로 이어진 건 없음, 관찰 항목 |

## 데이터 계보 (섞어 쓸 때 주의)

| 구간 | 차이 |
|---|---|
| ep 0 | **이미지 84×84 + 카메라 구도가 이후와 다름(원경 와이드)** — image 학습에서 제외 |
| ep 0–67 | meta 없음, 포켓 비연속 채움 가능(루프 인덱스 버그), 선반 항상 만재, 사이클마다 풀 리셋, 렌더 60Hz, T ~2배 김 |
| ep 68–199 | 패스 구조 + 연속 포켓 + 점유 랜덤화 + meta 키, 렌더 30Hz(데이터 무영향) |
| ep 200–331 | **1층 보드(0.72m) 픽 132개 (완료 2026-07-28)** + `meta_board_top` 키. 박스가 base 프레임에서 ~0.5m 낮게 보임 (obj z +0.28 → -0.22). 파킹: ep 200–229는 구앵커(x_off −0.117, y −0.15), ep 230+는 캘리브레이션 앵커(**x_off −0.155, y −0.160**) ± 노이즈 — 분포가 이중봉(다양성엔 유리). QC 통과(2026-07-28 전 구간 재실행): 신규 문제 0건 |

- **lowdim 학습**: 전 구간 혼용 가능 (obs에 포켓 점유가 없어 차이 무해)
- **image-hybrid 학습**: 포켓 선택/선반 점유까지 학습시키려면 **ep 0–67 제외 권장**
  (meta 키 부재로 판별: `'meta_box' not in np.load(f).files`)
- SPEED_SCALE=1.25, 카메라 구도, IMG 240×320은 전 구간 동일

## 배포/eval 시 불변량

반드시 일치: `CONTROL_HZ=10`, BASE 프레임 규약, 카메라 포즈·FOV·해상도
(canonical=베이스 마운트, `CANON_EYE/TARGET` in isaac_sim.py), `physics_dt=1/240`(시뮬 eval),
이미지 전처리(240×320), `--env fab` 배경.
무관: `rendering_dt`(≥10Hz면 등가), headless 여부, SPEED_SCALE(수집 전용).

주의: 정책에는 도달성 개념이 없음 — AGV가 학습 파킹 엔벨로프 밖에 정차하면 미정의
동작. 배포 시 미션 계층에서 시작 게이트(박스 상대 포즈가 학습 범위 내인지, meta_agv_xy로
엔벨로프 산출 가능) + 실행 중 기존 충돌/한계 검사를 유지할 것.

## npz 품질 점검 (2026-07-27 수행, 전 200 에피소드)

점검 스크립트: `diffusionpolicy/npz_qc.py` — 전 에피소드를 순회하며
키/shape/dtype 일치, T 키 간 길이 일치, NaN/Inf, 쿼터니언 노름, action[t] ↔ eef[t+1]
정합(절대 action 규약 검증), 틱당 eef 점프(>0.15m 플래그), 그리퍼 범위, 박스 이동량
(<5cm = 파지 실패 의심), 이미지 해상도 + 검은/정지 프레임(서브샘플 8프레임 std)을
검사하고 `qc_report.txt` + 계보 경계 4개 ep의 `qc_contact_sheet.png`(육안 검수용)를 남긴다.

```bash
/usr/bin/python3 diffusionpolicy/npz_qc.py   # ~3분 (21GB 풀 스캔)
```

결과 (`data/isaac_shelf/qc_report.txt`, `qc_contact_sheet.png`):

- **통과**: NaN/Inf 없음, 쿼터니언 정규화 완벽, action=eef[t+1] 정합 오차 0
  (각도 최대 0.001rad), 키 간 길이 전부 일치, 검은/정지 프레임 없음, 그리퍼
  [0.002, 0.015] 에피소드당 닫기+열기 2회, 박스 이동 중앙값 0.70m (전 ep 이동 확인)
- **ep 0**: 이미지 84×84 + 다른 카메라 구도 — lowdim은 사용 가능, image 학습 제외
- **ep 74, 192**: pre-grasp 접근 중(에피소드 13% 지점, 그리퍼 열림) 연속 두 샘플 사이
  eef가 0.19m/0.18m — 로봇 문제가 아니라 **기록 틱 누락**: `_tick` 내 TF lookup이
  최대 0.2s 블로킹(isaac_collect.py:125)하면 rclpy 타이머가 놓친 슬롯을 건너뛰어
  0.2~0.3s 간격이 연속 틱처럼 저장됨. 파지·안착은 정상. 학습에 치명적이진 않으나
  image 학습에서 빼려면 이 2개 제외

## 실행 방법 (재수집/추가수집)

```bash
# 1) 시뮬 (headless, 분리 실행)
cd ~/dobot_ws && nohup setsid ./run_mpo700_cr7_isaac.sh --env fab --headless > isaac_run.log 2>&1 &

# 2) 수집기 (기존 파일 수부터 이어서 저장)
source /opt/ros/humble/setup.bash && source ~/dobot_ws/install/setup.bash
nohup setsid /usr/bin/python3 -u diffusionpolicy/isaac_collect.py --episodes 200 \
  > diffusionpolicy/collect.log 2>&1 &

# 1층 보드 수집 (--board-top은 BOARD_AGV_Y에 검증된 파킹이 있는 높이만 허용:
# 1.22=기본, 0.72=1층. 기본 파킹은 1층에서 pre-grasp IK가 전부 AGV 충돌 —
# 스윕/검증 스크립트는 diffusionpolicy/lowboard_test/)
nohup setsid /usr/bin/python3 -u diffusionpolicy/isaac_collect.py --episodes 332 \
  --board-top 0.72 > diffusionpolicy/collect_lowboard.log 2>&1 &

# 모니터링
tail -f diffusionpolicy/collect.log
ls diffusionpolicy/data/isaac_shelf/episodes | wc -l
```

## zarr 변환 (2026-07-28 수행)

스크립트: `diffusionpolicy/npz_to_zarr.py`. diffusion_policy는 pip 설치가 아니라
`/home/user/fun/diffusion_policy` repo를 sys.path에 넣어 사용 — 실행 환경은
**robodiff conda env** (`~/miniforge3/envs/robodiff/bin/python`, zarr 2.12.0, numpy 1.23.3):

```bash
~/miniforge3/envs/robodiff/bin/python diffusionpolicy/npz_to_zarr.py
```

`ReplayBuffer.add_episode()` 규약(`data/<key>` concat + `meta/episode_ends`) 그대로,
npz 키명 유지. 이미지는 repo real-data 관례대로 **Jpeg2k(level=50)**, 청크 (1,240,320,3).

| 출력 (`data/isaac_shelf/`) | 구간 | 에피소드/스텝 | 키 | 크기 |
|---|---|---|---|---|
| `isaac_shelf_lowdim.zarr` | ep 68–331 | 264 / 160,182 | `robot_eef_pose(7), gripper(1), object(7), action(8)` | 11MB |
| `isaac_shelf_image.zarr` | ep 68–331 **− 74, 192**(틱 누락) | 262 / 158,865 | lowdim 키 + `agentview_image, robot0_eye_in_hand_image` (240×320×3 u8) | 5.8GB |

검증: 두 버퍼 모두 ReplayBuffer로 재오픈 → 에피소드 read-back에서 shape/dtype 일치,
이미지 디코딩 정상(std 32, 검은 프레임 아님). ep 0–67은 계보 정책상 처음부터 미포함.

**쿼터니언 주의 (2026-07-29 발견)**: 저장된 quat(xyzw)는 tool-down 자세가 qw≈0
경계라 **±q 부호가 에피소드 간/내에서 뒤섞여 있음** (33 ep 샘플에서 intra-episode
부호 점프 206회). 회전으로는 동일해 실행·QC는 무해하지만 **회귀 타깃으로는 쌍봉**
— 그래서 학습은 zarr를 그대로 두고 dataset 클래스(`IsaacShelfImageDataset`,
`rotation_6d: True`)가 로드 시점에 **6D rotation representation**으로 변환한다
(obs pose 7→9, action 8→10; 논문/robomimic abs 관례). 부호 모호성은 6D에서 원천
소멸. eval 쪽은 policy_server.py가 6d→quat로 되돌려 ROS 클라이언트는 계속 quat 사용.

## 다음 단계

- ~~npz 품질 점검~~ (완료, 위 섹션)
- ~~zarr 변환~~ (완료 2026-07-28, 위 섹션)
- lowdim 학습 → image-hybrid 학습 (후자는 ep 0–67 필터)
