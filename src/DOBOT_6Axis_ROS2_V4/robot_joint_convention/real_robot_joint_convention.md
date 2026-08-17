# 실물 로봇 관절 규약 불일치 — 문제·조사·해결

2026-08-06. CR7 실물에서 `cr7_pnp`(pinocchio 기반 CBiRRT/IK/충돌검사)를 처음
돌리면서 발견한 **URDF와 컨트롤러의 관절 부호 불일치**를 정리한다. 이 문제는
시뮬레이션에서는 원리적으로 드러날 수 없고, 실물에서도 **에러 없이 조용히**
팔을 거울 반전된 자세로 몰기 때문에 위험도가 높다.

`real_robot_jetson_bringup.md`(인프라·모션 실행 경로)와 별개로, 이 문서는
**기구학 모델의 정합성**을 다룬다.

---

## 0. 요약

| | |
|---|---|
| **증상** | pinocchio FK가 실제 로봇과 전혀 다른 플랜지 위치를 계산. 툴 방향까지 반대 |
| **원인** | URDF가 **J1·J5·J6의 회전축을 컨트롤러와 반대로** 정의. 파이프라인에 변환 없음 |
| **매핑** | `urdf_q = [-J1, +J2, +J3, +J4, -J5, -J6]` — **오프셋 없음, 부호만** |
| **해결** | `cr7_pnp/node.py`의 ROS 경계에서 양방향 부호 반전. `CR7_REAL_ROBOT=1`로 옵트인 |
| **시뮬 영향** | **없음.** URDF를 건드리지 않았으므로 Gazebo 경로는 그대로 |

---

## 1. 증상

실물에서 `test/cbirrt_p1p2_test.py --teach-surface`(현재 자세에서 바닥/벽 높이를
재는 기능)가 계속 엉뚱하게 거부했다:

- 툴을 수직 아래로 두고 선반에 올려놨는데 **"툴이 179° 위를 향한다"**
- `y ≤ -0.34` 구역(선반)에서 쟀는데 **"그 범위 안에 팔이 없다"**
- 지지대에서 재면 최저점이 툴이 아니라 **`Link1`**(어깨)으로 잡힘

같은 관절각을 로봇과 모델에 각각 넣어 비교하니 답이 나왔다:

```
관절각 (GetAngle) : 274.927, -68.523, -88.695, 67.117, 89.055, 16.996

로봇 GetPose(user=0,tool=0) : x= -90.7  y=-594.9  z=-103.9 mm   rx=-179.9 (툴 아래)
URDF + pinocchio FK         : x=+192.5  y=+571.4  z=+136.7 mm   툴 축 +z (위)
```

| 관절 | 차 이 | 피팅으로 나온 잔여 오프셋 |
| :--- | :--- | :--- |
| J1 | 부호 반대 | -0.07° |
| J2 | 같음 | -0.05° |
| J3 | 같음 | -0.11° |
| J4 | 같음 | +0.15° |
| J5 | 부호 반대 | $-0.31^{\circ}$ |
| J6 | 부호 반대 | -0.31° |

**세 축 전부 부호가 반대**이고 툴 방향도 반대. 물리적 실제(팔이 로봇 오른쪽
선반 위에 툴을 아래로 향한 채 있음)와 일치하는 건 로봇 쪽이었다.

## 2. 왜 시뮬레이션에서는 안 드러났나

**Gazebo가 시뮬레이트하는 로봇이 곧 그 URDF이기 때문**이다. 시뮬에서는
`/joint_states`도 URDF 규약으로 나오므로 pinocchio FK와 당연히 일치한다.
**자기일관적**이라 모순이 생길 여지가 없다.

실물에서는 `/joint_states`가 **컨트롤러의 관절 규약**으로 들어온다. 그런데
파이프라인 어디에도 변환이 없다:

```
로봇 RT 피드백 (30004, q_actual, 도)
  ↓  dobot_bringup_v4/src/command.cpp:52
     current_joint_[i] = deg2Rad(real_time_data_->q_actual[i]);     ← 단위 변환만
  ↓  dobot_bringup_v4/src/main.cpp        /joint_states_robot, 10 Hz
  ↓  dobot_moveit/dobot_moveit/joint_states.py                     ← 이름만 붙여 중계
  ↓  /joint_states
```

