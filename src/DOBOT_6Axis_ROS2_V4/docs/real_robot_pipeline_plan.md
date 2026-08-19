# 실물 파이프라인 이행 계획 — main.py 로 실물 운전 + 부분 가상화

작성 2026-08-16. 근거는 이 저장소의 코드이고, 인용한 위치는 전부 `파일:행` 으로 적었다.
이 문서는 **무엇을 만들지**가 아니라 **왜 그 순서여야 하는지**를 남기는 것이 목적이다.

관련 그림 (`docs/`):

| 파일 | 내용 |
|---|---|
| `control_chain.svg` | 현재(as-is) 명령 흐름 — 시뮬 기준으로 설계된 원본을 실측에 맞게 정정한 것 |
| `control_chain_target_new.svg` | **목표(to-be) 구조 — 최신.** 신규 + 부분 가상화 스위치 + 충돌 환경(4.4) |
| `control_chain_target.svg` | 위의 이전판 (충돌 환경 반영 전) |
| `vision_hover_chain.svg` | `cbirrt_p1p2_test.py --vision --run` 상세 (실물 검증본) |
| `vision_hover_chain_simple.svg` | 위의 요약판 |

관련 문서: `docs/collision_model_guide.md` — 충돌 모델에 무엇을 어떻게 넣는지(운용 매뉴얼).
아래 4.4 가 그 문서의 계획 측 근거다.

---

## 진행 상태 (2026-08-17)

| 단계 | 상태 | 비고 |
|---|---|---|
| 1 `robot_feed` | **완료** | `cr7_pnp/robot_feed.py` 하나. `RealtimeMonitor`/`RobotFeed` 는 그 별칭·import. `read_feedback.py` 삭제, `check_real_robot.py` 는 `read_one()` 사용. `--monitor` 실물 확인. (1.4 의 `jog_tcp/gamepad/reachability_map` 은 실제로는 30004 파서가 아니었다 — 조이스틱 `struct` 였음) |
| 2-1 TF | **완료(코드)** | `arm.launch.py profile:=real` 이 `odom ≡ base_link` 정적 TF 발행. Jetson 컨테이너에 `neo_simulation2` 가 없어 결합 xacro 가 안 빌드 → `main.py` 는 `collision_model_xacro()` 로 **팔 단독 모델**로 폴백하고 `node.root_frame = base_link` (모델에 `mpo_base_link` 가 있을 때만 그것을 씀). 큐브·AGV 몸체는 이때 표면 슬래브가 대신한다. 열린 질문 1 은 이걸로 닫힘 |
| 2-2 fail-closed | **완료** | `update_shelf/wirebonder_collision` TF 실패 → `node._unenforced` 에 기록, `execute_path/execute_trajectory` 가 **모든 모션 거부**. `main.refresh_collision_world` 실패 → `TF_UNAVAILABLE` 로 미션 중단. TF 끊고 실물 확인 |
| 2-3 분리·JSON | **완료(코드)** | `cr7_pnp/env/{sim,real}.json` + `geometry.py`/`gripper_params.py` 가 `DOBOT_ENV` 로 로드 (sim 43개 상수 값 동일 검증). `HUB_TCP` 하나로. `cr7_pnp/collision_env.py`(적용, base_link→루트 합성) / `tools/teach_env.py`(교시). `cr7_pnp/contact.py` 에 `guarded_descend`·`ContactDetector`·`Dashboard`·`arrived` 등 이동, 테스트는 import. `test/surfaces.json` → `cr7_pnp/env/real_surfaces.json` |
| 2-3 실측 | **미완 — 사람 작업** | `real.json` 은 시뮬 사본(`"measured": false`). 실측 전에는 `--profile real` 기동 거부(`--preflight` 만 허용). 현재 값으로 `--preflight` 를 돌리면 포켓 기준 IK 가 `wall_x`(x=+0.36) 뒤라 허브 IK 에서 정상 실패한다 — 파이프라인이 제 역할을 하는 것 |
| 4.1(c) 프로파일 | **완료** | `main.py --profile sim/real [--gripper] [--preflight]`, 6.1 검사(DOBOT_ENV 대조·30004·measured·/ATTACHLINK·CR7_REAL_ROBOT 자동 설정). `--preflight` = 가상 실행(모션 미전송, 가상 팔이 경로 끝으로 점프해 시퀀스가 이어짐). launch 인자 `profile` 이 비전 노드·TF·프로파일을 함께 고름 |
| 3a `vision_hover_node` | **완료(코드)** | `vision/vision_hover_node.py`: `/vision/capture` → `vision_target.snapshot()` → `/vision/device_pose`(hover, 10 Hz). 실물에서 러너 응답까지 확인(카메라가 매거진을 안 보고 있어 스냅샷은 타임아웃) |
| 3b ARUCO / 3c 포켓 | 미착수 | `locate_shelf` 는 `d405_optical_frame` TF 를 쓰므로 실물엔 카메라 TF(핸드아이) 발행이 함께 필요 |
| 4 하강 / 5 그리퍼 | 미착수 | 팔이 움직이는 단계 — 2-3 실측 뒤 |

## 0. 한 줄 요약

