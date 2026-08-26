# 03. 시퀀스 명세 — `sequences/` + `main.py`

시퀀스 파일 = "시퀀스 로직 + 그 시퀀스 전용 튜닝 상수 + 단독 실행용 main".
재사용 모션은 전부 `cr7_pnp`에 있고, main.py가 이 파일들을 **라이브러리로 import**해서
같은 함수를 호출함 (로직 중복 없음). 트리거만 다르다: 단독 실행 시 스페이스/숫자키,
디스패처 경유 시 MCS 명령.

## `main.py` — 미션 디스패처

- `REGISTRY`: location id → (종류, 파라미터). 현재 `wb1/wb2/wb3`(wirebonder 이송 1/2/3),
  `shelf`. **실물 스테이션이 늘면 여기 한 줄 추가**가 규약.
- 프로파일 (2026-08-17): `--profile sim|real [--preflight] [--gripper]`. 기동 시 대조검사 —
  `DOBOT_ENV` 불일치·`real.json` 미실측(`measured:false`)·30004 불통·sim인데 /ATTACHLINK
  부재 전부 **기동 거부**. real은 그리퍼/attach 더미, 포켓 점유 비전 미사용(정적 기본 포켓),
  `CR7_REAL_ROBOT=1` 자동. `--preflight` = 전 구간 IK/충돌 검증만, 모션 미전송.
- `locate_box()` (real 선반 LOCATE): 허브 → 교시된 obs 자세 → `/vision/capture` →
  `/vision/device_pose`(AI 매거진 검출 박스 중심) 15샘플 중앙값 + spread/작업영역 게이트 →
  `node.vision_box` → 역재생 복귀. 실패 시 ErrorCode 채우고 미션 중단.
- `run_mission()`: 충돌월드 갱신 → LOCATE → PICK/PLACE → REPORT (01 문서 참고).
- 명령 수신: `/mcs/command`(JSON, START만) → 큐에 적재, 메인 루프가 블로킹 실행.
  `/mcs/stop` → `node.abort` 플래그.
- TTY가 있으면 키보드로도 location id 입력 가능 (launch 하에서는 자동 비활성).
- `--selftest`: ROS 없이 REGISTRY 무결성 검사.

## `sequences/shelf_pick_place.py` — 선반 → 베이스 포켓 (박스 4개)

### 흐름 (박스 1개당, `pick_place_one_box(node, idx)`)

```
허브 ──(CBiRRT 스포크)──► pre-grasp ─► 삽입(insert) ─► J6 비틀기 ─► 하강 ─► 파지+attach
허브 ◄──(전진 경로 역재생, 비틀림 유지)──────────────────────────────────────────┘
허브 ──(CBiRRT 스포크)──► 포켓 hover ─► 하강 ─► 릴리즈 + AGV에 attach
허브 ◄──(역재생)──────────────────────────────────────────────────────────────┘
```

tier-1 박스 4개 → 포켓 4칸, 트리거 1회 = 1박스 (`node.box_idx`가 진행 상태).

### 주요 함수
| 함수 | 역할 |
|------|------|
| `bringup(node)` | 허브 확립 + 선반 보드/재고 팬텀 앵커 배치 + 포켓 점유 초기화. **두 플로우가 공유하는 브링업** (main.py가 시작 시 1회 호출) |
| `locate_shelf(node)` | LOCATE: 태그 조준(`aim_pose_at_tag`, 자동 조준 — 수동 조그 불필요) → `capture_shelf` → `refresh_shelf_pose`(n샘플, 산포 검증) → `node.shelf_pose=(x,y,yaw)` + 충돌모델 재배치 |
| `pick_place_one_box(node, idx)` | 위 흐름 전체. 내부: `shelf_pick_to_hub` + `pocket_place_from_hub` |
| `shelf_pick_to_hub(...)` | 파지 왕복. 전진 기록·역재생, J6-오프셋 복귀는 박스/재고 충돌쌍 켜고 preflight |
| `pocket_place_from_hub(...)` | 적재 왕복. `pocket_vision.next_free()`로 빈 포켓 선택 |
| `shelf_box(node, idx)` / `shelf_box_center` | 박스 중심·모델명. sim: **shelf_pose에 상대 합성**(레이아웃). real(`use_vision_box`): **`node.vision_box`(AI 검출값) 우선**, 레이아웃은 이웃 팬텀+조그 폴백 |
| `_abort_to_hub(node, done, reason)` | 실패 시 안전 복귀 + `node.last_error` 세팅 |

### 상수·특이점
- `SHELF_WORLD_POSE`: `--no-vision` 폴백 (값은 `env/<DOBOT_ENV>.json`의 `shelf.pose_in_base`).
- 삽입/열 방향은 선반 yaw(`pose_in_base[2]`)로 회전 (2026-08-18) — 실물 선반이 로봇
  ±y 어느 쪽이든 됨. sim yaw=0이라 기존 동작 불변.
