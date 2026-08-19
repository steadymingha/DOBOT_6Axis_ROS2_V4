# 충돌 모델 등록 매뉴얼 (초안)

무엇을 어떻게 계획용 충돌 모델에 넣는가. 이행 계획은 `docs/real_robot_pipeline_plan.md` 참고.

2026-08-17 이행 완료: 아래 파일이 전부 있다. `test/run.sh --teach-surface …` 도 같은 함수를
import 해서 그대로 동작한다. 표면 파일은 `cr7_pnp/env/<DOBOT_ENV>_surfaces.json`
(실물: `real_surfaces.json`, 옛 `test/surfaces.json` 을 옮긴 것).

| 파일 | 역할 |
|---|---|
| `tools/teach_env.py` | 교시 (사람이, 셋업 시 1회): `--teach-surface` `--set-surface` `--forget-surface` `--show` |
| `cr7_pnp/collision_env.py` | 적용 (`main.py` 기동 시 자동): `register_surfaces` — base_link → 모델 루트 합성 포함 |
| `cr7_pnp/env/{sim,real}.json` | 선반·포켓·박스·허브 실측 상수. `DOBOT_ENV` 로 선택, `geometry.py` 가 읽음 |
| `cr7_pnp/node.py` (선반·메시) | 동일 |

**`real.json` 은 아직 시뮬 사본이다** (`"measured": false`). 실측해 채우고 `true` 로 바꾸기 전에는
`main.py --profile real` 이 기동을 거부한다 (`--preflight` 만 허용).

---

## 1. 모델은 하나다

계획에 쓰이는 충돌 모델은 pinocchio `GeometryModel` **하나**이고
(`cr7_pnp/node.py:415-420`, `self.collision`), 넣는 방법이 네 가지다.

| 방식 | 무엇 | 코드 | 배치 |
|---|---|---|---|
| URDF/xacro | 로봇 + 큐브 + AGV 본체 | `COMBINED_XACRO` (`geometry.py:27-29`) | 링크 부착 (`parentJoint≠0`) |
| 프리미티브 | 선반 판, 재고 팬텀, carried_box | `coal.Box` (`node.py:503, 558`) | 월드 고정 (`parentJoint=0`) |
| STL 메시 | 와이어본더 본체 | `coal.MeshLoader().load()` (`node.py:638-651`) | 월드 고정 |
| 표면 슬래브 | 테이블, 벽, 트롤리 | `collision_env.add_surface()` | 월드 고정 |

### 공통 패턴

넷 다 결국 이 네 줄이다.

```python
go  = pin.GeometryObject(name, 0, placement, coal.Box(*size))   # 0 = 월드 고정
idx = geom.addGeometryObject(go)
for i in arm_links:                                  # 움직이는 팔 링크 전부와 짝
    geom.addCollisionPair(pin.CollisionPair(i, idx))
node.collision.geom_data = geom.createData()         # ★ 반드시 재생성
```

`arm_links` 는 `parentJoint != 0` 인 것만 고른다 — 월드 고정물끼리는 서로 부딪힐 일이 없다.

---

## 2. 등록됐다고 켜진 것은 아니다

물체는 **제거하지 않고 멀리 park** 한다. `far = SE3(z=-100)` 로 옮겨 "없는 것처럼" 만들어
두고, `update_shelf_collision()` (`node.py:582`) 이 매 사이클 실제 자리로 옮긴다.

→ **park 된 채로 남는 것이 곧 "충돌 모델 꺼짐"** 이다. 등록 로그가 정상으로 찍혀도 그럴 수
있으므로, 배치 함수의 반환값을 검사해 실패 시 중단해야 한다 (이행 계획 2-2, fail-closed).
운전 전에 4.5 의 확인을 거를 것.

(`geom_data` 재생성, `carried_box` 팬텀 페어 제외 같은 구현 주의는 `collision_env.py` 의
코드 주석에 둔다 — 이 문서는 쓰는 쪽이 아니라 운용하는 쪽을 위한 것이다.)

---

## 3. 모델에 없는 것

- **매거진, 집는 박스, 작업 대상물** — 어느 모델에도 없다. 수직 하강은 **접촉 정지**가
  담당하고(`guarded_descend`), 그래서 하강 속도와 토크 문턱이 안전의 마지막 방어선이다.
- **방(room) 자체** — URDF 는 방의 존재를 모른다. 테이블·벽·트롤리는 4장의 표면 교시로만
  들어간다. 교시하지 않으면 팔은 테이블이 없다고 믿고 계획한다.

---

## 4. 실행 방법

### 4.0 실행 환경

교시는 실물 팔을 움직이므로 `test/run.sh` 와 같은 조건이 필요하다 — 컨테이너 `ros2_dobot`,
워크스페이스 source, `DOBOT_TYPE=cr7`, pinocchio 가 있는 `.venv`. 사전에 떠 있어야 하는 것:

```
ros2 launch cr_robot_ros2 dobot_bringup_ros2.launch.py     # 실물 드라이버
ros2 launch dobot_moveit  dobot_joint.launch.py            # 액션 서버 + joint_states
```

### 4.1 표면 교시 — 접촉으로 재기

팔을 조그해 **툴을 표면에 댄 뒤**, 로봇 베이스에서 그 표면을 향하는 방향을 **축 먼저** 적는다
(`z-` = 아래 테이블, `y-` = -y 쪽 벽). 축을 먼저 쓰는 이유는 argparse 가 `-z` 를 플래그로
읽기 때문이다.

```bash
tools/teach_env.py --teach-surface z-                      # 테이블
tools/teach_env.py --teach-surface y-                      # -y 벽
tools/teach_env.py --teach-surface z- --name lower_shelf \
                   --bound y -0.34 inf                     # 같은 방향의 두 번째 면
```