부호 반전도, 오프셋도, 좌표계 변환도 없다. `joint_states.py`가 `dobot_moveit`
**패키지 안에** 있어서 MoveIt이 뭔가 한다고 오해하기 쉬운데, 28줄짜리 순수
릴레이다. MoveIt은 오히려 `/joint_states`를 소비하는 쪽이다.

## 3. 왜 조그는 멀쩡했나

`tools/jog_real.py`, `tools/jog_action.py` 모두 **URDF를 전혀 쓰지 않는다.**
컨트롤러 각도를 받아 델타를 더해 컨트롤러로 되돌려 보내는 순수 통과 경로다.
그래서 조그는 이 문제와 무관하게 정상 동작한다.

**영향받는 건 pinocchio 스택뿐이다** — CBiRRT, IK, 충돌검사, 직교 서보
(`linear_servo`/`guarded_descend`). 그리고 그 스택은 이번이 실물 첫 실행이었다.

## 4. 조사 과정 (헛짚은 것 포함)

인수인계 시 같은 길을 반복하지 않도록 **틀렸던 가설도 남긴다.**

| # | 가설 | 결과 |
|---|---|---|
| 1 | 게임패드 스틱 중립 오프셋 | **틀림.** `/dev/input/js0` 실측 결과 6축 전부 정확히 0 |
| 2 | `jog_action.py`의 목표값 누적 | **부분적으로 맞음.** 실제 버그지만 이 증상의 원인은 아니었음 (§8 참고) |
| 3 | 액션 서버 goal 큐 적체 | **틀림.** 근거 없이 코드만 보고 단정했음. 되돌림 |
| 4 | **액션 서버 중복 실행** | **맞음** — 조그가 튀던 문제의 원인. 기구학과는 별개 (§8) |
| 5 | 사용자/툴 좌표계 설정 | **틀림.** `userCoordinate=0`, `toolCoordinate=0` 확인 |
| 6 | URDF `base_link`에 회전이 박혀 있음 | **틀림.** 회전은 단위행렬 |
| 7 | **관절 부호 규약 불일치** | **맞음** |

결정적 단서는 **플랜지 수평 반경이 603.0 vs 601.8 mm로 1.2 mm 이내 일치**한
것이었다. 링크 길이가 틀렸다면 이렇게 맞을 수 없으므로, 형상은 맞고 **각도
해석만** 다르다는 뜻이었다.

## 5. 근본 원인과 매핑

URDF가 **J1, J5, J6의 양의 회전 방향을 컨트롤러와 반대로** 잡고 있다.

```
urdf_q = [ -J1, +J2, +J3, +J4, -J5, -J6 ]      오프셋 없음
```

즉 컨트롤러가 `J1 = +188.05°`라고 하면, **같은 물리적 자세**를 URDF에서는
`J1 = -188.05°`로 표현해야 한다.

## 6. 검증

관절별 부호(2가지) × 오프셋을 연속값으로 두고, 측정된 플랜지 자세에 맞도록
비선형 최적화했다(64가지 부호 조합 각각에 대해 damped Newton). 측정 자세 하나는
미지수 6개·제약 6개라 정확히 결정되는 계이므로 **두 번째 자세를 검증용으로
남겨뒀다.**

**측정 데이터** (`GetAngle` / `GetPose(user=0, tool=0)`):

```
자세 1 (피팅용)
  관절 : 274.9270, -68.5230, -88.6950, 67.1170, 89.0550, 16.9960
  플랜지: -90.6716, -594.9800, -103.8902, -179.9217, -0.6372, 167.6904

자세 2 (검증용, 피팅에 사용하지 않음)
  관절 : 188.0490, -36.3090, -122.5100, 67.1180, 89.0560, 16.9970
  플랜지: -462.0481, 77.4916, 55.7889, 178.5737, -1.1873, 80.9409
```

**피팅 결과** — 64조합 중 오프셋이 0으로 떨어지는 건 하나뿐:

```
signs  -1 +1 +1 +1 -1 -1   offsets(deg)  -0.07  -0.05  -0.11  +0.15  -0.31  -0.31
2등    -1 +1 +1 +1 +1 -1   offsets(deg)  ...  -178.42  ...        ← 178° 오프셋 필요
```

**검증 결과** — 두 자세 모두에 대해:

```
자세 1 (피팅용)   위치 오차 1.55 mm   자세 오차 0.40 deg
자세 2 (검증용)   위치 오차 1.39 mm   자세 오차 0.27 deg    ← 진짜 시험
```

64조합 전체를 두 자세에 동시에 걸면 이 하나만 살아남는다. 2등은 위치는 같지만
자세가 34.23° 어긋난다(J6는 플랜지 원점을 움직이지 않으므로 위치만으로는
구분되지 않고, 자세가 갈라준다).

잔차 1.4 mm / 0.3°는 공칭 URDF와 실기 캘리브레이션 차이 수준으로 허용 범위다.

## 7. 해결

### 7.1 왜 URDF를 고치지 않았나

두 가지 방법이 있었다:

- **(A) 채택** — URDF는 그대로 두고 **ROS 경계에서만** 부호 반전
- **(B) 기각** — URDF의 축 정의를 수정

(B)를 택하면 Gazebo가 시뮬레이트하는 로봇이 바뀐다. 그러면 시퀀스에 박혀 있는
**관절공간 상수가 전부 무효**가 된다 — `CAPTURE_A_JOINTS`, hub 설정, 교시된
자세, IK 분기 선택까지. 시뮬에서 다시 튜닝해야 한다. **(A)는 시뮬 경로를 한 줄도
건드리지 않는다.**

### 7.2 적용 위치

`cr7_pnp/node.py`:

```python
JOINT_SIGN_REAL = np.array([-1.0, 1.0, 1.0, 1.0, -1.0, -1.0])
```

- **`CR7Node.__init__`** — `CR7_REAL_ROBOT` 환경변수로 켜고 끔.
  시뮬은 `np.ones(6)`이므로 무영향
- **`joint_state_callback`** — 읽기: 컨트롤러 → URDF
- **`execute_trajectory`**, **`execute_path`** — 보내기: URDF → 컨트롤러

부호 반전은 자기역함수라 **같은 곱셈을 양방향에 적용**하면 된다.

```python
self.real_robot = os.getenv('CR7_REAL_ROBOT') == '1'
self.joint_sign = JOINT_SIGN_REAL if self.real_robot else np.ones(6)
```

`test/cbirrt_p1p2_test.py`는 실물 전용이므로 import 시점에
`os.environ.setdefault('CR7_REAL_ROBOT', '1')`로 스스로 켠다.

### 7.3 계층 구조

```
┌─────────────────────────────────────────────┐
│  시퀀스 (shelf/wirebonder/main.py)          │
│  CBiRRT · IK · 충돌검사 · 상수들            │   ← 전부 URDF 규약
├─────────────────────────────────────────────┤
│  cr7_pnp/node.py    ★ 여기서만 부호 반전    │
├─────────────────────────────────────────────┤
│  /joint_states · FollowJointTrajectory      │   ← 컨트롤러 규약
│  실제 로봇                                   │
└─────────────────────────────────────────────┘
```

**시퀀스 코드는 아래에서 부호가 뒤집히는지 알 수 없고 알 필요도 없다.**

## 8. 파급 효과

### 시뮬레이션은 영향 없음

URDF를 안 건드렸고 부호 반전은 옵트인이다. `CR7_REAL_ROBOT`을 안 켜면 이전과
바이트 단위로 동일하게 동작한다.

### 관절공간 상수가 그대로 이식됨

부호 반전이 정확히 규약 차이 그 자체이므로:

> 시뮬에서 자세 P를 만들던 URDF 관절벡터를 실물에 그대로 쓰면, 실물도 P로 간다.

링크 형상은 원래 맞았기 때문이다(반경 1.2 mm 일치). 직교좌표 상수
(`SHELF_WORLD_POSE`, `HUB_TCP` 등)도 URDF가 안 바뀌었으니 그대로다.

## 9. 함께 발견된 함정들

