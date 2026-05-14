# DOBOT 6Axis ROS2 V4 - 설치 및 에러 대응 로그

**작성일**: 2026-05-12  
**환경**: Ubuntu 22.04 LTS, ROS2 Humble, Python 3.10  
**목표**: 시뮬레이션 환경(Gazebo + MoveIt + RViz) 구축  
**사용 로봇 모델**: CR7 (`DOBOT_TYPE=cr7`)

---

## 1. ROS2 Humble 설치

### 1-1. locale 및 사전 준비
```bash
echo '비밀번호' | sudo -S apt update
echo '비밀번호' | sudo -S apt install -y locales software-properties-common curl
echo '비밀번호' | sudo -S locale-gen en_US en_US.UTF-8
echo '비밀번호' | sudo -S update-locale LC_ALL=en_US.UTF-8 LANG=en_US.UTF-8
```
**결과**: 성공

### 1-2. ROS2 apt 소스 등록
```bash
echo '비밀번호' | sudo -S add-apt-repository universe -y
sudo curl -sSL https://raw.githubusercontent.com/ros/rosdistro/master/ros.key \
  -o /usr/share/keyrings/ros-archive-keyring.gpg
echo "deb [arch=$(dpkg --print-architecture) signed-by=/usr/share/keyrings/ros-archive-keyring.gpg] \
  http://packages.ros.org/ros2/ubuntu jammy main" | sudo tee /etc/apt/sources.list.d/ros2.list
echo '비밀번호' | sudo -S apt update
```
**결과**: 성공

### 1-3. ROS2 Humble Base 설치 (Desktop 대신 최소 설치)
```bash
DEBIAN_FRONTEND=noninteractive echo '비밀번호' | sudo -S apt install -y \
  ros-humble-ros-base python3-colcon-common-extensions
```
**결과**: 성공 (ros-humble-ros-base 0.10.0)

---

## 2. Gazebo 관련 패키지 설치

```bash
DEBIAN_FRONTEND=noninteractive echo '비밀번호' | sudo -S apt install -y \
  ros-humble-gazebo-ros-pkgs
```
**설치된 주요 패키지**:
- `gazebo` 11.10.2 (Gazebo 물리 엔진)
- `ros-humble-gazebo-ros` 3.9.0 (ROS2-Gazebo 브리지)
- `ros-humble-gazebo-plugins` 3.9.0
- `ros-humble-gazebo-dev` 3.9.0

**결과**: 성공

---

## 3. MoveIt2 설치

```bash
DEBIAN_FRONTEND=noninteractive echo '비밀번호' | sudo -S apt install -y ros-humble-moveit
```
**설치된 주요 패키지**:
- `ros-humble-moveit` 2.5.9
- `ros-humble-moveit-ros-move-group`
- `ros-humble-moveit-planners-ompl` (OMPL, CHOMP, PILZ 플래너)
- `ros-humble-moveit-ros-visualization` (RViz 플러그인)
- `rviz2` (MoveIt 의존성으로 자동 설치)

**결과**: 성공

---

## 4. ros2_control 및 컨트롤러 설치

```bash
DEBIAN_FRONTEND=noninteractive echo '비밀번호' | sudo -S apt install -y \
  ros-humble-ros2-control \
  ros-humble-ros2-controllers \
  ros-humble-gazebo-ros2-control
```
**결과**: 성공

---

## 5. uv (Python 패키지 관리자) 설치 및 가상환경 설정

```bash
curl -LsSf https://astral.sh/uv/install.sh | sh
source $HOME/.local/bin/env

cd /home/user/workspace/DOBOT_6Axis_ROS2_V4
uv venv .venv --python 3.10
uv sync
```
**설치된 Python 패키지**:
- `moveit==0.7.5`
- `numpy==2.2.6`
- `svgwrite==1.4.3`

**결과**: 성공 (uv 0.11.13)

---

## 6. 워크스페이스 설정 및 빌드

```bash
mkdir -p ~/dobot_ws/src
ln -sfn /home/user/workspace/DOBOT_6Axis_ROS2_V4 ~/dobot_ws/src/DOBOT_6Axis_ROS2_V4

source /opt/ros/humble/setup.bash
export DOBOT_TYPE=cr7
cd ~/dobot_ws
colcon build --symlink-install
```

**빌드 완료 패키지** (3분 41초):
- cra_description ✓
- dobot_msgs_v4 ✓
- dobot_rviz ✓
- dobot_demo ✓
- dobot_demo_cpp ✓
- dobot_moveit ✓
- servo_action ✓
- dobot_gazebo ✓
- cr7_moveit ✓
- cr_robot_ros2 ✓

