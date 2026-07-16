# 실물 로봇 전환 가이드

sim에서 검증된 것을 실제 하드웨어(D405 카메라, 실물 팔/그리퍼)로 옮길 때
손봐야 하는 곳을 모은 문서. 필요한 항목이 생기면 아래에 계속 이어서 추가한다.

---

## 0. 실행 절차 (sim vs 실물)

카메라 토픽이 통일돼(아래 A) **차이는 1번 줄뿐**이다. arm.launch.py(vision +
dispatcher)와 시퀀스는 양쪽에서 동일.

```bash
# ── 시뮬레이션 ──────────────────────────────────────────────
cd ~/dobot_ws
./kill_sim.sh                # 잔여 프로세스 정리 (realsense 드라이버도 죽음 — sim 전엔 오히려 필요)
./run_mpo700_cr7.sh          # Gazebo + 컨트롤러 + MoveIt/RViz + d405 뷰어
# 별도 터미널:
ros2 launch src/DOBOT_6Axis_ROS2_V4/launch/arm.launch.py    # vision + dispatcher
# (또는 MCS 스텁 포함 로컬 테스트: ./run_test.sh + 별도 터미널에서 comms/mcs_server.py)

# ── 실물 (D405 연결 후) ─────────────────────────────────────
cd ~/dobot_ws
./kill_sim.sh                # sim이 떠 있으면 반드시 먼저 — 같은 토픽에 발행자 2개 금지
ros2 launch realsense2_camera rs_launch.py camera_name:=d405 align_depth.enable:=true
# 별도 터미널 — sim과 완전히 동일:
ros2 launch src/DOBOT_6Axis_ROS2_V4/launch/arm.launch.py
```

실물은 위 커맨드로 **카메라까지**만 올라온다. 실물 팔 구동은 실물 브리지
(`dobot_moveit`의 `dobot_joint.launch.py` — sim 컨트롤러와 같은 action 이름 서빙)를
추가로 띄워야 하고, TF(`odom`←`base_link`←`d405_optical_frame`)가 나와야 vision이
포즈를 낸다 (→ 아래 B의 hand-eye cal). depth 후처리를 쓰려면
`filters:=spatial,temporal` 추가 (hole_filling은 아래 C 참고 — 정확도 용도 주의).

## 1. Vision (D405 depth 하이브리드) 전환

### A. 안 하면 아예 안 도는 것 (배선)

**2026-07-11 이후 배선은 sim·실물 통일 완료.** vision 노드는 실물 realsense2_camera
토픽명(`/camera/d405/...`)을 그대로 구독하고, sim gazebo 플러그인
(`cra_description/urdf/cr7_on_mpo700.urdf.xacro`)이 **같은 이름으로 발행**한다
(depth는 플러그인 remap으로 `aligned_depth_to_color/image_raw`).
**실물 전환 = Gazebo 대신 카메라 드라이버를 띄우는 것이 전부:**

```bash
# Gazebo 대신 (D405 USB 연결 후):
ros2 launch realsense2_camera rs_launch.py camera_name:=d405 align_depth.enable:=true
# → /camera/d405/color/image_raw, /camera/d405/color/camera_info,
#   /camera/d405/aligned_depth_to_color/image_raw  (sim과 동일한 이름)

# vision 노드 + dispatcher는 sim과 완전히 동일하게:
ros2 launch src/DOBOT_6Axis_ROS2_V4/launch/arm.launch.py
```

**주의 — 토픽이 같아졌으므로 sim과 실물 드라이버를 동시에 띄우면 안 됨**:
같은 토픽에 발행자가 둘 생겨 vision 입력에 sim/실물 프레임이 섞인다.
`kill_sim.sh`는 "ros2 launch" 패턴을 죽이므로 **realsense 드라이버도 같이 죽는다**
— 드라이버를 살려둬야 하면 gazebo 쪽 프로세스만 골라 kill 할 것.

