# TODO

## Goal
`cbirrt_pick_place.py`를 확장하여, **스페이스바 트리거**로 선반의 박스 하나를 집어 로봇 베이스의
매거진 포켓(4칸)에 순차 적재하는 **shelf-to-base** 시퀀스를 구현한다. 충돌 검사는 cube_link·AGV까지
포함한 **pinocchio 자가충돌 모델**로 전환하고, **J1 한계를 확장**하며, **그리퍼를 81 mm 박스용으로 튜닝**하고,
선반 좌표는 런타임에 **odom(≈world)→base_link TF**로 변환한다.
**1차 구현 범위: 높은 층(tier2) 박스 1개 → 베이스 포켓 1개** (트리거 1회 = 1사이클). 4칸 적재는 1차 동작 확인 후 확장.

## Tasks

### 1. Pinocchio 충돌 모델 (팔 + cube + AGV + 그리퍼)
- [x] `cr7_on_mpo700.urdf.xacro`로 pinocchio 충돌 모델 빌드(`lock_non_arm=True`): 팔 6축 외 모든 가동 조인트(`gripper_finger_joint`)를 neutral lock → cube_link·MPO-700 본체가 고정 부착 geometry로 포함됨(geom 27개: arm/gripper + cube_link_0/1 + mpo_base + lidar/caster/wheel).
- [x] `addAllCollisionPairs` → SRDF-disabled 제거 → neutral-colliding 제거 + `arm_pairs_only=True`로 팔 포함 쌍만 유지(215쌍, arm↔fixed 179쌍 포함).
- [x] `is_collision_free(q_j1..j6) -> bool` 노출(`computeCollisions`, 0.11 ms/check).
- [x] mesh 로딩: cr7/gripper는 `file://`, MPO-700/lidar는 `package://`를 sourced ament로 해석 → 전부 로드 확인.
- [x] 검증: overhead standby → True, 접힌 자세 → False(`cube_link_0↔gripper_finger` = 팔이 cube에 부딪힘, `Link2↔Link4` = self 모두 검출).

### 2. cbirrt_pick_place에서 pinocchio 충돌 사용
- [x] `setup_planner`에서 `ReachabilityModel`(결합 모델) 1회 빌드(`self.collision`), `is_state_valid`를 pinocchio 검사기로 오버라이드 → RRT/CBiRRT/servo 전부 이 경로 사용(MoveIt `/check_state_validity` 대체).
- [x] IK 후보 게이팅: `compute_ik_ordered`에서 within-limit AND `is_state_valid`인 후보만 채택 → goal 자세가 충돌-free 보장.
- [x] IK 자체는 기존 `/compute_ik` 유지. 스모크 테스트: 노드 빌드 OK, override가 subclass에서 적용(overhead True / folded False).

### 3. J1 한계 확장
- [x] `CBiRRTPickPlace.setup_planner`에서 `joint_limits[0]=(-180°, 90°)` 오버라이드(test_w_gripper 미수정), J2~J6 유지. 검증: J1=(-180,90), 나머지 그대로.
- [x] URDF joint1 하드웨어 한계 ±6.27 rad 내 → soft limit만 변경.

### 4. 그리퍼 81 mm 박스 튜닝 (옆 매거진 충돌 회피)
- [x] grasp 자세 = `DOWN` + 런타임 yaw(`quat_mul(quat_about_z(phi), DOWN)`); `phi`는 선반 row 방향(world X)을 base_link로 변환해 `atan2`로 산출 → jaw가 매거진 열에 정렬. fixed jaw가 gap 쪽으로 진입(삽입 방향 world +Y ⟂ jaw 방향 world X).
- [x] `GRIPPER_OPEN=0.05`(박스가 fixed jaw↔finger 사이에 들어감), `GRIPPER_CLOSE=-0.036`(81 mm 가볍게 파지) 설정.
- [ ] (sim 필요) `GRASP_YAW_OFFSET`(Link6→jaw 방위 흡수)를 시뮬에서 튜닝, 박스가 튕기지 않고 옆 매거진을 안 치는지 확인.