**결과**: 모든 패키지 빌드 성공

---

## 7. 환경 변수 설정 (bashrc에 추가 권장)

```bash
echo "source /opt/ros/humble/setup.bash" >> ~/.bashrc
echo "source ~/dobot_ws/install/local_setup.bash" >> ~/.bashrc
echo "export DOBOT_TYPE=cr7" >> ~/.bashrc
echo "export DISPLAY=:0" >> ~/.bashrc
source ~/.bashrc
```

---

## 8. 실행 방법 (반드시 별도 터미널 2개)

### 터미널 1 - Gazebo 시뮬레이션 시작:
```bash
export DISPLAY=:0
source /opt/ros/humble/setup.bash
source ~/dobot_ws/install/local_setup.bash
export DOBOT_TYPE=cr7
ros2 launch dobot_gazebo gazebo_moveit.launch.py
```
**정상 출력 확인 포인트**:
- `Successfully spawned entity [cr7_robot]` → 로봇 스폰 완료
- `Successfully loaded controller joint_state_broadcaster into state active`
- `Successfully loaded controller cr7_group_controller into state active`

### 터미널 2 - MoveIt + RViz 시작 (터미널 1에서 Gazebo가 완전히 뜬 후):
```bash
export DISPLAY=:0
source /opt/ros/humble/setup.bash
source ~/dobot_ws/install/local_setup.bash
export DOBOT_TYPE=cr7
ros2 launch dobot_moveit moveit_gazebo.launch.py
```
**정상 출력 확인 포인트**:
- `You can start planning now!` → MoveIt 준비 완료
- RViz 창에 CR7 로봇 모델 표시

---

## 9. 에러 대응 이력

### 에러 1: gzclient 크래시 (DISPLAY 없을 때)
- **증상**: `[ERROR] [gzclient-2]: process has died [exit code -6]`
- **원인**: DISPLAY 환경변수 미설정 (헤드리스 환경)
- **해결**: `export DISPLAY=:0` 추가 후 정상 동작

### 에러 2: gzserver exit 255 (포트 충돌)
- **증상**: `[ERROR] [gzserver-1]: process has died [exit code 255]`
- **원인**: 이전 gzserver 인스턴스가 11345 포트를 점유 중
- **해결**: `pkill -f gzserver` 로 기존 프로세스 종료 후 재시작

### 에러 3: load_controller 실패 (백그라운드 실행 시)
- **증상**: `xmlrpc.client.Fault: <Fault 1: "<class 'RuntimeError'>:!rclpy.ok()">`
- **파일**: `/opt/ros/humble/local/lib/python3.10/dist-packages/controller_manager/controller_manager_services.py`
- **원인**: `ros2 launch`를 백그라운드(`&`)로 실행 시 controller_manager 초기화 전에 `load_controller` CLI가 실행되는 타이밍 문제
- **해결**: 반드시 **포그라운드(foreground)**에서 실행. 백그라운드 실행 금지

### 에러 4: RViz robot_description 못 찾음 (단독 실행 시)
- **증상**: `Could not find parameter robot_description ... within 10.000000 seconds`
- **원인**: `moveit_gazebo.launch.py`를 `gazebo_moveit.launch.py` 없이 단독 실행
- **해결**: 터미널 1에서 `gazebo_moveit.launch.py` 먼저 실행 후, 터미널 2에서 `moveit_gazebo.launch.py` 실행

### 에러 5: /recognize_objects Action server 없음 (경고, 무시 가능)
- **증상**: `[ERROR] Action server: /recognize_objects not available`
- **원인**: 3D 물체 인식 기능 미사용 (Octomap 플러그인 비활성)
- **해결**: 동작에 영향 없음. 무시해도 됨

---

## 10. 테스트 결과 요약

| 기능 | 상태 |
|------|------|
| ROS2 Humble 설치 | ✅ 성공 |
| Gazebo 11 설치 및 실행 | ✅ 성공 |
| MoveIt2 설치 | ✅ 성공 |
| ros2_control / 컨트롤러 | ✅ 성공 |
| uv 가상환경 설정 | ✅ 성공 |
| colcon 빌드 (10 패키지) | ✅ 성공 |
| CR7 로봇 Gazebo 스폰 | ✅ 성공 |
| MoveIt + RViz 연동 | ✅ 성공 |
| gemini_demo.py 로봇 구동 | ✅ 동작 확인 |