검증 로그 (2026-07-11): 실물 D405 + `rs_launch.py camera_name:=d405
align_depth.enable:=true`에 vision 노드를 수정 없이 붙여 이미지/CameraInfo(왜곡계수)/
depth(16UC1, BEST_EFFORT) 수신 및 검출 루프 동작 확인. sim gazebo 플러그인 remap
(`aligned_depth_to_color/image_raw`)도 토픽 리스트로 확인.

- [x] **토픽 이름**: sim이 실물 이름으로 발행하도록 통일 (2026-07-11).
      `tools/diag_camera_geometry.py`, `debug/view_d405.py`도 같은 이름 사용.
- [x] **depth 인코딩**: `read_depth()`가 32FC1(sim, m) / 16UC1(실물, mm) 모두 처리.
- [x] **depth QoS**: 구독이 BEST_EFFORT — 실물 realsense의 best-effort 발행과 매칭
      (reliable 구독이면 아예 매칭 안 됨), sim의 reliable 발행과도 호환.
- [ ] **OpenCV 버전**: 코드가 4.5.4 구 API 사용(`DetectorParameters_create`,
      `solvePnPGeneric`). 실물 머신 `cv2` 버전이 다르면 이 호출이 깨짐 → 확인/맞출 것.

### B. 정확도 위해 캘리/실측 필요 (핵심 갭)

- [ ] **hand-eye calibration** ⭐: `cra_description/urdf/cr7_on_mpo700.urdf.xacro`의
      `d405_joint origin xyz="0.071 0 0.147" rpy="0 0 0"`은 명목값(추측).
      실물 장착 오차 때문에 아루코 10~20 포즈로 hand-eye cal(AX=XB) 수행 →
      실측값으로 이 origin 교체. 안 하면 depth·two-view 좌표가 **똑같이 편향**됨.
      (two-view는 hand-eye cal이 아님 — 삼각측량은 hand-eye가 이미 맞다고 가정함.)
- [x] **카메라 왜곡(distortion)**: CameraInfo의 D 계수를 `detect_tag`(PnP)와
      depth 역투영(`cv2.undistortPoints`) 양쪽에 전달 (2026-07-11). sim은 D=0이라
      동작 불변; 실물은 자동으로 보정됨.
- [ ] **디바이스 모델 기하 실측**: `vision/wirebonder_vision.py`의 `SLOT_OFFSET`,
      `TAG0_XYZ`, `TAG0_RPY`는 sim `model.sdf` 기준. 실물 wirebonder/매거진의 태그
      부착 위치·슬롯 간격이 다르면 재측정해 이 상수 교체. `R_CV_TO_SDF`(sim SDF 기준
      empirical pin)도 실물 태그 장착 방향이 SDF 가정과 같은지 확인.
- [ ] **물리 태그**: `TAG_SIZE_M=0.03` → 실물 인쇄 태그가 정확히 30mm여야 함(아니면 수정).
      딕셔너리 `DICT_APRILTAG_36h11` 일치, 무광 인쇄, 평면 부착.

### C. 튜닝 (실물서 값만 조정)

- [ ] **`_depth_valid` 임계값**: 유효 픽셀 비율 / 예상 거리 창. sim은 완벽해서 안 걸림
      → 실물서 실측으로 잡음.
- [ ] **평면 피팅 knob**: `PLANE_SCALE`(태그 주변 확장 배율, 기본 2.0 — 태그가 붙은
      면이 평평하지 않으면 줄일 것), `PLANE_RESID_M`(평면 밖 기각 잔차, 기본 5mm).
      aruco_test_gemini.py(실물 D405 단독 테스트)에서 검증된 값이 기본값.
- [ ] **realsense 후처리**: temporal/spatial 필터 on. hole-filling은 추정값이라
      정확도용으론 주의.
- [ ] **조명/작업거리**: min range ~7cm 밖, 충분한 조명, 광택 없는 표면.

