# 02. 모션 라이브러리 — `cr7_pnp/`

자체 완결형 모션 패키지. 시퀀스는 여기서 import만 하고 프리미티브를 조합함.
트리거(스페이스바/MCS)는 라이브러리 밖. 더 상세한 영문 문서: `docs/ARCHITECTURE.md`.

```
cr7_pnp/
├── __init__.py        공개 API 재수출 (노드 3계층 + geometry 상수/헬퍼)
├── node.py            CR7Node → CBiRRTPickPlace → HubPickPlace  (핵심, ~1500줄)
├── model.py           ReachabilityModel — pinocchio FK/IK/충돌
├── cbirrt.py          ConstrainedPlanner — CBiRRT + 직선 데카르트 서보
├── geometry.py        순수 헬퍼(quat, pose_at) + 튜닝 상수 (값은 env/ JSON에서 로드)
├── gripper_params.py  그리퍼/박스 기하 단일 소스 (stdlib만; reachability_map도 공유)
├── env/{sim,real}.json  실측 상수 (선반·포켓·박스·허브). DOBOT_ENV 로 선택,
│                        real_surfaces.json = 교시된 테이블/벽 (tools/teach_env.py)
├── robot_feed.py      ★ 공용 30004 실시간 피드 파서 (읽기 전용, ROS 불필요) —
│                      구 RealtimeMonitor(test)/RobotFeed(handeye) 5벌을 하나로
├── contact.py         ★ 실물 접촉정지: ContactDetector·Dashboard·guarded_descend·
│                      arrived 등 (--vision --run 검증본을 test에서 이동)
└── collision_env.py   표면(테이블/벽) 슬래브 등록 — main.py 기동 시 자동,
                       base_link→모델루트 프레임 합성 포함
```

## 클래스 계층 (`node.py`)

### `CR7Node(rclpy.Node)` — ROS 배관
| 메서드 | 역할 |
|--------|------|
| `joint_state_callback` | `/joint_states` 구독, `current_joints` 유지 (실물은 `CR7_REAL_ROBOT=1`로 J1/J5/J6 부호 변환) |
| (프로파일 스위치) | `use_gripper`/`use_attach`/`use_vision_box`/`preflight` 불리언 — main.py `--profile`이 세팅. preflight면 모션 미전송 + 가상 팔이 경로 끝으로 점프 |
| `control_gripper(positions)` | 그리퍼 FollowJointTrajectory (`GRIPPER_OPEN/CLOSE`) |
| `attach_box()` / `detach_box()` | `/ATTACHLINK` `/DETACHLINK` (IFRA LinkAttacher) |
| `plan_rrt(start, goal)` | 자유 조인트공간 RRT (구속 없는 이동용) |
| `execute_trajectory(path)` | 조인트 경로 → 액션 goal 전송 + 정착 대기(`_wait_settled`) |

### `CBiRRTPickPlace(CR7Node)` — pinocchio 엔진 + 모션 프리미티브
| 메서드 | 역할 |
|--------|------|
| `setup_planner()` | ReachabilityModel 2개 + ConstrainedPlanner 구성. 조인트 리밋을 URDF 하드웨어 범위(±6.27 rad)로 확장 (충돌은 pinocchio가 막으므로) |
| `compute_ik_ordered(pose, max_retries=600)` | 현재 자세 근방 시드 → 랜덤 재시작, 전신 충돌 게이트, 현재 config에 가장 가까운 브랜치 반환. **retries 600은 튜닝값 — 줄이지 말 것** (07 참고) |
| `is_state_valid(q)` | pinocchio 전신 충돌 검사 (self + AGV/큐브 + 팬텀) |
| `move_constrained(pose)` | 기울기-구속 CBiRRT 운반 |
| `move_to_pose_ref(pose, ref_q)` | 지정 IK 브랜치로 자유 RRT 이동 |
| `linear_servo(delta)` | 직선 데카르트 서보. **짧게 끝나면 하드 실패** — 어긋난 위치에서 파지/적재하지 않고 abort |
| `guarded_descend` / `servo_to` / `rotate_j6` / `move_single_joint` | 미세 모션 프리미티브 |
| `update_shelf_collision(pose)` | 선반 보드+재고 팬텀을 TF/비전 포즈 기준 재배치. **fail-closed**: TF 실패 시 `_unenforced`에 기록되고 `execute_path/trajectory`가 모든 모션 거부 (경고-후-진행 아님, 2026-08-17) |
| `add_wirebonder_meshes(dir)` / `update_wirebonder_collision(pose)` | 장비 본체 STL 충돌 등록/재배치 |
| `set_shelf_stock_absent(tier, i)` | 집어간 박스의 재고 팬텀 토글 |

