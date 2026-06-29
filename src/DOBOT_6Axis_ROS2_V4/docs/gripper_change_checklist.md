# 그리퍼 형상 수정 체크리스트

Blender에서 그리퍼 메시를 다시 export한 뒤 **순서대로** 따라가면 visual/collision/grasp가
다시 일치한다. 핵심: visual 메시는 `gripper.xacro` 한 곳에서만 참조되고, collision 박스와
grasp 상수는 메시를 따라 **수동으로** 다시 맞춰야 한다 (자동 동기화 아님).

전체 좌표는 `gripper_base_link` 프레임 기준. 플랜지 면 = z=0.1401, 고정 jaw = +X 방향,
finger joint axis = -X (q>0 이면 열림, pad gap = JAW_GAP_AT_ZERO + q).

---

## 0. 메시 치수 측정 (먼저)

```bash
cd ~/dobot_ws/src/blender
python3 measure_gripper_dae.py gripper_long/meshes/base.dae gripper_long/meshes/finger.dae
```

출력의 per-part `origin(...) size(...)` 와 `X[..]/Y[..]/Z[..]` 를 아래 단계에 그대로 사용한다.
주요 부품 이름: `jaw_fixed`, `pad_fixed`, `body`, `flange` (base) / `jaw_moving`, `pad_moving` (finger).

---

## 1. visual 메시 경로 — `cra_description/urdf/gripper.xacro`

- `gripper_base_link` <visual> mesh, `gripper_finger_link` <visual> mesh.
- 새 폴더로 export 했으면 경로만 교체. (현재: `src/blender/gripper_long/meshes/*.dae`)
- export 폴더가 `GAZEBO_MODEL_PATH` (run 스크립트의 `src/blender`) 안에 있어야 Gazebo가 찾는다.

## 2. collision 박스 — `cra_description/urdf/gripper.xacro` (단일 소스, 3개 로봇 공용)

측정값으로 다시 fit. visual과 X/Y/Z가 일치해야 함 (이전 버그: X(jaw 길이)만 안 맞았음).

- `gripper_base_link` (박스 4개): fixed-jaw top beam(jaw_fixed 윗부분) / fixed-jaw column+pad
  (안쪽 면 = pad_fixed Xmin) / top plate(body) / mount boss(flange, 원통 r0.041).
- `gripper_finger_link` (박스 1개): jaw_moving+pad_moving 외피, 바깥(+X) 면 = pad_moving Xmax.
- 플랜지 면 위치가 바뀌면 `gripper_attach_joint` origin z (=0.1401) 도 수정.

## 3. finger joint 가동범위 — `cra_description/urdf/gripper.xacro`

- `gripper_finger_joint` `<limit lower upper>` : 물리적 열림/닫힘 한계.
- ros2_control command_interface min/max + initial_value 도 같이 (아래 4번).

## 4. ros2_control command range / 초기값 (2개 로봇에 각각)

- `cra_description/urdf/cr7_on_mpo700.urdf.xacro` : `gripper_finger_joint` command min/max, initial_value.
- `cra_description/urdf/cr7_robot.xacro` : 동일 블록.
- `GRIPPER_OPEN`/`GRIPPER_CLOSE` 값이 이 [min,max] 안에 들어와야 함 (안 그러면 컨트롤러가 clip).

## 5. grasp 상수 — `cbirrt_pick_place.py` (측정값으로 수정)

| 상수 | 의미 | 측정 출처 |
|------|------|-----------|
| `JAW_FIXED_PAD_X` | 고정 pad 안쪽 면 (gripper x) | base.dae `pad_fixed` **Xmin** |
| `JAW_MOVING_PAD_X0` | q=0 일 때 가동 pad 안쪽 면 | finger.dae `pad_moving` **Xmax** |
| `PAD_BOTTOM_BELOW_FLANGE` | pad 하단이 플랜지 아래로 | 0.1401 − pad **Zmin** |
| `BOX_SHORT` | 잡는 박스 폭 (그리퍼 아님, 박스 바뀔 때만) | — |

자동 파생(직접 수정 X, 단 결과 확인): `JAW_GAP_AT_ZERO`,
`GRIPPER_CLOSE = BOX_SHORT − JAW_GAP_AT_ZERO − CLOSE_SQUEEZE`,
그리고 `shelf_pick_place.py`의 `GRASP_LATERAL_M` (→ pre-grasp / pocket hover / hub 오프셋 모두 따라감).

- 같은 파일 `log_gripper_box_clearance()` 안의 `prims` dict (진단용, URDF collision 박스 복붙) 도 같이 갱신.
- grasp center 주석(`~123 mm off the tool axis`) 도 갱신.

## 6. 박스 phantom (박스 크기/장착 위치 바뀔 때만) — `shelf_pick_place.py`

- `BOX_SIZE`, `BOX_IN_LINK6_XYZ`.

## 7. SRDF self-collision (링크를 추가/이름변경 했을 때만) — `cr7_moveit/config/cr7_robot.srdf`

- `disable_collisions` 에 `gripper_base_link` / `gripper_finger_link` 인접 쌍이 있는지 확인.
- 박스만 키우면 보통 건드릴 필요 없음.

## 8. 빌드 & 검증

```bash
# xacro 파싱 확인
xacro src/DOBOT_6Axis_ROS2_V4/cra_description/urdf/cr7_on_mpo700.urdf.xacro > /dev/null

# gripper.xacro 는 cra_description 안에 있으므로 이 패키지만 빌드 (symlink 라 py는 빌드 불필요)
colcon build --packages-select cra_description --symlink-install

# 재시작
./kill_sim.sh && ./run_mpo700_cr7.sh
```