### D. 실물엔 없어지는 것

- [ ] **`DEVICES_GT` 검증**: sim은 device의 odom 참값을 알아 vis-vs-gt 프린트가 되지만
      (`wirebonder_vision_node.py`), 실물엔 ground truth가 없음. 그 side-by-side 검증은
      sim 전용 → 실물 검증은 "잡은 pose로 실제 집히나"로 대체.
- [ ] **`grasp()`의 Gazebo 모델명 해석 게이트** ⭐ (`wirebonder_pick_place.py::grasp`,
      shelf도 동일): `pockets.model_at`(`/gazebo/model_states` 최근접 모델)은 IFRA
      ATTACHLINK용 **sim 전용** 심인데, 현재 해석 실패 = 픽 거부로 짜여 있어 실물에선
      (`model_states` 부재로) **모든 픽이 거부됨**. 실물 전환 시: model_states 부재면
      해석을 건너뛰고 그리퍼를 닫은 뒤 **그리퍼 피드백(위치/전류)으로 grip 검증**으로
      대체. 실패 보고는 기존 `ErrorCode.ATTACH_FAILED` 재사용 (프로토콜 추가 불필요).
      sim에서 겪은 스폰-vs-정착 높이류 실패(2026-07-15 seq2)는 실물엔 없는 부류.

## 2. 충돌 계층 (collision layer) 전환

**방식 자체는 sim 그대로 간다** — 프리미티브 팬텀 + 태그 로컬라이제이션은 구조화된
셀의 표준. 등록은 "정확한 좌표"가 아니라 **측정 가능한 프레임 하나(ArUco 선반 포즈)
+ CAD 레이아웃(`SHELF_BOX_XS` 등) + 오차만큼의 팽창**으로 한다. 실물이 팽창된 팬텀
안에 있다는 것만 보장되면 계획은 안전하다. (원리·수정 이력:
`shelf_pick_collision_fixes.md`)

### A. 안 하면 위험한 것

- [ ] **`STOCK_SHRINK` 음수(팽창) 설정** ⭐ (`cr7_pnp/node.py::_add_shelf_stock`):
      sim은 0(실치수)이지만 실물 팬텀 위치는 측정값이라 오차가 얹힌다. 팽창량은
      감이 아니라 오차 예산 합산으로:
      `태그 검출(~1-3mm) + hand-eye cal 잔차(~2-5mm) + FK(~1mm) + 태그↔상자
      레이아웃 공차(실측)` → 합(또는 RSS)을 반영. 상자 간 빈틈이 ~100mm이므로
      오차 합이 그 절반을 넘으면 플래너가 아니라 **측정을 개선**할 것
      (더 가까운 캡처, 태그 추가, 재캘리).
- [ ] **레이아웃 공차 실측**: `SHELF_BOX_XS` / `SHELF_TIER_TOPS` / `SHELF_TAG_XY`는
      sim 모델 기준. 실물 선반의 태그 부착 위치·칸 간격을 실측해 도면과의 차이를
      위 오차 예산에 넣거나 상수를 교체.
- [ ] **팔 내장 충돌 감지(전류 기반) 활성화**: 실행 중 안전은 플래너 몫이 아니다.
      계획 시점 검사(지금 방식) / 실행 중 감시 / 인증 안전장치(팔 충돌 정지 +
      MPO-700 안전 라이다)는 별개 계층 — 비전·옥토맵은 인증 안전장치가 될 수 없음.
- [ ] **그리퍼 실측 재확인**: `FINGER_OPEN_M`·`JAW_FIXED_PAD_X` 등이 실물 그리퍼
      스트로크와 다르면 충돌 모델이 실물보다 작아진다(sim에서 30mm 사고의 원인).
      모델은 항상 실물을 포함해야 함.
- [ ] **D405 충돌 지오메트리 복원**: `cr7_on_mpo700.urdf.xacro`의 카메라 collision이
      sim 편의로 주석 처리돼 있음 — 실물은 카메라 몸체 보호를 위해 복원 검토.

