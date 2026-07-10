# 시퀀스 1 실패 / 복귀 스윙 / 전체 preflight 정비 기록 (2026-07-10)

증상 3건 → 계측 진단 → 수정 → 세 시퀀스 vision 모드 end-to-end 검증까지의 기록.

## 증상

1. **seq 1 (base→slot A)**: pick 후 place에서 `place wb1:A insert: stopped at 48 mm
   of 130 mm -> collision (gripper_base_link_2, wb_Cube_C)` — 박스를 문 채 정지.
2. **seq 2 (B→C)**: place 후 hub 복귀가 free RRT라 매번 다른 경로로 팔을 크게
   휘두르며 선반 쪽을 스침.
3. **preflight 부재**: seq 1은 place 일부만, seq 2/3은 전혀 사전 검증이 없어
   불가능한 transfer가 중간에 멈춤(박스 스트랜딩).

## 진단 (`tools/diag_seq_dryrun.py`)

세 시퀀스의 모든 레그를 live와 같은 솔버(`cbirrt.linear_path` + `is_state_valid`)로
팔을 움직이지 않고 체인 재현. 발견:

- **seq 1의 진범은 insert가 아니라 접근 경로 전체**: 캐리 박스는 TCP에서 죠
  방향으로 **~256 mm 돌출**(lateral 138 + 길이 236/2). 장치 전면(Cube_A/B,
  전면 world y≈0.24)과 hub→slot A 대각선의 여유가 사실상 0이라, 도착 자세
  (IK 적분 branch)에 따라 approach에서 박스가 타워에 걸리거나(오프라인 재현:
  carried_box×Cube_B), 통과해도 다른 자세로 도착해 insert에서 그리퍼가 C에
  걸림(사용자 로그). 깨끗한 자세에서는 insert 130/130 mm 통과 — 즉 **경로/자세
  문제이지 슬롯 기하 문제가 아님**.
- **seq 2 복귀**: post-place 자세가 hub와 J1/J6 기준 +2.75 rad 다른 branch.
  직선 joint 보간은 t=0.13에서 장치와 충돌(계측) → **joint_move 전환은 불가**.
- 이 dry-run 체인이 곧 full preflight의 프로토타입.

## 수정 (`sequences/wirebonder_pick_place.py`)

1. **seq 1 place = 고공 transit 접근** (`PLACE_TRANSIT_Z=1.40`,
   `PLACE_STAGE_BACKOFF=0.07`, `front_place_legs()`):
   lift(박스 바닥이 Cube_C 상단 1.275를 ~40 mm 클리어) → 슬롯 위로 traverse →
   approach보다 70 mm 앞(y)에서 descend(내려올 때 박스 전면이 C 전면을 ~23 mm
   클리어) → +y slide → insert. 전부 직선 servo라 사전 검증 가능, capture/역재생
   복귀도 그대로 유지. 상한: transit z ~1.57 이상은 tool-down lift가 자기충돌
   (carried_box×Link2) — 주석에 명시.
2. **seq 2 복귀 = transit 역재생**: pick hover / place approach 두 이동 레그를
   기록(`node._pick_transit/_place_transit`)해 두고, place 후 `join(rev, rev)`로
   한 번에 되짚어 hub 복귀(+`go_to_hub` 스냅). 방금 실행한 경로의 역이라 안전이
   구성적으로 보장, RRT 무작위 스윙 제거.
3. **전 시퀀스 full preflight** (`preflight_transfer()`, 기존 `preflight_place`
   대체): pick과 place의 **모든** 계획 가능한 레그를 hub 기준으로 체인
   dry-run(레그 끝 config가 다음 레그의 시작; phantom 토글도 live와 동일).
   실패 시 팔이 **전혀 움직이기 전에** `fail(UNREACHABLE, ...)` →
   `node.last_error`/`last_error_detail`에 기록되어 dispatcher의 [REPORT] seam이
   MCS로 보고. 런타임 전용 이벤트(attach, guarded 접촉, 유효한 goal의 RRT 탐색)
   는 기존 런타임 실패 경로 유지.
   ponytail: 레그 목록이 pick()/place()와 수동 미러 — Live/Dry executor 통합
   (`wirebonder_refactor_plan.md` Phase 2)이 이 중복을 없애는 다음 단계.

## 검증

- 오프라인: `tools/preflight_check.py` — 세 시퀀스 preflight 전부 PASS (팔 정지
  상태에서 회귀 확인용; waypoint/기하 상수 변경 후 돌릴 것).
- 라이브(vision 모드, MCS `/mcs/command` 트리거, 깨끗한 sim):
  - seq 1: 전 레그 통과(insert 130/130), box_l2c → slot A (2.000, 0.440, 0.960),
    오차 ~2 mm, 역재생 복귀.
  - seq 2: box_l2a → slot C (2.699, 0.442, 1.345) 오차 ~1.5 mm, yaw −0.02 rad,
    transit 역재생으로 hub 정확 복귀(스윙 없음).
  - seq 3: box_l2b → base pocket (1.971, −0.139) guarded 접촉 안착(87 mm에서
    CONTACT), 기존 경로 그대로.
  - vision read가 anchor와 정확히 일치(dxyz 0/0/0 mm) — depth-upright 파이프라인
    (`vision_viewpoint_dependence_fix.md`)의 상쇄가 live에서도 성립.

## 남은 일

- [ ] preflight 거부 케이스의 실제 MCS 통신 확인은 comms 브리지 연결 후
      (현재는 [REPORT] seam의 print + `node.last_error`까지 검증).
- [ ] Live/Dry executor 통합 리팩터링(plan doc Phase 1–2)으로 preflight와 live
      레그 정의의 수동 중복 제거.
- [ ] `PLACE_TRANSIT_Z`/`PLACE_STAGE_BACKOFF`는 sim 기준 TUNE 값 — 실물 장치
      실측 후 재확인.