기구학과 별개이지만 실물 작업 시 반드시 알아야 할 것들.

### 9.1 `fk_tcp()`의 기준 프레임

`ReachabilityModel.fk_tcp()`는 **모델 루트** 기준으로 반환한다. `base_link`는
모델 루트보다 30 mm 위에 있다. 게다가 `TCP_OFFSET_M`(약 120 mm)을 툴 축 방향으로
더한다 — **실물에 달려 있지 않은 그리퍼** 길이다.

플래너 내부는 차분만 쓰므로 둘 다 상쇄되지만, **펜던트 값과 비교할 때는 환산이
필요**하다. `test/cbirrt_p1p2_test.py`의 `flange_in_base()`가 `GetPose(user=0,
tool=0)`와 직접 비교 가능한 값을 준다.

### 9.2 충돌 모델의 팬텀 그리퍼

URDF에 `gripper_base_link`/`gripper_finger_link`가 들어 있으나 실물에는 아직
그리퍼가 없다(카메라 홀더만 장착). 모델이 툴을 실제보다 크게 보므로 **보수적인
쪽**이라 안전하지만, 면 높이 측정값이 줄자보다 낮게 나온다. 측정과 강제를 같은
팬텀으로 하므로 자기일관적이다 — 단 **측정 시 손목 자세를 작업 시와 같게** 두어야
두 툴의 치수 차이가 상수로 상쇄된다. 그리퍼 도착 시 `gripper.xacro`를 실측으로
맞출 것.

### 9.3 `neo_simulation2` 부재 → 조합 충돌 모델 사용 불가

`COMBINED_XACRO`(`cr7_on_mpo700.urdf.xacro`)는 MPO-700 AGV 본체를
`neo_simulation2`에서 가져오는데, 실물 워크스페이스에는 없다. 받침대 고정형
설치라 AGV가 실제로 없으므로 **팔 전용 모델이 오히려 맞다.** `setup_planner()`에
`combined_xacro` 인자를 추가해 대체 가능하게 했다.

### 9.4 액션 서버 중복 실행

터미널 B(`dobot_joint.launch.py`)를 이미 떠 있는 상태에서 또 띄우면
`/cr7_group_controller/follow_joint_trajectory`에 **서버가 두 개** 붙는다. ROS 2는
이를 막지 않는다. 그러면 goal 하나를 두 서버가 각각 받아 각자 `ServoJ`를 쏘고,
두 목표 스트림이 경쟁해 **팔이 튀고 추종 오차로 충돌 트립**이 난다. 조그 버그처럼
보이지만 아니다.

`jog_action.py`가 이 경고를 그대로 출력한다:

```
There may be more than one action server for the action
'/cr7_group_controller/follow_joint_trajectory'
```

`test/jog_bringup.sh --status`가 `UP x2 <<< DUPLICATE`로 표시한다.

### 9.5 `jog_action.py`의 목표값 누적

`self.target`이 기동 시 1회만 시드되고 실제 위치와 재동기화되지 않았다. 드래그
버튼으로 팔을 손으로 옮기거나 충돌 트립으로 후퇴하면 `target`이 옛 자세에 남고,
다음에 스틱을 건드리는 순간 **옛 자세로 확 달린다.** 유휴 시 재동기화 + 선행 거리
클램프(2틱)를 추가했다.

### 9.6 `TCP_force`는 이 펌웨어에서 전부 0

RT 피드백(30004) offset 720의 `TCP_force[6]`은 항상 0이다. 접촉 감지에 쓸 수
없다. 대신:

- `m_actual[6]` (offset 1120) — 실제 관절 토크. 정지 시 노이즈 p2p는 J1~J6
  각각 0.40 / 0.61 / 0.20 / 0.03 / 0.03 / 0.00 N·m. J6는 항상 0이라 제외
- `robot_mode`(offset 24) == 11 또는 `CollisionStates`(offset 1038) != 0 —
  컨트롤러 자체 충돌 감지 (`SetCollisionLevel`로 무장)

