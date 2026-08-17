# 실물 로봇 Jetson 브링업 — 문제·해결·실행법

2026-08-04, CR7 실물 로봇을 Jetson AGX Orin(JetPack 5.1.2 / Ubuntu 20.04, L4T R35.4.1)
컨트롤러에 연결하면서 겪은 문제와 해결, 새로 만든 도구, 실행 방법을 정리한다.
`real_robot_transition.md`(비전 관련)와 별개로, 이 문서는 **Jetson 인프라 + 모션
실행 경로**를 다룬다.

---

## 1. 왜 Docker인가

Jetson은 Ubuntu 20.04(focal)인데 ROS2 Humble은 22.04(jammy)용으로만 공식 apt 배포된다.
호스트에 Humble을 소스빌드하면 rosdep이 시스템 apt 패키지(opencv, numpy 등)를 건드려
**비전 담당자가 이미 세팅해둔 Python 환경(YOLOX, venv_ammr, torchvision)과 충돌할 위험**이
있었다. → 호스트는 전혀 건드리지 않는 **Docker 컨테이너**(공식 `ros:humble-ros-base`
arm64 이미지)로 격리했다.

```bash
docker run -d --name ros2_dobot --network host --restart unless-stopped \
  -v /home/ammrdev/dobot_ws:/root/dobot_ws \
  -v /dev:/dev \
  -v /tmp/.X11-unix:/tmp/.X11-unix \
  -v /run/user/1000/gdm/Xauthority:/root/.Xauthority \
  -e DISPLAY=:1 \
  -e XAUTHORITY=/root/.Xauthority \
  --device-cgroup-rule="c 81:* rmw" \
  --device-cgroup-rule="c 189:* rmw" \
  --device-cgroup-rule="c 13:* rmw" \
  ros:humble-ros-base tail -f /dev/null
```

- `--network host`: 컨테이너가 호스트 네트워크 스택을 그대로 씀 (로봇 TCP/IP 통신,
  D405 realsense 토픽에 별도 포트포워딩 불필요)
- `-v /dev:/dev` + `--device-cgroup-rule`: `/dev`를 마운트하는 것만으론 안 됨 —
  Docker의 device cgroup이 major 번호별로 따로 막는다. major 81(video4linux, D405),
  189(USB), 13(input, 게임패드 `/dev/input/js0`)를 각각 허용해야 실제로 열린다.
  **cgroup 허용 안 하면 파일은 보이는데 `open()`이 `Operation not permitted`로 막힘.**
- X11 마운트: D405 라이브 화면을 Jetson 모니터에 띄우기 위함 (호스트 Xorg `:1`,
  `ammrdev` 세션의 Xauthority 재사용)
- apt로 깐 패키지(`ros-humble-control-msgs`, `ros-humble-realsense2-camera` 등)는
  컨테이너의 쓰기 가능 레이어에만 있음 → 컨테이너를 지우면 같이 날아간다. 재생성할 때는
  꼭 `docker commit ros2_dobot ros2_dobot:snapshotN`으로 먼저 스냅샷 떠서 보존.

## 2. 로봇 네트워크 연결

CR7 컨트롤러 공장 기본 IP는 `192.168.5.1` — 사내 LAN(`192.168.0.0/22`)과 다른 대역이라
**LAN 케이블 직결**이 전제. Jetson의 유선 포트(`eth0`)에 연결.

### 문제: `ip addr add`가 자꾸 사라짐

`sudo ip addr add 192.168.5.100/24 dev eth0`로 임시로 넣었는데, 얼마 뒤 확인하면
`eth0`에 IP가 통째로 사라져 있었다. 원인: **NetworkManager가 `eth0`를 관리 중**이고,
커널에 직접 넣은 IP는 NM이 모르는 상태라 NM이 인터페이스를 재조정할 때 지워버림.

### 해결: NetworkManager 프로파일에 고정

```bash
nmcli device status | grep eth0                # 관리 여부 확인
sudo nmcli connection modify "Wired connection 1" \
  ipv4.method manual ipv4.addresses 192.168.5.100/24
sudo nmcli connection up "Wired connection 1"
```

이제 재부팅해도 유지된다. 로봇 도달성 확인:

```bash
ping -c 3 192.168.5.1
```

## 3. `MoveJog`의 `coordtype` 함정 (ErrorID -6)

Cartesian 축(X/Y/Z/Rx/Ry/Rz)을 `MoveJog(X+)`처럼 파라미터 없이 보내면 **항상 -6 에러**로
거부된다. 공식 프로토콜 문서(`V4新增指令/Dobot TCP_IP Remote Control Interface Guide
V4.6.0`) 93페이지:

> If axisID corresponds to a Cartesian axis, coordtype must be either 1 or 2.
> Using 0 will return error code -6.

관절 축(J1..J6)은 `coordtype`이 필요 없어서 이 문제를 안 겪는다 — 그래서 처음엔
"JOINT 모드는 되는데 TCP 모드만 아무 반응 없음"으로 보였다. 해결: Cartesian 축일 때
`param_value=["coordtype=1", "user=0"]`을 같이 보낸다 (`tools/jog_real.py`의
`Jogger.jog()`).

## 4. 게임패드 (joydev, `/dev/input/js0`)

연결된 패드는 "Dual PSX Adaptor"(`0810:0001`) — Xbox가 아니라 **범용 PS형 패드**.
문서에 적힌 세 가지 패드 부류(Xbox+xpad / Xbox One+hid-generic / 범용 PS DirectInput)
중 세 번째에 해당. 실측 결과:

- axis 0/1 = 왼쪽 스틱 X/Y, axis 2/3 = 오른쪽 스틱 X/Y — 전부 풀 아날로그
- axis 4/5는 존재는 하지만 한 번도 반응 안 함 (이 어댑터엔 아날로그 트리거 없음)
- D-pad를 눌러도 axis 0/1과 같은 축을 재사용함 → **별도 D-pad 축이 없어서 Rx/Ry는
  게임패드로 못 뽑음** (키보드로만 가능)
- 버튼 4 = L1 (`--js-test`로 직접 눌러서 확인)

이런 매핑은 패드마다 다르므로 절대 추측하지 말고 `tools/gamepad.py --js-test`로
실측해야 한다 (아무 명령도 로봇에 안 보냄, 순수 축/버튼 리더).

### 버그: `close()`가 프로세스를 영원히 멈춤

`Gamepad.close()`가 리더 스레드가 읽고 있는 파일 객체를 `self._fh.close()`로 닫으려
했는데, 리더 스레드가 `read()`에 블로킹돼 있는 상태에서 메인 스레드가 `close()`를
부르면 **둘 다 같은 내부 IO 락을 원해서 데드락**이 난다 — Ctrl+C를 눌러도 `finally`
블록이 `close()`에서 영원히 멈춰 프로세스가 안 죽었다. `py-spy dump`로 실제 스택을
떠서 확인함 (컨테이너는 `SYS_PTRACE` capability가 없어서 py-spy가 안 됨 → 호스트에서
`docker top`으로 실제 호스트 PID를 찾아 호스트에서 떴다). 해결: 리더 스레드는 데몬
스레드이므로 `_fh.close()`를 아예 안 부르고 `_running=False`만 세팅 — 프로세스 종료
시 OS가 fd를 알아서 정리한다.

## 5. `rclpy.init()`이 Ctrl+C를 삼킴

`rclpy.init()`은 기본적으로 자체 SIGINT 핸들러를 설치해서, Ctrl+C가 파이썬 표준
`KeyboardInterrupt`로 올라가지 않고 rclpy가 조용히 처리해버린다. `finally` 블록(로봇
정지 코드)이 아예 실행이 안 되는 문제였다. 해결:

```python
from rclpy.signals import SignalHandlerOptions
rclpy.init(signal_handler_options=SignalHandlerOptions.NO)
```

## 6. `action_move_server.py`가 `time_from_start`를 무시함

`cr7_pnp/node.py`의 `execute_path`/`linear_servo`는 CBiRRT/Jacobian으로 계산한
조밀한 관절공간 웨이포인트를 각자 다른 `time_from_start`로 하나의
`FollowJointTrajectory` goal에 담아 보낸다 (직선 이동은 이미 소프트웨어에서 관절공간
경로로 미리 풀어놓으므로 **`ServoP`는 필요 없다** — 실행은 항상 `ServoJ` 하나로 충분).

그런데 서버(`action_move_server.py`)는 이 타이밍을 통째로 무시하고 **포인트 하나당
고정 0.18초**로 `ServoJ`를 쐈다. Dobot 공식 문서(`ServoJ` 섹션)는 **33Hz(30ms 간격)**
호출을 권장한다:

> The calling frequency is recommended to be set to 33Hz ... issue the
> speed-planned points at a fixed interval t to ensure that the robot can
> smoothly track the target point.