### B. 알고 분리할 것

- **그래스프 잔여 오차는 회피 예산과 별개**: 회피는 상한만큼 불리고, 잡기는 캡처
  범위(조 111mm vs 상자 81mm = ±15mm)와 그래스프 직전 근접 캡처(refinement)로
  흡수한다.
- **OctoMap은 지금 셀엔 불필요**: 파크-앤-픽 + 알려진 구조물이라 look-then-move
  가정이 성립. 미모델 잡동사니(사람 손, 임시 적재물)가 등장하는 환경이 되면
  "아는 물체는 프리미티브 + 나머지만 옥토맵 + **그래스프 존은 옥토맵 제외**"
  하이브리드로 추가 (복셀 해상도가 여유를 잡아먹어 100mm 틈새 insert가
  옥토맵으로는 항상 충돌 판정 남).

---

## OLD_DEVICE_POSE 재캡처 (파이프라인/센서/capture 포즈 바뀔 때마다)

`wirebonder_pick_place.py`의 waypoint(SLOT_WORLD)는 `OLD_DEVICE_POSE`에 재anchor돼,
런타임에 vision이 읽는 device pose와 합성됨. 상쇄가 성립하려면 **OLD = 그 vision
파이프라인이 고정 capture 포즈(`CAPTURE_A_JOINTS`)에서 읽는 값**이어야 함. 따라서:

- **vision 알고리즘**(예: two-view→depth-upright), **센서**(sim→실물 D405), 또는
  **capture 포즈**를 바꾸면 → 그 조건에서 device를 한 번 캡처해 `OLD_DEVICE_POSE`
  (+ `DEVICES['wb1']`)를 그 read로 **재캘리**. SLOT_WORLD는 건드리지 않음.
- 현재(sim depth-UPRIGHT, 2026-07-10 이후): 위치+yaw 모두 depth에서 직접 구성,
  PnP 회전 미사용 (`docs/vision_viewpoint_dependence_fix.md`). read가 **뷰포인트
  불변**(5개 config에서 spread <1 mm)이므로 캡처 포즈/AGV park가 조금 달라져도
  read는 같은 값 — 재캡처는 파이프라인/센서 변경 때만 실질적으로 필요.
- 실물: hand-eye cal 후 재캡처. `CAPTURE_A_JOINTS`도 실물 팔에서 태그를 잘 보는
  (가급적 fronto-parallel) config로 재조깅 권장. 브링업 검증은
  `tools/diag_camera_geometry.py`의 다중 뷰포인트 일관성 체크(GT 비교 제외)로.

## 환경 노트 (sim/실물 공통)

- **시퀀스 실행 python**: `.venv`(numpy 2.2.6)로 `wirebonder_pick_place.py`를 돌리면
  `/opt/ros/humble`의 pinocchio(numpy 1.x 빌드)와 충돌해 **segfault**. system python3
  (`/usr/bin/python3`, numpy 1.21)로 돌리면 정상. vision 노드도 system python3 사용.
  (conda 활성 시 PATH에서 벗겨야 함 — `run_mpo700_cr7.sh`가 그 처리를 함.)

