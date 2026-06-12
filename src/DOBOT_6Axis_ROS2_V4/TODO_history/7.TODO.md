# TODO

## Goal
OnRobot 2FG7 그리퍼를 Blender 제작 fixed-jaw parallel 그리퍼(`src/blender/gripper/`)로 교체한다.

## Tasks

### 1. URDF 교체 — cr7_on_mpo700.urdf.xacro (Gazebo 시뮬 메인)
- [x] `<xacro:include>` onrobot_2fg7 제거
- [x] `<xacro:onrobot_2fg7>` + `<xacro:onrobot_2fg7_fingertip>` 3개 호출 제거
- [x] 기존 `<gazebo reference="gripper_*_onrobot_*">` 블록 제거
- [x] 기존 `<ros2_control>` 내 `gripper_gripper_joint` / `gripper_right_finger_joint` 항목 제거
- [x] 새 gripper_base_link / gripper_finger_link 및 고정 마운트 joint(`gripper_attach_joint`, z=-0.1443) 추가
- [x] 새 `gripper_finger_joint` (prismatic, axis +X, [-0.05, 0.07]) 추가
- [x] 새 Gazebo 링크 설정 (DarkGrey/Orange, gravity=false, 마찰계수) 추가
- [x] `<ros2_control>` 에 `gripper_finger_joint` 단일 항목 추가 (초기값 -0.04)

### 2. URDF 교체 — cr7_robot.xacro (pinocchio용 standalone)
- [x] cr7_on_mpo700.urdf.xacro 와 동일한 그리퍼 섹션 교체

### 3. ros2_controllers.yaml 업데이트
- [x] `gripper_controller.joints` 를 `[gripper_finger_joint]` 으로 교체
- [x] `constraints` 섹션을 `gripper_finger_joint` 단일 항목으로 교체

### 4. cr7_robot.srdf 업데이트
- [x] 새 그리퍼 링크 쌍에 대한 `<disable_collisions>` 항목 추가

### 5. test_w_gripper.py 업데이트
- [x] `control_gripper()` joint_names: `['gripper_finger_joint']`, positions 단일값
- [x] `self.gripper_link = 'gripper_base_link'` 로 변경

### 6. constrained_cbirrt.py 업데이트
- [x] `locked_joints` 기본값: `('gripper_finger_joint',)` 으로 변경

### 7. cbirrt_pick_place.py 업데이트
- [x] `GRIPPER_OPEN = [-0.04]` (pusher 후퇴, 약 4 cm 오픈)
- [x] `GRIPPER_CLOSE = [0.01]` (박스 클램핑, 0 보다 약간 닫힘)

### 8. 빌드
- [x] `colcon build --packages-select cra_description cr7_moveit`

## 참고사항
- 새 그리퍼 메시: `file:///home/user/dobot_ws/src/blender/gripper/meshes/base.dae` (절대경로, blender 는 ROS 패키지 아님)
- 마운트 오프셋 z = -0.1443: 플랜지 장착면(z=+0.1443 in gripper frame)이 Link6 원점에 맞도록
- 그립 중심은 gripper frame z ≈ 0 ~ +0.007 (Link6 에서 약 0.137 m 아래)
- 현재 Z_GRASP=0.24 는 새 그리퍼에서도 그립 범위 내 → 변경 불필요
- 어태치 링크: `gripper_left_attachment` → `gripper_base_link` (고정 L-jaw)
- `reachability_map.py` 의 GRIPPER_JOINTS / TCP_OFFSET_M 은 이번 범위 외 (run_mpo700_cr7.sh 와 무관)