고정 0.18초 방식의 문제:
- 웨이포인트가 촘촘한 정밀 구간(느린 직선 이동) → 의도한 것보다 훨씬 느려짐
- 웨이포인트가 듬성듬성한 구간(빠른 이동) → 큰 관절 이동을 0.2초 안에 욱여넣어 저크·
  추종오차 알람 위험

해결: 30ms(`SERVOJ_DT`) 간격으로 궤적을 **선형보간 리샘플링**해서 `ServoJ`를 흘려보내게
`execution_trajectory()`를 재작성. 포인트가 1개(조그처럼 단일 목표)면 예전처럼
`ServoJ` 한 번으로 끝냄 (리샘플링 불필요 — 로봇이 그 자체로 부드럽게 도달함).

**재빌드 필요**: 이 파일은 `ros2 launch dobot_moveit ...`가 `install/`의 설치된
사본을 실행하므로, 소스만 고치면 반영 안 됨:

```bash
cd ~/dobot_ws && colcon build --packages-select dobot_moveit --cmake-args -DBUILD_TESTING=OFF
```

## 7. 새로 만든 도구 (`tools/`)

| 파일 | 역할 | 의존 |
|---|---|---|
| `check_real_robot.py` | 소켓 레벨 순수 조회(29999/30004). 로봇에 명령 안 보냄(`--request-control` 제외). bringup도 필요 없음, param.json IP만 읽음 | ROS2 불필요, stdlib만 |
| `gamepad.py` | joydev(`/dev/input/jsN`) 리더. `--js-test`로 축/버튼 실측 | stdlib만 |
| `jog_real.py` | 키보드+게임패드 단축 조그. `MoveJog`/`StopMoveJog` 기반, 워치독으로 자동 정지 | 터미널 A만 |
| `jog_action.py` | 게임패드 4축 + 키보드 J5/J6 **동시** 다관절 조그. `FollowJointTrajectory` 기반 | 터미널 A + B |

### `jog_real.py` 안전 모델

`MoveJog`는 "정지 명령 받을 때까지 계속" 움직이는 연속 지령이라, 키를 누를 때마다
데드라인(0.25초)이 갱신되고 워치독 스레드가 만료 즉시 `StopMoveJog`를 보낸다. 종료 시
워치독/게임패드 스레드를 먼저 멈추고(`stop_event.set()`) 나서 마지막 정지 명령을
보내도록 순서를 잡았다 — 반대 순서로 하면 여러 스레드가 동시에 같은 ROS 노드를
스핀하려다 종료가 멈추는 문제가 있었다.

### `jog_action.py` 안전 모델

`ServoJ`는 "이 목표로 t초 안에 이동 후 홀드"라 `MoveJog`와 달리 별도 정지 명령이
필요 없다 — 입력이 없으면 그냥 새 goal을 안 보내고, 로봇은 마지막 목표에서 멈춰
있는다. 대신 게임패드는 **데드맨(L1, 버튼4)을 누르고 있을 때만** 스틱이 작동한다.
백그라운드에서 `rclpy.spin(node)`를 계속 돌려야 액션 goal의 accept/result 콜백이
처리된다 — 처음엔 이걸 빼먹어서 goal을 쏘기만 하고 응답을 처리 안 했다(양쪽 스틱을
동시에 움직여도 조합이 안 되는 것처럼 보였던 원인 중 하나).

## 8. 실행 방법

### 8.1 사전 준비 (한 번만)

```bash
# Jetson eth0 고정 IP (§2)
sudo nmcli connection modify "Wired connection 1" ipv4.method manual ipv4.addresses 192.168.5.100/24
sudo nmcli connection up "Wired connection 1"

# 컨테이너 (§1) — 이미 떠 있으면 생략
docker run -d --name ros2_dobot --network host --restart unless-stopped \
  -v /home/ammrdev/dobot_ws:/root/dobot_ws -v /dev:/dev \
  -v /tmp/.X11-unix:/tmp/.X11-unix -v /run/user/1000/gdm/Xauthority:/root/.Xauthority \
  -e DISPLAY=:1 -e XAUTHORITY=/root/.Xauthority \
  --device-cgroup-rule="c 81:* rmw" --device-cgroup-rule="c 189:* rmw" --device-cgroup-rule="c 13:* rmw" \
  ros:humble-ros-base tail -f /dev/null
docker exec ros2_dobot bash -c "apt-get update && apt-get install -y ros-humble-control-msgs python3-colcon-common-extensions"

# 워크스페이스 빌드
docker exec ros2_dobot bash -c "source /opt/ros/humble/setup.bash && cd /root/dobot_ws && colcon build --cmake-args -DBUILD_TESTING=OFF"
```