- `PLACE_ORDER_Y`: 포켓 적재 순서 (pocket_vision.POCKET_ORDER_Y와 동일해야 함).
- 재고 팬텀 축소값 `STOCK_SHRINK=-0.006`(측면 3 mm)은 **사용자 승인된 튜닝값** — 임의 변경 금지.

## `sequences/wirebonder_pick_place.py` — 베이스 포켓 ↔ 장비 슬롯

### 이송 3종 (같은 `transfer(node, src, dst)` 하나, src/dst만 다름)
| # | src → dst | 의미 | MCS TargetID |
|---|-----------|------|--------------|
| 1 | base 포켓 → slot A | 매거진 장비 투입 | A |
| 2 | slot B → slot C | 슬롯 간 이동 | B |
| 3 | slot D → base 포켓 | 베이스 회수 | C |

- 슬롯 명명: A=H_L, B=G_L, C=G_R, D=H_R. 슬롯 주소는 `(device, letter)` —
  장비가 여러 대여도 대응 (현재 `DEVICES = {'wb1': ...}`).
- 슬롯 월드 포즈 = 장비 인스턴스 포즈(비전 갱신) ∘ 모델프레임 상수 `SLOT_OFFSET`
  (단일 소스는 `vision/tag_vision.py`의 SLOT_OFFSET).
- 이동은 **자유 RRT + 직선 서보** (CBiRRT 아님 — 장비 접근은 브랜치 문제가 없어 가벼운 쪽 사용).

### 주요 함수
| 함수 | 역할 |
|------|------|
| `capture_device(node)` | LOCATE: 허브 → `CAPTURE_FLANGE` 촬영 자세 서보 → `/vision/device_pose` n샘플 → `DEVICES` 갱신 + 장비 충돌 재배치 → 허브 복귀. **1회 캡처를 모든 이송이 재사용** |
| `refresh_device_pose(node, dev)` | 샘플 수집 + 산포/타당성 검증 (`CAPTURE_SPREAD_HIGH`/`CAPTURE_IMPLAUSIBLE`) |
| `resolve(node, loc)` | Location → base_link 좌표 (TF 경유; AGV가 장비를 향해 주차돼 있어야 함) |
| `strategy(loc)` | 슬롯별 pick/place 전략 선택: `top`(위에서) / `front`(전면 삽입, 스테이징) / `base`(포켓) |
| `pick_top/front/front_staged/base`, `place_top/front/base` | 전략별 왕복 레그. front 계열은 전진 기록·역재생 |
| `grasp_tcp_pose(node, center, quat)` | 박스가 툴축에서 `GRASP_LATERAL_M`(롱 그리퍼 ≈0.14 m) 매달리는 것을 FK(`gripper_x_in_base_fk`)로 보정한 TCP 목표 |
| `preflight_transfer(node, src, dst, direct)` | **이송 전체(픽+플레이스)를 무동작 검증**. 실패 시 허브에서 거부. 회귀 체크: `tools/preflight_check.py` |
| `transfer(node, src, dst)` | preflight → pick → (허브) → place → 허브. 실패 시 `fail()`로 에러코드 세팅 |
| `fail(node, code, detail)` | `node.last_error` + 상세 문자열 (05 문서 ErrorCode) |

### 단독 실행
```bash
python3 vision/tag_vision_node.py                          # 터미널 A
/usr/bin/python3 sequences/wirebonder_pick_place.py        # 터미널 B, 1/2/3 키로 이송
#   c 키 = 재주차 후 재캡처, --no-vision = DEVICES 하드코딩 사용(정밀 주차 필요)
```
(docstring에 `.venv` 경로가 남아 있는데 **시스템 python을 쓸 것** — 07 참고)

## `sequences/test_wirebonder_dispatch.py`

디스패처 경유 wirebonder 구동의 스모크 테스트 (main.py 경로 검증용).

## 설계 노트 — 왜 이렇게 생겼나 (docstring에서 이관)

### shelf_pick_place.py
- **`shelf_pick_to_hub`의 재고 팬텀 취급**: 타깃 박스의 재고 팬텀(`stock_key`)은
  pregrasp가 250 mm 앞, insert/twist가 박스 상단 73 mm 위를 지나므로 접근 스포크·insert
  preflight 동안 **켜 둔다** — 꺼두면 스포크가 타깃 볼륨을 관통해 실제로 스쳤다(실측).
  조가 박스를 실제로 만나는 구간에서만 파킹: descend preflight는 후보별로, 복귀
  스윕+실행은 브랜치 확정 후에.
