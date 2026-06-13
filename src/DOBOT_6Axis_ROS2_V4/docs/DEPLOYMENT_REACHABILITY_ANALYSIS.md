# 배치·도달성 분석 (선반→포켓 왕복이 막히는 이유와 해법)

작성 2026-06-13. 관련 코드: `deploy_optimizer.py`, `reachability_map.py`,
출력물 `reachability_out/deploy_*.csv`, `reachability_out/reach_*.csv`.

> **⚠️ 최종 정정 (2026-06-13, §7 참조).** 아래 §3~4의 "배치 azimuth가 원인,
> 포켓을 옮겨라"는 결론은 **틀렸습니다.** 실제 azimuth 차이는 ~90°뿐이고 그 고유
> carry는 ~126°(bridgeable)입니다. 5-rad의 진짜 원인은 **J6 손목 와인딩**(grasp
> yaw + 인-갭 트위스트 vs 고정 `PLACE_YAW=0` goal)이며, **소프트웨어로 수정**했습니다
> (`move_constrained` yaw-자유 goal). §7에 정정된 진단·해법·구현이 있습니다.
> §3~6은 그 과정에서 나온 보조 결과(마운트 비교 등)로 유효합니다.

---

## 1. 무엇을 풀려고 했나 (문제)

선반에서 상자를 꺼낸 뒤 **포켓으로 옮기는 carry 단계에서 CBiRRT가 해를 못 구함.**
로그상 현재 자세와 포켓 IK 해가 관절공간에서 **약 5 rad(≈286°) 떨어져** 있고,
J1이 포켓 반대 방향으로 도는 증상이 반복됨. 단순히 브랜치 선택 로직을 고쳐도
증상이 그대로였음 → "왜 5 rad나 떨어지는가"를 근본부터 분석할 필요.

핵심 질문 4가지:
- **Q3** 로봇을 어느 방향으로 세팅해야 reachability가 최대인가? (현장에서 회전 불가)
- **Q4** 갈 때는 길이 있는데 올 때(CBiRRT) 길이 없으면 그 시퀀스는 불가능. 편도가
  아니라 **왕복**으로 판단해야 한다.
- **Q5** flange면이 바닥에 수직인 자세가 아래보기보다 reachability가 좋은가?
- 관절제한은 자가·환경 충돌만 없으면 성공할 때까지 푼다.

---

## 2. 무엇을 도구로 썼나 (방법)

### 2-1. 기존 reachability_map 데이터 (`reachability_out/reach_*.csv`)
- voxel별 reachability: `x,y,z, RI, n_ok, n_total, down`.
  - `RI` = 테스트한 자세 중 IK 풀린 비율 (0~1).
  - `down` = "거의 정확히 아래보기(±15° 또는 ±5°)" 자세로 도달 가능 여부.
- **한계**: orientation을 "down 여부" 이진 플래그로만 저장 → "flange 수직(수평
  접근)"을 직접 구분 불가. 그래서 Q5는 재계산이 필요했음.
- pinocchio가 ROS 환경엔 없고 **`dobot_ws/.venv`에 있음** → 재계산은 venv로 실행.

### 2-2. 새 도구 `deploy_optimizer.py` (이번에 작성)
경로를 짜는 게 아니라, **배치 질문에 숫자로 답하는 도구**:
- 자유 변수를 스윕하며 각 설정에서 **선반 grasp + 포켓 4개의 실제 IK를 풀어**
  왕복의 관절공간 비용을 측정.
- **목적함수** `round_cost = max_over_pockets( ||q_pocket − q_shelf|| )` (rad).
  - `q_shelf` = standby에 가장 가까운 충돌프리 선반 grasp IK.
  - `q_pocket` = `q_shelf`에서 seed해 가장 가까운(=carry가 실제로 이을) 포켓 IK.
- 부가 지표: J1 스윙, 최악 단일관절 갭, **flip 여부**(단일관절 갭 > 180° = 브랜치
  뒤집힘 → CBiRRT가 못 잇는 신호).
- IK·FK는 reachability map과 **동일한 솔버**(`ReachabilityModel`) 사용.
- 접근 모드 4종: `down`(현재, tool-z 아래) / `side`(수평 삽입) /
  `wrap`(그리퍼 90° 재장착, Link6를 grasp 위에) / `pusher`(그리퍼 그대로, 플랜지를
  푸셔 뒤에 브래킷 장착, Link6를 grasp 뒤에).

> **주의(범위)**: deploy_optimizer의 충돌검사는 **팔 자가충돌만** 본다(환경/큐브/AGV/
> 선반 제외). 상대 비교엔 충분하지만 절대 충돌 보장은 아님. 선반 거리·높이는
> `SHELF_R=0.55, SHELF_Z=0.28`(물리값 추정, 라이브 TF 아님)으로 두고 azimuth만 스윕.

---

## 3. 무엇이 나왔나 (결과)

### 3-1. 이 팔에게 "정확히 아래보기"는 희귀 자세다
기존 reach 데이터 집계:

| 측정 | down 가능 비율 |
|---|---|
| reachable voxel 중 down (±15°) | **3.7%** (3352/91140) |
| reachable voxel 중 down (±5°) | **0.7%** (1288/180070) |
| FK 샘플 중 down 자세 (±15° / ±5°) | 1.2% / 0.13% |

→ CR7 손목 구조상 tool-z를 정확히 −Z로 맞추면 J5/J6가 한계 근처로 몰림.
나머지 96%의 reachable 영역은 기울어진/수평 자세. **포켓도 down 가능하나 dexterity가
낮음**(RI 0.12~0.17, 포켓 band에서 down voxel은 9%).

### 3-2. 5-rad 갭의 뿌리 = pick과 place의 azimuth 불일치 (기하)
`cr7_on_mpo700.urdf.xacro`의 `arm_mount_joint`가 `rpy="0 0 π"` →
**팔이 AMR 대비 yaw 180° 뒤집혀 장착**. 결과로:
- 포켓: 팔 정면(base +x) = AMR 후방, azimuth ≈ 0°.
- 선반: AMR 측면에서 잡음, azimuth ≈ ±90° 이상.
- 둘이 90°+ 벌어져 → J1 대각 스윙 강제 → 5 rad + 브랜치 flip.

### 3-3. 선반 azimuth(ψ) vs 왕복비용 (down grasp, seed 무관 재현)
ψ = base_link에서 선반이 놓이는 azimuth (= AMR 주차 방향으로 조절).

| 선반 ψ | 왕복 갭 | J1 스윙 | flip |
|---|---|---|---|
| **0° (팔 정면)** | **106°** | **31°** | 없음 |
| ±45° | 116–123° | 65–76° | 없음 |
| ±90° | 146° | 110° | 없음 |
| ±135° | 183–192° | 155–166° | 없음 |
| ±165~180° (**≈ 현재, 선반 후방**) | 209–232° | 186–211° | **YES** |

현재 설정이 맨 아랫줄(5 rad + flip). **선반을 팔 정면(ψ≈0)으로 가져오면
286°→106°, flip 소멸.** ψ ±45° 이내면 여유 충분(≤123°, no flip).

### 3-4. "팔 장착 yaw만 돌리면 되지 않나?" → 무효 (검증)
팔을 돌리면 선반·포켓이 **같이** 돌아 상대 갭 불변. 선반-포켓 상대갭을 135°로 고정하고
전체를 회전:

| 전체 yaw 회전 | 왕복 갭 |
|---|---|
| 0° | 183° |
| +45° / +90° | **273°** (오히려 악화) |
| −90° | 183° |
| +180° | 226° |

효과 있는 건 **상대 azimuth를 줄이는 것**뿐. 포켓을 선반 쪽으로 이전(상대갭 축소):

| 선반-포켓 상대갭 | 왕복 갭 |
|---|---|
| 135° | 183° |
| 90° | 147° |
| 45° | 116° |
| **0°** | **106°** |

### 3-5. Q5: 접근/마운트 4종 비교 (최적 왕복 갭)

| 모드 | 설명 | 최적 왕복 갭 | flip |
|---|---|---|---|
| **down** | 현재, tool-z 아래 | **106° (ψ=0)** | 없음 |
| side | 수평 삽입 | 265° | YES |
| wrap | 그리퍼 90° 재장착, Link6 위 | 193–327° | — |
| pusher | 그리퍼 그대로, 플랜지 푸셔 뒤 | 213–335° (237°@ψ=0) | — |

**flange 수직 마운트(wrap·pusher)는 Link6 위치(위 vs 뒤)와 무관하게 모두 2~3배 악화.**
원인은 위치가 아니라 **방향**: 포켓이 **top-load(위에서 수직 삽입)** 라, Link6 z를
수평으로 둔 채 낮은(z≈−0.05) 포켓에 내려가면 손목이 꺾여 J4≈120°·J5≈0° 한계에 몰리고
인접 4포켓이 **끊어진 손목 브랜치**로 흩어짐(carry 불가).

> Q5 직관 평가: "flange 수직이 일반 reachability는 더 좋다"는 **맞다**(down은 3.7%뿐).
> 그러나 이 task가 필요로 하는 *특정* 포켓-place 자세는 그 나쁜·조각난 영역에 떨어지고,
> task를 막는 건 일반 reachability가 아니라 **왕복 연결성**이다.

---

## 4. 결론 / 권고

1. **end-effector 마운트로는 답이 없다.** down/side/wrap/pusher 중 **down(현재)** 만
   carry가 싸다. 그리퍼·마운트는 현재 top-down 유지.
2. **진짜 지렛대 = pick↔place 상대 azimuth 축소.** 둘 중 하나:
   - 매거진 포켓을 AMR의 **선반-마주보는 면**으로 이전(하드웨어, 가장 견고), 또는
   - AMR을 선반에 **후면 수직 주차**(하드웨어 무변경, 통로 폭 필요).
   → 왕복 286°+flip → **106°·flip 없음**.