검증 포인트: RViz/Gazebo에서 collision == visual, GRIPPER_CLOSE 시 pad gap == BOX_SHORT,
선반 pre-grasp IK 성공, hub에서 박스가 의도한 위치, un-twist 시 옆 박스 안 침.

---

## 빠뜨리기 쉬운 곳 (과거 버그)

- collision X(jaw 길이)만 안 맞아서 visual≠collision → **2번** 잊지 말 것.
- `cbirrt_pick_place.py`의 `prims` dict이 URDF와 따로 노는 것 → **5번**.
- command range 밖의 OPEN/CLOSE 값 → 컨트롤러가 조용히 clip → **4번**.

---

# 부록: 그리퍼 마운트 방향 변경 (재배향) 체크리스트

> 위 1~8번은 "형상만" 바뀔 때. **플랜지가 붙는 위치/방향을 바꾸면**(예: 윗면 부착 →
> 뒷면 부착, 공구축이 아래→옆) 이건 형상 변경이 아니라 **"tool-down 가정" 자체를
> 흔드는 변경**이다. 지금 파이프라인 전체(hub-and-spoke, CBiRRT manifold,
> reverse-replay)가 "공구축이 아래를 향한다"는 전제 위에 있음.

**먼저 결정**: 마운트의 의미
- **(A) 마운트 면만 90° 회전** → 접근 방향만 수평(공구축이 jaw 닫는 방향). 운동체인 그대로,
  변환만 변경. ← 보통 이쪽.
- **(B) 진짜 움직이는 finger에 플랜지 부착** → 부모/자식 체인이 뒤집힘(base_link가 finger의
  자식). URDF 구조 재설계. ← 권장 안 함.

**먼저 결정 2**: 포켓 배치도 같이 수평으로? 그랩만 수평/배치는 수직이면 한 사이클에
공구 방향이 2개 → 단일 매니폴드 전제가 깨져 허브 재설계 필요(아래 R5). **그랩·배치를
같은 새 방향으로 통일하면 일이 크게 줄어든다.**

아래는 (A) 기준. 난이도 큰 순서: **R2·R3·R5 > R1·R4·R6 > R7·R8.**

## R1. 마운트 변환 (핵심) — `gripper.xacro`
- `gripper_attach_joint` 의 **origin xyz + rpy** 를 새 자세로
  (현재 `xyz="0 0 0.1401" rpy="3.14159 0 0"` = 공구 아래).
- 실제 플랜지 피처(mount_boss)가 윗면→뒷면으로 이동하면 **메시 재export** →
  위 1~8번 형상 체크리스트도 전부 다시.

## R2. 공구 방향 기준 (가장 광범위) — `cbirrt_pick_place.py`
- **`DOWN = (0.707,0.707,0,0)`** 이 더는 grasp 기준이 아님. DOWN 의존 전부 재정의:
  `grasp_quat`, `place_quat()`, `self.cbirrt.set_reference(DOWN)`(제약 매니폴드 기준),
  `GRASP_YAW_OFFSET`, `GRIPPER_YAW_TWIST`, `PLACE_YAW`(공구축 둘레 yaw의 물리적 의미가 바뀜).

## R3. jaw 방향 예측 (수직 가정 박힘) — `cbirrt_pick_place.py`
- `gripper_x_in_base()` 는 공구가 아래일 때만 동작(수평 투영 + "near-vertical이면 에러" 가드).
  수평 그랩이면 이 가드와 `_compute_gripper_x_offset` / `gripper_x_in_base_fk`(jaw 측면오프셋
  예측)를 새 축 기준으로. `GRASP_LATERAL_M` 더하는 방향도 바뀜.

## R4. 접근/삽입/하강 방향 상수
- 지금: pregrasp(박스 위 `+Z`) → 삽입(수평) → 하강(`[0,0,-descend]`, 수직) → 닫기.
- 수평 그랩이면 "하강"이 공구축 따라 수평 전진으로:
  `INSERT_TCP_ABOVE`, `GRASP_TCP_ABOVE`, `PREGRASP_BACK` 재유도,
  `linear_servo([0,0,-descend...])` 축 변경(shelf_pick_place의 descend/place-descend 포함).

## R5. 허브 + "두 방향 사이클" — `shelf_pick_place.py`
- `HUB_TCP` 와 "tool-down HUB" 개념 자체.
- 그랩 수평인데 배치는 수직이면 사이클에 공구 방향 2개 → "모든 spoke가 같은 tool-down
  매니폴드" 전제가 깨짐 → 허브가 재배향을 다리놓아야 함(여기가 제일 손 많이 감).
  → 가능하면 "먼저 결정 2"에서 둘을 통일.

## R6. 운반 박스 팬텀 — `shelf_pick_place.py`
- `BOX_IN_LINK6_XYZ = (0,0,0.135)`(박스 중심이 플랜지 아래로) 를 새 방향으로 재유도.
  `BOX_SIZE`는 박스 안 바뀌면 그대로.

## R7. SRDF 인접 — `cr7_moveit/config/cr7_robot.srdf`
- Link6 인접 링크가 바뀌면 `disable_collisions` 쌍 갱신.

## R8. 스폰 대기자세 — `cr7_on_mpo700.urdf.xacro` / `cr7_robot.xacro`
- joint `initial_value` 6개를 새 그랩 방향에 맞는 표준 대기자세로.

**한 줄 정리**: R2·R3·R5가 진짜 부담(DOWN 기준 + jaw축 수직 투영 + 단일 매니폴드 허브).
R1·R4·R6은 "방향 다시 재기", R7·R8은 마무리. 설계 개선 팁: DOWN을 상수가 아니라
**공구축 파라미터**로 일반화하면 다음 재배향이 쉬워짐.