- **`pocket_place_from_hub`의 guarded place**: v1 복원판이 고정 80 mm 드롭으로 퇴행해
  매 사이클 박스를 포켓 바닥 5 mm **안으로** 릴리즈했고(릴리즈 로그에 매번 찍혔음),
  그 압박 반작용이 AGV 베이스를 아래로 래칫시킴 → 2026-07-15 wirebonder `place_base`의
  guarded 방식(팬텀=접촉 센서, 닿는 순간 정지)을 이식. 포켓별 실제 높이에 안착.
- **`bringup`**: 선반 충돌(보드+재고 팬텀)을 **모션 전에** 앵커 포즈로 먼저 배치 —
  spawn→hub RRT가 이미 선반을 알아야 함. vision-agnostic (라이브 ArUco 읽기는
  호출자 몫: 단독 실행 main()은 즉시, main.py는 LOCATE에서). 포켓 위 스토어웨이
  박스(box_l2c)와 그 삭제 로직은 2026-07-15 제거 — 이제 포켓 위에 아무것도 스폰 안 됨.
- **`capture_shelf`/`aim_pose_at_tag`**: 읽기는 **정지 상태에서만**(dwell + 중앙값).
  조준은 태그의 nominal 월드 위치로 충분 — FOV에만 들어오면 되고 READ 자체가
  range-exact. 왕복은 기록·역재생 후 hub_q로 스냅 (capture_device와 동일 패턴).
- **`refresh_shelf_pose`/`refresh_device_pose`**: stamp로 중복 제거한 n프레임의 축별
  **중앙값**(단일 태그 지터·소수 모호성 플립에 강건). 실패 시 stale 값 폴백 금지 —
  호출자가 abort하거나 기본값 유지를 명시적으로 선택.

### wirebonder_pick_place.py
- **`base_hover_delta`**: 허브가 `base_loc`에서 시드되어 베이스 포켓과 조(jaw) 방위각을
  공유 → 허브↔베이스는 **순수 병진**이라 파지 lateral 오프셋이 양끝에서 상쇄됨.
  덕분에 이 구간은 자유 RRT 대신 결정론적 충돌-게이트 linear servo로 동작
  (슬롯 쪽은 재방향이 필요해 RRT 유지).
- **`top_grasp_pose`**: lateral 방향을 `grasp_tcp_pose`의 IK 유도 jaw_x가 아니라
  **알려진 장비 +y**로 잡음 — slot B에서 그 IK가 뒤틀려(nearest dist ~2.5 rad)
  오프셋이 박스 뒤 wb_Cube_E 쪽으로 플립됐던 실측 버그.
- **`box_world_center`**: top 슬롯은 SLOT_OFFSET 합성값(스폰 높이)이 아니라 **안착
  박스 중심**(SLOT_WORLD 'box')을 씀 — Gazebo가 Cube_C 위로 박스를 튕겨 올려 스폰
  높이보다 ~70 mm 위에 안착하고, 스폰 높이로는 model_at 반경을 벗어난다
  (실측: seq2 "no box model near the target").
- **`grasp`의 tol**: 베이스 포켓은 118 mm 피치라 반경을 좁게, 장비 슬롯은 >300 mm
  간격이라 안착/모델 오프셋을 흡수하게 넉넉히.
- **`pick_front_staged`(slot D)**: 자유 RRT가 못 푸는 접근이라 J1만 스윙해 슬롯 앞으로
  간 뒤 직선 서보로 조그된 접근점→안착→하강. 조그는 접근 **위치**만 캡처했고 조인트
  config가 아니므로 완전한 스테이징 config는 없음. 전체 outbound를 역재생 복귀.
- **`pick_front`**: 현재 시퀀스에서 미사용 — STAGE_JOINTS 없는 front 슬롯(예: A를
  src로)의 문서화된 폴백으로 유지.
- **`place_base`**: 접근은 팬텀 **OFF**로 (표면 근처에서 IK 게이트가 "ok=94,
  collision-free=0"으로 전멸시킴), 하강은 팬텀 **ON**으로 guarded.
- **`transfer` 라우팅**: slot→slot은 직행(들어올린 뒤 팬텀 ON으로 dst까지 RRT),
  베이스 포켓이 낀 이송은 허브 경유(허브가 베이스 방위각 공유, linear servo 구간).
- **`capture_device`**: **단일 뷰** depth 하이브리드(회전=PnP, 거리=depth Z).
  두-뷰 삼각측량(뷰 B)은 폐기 — 뷰 A에선 PnP 회전이 안정적이었지만 사선 뷰 B에서
  플리커했고, 거리는 depth가 한 뷰로 해결. 두-뷰 코드는 vision 모듈에 미사용 폴백으로
  잔존. 틸트가 툴을 재방향시키므로 planned move(goto) 사용.
- **`preflight_transfer`**: dry 레그 목록이 pick()/place()를 수작업으로 미러링함 —
  Live/Dry 실행기 리팩터(`docs/wirebonder_refactor_plan.md`)가 이 중복을 없앨 예정.