**선반 시퀀스(`sequences/shelf_pick_place.py`)는 한 줄도 고치지 않는다.** 그 밑의 기반
— 충돌 상수, 비전 소스, 하강 방식, 로봇 피드백 — 만 실물로 바꾼다. 시퀀스가 이웃
매거진·선반을 안 건드리는 근거는 궤적이 아니라 **충돌 모델 상수 + 무동작 pre-flight**
이므로, 그 근거를 실물로 옮기는 것이 이 작업의 전부다.

---

## 1. 현재 상태 (as-is)

### 1.1 두 파이프라인이 따로 산다

| | 시뮬 파이프라인 | 실물 검증 파이프라인 |
|---|---|---|
| 진입점 | `main.py` (`run_test.sh` → `launch/arm.launch.py`) | `test/cbirrt_p1p2_test.py` (`test/run.sh`) |
| 오케스트레이터 | `main.py` — MCS 토픽 구동, REGISTRY 라우팅 | `run_vision()` (`:1360`) — 단일 사이클, 사람이 실행 |
| 비전 | `vision/tag_vision_node.py` → `/vision/device_pose` 계속 발행 | `test/vision_target.py` + `vision_bridge/` — 호출 시 1회 |
| 로봇 피드백 | `/joint_states` (`dobot_moveit/joint_states.py`) | `RealtimeMonitor` (`:179`) — 30004 직접 탭 |
| 하강 | 액션 경유 일반 이동 | `guarded_descend()` (`:1077`) — 접촉 정지 |
| 그리퍼 | `gripper_controller` 액션 (Gazebo) | 없음 → 3초 대기로 대체 |

### 1.2 계획·실행 계층은 이미 공통이다

`main.py` 도 `cbirrt_p1p2_test.py` 도 같은 `cr7_pnp.HubPickPlace` 를 쓴다.

- `move_group` 은 **양쪽 다 안 쓴다.** `cr7_pnp` 가 pinocchio IK + CBiRRT 로 직접 계획하고
  `/cr7_group_controller/follow_joint_trajectory` 액션 클라이언트만 물린다 (`cr7_pnp/node.py:96-99`).
  `move_group` 은 RViz 수동 조작과 `debug/wc_demo.py` 의 FK 서비스용으로만 존재한다.
- 모델은 `cra_description` 의 xacro 를 파일로 직접 로드하고 (`cr7_pnp/geometry.py:23-29`),
  자기충돌 제외 쌍은 `cr7_moveit/config/cr7_robot.srdf` 를 파일로 직접 읽는다
  (`cr7_pnp/model.py:27-29`). `move_group` 프로세스 없이 성립한다.
- 시뮬/실물 전환점은 **액션 이름 하나**다. 시뮬은 `gazebo_ros2_control` 이,
  실물은 `dobot_moveit/action_move_server.py:23` 이 같은 이름으로 서버를 연다.

→ **계획 계층은 이미 실물/시뮬 공용이다.** 이행 작업의 대상이 아니다.

### 1.3 실물에서 이미 검증된 것

`--vision --run` 이 한 사이클로 검증한 내용 (`run_vision()`, 결과는 `test/vision_hover_*.json`):

```
obs 이동(CBiRRT, 툴 자세 유지) → 비전 스냅샷 → hover → J6 정렬
  → 접촉 정지 하강 → 3초 대기 → 측정한 만큼 정확히 되올림 → 기록 경로 역재생 → 언트위스트
```

- 하강은 기본 동작이다 (`:1592` `if not a.no_descend:`). `--no-descend` 를 줘야 J6 에서 멈춘다.
  ※ 파일 상단 docstring `:76` 과 `--vision` 도움말의 "NO DESCEND" 는 **옛 문구**다. 정정 필요.
- 이 모양은 `sequences/shelf_pick_place.py` 의 place 쪽과 **같은 형태를 일부러 따온 것**이다
  (`:1602-1610` 주석). 즉 검증 대상이 처음부터 선반 파이프라인이었다.

### 1.4 30004 파서가 5벌 흩어져 있다

| 구현 | 위치 | 소비자 |
|---|---|---|
| `RobotFeed` | `handeye_calib/handeye_calib.py:94` | `vision_bridge/verify_chain.py:255` |
| `RealtimeMonitor` | `test/cbirrt_p1p2_test.py:179` | `--vision`, `--run`, `vision_target` |
| 임시 파서 | `read_feedback.py`, `tools/check_real_robot.py`, `tools/jog_tcp.py`, `tools/gamepad.py`, `tools/reachability_map.py` | 각자 |

같은 바이너리 프레임의 오프셋 상수가 다섯 군데에 중복돼 있다. 펌웨어가 레이아웃을 바꾸면
다섯 곳을 고쳐야 한다.

---

## 2. 왜 시퀀스를 고치면 안 되는가 (이 계획의 전제)

선반 시퀀스가 이웃 매거진·선반을 안 건드리는 근거는 세 가지이고, 전부 **데이터**다.

1. **선반 판 팬텀** — `cr7_pnp/node.py:490-517`. `SHELF_FOOTPRINT`, `SHELF_BOARD_TOPS`,
   `SHELF_BOARD_THICK` 로 생성.
