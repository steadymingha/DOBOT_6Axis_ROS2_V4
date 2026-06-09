# gazebo_ros2_control controller_manager 미시작 버그 수정 로그

## 증상

`run_mpo700_cr7.sh` 실행 후 그리퍼(및 팔) 명령이 전혀 동작하지 않음.

```
[ERROR] [gazebo_ros2_control]: parser error Couldn't parse parameter override rule:
  '--param robot_description:=<?xml version="1.0" ?>...'
[INFO] [_ros2cli]: waiting for service /controller_manager/list_controllers to become available...
[WARN] [_ros2cli]: Could not contact service /controller_manager/list_controllers
(무한 반복)
```

`ros2 control list_controllers` → `/controller_manager` 서비스 자체가 존재하지 않음.

---

## 근본 원인

`gazebo_ros2_control` 0.4.10 (ROS 2 Humble) 플러그인 버그.

**문제 코드** (`gazebo_ros2_control_plugin.cpp` 222~268행):

```cpp
// URDF를 --param 방식으로 전달 시도
std::string rb_arg = std::string("robot_description:=") + urdf_string;
arguments.push_back(RCL_PARAM_FLAG);   // "--param"
arguments.push_back(rb_arg);           // "robot_description:=<robot name="...">..."

rcl_ret_t rcl_ret = rcl_parse_arguments(..., &rcl_args);
if (rcl_ret != RCL_RET_OK) {
    RCLCPP_ERROR(..., "parser error ...");
    return;  // ← controller_manager 생성 전 조기 종료!
}
```

URDF XML 속성값의 `"` 문자(예: `name="cr7_on_mpo700"`)가 rcl의 YAML 파서를 실패시키고, 플러그인이 `controller_manager`를 생성하기 전에 종료됨. 결과적으로 모든 컨트롤러(arm, gripper)가 로드되지 않음.

---

## 수정 내용

### 1. gazebo_ros2_control 패치 빌드

`src/gazebo_ros2_control_patched/` — 0.4.10 소스를 복사해 아래와 같이 패치 후 워크스페이스에서 빌드 (시스템 패키지를 오버레이).

**패치 핵심** (`gazebo_ros2_control_plugin.cpp`):

```cpp
// 수정 전: --param robot_description:=<xml> → YAML 파싱 실패
// 수정 후: 임시 YAML 파일에 block scalar(|)로 기록 → --params-file로 전달

const char * tmpdir = std::getenv("TMPDIR");
std::string tmp_yaml_path = std::string(tmpdir ? tmpdir : "/tmp") +
  "/gazebo_ros2_control_robot_description.yaml";
{
  std::ofstream tmp_yaml(tmp_yaml_path);
  tmp_yaml << "/**:\n";
  tmp_yaml << "  ros__parameters:\n";
  tmp_yaml << "    robot_description: |\n";
  std::istringstream urdf_stream(urdf_string);
  std::string line;
  while (std::getline(urdf_stream, line)) {
    tmp_yaml << "      " << line << "\n";
  }
}
arguments.push_back(RCL_PARAM_FILE_FLAG);  // "--params-file"
arguments.push_back(tmp_yaml_path);
```

YAML `|` block scalar은 `"`, `<`, `>` 등 XML 특수문자를 이스케이프 없이 그대로 저장하므로 파싱 오류 없음.

```bash
colcon build --packages-select gazebo_ros2_control --allow-overriding gazebo_ros2_control
```

### 2. test_w_gripper.py 모델 이름 수정

link attacher(AttachLink/DetachLink) 서비스가 잘못된 Gazebo 모델 이름을 참조하고 있었음.

```python
# 수정 전
self.robot_model = 'cr7_robot'
# 수정 후
self.robot_model = 'cr7_on_mpo700'
```

---

## 결과

수정 후 `ros2 control list_controllers`:

```
joint_state_broadcaster   [joint_state_broadcaster/JointStateBroadcaster]     active
cr7_group_controller      [joint_trajectory_controller/JointTrajectoryController] active
gripper_controller        [joint_trajectory_controller/JointTrajectoryController] active
```

---

## 그리퍼 명령어

| 동작 | positions 값 | 명령 |
|------|-------------|------|
| 완전 열기 | `-0.05` | `ros2 action send_goal /gripper_controller/follow_joint_trajectory ...` |
| 박스 파지 | `0.036` | 동일 |
| 완전 닫기 | `0.07` | 동일 |

> **주의**: `gripper_finger_joint`는 prismatic 조인트, 축 방향 +X = 닫힘.
> 따라서 값이 클수록 닫히고, 작을수록(음수) 열림.

전체 명령 예시 (열기):
```bash
ros2 action send_goal /gripper_controller/follow_joint_trajectory \
  control_msgs/action/FollowJointTrajectory \
  "{trajectory: {joint_names: [gripper_finger_joint], points: [{positions: [-0.05], velocities: [0.0], time_from_start: {sec: 2, nanosec: 0}}]}}"
```