### `HubPickPlace(CBiRRTPickPlace)` — 허브 라우팅 + 운반 (시퀀스가 쓰는 클래스)
| 메서드 | 역할 |
|--------|------|
| `init_hub()` / `go_to_hub()` | tool-down 대기 웨이포인트 확립/복귀 |
| `attach_box_collision()` / `detach_box_collision()` | 운반 중 박스 팬텀을 충돌모델에 부착/해제 |
| `set_box_stock_collision(on)` | 박스-vs-재고 충돌쌍 토글 (twisted-return preflight용) |
| `plan_spoke(start_q, goal_pose, ref_q)` | 허브↔목표 CBiRRT 스포크 |
| `preflight_linear(start_q, delta, label)` | **무동작** 직선 구간 사전검증 |
| `capture(fn)` / `replay_reverse(path)` / `rev` / `offset_j6` / `join` | 전진 경로 기록·역재생·변형 유틸 |
| `gripper_x_in_base_fk(q)` | FK로 조(jaw) 축 예측 (파지 오프셋 계산) |
| `grasp_object(model, link)` / `release_object()` | 파지 = 그리퍼 닫기 + ATTACHLINK + 팬텀 부착 |
| `attach_box_to_magazine()` / `detach_box_from_magazine()` | 적재 박스를 AGV(`mpo_base_link`)에 고정 — 마찰만으로는 AGV 주행 시 박스가 안 따라옴 |
| `level_base()` | 베이스 수평/높이 검증 |

## `model.py` — `ReachabilityModel`

`setup_planner()`에서 2개 인스턴스:

| 인스턴스 | URDF | 용도 |
|----------|------|------|
| `self.collision` | `cr7_on_mpo700` (팔+큐브+AGV+그리퍼) | `is_collision_free()` — 전신/씬 충돌 |
| `self.ik_model` | `cr7_robot` (팔만) | `inverse_kinematics()` — DLS CLIK |

- xacro → `/tmp/cr7_*_model_<pid>.urdf` → pinocchio 로드 → 팔 6자유도만 남기고 고정
  (`buildReducedModel`), 충돌쌍 = 전체 − SRDF 비활성 − 중립자세 충돌쌍.
- **IK 타깃 = 그리퍼 TCP** = Link6 원점 + `TCP_OFFSET_M`(0.12005 m) 툴 z 방향.
  이건 reachability map과 공유하는 **추상 규약**이지 실제 패드 위치가 아님
  (실제 패드 하단은 플랜지 아래 0.0821 m). 시퀀스 상수(`GRASP_TCP_ABOVE` 등)가 보정.

## `cbirrt.py` — `ConstrainedPlanner`

- **자세 구속**: SO3 log-map 오차 `e = log3(R0ᵀR(q))`. 기본 `lock_tilt_only=True`
  → e/J의 앞 2행만 사용 = 툴 z(접근축)만 고정, 그 축 둘레 yaw는 자유 → 운반 중 박스 수평 유지.