2. **재고 박스 팬텀** — `:519-570`. 면당 **3 mm 팽창**(`STOCK_SHRINK = -0.006`). 파지 편차가
   ±3~4 mm 라 팬텀을 스치던 carry 가 실제 이웃 박스를 건드린 사례(carry #2 가 박스 #3 을 침)
   이후 넣은 여유다. 대상 박스만 `set_shelf_stock_absent` 로 잠시 치운다.
3. **무동작 pre-flight** — `sequences/shelf_pick_place.py:163-200`. 삽입 스포크 + 파지 서보 +
   트위스트 복귀를 **움직이기 전에 전부 IK 로 검증**하고, 전진 측 실패는 허브로 역재생한다.

→ **시뮬 수치를 그대로 두고 실물을 돌리면 이 보장이 옮겨지지 않는다.** 그래서 충돌 상수
실측이 다른 무엇보다 먼저다. 시퀀스 로직 자체는 이미 검증된 자산이므로 손대지 않는다.

---

## 3. 갭 목록 (실물로 옮길 때 걸리는 것)

| # | 갭 | 현재 | 필요 | 난이도 |
|---|---|---|---|---|
| G1 | 충돌 상수 | 시뮬 수치 | 실측 수치 | 측정 작업 (코드 변경 최소) |
| G2 | 선반 위치 비전 | `/vision/shelf_pose` ← tag_vision_node (ArUco) | 새 체인의 ARUCO 모드 | 중 |
| G3 | 포켓 점유 비전 | `/vision/pocket_state` ← `tag_vision_node.py:118` (depth, `pocket_vision` 사용) | 실물 카메라 경로 | 중 |
| G4 | 하강 | 액션 경유 일반 이동 | `guarded_descend` 접촉 정지 | 이동만 (검증 완료) |
| G5 | 로봇 피드백 | 파서 5벌 | 공용 모듈 1벌 | 낮음 (동작 변화 0) |
| G6 | 그리퍼 | Gazebo `gripper_controller` | 실물 서버 없음 | **하드웨어 대기** |
| G7 | attach/detach | 시뮬 전용, 실물은 자동 거부 (`cr7_pnp/node.py:106-113`) | 실물은 물리 파지 | G6 종속 |
| G8 | 표면 (테이블·벽) | 테스트 파일에만 존재 | 공용 모듈로 이동 | 낮음 |
| G9 | 좌표 기준 · TF 트리 | odom 경유 (시뮬 Gazebo 가 발행) | base_link 통일 + 정적 TF | **중 — 조용히 실패함** |

수평 insert 는 갭이 아니다 — 충돌 모델이 담당하는 구간이고, 모델 상수가 맞으면(G1) 액션
경로 그대로 둔다. 접촉 정지가 필요한 곳은 박스 위로 내려앉는 하강뿐이다.

---

## 4. 만들 것 / 옮길 것

### 4.1 신규 3개

**(a) `robot_feed` — 공용 30004 파서** (`cr7_pnp/robot_feed.py`)

- 인터페이스는 새로 설계하지 않는다. `RealtimeMonitor` 가 이미 그것이다:
  `state()` → 관절각·`tool_vector`·`robot_mode`·`collision`, `torque_window()`, `wait_ready()`.
  `vision_target` 이 쓰는 것은 `state()` 하나뿐이다 (`test/vision_target.py:383, 443`).
- `handeye_calib.RobotFeed` 는 이 클래스를 import 해 이름만 유지(호환), `cbirrt_p1p2_test.py`
  는 자기 사본을 지우고 import. **동작 변화 0, 삭제만 일어난다.**
- 읽기 전용 계약을 코드로 못박는다. 지금은 `vision_bridge/config.py:21` 의 주석뿐이다.
  명령은 ServoJ/대시보드 서비스가, 피드는 이 모듈이 — 방향이 섞이지 않게.
- **인스턴스는 하나만.** 소비자가 셋(비전 shim 의 `tool_vector`, 하강의 `m_actual`,
  보호정지 감지)인데 두 벌을 열면 스냅샷 평균 구간의 정지 판정이 서로 다른 프레임을 본다.

**(b) `vision_hover_node` — 비전 shim** (약 100줄)

```
/vision/capture 구독 → vision_target.snapshot() (vision_bridge 그대로)
                     → /vision/device_pose (PoseStamped) 발행
```

- `vision_target` 이 내부에서 이미 `P_base` 와 `hover` 를 만든다 (`test/vision_target.py:556`).
  `q` 대신 `hover` 를 발행하는 것이 전부다. 게이트도 변환 순서도 재구현하지 않는다.
- 이 노드가 `tag_vision_node` **자리에 그대로 들어간다.** 토픽이 계약이므로 `main.py` 와
  `sequences/` 는 한 줄도 안 바뀐다. 시퀀스의 `refresh_device_pose` / `refresh_shelf_pose`
  median-15 + spread 게이트가 그대로 살아 `vision_bridge` 게이트와 이중으로 걸린다.
- `tool_vector` 는 `robot_feed` 에서 받는다 → (a) 가 선행이다.
- 프레임은 **base_link 통일**로 확정한다(2단계에서 TF 트리를 세운다). `vision_target` 이
  이미 base_link 로 계산하고 그 프레임으로 검증됐으므로, shim 은 값을 그대로 싣고
  `frame_id` 만 기존 계약(`odom`)에 맞춘다 — 2단계의 항등 정적 TF 가 그것을 성립시킨다.
- G2(선반 ArUco)·G3(포켓)은 이 노드에 **모드와 발행 토픽이 하나씩 느는 형태**로 확장한다.
  구조는 바뀌지 않는다.

**(c) 부분 가상화 스위치** — `main.py` 프로파일

```bash
python3 main.py --profile sim                # 기본값
python3 main.py --profile real
python3 main.py --profile real --gripper     # 개별 덮어쓰기
python3 main.py --profile real --preflight   # 무동작 검증 (4.3 참고)
```

- 프로파일은 딕셔너리 두 개, 개별 플래그가 그 위에 덮인다. 구현이 각각 하나뿐이므로
  **불리언으로 충분하다. 인터페이스/추상 계층을 만들지 않는다.**
- 경계는 5곳뿐이다: 충돌 상수 / 비전 / 하강 / 그리퍼 / attach.
- `main.py` 에는 아직 argparse 가 없다 — `'--selftest' in sys.argv` 만 본다(`main.py:155`).
  argparse 도입이 이 작업에 포함된다.
- **충돌 상수 경계만은 `--profile` 로 못 고른다.** `geometry.py` 는 import 시점에 모듈 상수를
  확정하는데 argparse 는 그 뒤에 돈다. 그래서 상수 파일 선택은 환경변수 `DOBOT_ENV` 가
  맡고(4.4), `--profile` 은 그것과 **일치하는지 검증**하는 역할을 한다 — 6.1 방어에 포함된다.

**launch 를 통해 넘기는 방법**

`launch/arm.launch.py` 는 지금 `cmd` 를 하드코딩하고 있어(`:24-26`) 옵션을 줄 방법이 없다.
`ros2 launch` 는 `name:=value` 만 받으므로 launch 인자를 만들어 전달한다 (약 10줄):

```python
profile = LaunchConfiguration('profile')
is_sim  = PythonExpression(["'", profile, "' == 'sim'"])

DeclareLaunchArgument('profile', default_value='sim'),
ExecuteProcess(cmd=[PY, tag_vision_py],   condition=IfCondition(is_sim),     ...),
ExecuteProcess(cmd=[PY, vision_hover_py], condition=UnlessCondition(is_sim), ...),
ExecuteProcess(cmd=[PY, main_py, '--profile', profile], ...),
```

```bash
ros2 launch .../arm.launch.py                 # sim (기본값)
ros2 launch .../arm.launch.py profile:=real
```

**비전 노드를 같은 인자로 고르는 것이 핵심이다.** launch 파일을 둘로 나누는 대신 인자 하나가
노드와 프로파일을 함께 바꾸면, "실물 프로파일인데 시뮬 비전 노드가 떠 있는" 상태가
**구조적으로 불가능**해진다 — 6.1 의 불일치 위험 절반이 여기서 사라진다.

환경변수(`DOBOT_PROFILE=real`)로도 가능하고 `DOBOT_TYPE`/`ARM_TYPE` 전례도 있지만, 개별
덮어쓰기(`--gripper`, `--preflight`)가 CLI 라 두 방식이 섞인다. **launch 인자 쪽을 쓴다.**

### 4.2 옮길 것 (테스트 파일 → 공용)

`test/cbirrt_p1p2_test.py` 에만 있어서 `sequences/` 가 못 쓰는 검증 완료 코드:

| 대상 | 행 | 비고 |
|---|---|---|
| `guarded_descend()` | `:1077` | 접촉 정지 하강 |
| `ContactDetector` | `:264` | hard(컨트롤러 충돌검출) + soft(지연 기준선 토크 계단) 2채널 |
| `recover_after_contact()`, `warn_descend_speed()` | — | 하강 부속 |
| `wait_until_still()`, `joint_gap_deg()`, `arrived()` | `:1040, :1063, ~:1680` | 안전 검사 |
| 표면 등록 (`add_surface` / `move_surface` / `load_surfaces` / `register_surfaces`) | `:645-846` | G8 → `cr7_pnp/collision_env.py` |
| 표면 교시 (`measure_surface` / `teach_surface` / `set_surface` / `save_surface`) | `:681-818` | G8 → `tools/teach_env.py` |
| `Dashboard` | `:340` | 속도·충돌레벨·에러해제. `servoj()` 포함 |

옮기면서 **로직을 바꾸지 않는다.** 이동 후 `test/run.sh --vision --run` 이 이전과 동일하게
도는 것이 이동의 합격 기준이다.

**단 하나 예외: `add_surface` 의 프레임.** 주석에 "arm-only model is rooted at base_link, so
this placement is already in base_link" 라고 적혀 있지만 그것은 테스트가 결합 xacro 로드에
실패해 팔 단독 모델로 폴백했을 때만 참이다(`test/cbirrt_p1p2_test.py:1960-1983`). 메인
파이프라인의 충돌 모델은 **결합 모델이고 루트가 `mpo_base_link`** 다(`cr7_pnp/node.py:415-420`).
그대로 옮기면 교시한 테이블·벽이 `base_link → mpo_base_link` 오프셋만큼 어긋난 자리에
조용히 놓인다. 선반과 같은 방식으로 프레임을 합성할 것 — 2-1 에 포함된다.

### 4.3 `--preflight` (가장 값싼 검증 수단)

`shelf_pick_to_hub` 는 이미 삽입·파지·트위스트 복귀를 **모션 없이** IK 로 전부 검증한다
(`sequences/shelf_pick_place.py:163-200`). 실행만 막으면 **실물 비전 + 실측 상수 위에서
시퀀스 전체를 검증하되 팔은 안 움직이는** 모드가 된다. 새 로직이 필요 없다.

### 4.4 충돌 환경 구성 — 교시와 적용을 가른다

충돌 모델에 들어가는 것은 네 종류다: URDF(로봇·큐브·AGV), 프리미티브 박스(선반 판·재고
팬텀), STL 메시(와이어본더), 표면 슬래브(테이블·벽·트롤리). **등록 메커니즘은 이미 다 있고
새로 만들 것은 없다** — 바뀌는 것은 값의 출처와 코드의 위치뿐이다. 사용법은
`docs/collision_model_guide.md`.

**(a) 교시 ↔ 적용 분리**

성격이 다르므로 파일을 나눈다. 지금은 둘 다 `test/cbirrt_p1p2_test.py` 안에 섞여 있다.

| | 교시 | 적용 |
|---|---|---|
| 위치 | `tools/teach_env.py` | `cr7_pnp/collision_env.py` |
| 실행 | 셋업 시 1회, 사람이 | `main.py` 기동 시 자동 |
| 팔 이동 | **있음** (접촉으로 표면을 찾음) | 없음 |
| 내용 | `measure_surface` · `teach_surface` · `set_surface` · `save_surface` · `--show` | `load_surfaces` · `add_surface` · `move_surface` · `register_surfaces` |

의존은 **`tools/` → `cr7_pnp/` 단방향**이다. `measure_surface` 가 접촉 정지로 표면을 찾으므로
`guarded_descend`/`ContactDetector`(4.2 에서 `cr7_pnp` 로 이동)를 import 한다. 역방향은 없다.

**(b) 실측값을 코드에서 파일로**

선반·포켓 상수는 `cr7_pnp/geometry.py` 에 모여 있어 절반은 이미 정리된 상태다. 이것을
프로파일별 JSON 으로 뺀다.

```
cr7_pnp/env/sim.json     현재 geometry.py 의 시뮬 수치
cr7_pnp/env/real.json    실측값
```

`geometry.py` 가 기동 시 하나를 읽어 **같은 이름의 모듈 상수를 채운다.** 따라서
`from .geometry import SHELF_BOARD_TOPS` 같은 사용처는 한 곳도 바뀌지 않는다.

```python
_ENV = json.load(open(os.path.join(HERE, 'env', os.getenv('DOBOT_ENV', 'sim') + '.json')))
SHELF_BOARD_TOPS = tuple(_ENV['shelf']['board_tops'])
```

옮길 값과 현재 위치:

| 값 | 현재 |
|---|---|
| `SHELF_WORLD_POSE` · `SHELF_BOARD_TOPS` · `SHELF_FOOTPRINT` · `SHELF_BOARD_THICK` | `geometry.py:52-62` |
| `SHELF_TIER_TOPS` · `SHELF_BOX_XS` · `SHELF_BOX_Y` · `SHELF_TAG_XY` | `geometry.py:65-86` |
| `POCKET_X` · `POCKET_Y` · `POCKET_SURFACE_Z` · `POCKET_HOVER` | `geometry.py:218-242` |
| `BOX_SIZE` | `gripper_params.py:40` |
| `HUB_TCP` | `shelf_pick_place.py:69` + `wirebonder_pick_place.py:156` — **두 파일에 중복.** 옮기면서 하나로 |

**요구사항: 등록 함수는 개수 로그를 남긴다.** 선반이 `[collision] added 4 shelf boards` 를
찍듯 `register_surfaces` 도 같은 형식으로 찍는다. 매뉴얼의 확인 절차(`collision_model_guide.md`
4.5)와 fail-closed 판정이 둘 다 이 로그를 근거로 삼는다 — 로그가 없으면 "등록됐는지"를
확인할 방법이 없다.

`STOCK_SHRINK = -0.006`(면당 3 mm 팽창)은 값이 아니라 **정책**이므로 JSON 으로 빼지 않는다.
실물 파지 편차를 재측정하기 전까지 유지한다(5단계).

`SLOT_WORLD`(`wirebonder_pick_place.py:202`) 등 시퀀스 안의 실측 상수를 이 범위에 넣을지는
와이어본더를 실물화할 때 정한다. 선반 경로에는 필요 없다.

**(c) 표면의 좌표 기준** — 2-1 에서 확정한다. 4.2 의 `add_surface` 항목 참고.

---

## 5. 진행 순서와 각 단계의 합격 기준

각 단계는 이전 단계가 실물로 확정된 뒤에 시작한다. 조합이 2^5 가 되지 않도록 **선형으로** 간다.

사전에 떠 있어야 하는 프로세스(러너 · bringup · 액션 서버)는 `docs/vision_hover_chain.svg`
의 주황 태그를 볼 것 — 여기 다시 적지 않는다(두 벌은 갈라진다).

### 회귀를 무엇으로 판정하는가

실물은 결정론적이지 않다. 비전 노이즈와 접촉 지점 편차 때문에 같은 자리에서 두 번 돌려도
숫자가 조금씩 다르다. 그래서 "이전과 동일"이 아니라 **단계마다 판정 방법을 다르게** 둔다.

| 단계 | 판정 | 팔 |
|---|---|---|
| 1 (`robot_feed`) | `--monitor` 로 30004 필드가 같은 값을 찍는지, `verify_chain.py` 잔차가 그대로인지 | 안 움직임 |
| 2 · 3 | `--preflight` 통과 (각 단계 합격 기준 참고) | 안 움직임 |
| 4 (하강) | `test/vision_hover_*.json` 의 `arrival.tracking_err_mm` 이 기존 기록 분포 안인지 | 움직임 |

1단계는 순수 코드 이동이라 팔을 움직이지 않고 판정할 수 있고, 그래서 위험이 0 이다.
4단계의 허용 범위는 기존 `vision_hover_*.json` 기록에서 분포를 뽑아 착수 전에 정한다.
비교 가능한 필드: `arrival.tracking_err_mm` · `detection` · `descend.dropped_m` ·
`handeye_t_flange_cam_m`.

### 1단계 — `robot_feed` 통합
- 작업: 4.1(a). 사본 5벌 제거, import 로 교체.
- 합격: `test/run.sh --monitor` 와 `--vision --run` 이 이전과 동일. `verify_chain.py` 동일.
- 이유: 이후 모든 단계가 이 피드를 쓴다. 동작 변화가 없어 위험이 가장 낮다.

### 2단계 — 좌표 기준·TF 트리 확정 + 충돌 환경 구성 + 실측
작업이 셋이지만 **하나의 목표**다: 시퀀스의 안전 근거(충돌 모델)를 실물에서 실제로 켜는 것.

**2-1. 좌표 기준을 base_link 로 통일 (G9)**

실물 팔은 세계 안에서 자기 위치를 모른다. 카메라도 플랜지에 달려 있어 비전 값이 원래
base_link 로 나온다 — `vision_target` 이 그렇게 계산하고 그 프레임으로 검증됐다
(`docs/VISION_CHAIN_VERIFICATION.md` p95 1.37 mm). odom 은 시뮬에서 AGV 가 움직이기 때문에
존재하는 프레임이고, 실물에서 그것을 발행하는 주체는 (AGV 벤더 쪽을 붙이지 않는 한) 없다.

- 정적 TF `odom ≡ base_link`(항등)를 발행한다 → `transform_world_pose()` 가 항등이 되어
  비전 값이 변환 없이 통과한다. **시퀀스는 한 줄도 안 바뀐다.**
- `mpo_base_link` 가 TF 에 있어야 한다. 실물 bring-up 은 `cr7_moveit` 이 **팔 단독**
  URDF(`config/cr7_robot.urdf.xacro`)로 robot_state_publisher 를 띄우므로 AGV 링크가 없다.
  결합 URDF(`cra_description/urdf/cr7_on_mpo700.urdf.xacro`)를 쓰거나
  `base_link ↔ mpo_base_link` 정적 TF 를 추가한다. **어느 쪽인지 먼저 확인할 것.**
- 항등 TF 는 "미션 중 AGV 가 움직이지 않는다"는 가정을 코드로 박는 것이다. AGV 를 옮긴
  뒤에는 반드시 `locate_shelf` 로 재측정한다. 나중에 AGV 주행을 운용에 넣으면 이 항등을
  진짜 odom 발행으로 교체해야 한다.

**2-2. 충돌 모델 미적용 시 중단 (fail-closed)**

`update_shelf_collision()` 은 TF 조회 실패를 **경고로만 넘기고 `False` 를 반환한다**
(`cr7_pnp/node.py:587-592`). 그러면 선반 판 팬텀이 z=-100 에 park 된 채 남아
**선반 충돌 모델이 꺼진 상태로 시퀀스가 진행된다.** 2절에서 안전 근거로 든 바로 그 팬텀이다.

- 호출부에서 반환값을 검사해 `ErrorCode.TF_UNAVAILABLE`(이미 존재)로 **중단**시킨다.
- 같은 기준으로 다른 "조용한 실패"도 점검한다: 팬텀이 하나도 등록되지 않은 상태,
  `shelf_pose` 가 기본값(`SHELF_WORLD_POSE`)으로 남은 상태.

**2-3. 충돌 상수 실측 (G1) + 충돌 환경 구성 (G8)**

- 4.4(a): 교시(`tools/teach_env.py`) / 적용(`cr7_pnp/collision_env.py`) 분리.
- 4.4(b): `cr7_pnp/env/{sim,real}.json` 신설, `geometry.py` 가 `DOBOT_ENV` 로 하나를 로드.
  `sim.json` 은 현재 값을 그대로 옮겨 담는다 — **시뮬 동작이 안 바뀌는 것이 이 단계의 회귀
  기준**이다.
- 선반 판 높이·발자국, 박스 치수, 포켓 위치를 실측해 `real.json` 을 채운다.
- 테이블·벽·트롤리는 `tools/teach_env.py --teach-surface` 로 교시 → `surfaces.json`.
  툴을 댈 수 없는 면은 `--set-surface --at` 로 숫자 지정.
- 팬텀 3 mm 팽창 정책은 **유지**하되 실물 파지 편차를 재측정해 값을 재검토(5단계에서 확정).

**합격 기준**
- `ros2 run tf2_tools view_frames` 로 `odom → base_link → mpo_base_link` 가 실제로 이어질 것.
- 선반 판·재고 팬텀이 **등록된 로그가 실제로 찍힐 것** (`[collision] added N shelf boards`).
- `DOBOT_ENV=sim` 으로 돌렸을 때 시뮬 동작이 이전과 동일할 것 (JSON 분리의 회귀 기준).
- 실측 상수 + `--profile real --preflight` 로 선반 시퀀스 pre-flight 통과.
  **팔은 움직이지 않는다.**
- TF 를 일부러 끊었을 때 경고가 아니라 **중단**될 것 (fail-closed 확인).
- `--profile` 과 `DOBOT_ENV` 가 어긋나면 기동 거부될 것 (6.1).

**이유**: 안전 보장의 근거 자체다. 여기가 틀리면 이후 단계의 모든 통과가 의미 없다.

### 3단계 — 비전 실물화
- 3a: `vision_hover_node` + 매거진(MAGAZINE) 경로 → `/vision/device_pose`.
- 3b: ARUCO 모드 추가 → `/vision/shelf_pose` (G2).
- 3c: 포켓 depth 스캔 → `/vision/pocket_state` (G3). 현재 발행자는 `tag_vision_node.py:118`
  이고 `vision/pocket_vision.py:201` 이 구독한다 — 즉 포켓 스캔도 태그 노드가 겸하고 있다.
- 합격: 각 단계마다 (1) 시뮬에서 `tag_vision_node` 로 되돌려도 그대로 동작(계약 유지 확인),
  (2) 실물에서 `--preflight` 통과, (3) `verify_chain.py` 잔차가 기존 수준
  (`docs/VISION_CHAIN_VERIFICATION.md` p95 1.37 mm) 유지.
- 이유: 프레임 정합 오류는 pre-flight 로 잡힌다. 움직이기 전에 확정한다.

### 4단계 — 접촉 정지 하강 투입
- 작업: G4. 시퀀스의 하강 지점에 `guarded_descend` 를 끼운다. 수평 insert 는 액션 유지.
- 합격: 실물에서 파지 없이(그리퍼 더미) 접근 → 하강 → 되올림 → 역재생 복귀까지 완주.
  `--vision --run` 이 이미 보여준 것과 같은 모양이므로 회귀 비교가 가능하다.
- 주의: 하강만 액션을 우회한다. `action_move_server` 에 cancel 콜백이 없어 진행 중 goal 을
  못 멈추기 때문이고, ServoJ 스트림을 끊는 것 자체가 깨끗한 정지다. 대신 URDF→컨트롤러
  부호 뒤집기를 호출부가 떠안는다 (`test/cbirrt_p1p2_test.py:380` docstring — 첫 `--run`
  시도에서 이걸 빠뜨려 J1 이 최대 370° 어긋난 목표를 받고 보호정지했다).
  `joint_gap_deg > 5°` 중단 가드가 그 재발 방지용이므로 반드시 함께 옮긴다.

### 5단계 — 그리퍼 (하드웨어 대기)
- 작업: G6, G7. 실물 그리퍼 액션 서버 확보 → `control_gripper` 실물 경로 활성화.
- 합격: 실제 파지로 선반 1박스 전체 사이클.
- 주의: 그리퍼 더미로 통과한 carry 는 **무게·미끄러짐·파지 편차가 전부 빠진 상태**다.
  실물 파지 후 2단계의 팽창값을 재검증한다.

---

## 6. 고려사항 / 위험

### 6.1 프로파일 불일치 (제일 위험)
`--profile real` 인데 시뮬 충돌 상수로 움직이는 경우. **부팅 시 대조 검사를 넣는다:**

- `--profile real` 인데 30004 연결 실패 → 중단
- `--profile sim` 인데 `/ATTACHLINK` 없음 → 중단
- `--profile` 과 `DOBOT_ENV`(충돌 상수 파일) 가 다르면 → 중단. **실물인데 시뮬 치수로
  움직이는 것**이 이 문서에서 가장 위험한 상태다
- 기본값은 **항상 sim.** 실물은 명시적으로 적어야 돌아가게

5줄이면 되고, 이 파이프라인에서 가장 값싼 보험이다.

### 6.2 launch 는 여전히 따로다
`main.py` 옵션이 Gazebo 나 bringup 을 띄우지는 않는다. 실제 전환은 **어느 launch 를
띄우느냐 + 프로파일** 두 겹이다. `arm.launch.py` 안쪽(비전 노드 + 디스패처)은 4.1(c) 의
launch 인자로 한 겹이 되지만, 그 바깥(Gazebo vs bringup)은 여전히 진입 스크립트가 정한다. `run_mpo700_cr7.sh`(시뮬) / `run_test.sh` 패턴을 따라
`run_real.sh` 를 추가하면 사용자에게는 스크립트 하나 고르는 것이 된다.

### 6.3 더미가 늘수록 쌓이는 부채
더미로 통과한 구간은 실물의 실패 모드를 겪지 않은 것이다. 특히 그리퍼가 그렇다(5단계 주의).
각 단계 합격 기준에 "이전 단계 실물 확정"을 못박은 이유다.

### 6.4 `action_move_server` 는 손대지 않는다
액션에 cancel 을 붙이는 것은 기술적으로 가능하다(`cancel_callback` + `MultiThreadedExecutor`,
약 20줄). 하지만 (1) 벤더 패키지 수정이라 업스트림 갱신 시 충돌하고, (2) 접촉 판정은 어차피
클라이언트가 30004 를 보며 해야 해서 cancel 왕복 지연(1~2틱)만 늘며, (3) 접촉 정지 하강은
궤적 실행이 아니라 **센서 되먹임으로 닫힌 서보 루프**라 액션 인터페이스에 맞지 않는다.
호출부의 일관성 문제는 `guarded_descend` 를 공용 모듈로 옮겨 함수 하나로 만들면 해소된다.

### 6.5 조용한 실패 (silent failure)
이 저장소에서 가장 위험한 패턴은 예외를 삼키고 진행하는 코드다. 확인된 것:

- `update_shelf_collision()` — TF 실패 시 경고 후 `False` 반환, **충돌 모델 없이 진행**
  (2-2 에서 fail-closed 로 고친다)
- `execute_path` 는 `_wait_settled` 타임아웃에도 `True` 를 반환한다. 시뮬에선 맞지만 실물의
  **보호정지가 이것과 똑같이 보인다** — 그래서 테스트가 `arrived()` 로 매 구간을 30004 상태와
  대조한다(2026-08-11 에 mode 11 트립이 완료된 사이클로 기록된 사례). 4.2 에서 함께 옮긴다.

새 코드를 넣을 때도 같은 기준을 적용한다: **모델이 꺼진 채 움직이는 경로를 만들지 않는다.**

### 6.6 오버레이된 gazebo_ros2_control
`~/dobot_ws/src/gazebo_ros2_control_patched` 의 소스 수정본이 apt 버전을 덮어쓴다(디렉터리
이름만 `_patched`, 패키지명은 동일). 시뮬이 갑자기 안 뜨면 어느 워크스페이스를 source 했는지
부터 확인한다.

### 6.7 그리퍼 액션 서버 부재 (기존 구멍)
`cr7_pnp/node.py:102-103` 이 `/gripper_controller/follow_joint_trajectory` 를 쓰는데 실물에는
서버가 없다. `action_move_server` 는 group controller 만 연다. 지금은 `control_gripper` 가
5초 타임아웃 후 실패한다(`:164`). 3단계까지는 더미로 우회하지만, 5단계 전에 이 서버를
누가 열 것인지 정해야 한다.

---

## 7. 정정해야 할 문서/주석

접촉 정지 하강은 **실물에서 검증 완료**인데(1.3) 문서·주석이 "하강 없음"으로 남아 있던 것들.
2026-08-16 에 전부 정정했다.

| 위치 | 내용 | 상태 |
|---|---|---|
| `test/cbirrt_p1p2_test.py:76` | 실행 예시 — 하강 포함 + `--no-descend` 예시 추가 | 완료 |
| `test/cbirrt_p1p2_test.py:1326` | 절 머리 주석 — 전체 사이클로 다시 씀 | 완료 |
| `test/cbirrt_p1p2_test.py:1391` | **실행 직전 콘솔 확인 문구**가 "NO DESCEND" 로 고정돼 있었다. `--no-descend` 여부에 따라 갈리게 수정 (조작자가 하강 없는 줄 알고 승인하는 상황이었다) | 완료 |
| `test/cbirrt_p1p2_test.py:2054` | `--vision` 도움말 | 완료 |
| `docs/vision_hover_chain*.svg` | "하강 없음" 표기 2곳 | 완료 |

---

## 8. 열린 질문 (결정 필요)

1. **실물 TF 트리를 어떻게 세우는가** — 결합 URDF 로드 vs 정적 TF 추가. 2단계 착수 시
   실물 bring-up 이 실제로 발행하는 프레임을 확인한 뒤 결정한다.
   (좌표 기준 자체는 base_link 통일로 **결정됨** — 2-1 참고.)
2. **실물 그리퍼 액션 서버**를 누가 여는가 (`dobot_moveit` 확장 vs 별도 노드).
3. **팬텀 팽창값 3 mm** 를 실물 파지 편차 재측정 후 유지할 것인가.
4. `run_real.sh` 를 만들 것인가, `run_mpo700_cr7.sh`(현 시뮬 진입점) 옆에 프로파일 인자를
   받는 공용 스크립트를 둘 것인가.

---

## 9. 요약 표

| 단계 | 내용 | 산출물 | 팔 움직임 |
|---|---|---|---|
| 1 | `robot_feed` 통합 | 공용 모듈 1개, 사본 5벌 제거 | 없음 |
| 2 | 좌표 기준·TF 트리 + 충돌 환경 구성 + 실측 | 정적 TF, fail-closed, `collision_env.py`, `teach_env.py`, `env/*.json`, `--preflight` | **없음** |
| 3 | 비전 실물화 (매거진→선반→포켓) | `vision_hover_node` | 없음(pre-flight) |
| 4 | 접촉 정지 하강 | `guarded_descend` 공용화 | 있음 (파지 없이) |
| 5 | 그리퍼 | 실물 파지 | 있음 (전체 사이클) |

**손대지 않는 것**: `main.py` 라우팅, `sequences/` 시퀀스 로직, `action_move_server`,
`dobot_bringup_v4`, `cr7_moveit`, 시뮬 경로 전체.