(`test/run.sh --teach-surface z-` 도 동일하게 동작)

- `--name` — 같은 방향을 보는 면이 둘 이상이면 필수 (베이스 받침과 하단 선반이 둘 다 `z-`)
- `--bound AXIS LO HI` — 그 축으로 면을 잘라낸다. `inf` 로 한쪽 개방. 반복 가능
- 결과는 `cr7_pnp/env/real_surfaces.json` 에 누적된다 (`DOBOT_ENV` 기본값: teach_env 는 real)

### 4.2 표면 지정 — 숫자로 넣기

아직 없는 벽, 도면에서 딴 치수, 툴을 대면 안 되는 면.

```bash
tools/teach_env.py --set-surface x+ --at 1.25 --name trolley
```

로봇이 필요 없다.

### 4.3 선반·포켓 실측값

프리미티브 박스는 **코드가 이미 만든다**(`_add_shelf_boards:487`, `_add_shelf_stock:520`).
새로 만들 것은 없고, 바뀌는 것은 값의 출처뿐이다.

```
cr7_pnp/env/real.json      ← 실측값을 여기에
cr7_pnp/env/sim.json       ← 시뮬 수치 (geometry.py 에서 옮겨 담음, 값 동일 검증됨)
```

`geometry.py` 가 기동 시 하나를 읽어 같은 이름의 모듈 상수를 채우므로 **사용처는 바뀌지
않는다.** 어느 파일을 읽을지는 환경변수로 정한다 — 상수는 import 시점에 결정되므로
`--profile` 로는 늦다.

```bash
DOBOT_ENV=real ros2 launch .../arm.launch.py profile:=real
```

옮길 값 (현재 위치):

| 값 | 현재 |
|---|---|
| `SHELF_WORLD_POSE`, `SHELF_BOARD_TOPS`, `SHELF_FOOTPRINT`, `SHELF_BOARD_THICK` | `geometry.py:52-62` |
| `SHELF_TIER_TOPS`, `SHELF_BOX_XS`, `SHELF_BOX_Y`, `SHELF_TAG_XY` | `geometry.py:65-86` |
| `POCKET_X`, `POCKET_Y`, `POCKET_SURFACE_Z`, `POCKET_HOVER` | `geometry.py:218-242` |
| `BOX_SIZE` | `gripper_params.py:40` |
| `HUB_TCP` (두 파일에 중복) | `shelf_pick_place.py:69`, `wirebonder_pick_place.py:156` |

`STOCK_SHRINK = -0.006` (면당 3 mm 팽창)은 값이 아니라 **정책**이다. 실물 파지 편차를
재측정하기 전까지 유지한다.

### 4.4 런타임 등록 — 자동

`main.py` 기동 시 일어난다. 사람이 할 일은 없다.

```
setup_planner()               node.py:393
 ├─ _add_shelf_boards()       node.py:487   coal.Box 4장
 ├─ _add_shelf_stock()        node.py:520   재고 팬텀 8개
 └─ register_surfaces()       collision_env.py  env/<DOBOT_ENV>_surfaces.json → 슬래브 (main.py 가 호출)
add_wirebonder_meshes(dir)    node.py:618   STL (와이어본더를 쓸 때만)
update_shelf_collision()      node.py:582   매 사이클, 비전 자세로 위치 갱신
```

### 4.5 확인

```bash
tools/teach_env.py --show          # 교시된 표면 목록
```

기동 로그에 아래가 **실제로 찍히는지** 본다. 안 찍히면 모델이 비어 있는 것이다.

```
[collision] pinocchio model ready: N active pairs from cr7_on_mpo700.urdf.xacro
[collision] added 4 shelf boards (open gaps, avoided by the planner)
[collision] added 8 shelf stock phantoms (resting boxes)
```

```
[collision] added 4 surfaces (table/walls, base_link -> model root composed)
```

표면을 하나도 교시하지 않았으면 이렇게 경고한다 — 이 상태로 운전하지 말 것.

```
[collision] added 0 surfaces: none taught in .../env/real_surfaces.json (...)
```

TF 가 없어 선반 팬텀을 못 놓으면 경고가 아니라 **모든 모션이 잠긴다** (fail-closed):

```
[shelf] TF unavailable -> collision model NOT enforced, motion locked: ...
```

---

## 5. 표면의 좌표 기준 — 확정 (2026-08-17)

표면의 `at` 은 **base_link** 기준이고, `collision_env.add_surface` 가 `T_root_base(node)`(충돌 모델
자체에서 읽은 base_link 의 루트 프레임 내 위치)를 곱해 모델 루트에 놓는다. 팔 단독 모델(루트 =
base_link)이면 항등, 결합 모델(루트 = mpo_base_link)이면 마운트 오프셋 — 어느 쪽이든 같은 자리에
놓인다. 옛 주석("이미 base_link")의 함정은 이걸로 닫혔다.

실물 Jetson 에는 `neo_simulation2` 가 없어 결합 xacro 가 빌드되지 않는다 → `main.py` 는
`collision_model_xacro()` 로 팔 단독 모델로 폴백하고 (`[collision] model root frame: base_link`),
`node.root_frame` 도 base_link 가 되어 선반 팬텀 TF 조회가 `base_link ← odom` 으로 간다.
큐브·AGV 몸체는 이때 모델에 없다 — 표면 슬래브(base_support, low_shelf, wall_x, wall_y)가 그 역할을
대신한다. AGV 몸체까지 모델에 넣으려면 컨테이너에 `neo_simulation2` 를 설치할 것.