- **`plan()`**: 양방향 CBiRRT. 샘플→`_project`(뉴턴랩슨으로 구속면에 사영)→확장→연결.
- **`linear_path(delta, start_q)`**: 현재 자세 유지 직선 이동. 조인트리밋/충돌/특이점에서
  중단하고 `(path, 도달거리, 사유)` 반환 — 도달거리 미달은 시퀀스 레벨에서 하드 실패 처리.

## 상수 (`geometry.py`, `gripper_params.py`)

**바꾸기 전에 반드시 승인/검증** — 전부 측정·튜닝된 값 (07 참고).
2026-08-17부터 워크스페이스 실측값(선반·포켓·박스 치수, `HUB_TCP`)은 코드가 아니라
**`cr7_pnp/env/<DOBOT_ENV>.json`** 에 있음 (`sim.json` = 기존 값 그대로, 검증됨;
`real.json` = 실물 실측, `tools/teach_env.py --teach-shelf`로 채움). `geometry.py`가
기동 시 읽어 같은 상수명을 채우므로 사용처는 불변. 아래 표의 값은 sim 기준.

| 상수 | 값 | 의미 |
|------|----|------|
| `DOWN` | (0.707, 0.707, 0, 0) | 툴 z → 월드 −Z (수직 하향) |
| `TCP_OFFSET_M` | 0.12005 m | Link6→IK-TCP (규약) |
| `BOX_SIZE` | (0.081, 0.236, 0.14) | 매거진 박스 (短, 長, 높이) |
| `JAW_GAP_AT_ZERO` | 81.0 mm | q=0 패드 간격 (== BOX_SHORT) |
| `PAD_BOTTOM_BELOW_FLANGE` | 0.0821 m | 패드 하단, 플랜지 아래 |
| `GRASP_LATERAL_M` | ≈0.140 m (계산값) | 박스 중심이 툴축에서 고정조 쪽으로 매달리는 오프셋. **롱 그리퍼 기준** — 옛 문서/docstring의 "≈46–48 mm"는 구형 숏 그리퍼 수치이니 무시. 운반 중 236 mm 박스가 TCP 앞으로 ~256 mm 뻗음 (`docs/wirebonder_transfer_hardening.md`) |
| `GRIPPER_OPEN`/`CLOSE` | [0.03]/[0.0] | 핑거 명령 (m) |
| `GRASP_TCP_ABOVE` / `INSERT_TCP_ABOVE` / `PREGRASP_BACK` | 0.015 / 0.105 / 0.25 m | 파지 접근 오프셋들 |
| `POCKET_X` / `POCKET_Y` | 0.3705 / ±0.177, ±0.059 | 베이스 포켓 중심 (base_link) |
| `STANDBY_POSE_DEG` | [-8,-39,-105,0,0,0] | 접힘 대기 자세 |

- `gripper_params.py`는 **stdlib만 쓰는 순수 모듈** — reachability_map이 ROS 없이도 로드.
  그리퍼/박스/툴 오프셋 변경은 여기 한 곳만 수정 (베이스 장착 오프셋은 URDF 소관).
- 시퀀스 전용 튜닝값(`PLACE_ORDER_Y`, `SLOT_OFFSET`, `HOVER_ABOVE` 등)은 각 시퀀스
  파일에 있음 — 라이브러리에 없다고 당황하지 말 것. `HUB_TCP`는 두 시퀀스 중복이던 것을
  env JSON + `cr7_pnp` 재수출로 통합 (2026-08-17).

## 설계 노트 — 왜 이렇게 생겼나 (docstring에서 이관)

코드 docstring은 계약만 남기고, 각 결정의 근거·사고 이력은 여기에 보존함.

### `node.py`
- **`_wait_settled`**: trajectory 컨트롤러는 goal 허용오차가 꺼져 있어 명령 **스케줄
  시점에** 결과를 반환하고, Gazebo 조인트는 보간 명령을 최대 ~2 s 뒤따름. "완료" 직후
  `current_joints`를 읽으면 이동 중 값이 나오므로, 모든 실행 끝에 실제 도착을 대기함.