### 8.2 연결 확인 (로봇에 명령 안 보냄)

```bash
docker exec ros2_dobot python3 /root/dobot_ws/src/DOBOT_6Axis_ROS2_V4/tools/check_real_robot.py 192.168.5.1
```

`Control Mode Is Not Tcp` 응답이 오면 펜던트가 TCP/IP 이차개발 모드가 아니거나 다른
클라이언트가 붙어있는 것 — `--request-control`(disable 상태에서만 먹음) 시도.

### 8.3 터미널 A — bringup

```bash
docker exec -it ros2_dobot bash -c "
source /opt/ros/humble/setup.bash
source /root/dobot_ws/install/setup.bash
export DOBOT_TYPE=cr7
ros2 launch cr_robot_ros2 dobot_bringup_ros2.launch.py"
```

### 8.4 터미널 B — action server + joint_states (다관절 조그/pick-place용, 단축 조그만 할 거면 생략 가능)

```bash
docker exec -it ros2_dobot bash -c "
source /opt/ros/humble/setup.bash
source /root/dobot_ws/install/setup.bash
export DOBOT_TYPE=cr7
ros2 launch dobot_moveit dobot_joint.launch.py"
```

### 8.5 조그

```bash
# 단축(MoveJog) — 터미널 A만 필요
docker exec -it ros2_dobot bash -c "
source /opt/ros/humble/setup.bash
source /root/dobot_ws/install/setup.bash
python3 /root/dobot_ws/src/DOBOT_6Axis_ROS2_V4/tools/jog_real.py --speed 5"

# 동시 다관절(FollowJointTrajectory) — 터미널 A+B 필요
docker exec -it ros2_dobot bash -c "
source /opt/ros/humble/setup.bash
source /root/dobot_ws/install/setup.bash
export DOBOT_TYPE=cr7
python3 /root/dobot_ws/src/DOBOT_6Axis_ROS2_V4/tools/jog_action.py --speed 10"
```

`jog_real.py` 키: `w/s a/d r/f`=X/Y/Z(JOINT: J1/J2/J3), `u/j i/k o/l`=Rx/Ry/Rz(JOINT:
J4/J5/J6), `SPACE` 정지, `m` TCP↔JOINT, `-`/`+` 속도, `p` 위치, `e`/`x`/`c`
enable/disable/clear error, `q` 종료.

`jog_action.py` 키: 관절 전용(JOINT_KEYS 동일 배치), 게임패드는 **L1(버튼4) 누른 채
양쪽 스틱**으로 최대 4관절(J1~J4) 동시 조그 + 키보드로 J5/J6 추가 조합 가능.

### 8.6 코드 수정 후 반영

- `tools/*.py`: 소스에서 직접 실행하므로 **재빌드 불필요**, 파일만 바꾸고 재실행
- `dobot_moveit/dobot_moveit/action_move_server.py`, `dobot_bringup_v4/config/param.json`
  등 패키지에 설치되는 파일: **`colcon build --packages-select <pkg>` 후 해당 터미널
  재시작 필수** (안 하면 옛날 `install/` 사본이 계속 실행됨 — 이걸 몰라서 몇 번
  헤맸다)

## 9. 남은 것

> **먼저 읽을 것 — `real_robot_joint_convention.md`**
> URDF가 J1/J5/J6의 회전축을 컨트롤러와 **반대로** 정의하고 있어, 그대로 두면
> pinocchio 스택(CBiRRT/IK/충돌검사)이 팔을 거울 반전된 자세로 몬다. 에러 없이
> 조용히 일어난다. 2026-08-06에 원인 규명 및 수정 완료(`CR7_REAL_ROBOT=1`로
> 옵트인). 실물에서 `cr7_pnp`를 쓰는 모든 작업은 그 문서를 먼저 볼 것.
> 조그(`jog_real.py`/`jog_action.py`)는 URDF를 안 거치므로 이 문제와 무관하다.

- 그리퍼: DO 또는 RS485로 컨트롤러 직결 예정 (이 문서 범위 밖)
- `shelf_pick_place.py` 실물 전환: 그리퍼 제외하면 이동 경로(`execute_path`/
  `linear_servo`)는 서버 수정 없이 그대로 실행 가능 — 남은 건 `real_robot_transition.md`
  1-D의 `AttachLink`/`DetachLink`(Gazebo 전용, grasp 확인용) 및 `/gazebo/get_entity_state`
  (물체 위치 참값) 교체
- 게임패드 정지 버튼(버튼5, R1으로 추정)은 미검증 — 키보드 SPACE가 항상 최종 안전장치