30004 포트는 **다중 읽기 클라이언트를 허용**하므로 브링업과 동시에 열어도 된다.
반면 29999 대시보드 소켓은 단일 클라이언트이며 브링업이 점유한다 — 설정 명령은
`/dobot_bringup_ros2/srv/*` 서비스를 거쳐야 한다.

### 9.7 `PositiveKin` 서비스 사용 불가

`dobot_bringup_v4/src/parseTool.cpp`의
`parserPositiveKinRequest2String()`이 j6 뒤에 항상 쉼표를 붙인 뒤 `,user=`를
또 붙여 **이중 쉼표**를 만든다. 결과적으로 항상 `res=-50001`. 로봇을 움직이지 않고
정기구학을 질의하려면 이 함수를 고치고 `dobot_bringup_v4`를 재빌드해야 한다.
이번 조사에서는 실측 자세 두 개로 대신했다.

## 10. 남은 이식 작업

시퀀스(`main.py`, `sequences/shelf_pick_place.py`, `wirebonder_pick_place.py`)를
실물에서 돌리려면 남은 것들. **기구학은 더 이상 여기 없다.**

| 항목 | 내용 | 비고 |
|---|---|---|
| 그리퍼 | `/gripper_controller` 액션, `AttachLink`/`DetachLink` | 하드웨어 대기. import는 이미 선택적으로 처리 |
| `/gazebo/model_states` | 어느 박스 모델이 그 자리인지 조회 | 실물 그리퍼 쓰면 불필요 |
| TF `odom`, `mpo_base_link` | AGV 프레임. 받침대 고정형엔 없음 | `static_transform_publisher` 두 줄 |
| 비전 | 실물 D405 + tag 노드 | `real_robot_transition.md` 참고 |
| 접촉 감지 | 시뮬은 충돌모델을 센서로 사용 | 실물 토크 터치오프는 `test/cbirrt_p1p2_test.py`에 구현됨 |

받침대 고정형이라 `odom`은 사실상 `base_link`와 같다:

```bash
ros2 run tf2_ros static_transform_publisher 0 0 0 0 0 0 odom base_link
ros2 run tf2_ros static_transform_publisher 0 0 0 0 0 0 odom mpo_base_link
```

## 11. 재검증 방법

모델이 실제와 맞는지 **언제든** 확인하는 법. 로봇을 움직이지 않는다.

```bash
# 1) 로봇 자신의 값
docker exec ros2_dobot bash -lc 'source /opt/ros/humble/setup.bash
source /root/dobot_ws/install/setup.bash
ros2 service call /dobot_bringup_ros2/srv/GetPose dobot_msgs_v4/srv/GetPose "{user: 0, tool: 0}"'

# 2) 모델이 계산한 값 (flange: 줄)
~/dobot_ws/src/DOBOT_6Axis_ROS2_V4/test/run.sh --show
```

`flange:` 줄과 `GetPose` 값이 **수 mm 이내**로 일치해야 한다. 어긋나면 부호 반전이
꺼졌거나(`CR7_REAL_ROBOT` 미설정) URDF가 바뀐 것이다.

상시 감시가 필요하면 `/dobot_msgs_v4/msg/ToolVectorActual` 토픽이 로봇 자신의
플랜지 좌표를 10 Hz로 계속 발행하므로, pinocchio FK와 대조하는 워치독을 붙일 수
있다.

## 12. 변경된 파일

| 파일 | 변경 |
|---|---|
| `cr7_pnp/node.py` | `JOINT_SIGN_REAL` 추가, 읽기/보내기 양방향 적용, `CR7_REAL_ROBOT` 옵트인. 별도로 `linkattacher_msgs` import 선택화, `setup_planner(combined_xacro=...)` 인자 추가 |
| `tools/jog_action.py` | 유휴 시 목표 재동기화 + 선행 거리 클램프 (§9.5) |
| `test/cbirrt_p1p2_test.py` | 실물 p1↔p2 pick-place 리허설. 면(책상·벽) 교시, 토크 접촉 감지 |
| `test/run.sh` | 위 스크립트를 컨테이너 안에서 실행하는 래퍼 |
| `test/jog_bringup.sh` | 터미널 A/B + 조그 통합 실행, 상태·중복 확인, enable, 에러 클리어 |