- **`_add_shelf_stock`**: 미모델 상태의 안착 박스를 캡처 스윙이 강타해 AGV까지 밀어버린
  실측 사고가 계기. 양 tier 전 박스에 정적 팬텀을 두되 **10 mm/측 축소** — 이 팬텀은
  대형 스윕 가드용이고, pick 자체의 파지 레그는 타깃 박스 수 mm 옆을 지나므로 타깃
  박스만 `set_shelf_stock_absent`로 pick 동안 파킹함.
- **`add_wirebonder_meshes`**: 장비 충돌 STL 파트들은 **슬롯 리세스가 뚫려 있게 제작**
  되어 front-load 삽입 경로가 유효함 — 몸통 벌크만 막음.
- **`guarded_descend`**: 실물 조인트-토크 touch-off의 sim 아날로그
  (`place_command_guide.md`). 과잉 하강을 명령하되 box 팬텀 ON 상태로 linear_path가
  보고한 **충돌 지점까지만 실행** — 과거 place IK를 false-reject하던 box-vs-표면
  충돌쌍을 여기서는 센서로 씀. 포켓별 높이 편차·파지 오프셋 편차를 흡수함.
- **`level_base`(SIM 전용)**: gazebo_ros_planar_move는 틸트 **속도**만 매 틱 0으로 만들
  뿐 복원력이 없어, 접촉 임펄스(파지 압박, attach 싸움)가 roll/pitch/z를 사이클마다
  래칫시킴 (실측: 한 세션에 −1.7/−1.3° → slot-D 리치에서 TCP ~30 mm 오차).
  **2026-07-16 밸러스트 ×10**(cube 5000 kg/I=20000, mpo 1400 kg) 이후 임펄스당 래칫이
  무의미해져 사이클마다 부르던 호출은 제거 — 마라톤 세션에서 드리프트가 보이면 수동
  복구/진단용으로 호출함 (드리프트는 저절로 안 돌아옴).
- **`joint_move`**: MoveJ 스타일 직선 조인트 보간, **충돌검사 없음** — 직선 조인트
  경로가 안전하다고 알려진 곳(캡처 뷰 조그 등) 전용.
- **`plan_spoke`의 `goal_q` 핀**: IK가 확률적이라 재풀이하면 caller가 preflight한 것과
  **다른 브랜치**에 착지할 수 있음 → 검증된 goal config를 그대로 핀함.
- **`preflight_linear`의 `severity='info'`**: ERROR 로그 스트림은 MCS로 전달될 예정이라,
  후보 여러 개를 찔러보는 정상 흐름의 거부는 info로 낮춰 기록함.
- **`init_hub`의 `ref_pose`**: 허브를 호출 시퀀스의 place 기하에 **의도적으로 결합** —
  각 시퀀스가 자기 기준 포즈를 넘김 (elbow/wrist 브랜치 시드 + standby 방향 공급).
- **`move_constrained`의 `yaw_free`**: 박스 대칭인 두 place 방향(yaw, yaw+180) 중 현재
  자세에 가까운 goal을 골라 J6 와인딩을 없애고, 불가피한 J1/elbow 재구성만 CBiRRT에 남김.

### `model.py` / `cbirrt.py`
- fixed↔fixed 충돌쌍은 결과가 상수라 매 IK 호출마다 재검사할 이유가 없어 제거
  (`_keep_only_movable_pairs`).
- CBiRRT를 직접 구현한 이유: pip OMPL 휠은 python에서 ProjectedStateSpace 상태를
  읽고 쓸 수 없어 OMPL constrained 프레임워크를 못 씀 (Berenson 알고리즘 그대로).
- `linear_path`의 중단 사유: `done`(완주) / `singular`(수렴 실패) / `limit` / `collision`.
- `max_reach`: 링크 오프셋 합 + TCP 오프셋 = 삼각부등식 상한 — 이 밖 복셀 프루닝은
  도달 가능 복셀을 잃지 않음.