- **`ROS_LOCALHOST_ONLY=1` 고정** (2026-07-15): 이 셀은 ROS 그래프 전체가 한 대에서
  돈다(MCS 통신은 자체 TCP, 실물 팔 브리지도 로봇 컨트롤러와 자체 TCP — 둘 다 DDS
  아님 → 무관). `=0`이면 DDS discovery가 모든 네트워크 인터페이스로 나가는데, sim
  재기동 직후 수 분간 신규 노드의 엔드포인트 매칭이 ~1/10로 늘어져 **첫 trajectory
  goal 전송이 20초 타임아웃**되는 사례를 실측함 (`Trajectory goal send timed out`
  → `Could not reach the hub`; 몇 분 뒤 자연 회복). `~/.bashrc` +
  `run_mpo700_cr7.sh`/`run_mpo700_cr10.sh`/`teleop_agv.sh`에 `export
  ROS_LOCALHOST_ONLY=1` 반영 완료. **모든 터미널이 같은 값이어야 함** (bashrc 반영
  전에 연 터미널 주의). 다른 PC에서 ros2 토픽을 봐야 할 때만 해제.
  (goal 전송 타임아웃의 **근본 원인은 아래 UDP 버퍼**였음 — `=1`은 /22 사내망
  discovery 유입을 끊어 그 부하를 줄이는 보조 조치로 유지.)

- **커널 UDP 버퍼 확대 필수** (2026-07-15, 실물 PC 새로 세팅할 때도 동일): 우분투
  기본 `net.core.rmem_max`(208 KB)로는 gzserver급 대형 참가자(엔드포인트 수백 개)의
  DDS discovery 버스트가 커널에서 드롭됨 — `netstat -su`의 "receive buffer errors"
  94만+ 누적, 신규 클라이언트 서비스 콜 1회당 +4천 드롭 실측. 결과: **새로 시작한
  노드만** 액션/서비스 매칭이 복불복 → `Trajectory goal send timed out (20s)` 간헐
  재발 (이미 붙어 있던 노드는 멀쩡 — "shelf는 되는데 새로 띄운 wirebonder만 실패",
  재부팅 직후엔 그래프가 가벼워 정상 — 그래서 오래 미궁이었음). 해결:
  `/etc/sysctl.d/60-ros2-dds.conf`에 `net.core.rmem_max=67108864`,
  `net.core.rmem_default=8388608`, wmem 동일 → `sudo sysctl --system` →
  **sim 재기동**(기존 소켓은 작은 버퍼를 유지하므로). 검증: 카운터가 더 안 늘고
  새 프로세스의 첫 goal이 즉시 수락. 진단 트릭: 현재 관절값 그대로의 hold goal을
  `ros2 action send_goal`로 보내면(무동작) 어느 셸에서든 클라→서버→결과 전 구간을
  안전하게 테스트할 수 있음.

- **Fast DDS wide-probe 프로파일 필수** (2026-07-15, goal 타임아웃의 3번째 층):
  `ROS_LOCALHOST_ONLY=1`의 로컬 참가자 탐색은 기본으로 127.0.0.1의 **앞쪽 4개
  참가자 슬롯만** 두드림(`maxInitialPeersRange=4`). 이 셀은 참가자 9개+라
  컨트롤러가 뒤쪽 슬롯에 배정된 세션에서는 신규 노드가 액션 서버를 찾는 데
  8~20초(재기동마다 슬롯 추첨이 달라져 복불복) → 20초 예산과 겹쳐 간헐 타임아웃.
  해결: `~/dobot_ws/fastdds_localhost.xml`(probe 64 + SHM/UDP 유지)을
  `FASTRTPS_DEFAULT_PROFILES_FILE`로 로드 — bashrc + run 스크립트들에 반영 완료.
  A/B 실측: 신규 클라이언트 goal 왕복 8.7~12.3 s(들쭉) → 5.9~6.2 s(일정, 클라이언트만
  적용 시; 서버까지 재기동하면 더 단축). 이 값도 **모든 터미널 공통**이어야 함.

<!-- 이후: 그리퍼 교체 시 재조정 파라미터 등 이어서 추가 -->

- 로봇팔에 충격이 가해졌을때 물건을 놓거나 집으려 닿은건지 아니면 다른 장애물에 부딫힌건지 단계 설정해야함, 후자일경우 error 처리에 넣을것

- 선반의 상자, wirebonding 장비의 접근 위치 모두 aruco 상대위치값을 로봇 jog로 뽑아낸다음 mcs 서버에 저장하도록할것(명령으로 받아서 수행)