### 5. 선반 박스 좌표 런타임 변환 (world→base_link)
- [x] 노드에 TF2 listener 추가, `transform_world_pose`(odom→base_link PoseStamped)·`transform_world_vector`(방향 회전) 구현. odom을 world 프록시로 사용.
- [x] TF 조회 실패 시 명확한 에러 로그 + None 반환(사이클 중단 가능). 스모크 테스트: /tf 없을 때 graceful None.
- [ ] (sim 필요) 선반 박스 world 좌표(tier2 `box_l2a` 0.7095/0.5/0.77)를 실제 TF로 변환한 base_link 값이 손계산(x=0.18525−x_w 등)과 일치하는지 확인.

### 6. 매거진 포켓 타깃 (base_link, 상수)
- [x] 상수 정의: `POCKET_X=0.3705`, `POCKET_Y=[0.177,0.059,-0.059,-0.177]`, `POCKET_SURFACE_Z=-0.05`. EE 놓기 높이 = 표면 + `PLACE_ABOVE_SURFACE`(튜닝값). 1차는 `POCKET_Y[0]` 사용.

### 7. 일반 Cartesian 직선 servo
- [x] `constrained_cbirrt.linear_path`(임의 base_link delta) 추가, `lift_path`는 이를 호출하는 래퍼로 유지. `cbirrt_pick_place.linear_servo` 추가, `vertical_servo`는 래퍼. 검증: +z/−z/+x/−x 정상(특이점에선 graceful stop).

### 8. shelf-to-base 시퀀스 (1차: tier2 박스 1개 → 포켓 1개)
- [ ] 순서대로 세그먼트 구현:
  1. RRT로 선반 박스 앞 pre-grasp(그리퍼 down + **J6 yaw로 jaw축을 매거진 열 방향에 정렬**, fixed jaw가 gap 쪽).
  2. 직선 전진으로 틈(박스 윗면 ↔ 위 선반판) 진입 — fixed jaw가 옆 매거진 사이 gap으로 들어가도록.
  3. 직선 하강하여 박스에 접근.
  4. 그리퍼 닫기(+선택적 attach).
  5. 직선 상승.
  6. 직선 후퇴로 선반에서 빠져나옴.
  7. RRT로 베이스 포켓 위 hover.
  8. 직선 하강.
  9. 그리퍼 열기(+선택적 detach).
  10. standby 자세 복귀.
- [x] `shelf_to_base_cycle(node, box_world, pocket_y)` 10단계 구현. 박스 위치/축은 런타임 TF로 해석, grasp/place 자세는 위 yaw 로직 사용.
- [x] 각 세그먼트 실패 시 명확한 로그 + `False` 반환 → main 루프가 "재배치 후 재트리거" 안내.
- [ ] (sim 필요) 높이/클리어런스 상수(`EE_TO_GRIP`, `INSERT_LIFT`, `PREGRASP_BACK`, `PLACE_ABOVE_SURFACE` 등) 시뮬 튜닝. tier2 위 6 cm 틈 진입 가능성 확인(불가 시 다른 방법 협의).

### 9. 스페이스바 트리거 루프
- [x] `wait_for_spacebar()` 추가: termios raw mode로 SPACE→'go', q/Esc/Ctrl-C→'quit'. ROS 토픽/서비스 교체 쉽게 분리.
- [x] main 루프: standby 이동 → SPACE 대기 → `shelf_to_base_cycle`(tier2 박스→포켓0) → 성공/실패 안내 → 반복. 다중 박스/4칸은 추후 확장.
- [ ] (sim 필요) 실제 Gazebo+MoveIt 기동 상태에서 스페이스바 트리거 1사이클 end-to-end 동작 확인.

## 참고사항
- **프레임:** base_link는 팔 루트로 AGV에서 yaw 180° 회전. 베이스 포켓은 base_link에 **상수**, 선반 박스는 world에 있어 런타임 `odom(≈world)→base_link` 변환 필요.
- TF 트리에 `world` 프레임 없음 → `odom`을 world 프록시로 사용(AGV가 world 원점 스폰 / odom 미리셋 동안만 유효).
- pinocchio 모델은 팔+cube+AGV+그리퍼 포함하나 **선반은 미포함**(선반은 Gazebo world 모델, 로봇 URDF 아님) → 선반 충돌은 plan이 아닌 직선 waypoint로 회피.
- 실행 시 ROS 워크스페이스 source 필요(결합 모델 xacro가 `cra_description`+`neo_simulation2` include를 ament로 해석).
- 리스크: MPO-700 충돌 메시가 무거울 수 있음 → 팔 포함 쌍만 남겨 `computeCollisions` 속도 확보.