3. 배치 수정 후엔 `cbirrt_pick_place.py` step 7의 J1/J6 사전정렬 해킹이 **불필요** —
   평범한 carry로 풀린다.
4. 관절제한은 현재 J1[−180,90] 등 넓힌 상태로 분석했고, 충돌프리면 더 풀어도 무방.

---

## 5. 남은 한계 / 다음 단계 후보

- **환경충돌 미반영**: deploy_optimizer는 자가충돌만. 포켓 재배치 안을 확정하면
  큐브/AGV/선반 포함 모델로 재검증 필요.
- **선반 r/z 추정값**: 라이브 TF로 현재 선반 base_link 좌표를 받아 "수정 전" ψ를
  정확히 찍으면 기준선이 확정됨.
- **side-load 포켓 시나리오 미검토**: 매거진을 옆으로 꽂는 방식이면 flange 수직이
  유리해질 수 있음 → 필요시 `pusher`/`wrap` place를 side-load로 모델해 재비교.
- **Q4 일반화(왕복 roadmap)**: 지금은 "선반·포켓 IK 갭"으로 왕복을 근사. 더 엄밀히는
  voxel별 IK 해를 저장해 (voxel,branch) 그래프로 연결성을 판정하는 roadmap 확장.

---

## 7. 최종 정정: 진짜 원인은 J6 yaw 와인딩 (구현 완료)

사용자 지적("azimuth 차이는 ~90°뿐, 180°는 이상하다")이 옳았다. 분해 결과:

- ψ=90°에서 최근접 브랜치 carry = **~126°**, J1 스윙 74°, **flip 없음**. 90° 배치는
  문제가 아니다.
- 5-rad의 정체 = **J6 손목 와인딩**. grasp는 매거진 행에 맞춰 `yaw=phi=atan2(row_dir)`
  로 잡는데 π 장착 탓 phi≈180°, 거기에 인-갭 `rotate_j6(+90°)`가 더해진다. place가
  `PLACE_YAW=0` 고정이라 손목이 그 ~180°를 도로 풀어야 한다.
- grasp yaw 0→180 스윕 시 J1은 −105°로 **고정**인 채 carry만 146→283°로 증가 →
  J1/azimuth가 아니라 J6(yaw)임이 증명됨.

**관절별 분해 (carry-start → 포켓):**

| | J1 | J2 | J3 | J4 | J5 | J6 | 합 |
|---|---|---|---|---|---|---|---|
| goal yaw=0 고정 (구) | 74 | 42 | 41 | 83 | 0 | **164** | **206°** |
| goal yaw 자유→최근접 | 74 | 42 | 41 | 83 | 0 | **4** | **126°** |

차이는 오직 J6(164°↔4°). 나머지는 동일한 고유 재구성.

**왜 yaw가 박혔나:** carry 경로 제약은 이미 yaw 자유다(`ConstrainedPlanner`
`lock_tilt_only=True` → `_project`가 `e[:2]`만 써 tool-z만 고정). 그러나
`move_constrained`가 `goal_q = compute_ik_ordered(place_quat)`로 **full orientation
(yaw 포함)을 풀어** goal에 J6를 박았다. 경로는 yaw 자유여도 그 고정 goal에 도착해야
하니 J6를 감았다 푼다.

**구현한 해법 (하드웨어 무변경):**
- `move_constrained(..., yaw_free=True)`: goal을 박스 180° 대칭 `{place, place+180}`
  (둘 다 tool-z 아래, 같은 footprint) 중 **carry-start에 가장 가까운 IK 해**로 선택.
  `_twist_pose_180` 헬퍼가 tool-z를 보존한 채 yaw만 180° 돌린 후보를 만든다.
- step 7의 J1/J6 사전정렬 해킹(구 7a~7d) 제거 → 평범한 `move_constrained(hover_pose)`.
- 검증: carry 206°→126°, flip 없음, tool-z 전 구간 아래 유지.

**보장:** 경로(tilt-only)와 goal(twin, tool-z 아래) 모두 "아래 보기"를 유지하므로
선반→베이스 이송 내내 그리퍼가 아래를 향한다. §4의 "포켓 이전/재주차/재장착" 권고는
**철회**한다.

---

## 6. 재현 방법

```bash
cd ~/dobot_ws/src/DOBOT_6Axis_ROS2_V4
# 마운트 4종 비교 (psi 전체 스윕)
~/dobot_ws/.venv/bin/python deploy_optimizer.py --approaches down,side,wrap,pusher
# 선반 거리/높이 바꿔 재확인
~/dobot_ws/.venv/bin/python deploy_optimizer.py --approaches down --shelf-r 0.6 --shelf-z 0.25
# 출력: 콘솔 랭킹 + reachability_out/deploy_<timestamp>.csv
```

조정 파라미터: `POCKET_X/Y/Z`, `SHELF_R/Z`, `DEFAULT_LIMITS_DEG`,
`PUSHER_BACK/UP`, `--psi-step-deg`.
