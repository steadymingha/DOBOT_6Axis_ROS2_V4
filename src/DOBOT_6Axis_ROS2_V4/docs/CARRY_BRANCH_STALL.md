# 제약 carry가 멈추는 문제: 엘보/리스트 브랜치 불일치

작성 2026-06-13. 첫 full cycle 성공 직후 기록. 다음에 **"선반에서 잡고 나와
포켓으로 옮기는 CBiRRT carry가 해를 못 구하고 멈출 때"** 그대로 활용할 것.

관련 파일: `cbirrt_pick_place.py`(수정처), `constrained_cbirrt.py`(제약 플래너),
`deploy_optimizer.py`(오프라인 진단 도구, `.venv`로 실행), 탐색 기록은
`docs/DEPLOYMENT_REACHABILITY_ANALYSIS.md`.

---

## 1. 증상

```
===== [7/10] Carry (gripper held down) -> hover above pocket =====
[IK] 12 candidates; nearest dist=4.962
[CBiRRT] goal: nearest of 24 cand(s), joint dist=4.96 rad
[CBiRRT] planning (orientation held)...        <- 여기서 45초 멈췄다 실패
```

- 잡고 나오는 것까지는 성공, **carry(step 7)에서 멈춤.**
- 목표 IK까지의 거리(`nearest dist`)가 ~5 rad(≈286°)로 비정상적으로 큼.
- 방향 고정(tool-down) 매니폴드 위에서 그 먼 거리를 잇지 못해 CBiRRT가 stall.

---

## 2. 진단 절차 (가장 중요)

**추측하지 말고 관절별 갭을 찍어라.** `move_constrained`에 다음 로그가 있다
(없으면 추가): start/goal config와 per-joint gap.

```
[CBiRRT] start(deg)= -112,-2,+82,+11,+90,+158
[CBiRRT] goal (deg)= -46,-5,-125,+40,-90,+134
[CBiRRT] per-joint gap(deg)= J1 +66,J2 -2,J3 -207,J4 +30,J5 -180,J6 -24
```

이 한 줄이 5-rad가 **어느 관절**에 있는지 말해 주고, 그게 해법을 가른다:

| 갭이 지배하는 관절 | 의미 | 방향 |
|---|---|---|
| J1 (~140°+) | pick/place azimuth 차이 = 배치 | AMR/선반 방향 |
| **J3·J5 (각 ~180~200°)** | **엘보+리스트 플립** | **본 문서의 케이스** |
| J6 (~180°) | 그리퍼 yaw 와인딩 | place yaw / J6 언와인드 |

이번 사례: **J3 −207°, J5 −180°**가 지배. J1은 66°, J6는 24°에 불과 →
**엘보+리스트 플립**이 원인. (J1이 작으니 AMR 재배치는 무의미, J6가 작으니 yaw도
무의미.)

오프라인으로 더 파보려면 `deploy_optimizer.py`의 모델로 start config를 FK해
선반 grasp 자세를 복원하고, 그 자세의 IK 브랜치를 결합 충돌모델로 열거한다
(아래 §6 참고).

---

## 3. 근본 원인

**높은 선반과 낮은 포켓이 서로 반대 엘보 브랜치를 강제한다.**

| | 위치(base_link) | 강제되는 브랜치 |
|---|---|---|
| 선반 grasp | z≈0.38 (**높음**), r≈0.43 | 엘보-업 (J3>0, J5>0) |
| 포켓 hover | z≈0.13 (**낮음**), r≈0.38 | 엘보-다운 (J3<0, J5<0) |

6축 팔에서 높은 곳은 팔꿈치를 위로(elbow-up), 낮고 가까운 곳은 아래로(elbow-down)
접는 게 자연스럽다. 두 자세는 **다른 IK family**이고, 그 사이를 가려면 J3·J5가
부호를 바꿔야(특이점 통과) 한다.

carry는 `move_constrained` = **tool-down을 고정한 CBiRRT**(`lock_tilt_only=True`,
tool z만 고정하고 yaw는 자유)로 한다. 이 제약 매니폴드 위에서는 **엘보/리스트
플립을 할 수 없다**(박스를 수평으로 든 채 팔꿈치를 뒤집을 길이 없음). 그래서 stall.

**핵심**: 막힌 건 "경로를 못 찾아서"가 아니라 **시작·목표가 애초에 다른 IK
브랜치라서**다. 같은 브랜치였다면 carry는 ~124°짜리 평범한 이동이다.

---

## 4. 오답 노트 (배제한 가설 — 다음에 반복하지 말 것)

이 문제는 그럴듯한 오진이 많았다. 각각 **왜 틀렸는지**:

1. **"브랜치 선택 로직이 문제다"** → 선택을 고쳐도 후보가 전부 같은(먼) family면
   소용없음. 후보 풀 자체가 문제였다.
2. **"pick/place azimuth가 ~180° 틀어져서다(AMR 재배치 필요)"** → 실제 azimuth
   차이는 ~58~90°(J1 66°)뿐. 로그의 J1=−170°는 *결과*(엘보-업 브랜치의 J1)였지
   *원인*이 아니었다. **AMR 재배치는 J1만 바꾸고 엘보 플립은 못 고친다.**
3. **"그리퍼 yaw(J6) 와인딩이다"** → 실제 J6 갭은 24°뿐. (place yaw를 grasp에
   맞추는 fix도 넣었고 도움은 되지만, 이 케이스의 5-rad는 J6가 아니었다. 검증:
   yaw-자유 twin이 오히려 더 멀었음 → J6 무관.)
4. **"그리퍼를 90° 재장착(flange 수직)하면 reachability가 낫다"** → `deploy_optimizer`
   wrap/pusher 모드로 검증: carry 193~335°로 **오히려 악화**. top-load 포켓은
   수직 손목을 원해서, flange 수직은 낮은 포켓에서 손목을 꼬아 매니폴드를
   조각낸다. **top-down 그리퍼 유지가 맞다.**

교훈: **per-joint gap부터 찍어라.** 위 오답들은 전부 "어느 관절이 5-rad를
차지하는지"를 안 보고 추론해서 생긴 것이다.

---

## 5. 해결책 (구현됨, `cbirrt_pick_place.py`)

원리: **pick을 place와 같은 엘보/리스트 family로 잡으면**, 픽→carry→플레이스
전체가 한 브랜치 안에 머물러 플립이 사라진다(carry ~124°, CBiRRT가 바로 연결).

1. **`J4` 한계를 음수 허용** (`setup_planner`):
   `self.joint_limits[3] = (radians(-60), radians(120))` (기존 `[0,120]`).
   - 엘보-다운 선반 grasp 브랜치는 `J4≈−11°`를 필요로 하는데, `[0,120]`가 막고
     있었다. **이 소프트웨어 클램프 하나가 진짜 병목**(하드웨어는 ±6.27 rad,
     원래 base는 `(-π,π)`였음).

2. **`move_to_pose_ref(target, ref_q)`** 추가:
   목표 IK 후보 중 **참조 config(ref_q)에 가장 가까운 브랜치**를 골라 free-RRT.
   (기존 `move_to_pose`는 *현재 자세* 최근접 = 엘보-업을 골랐다.)

3. **사이클이 포켓 브랜치로 grasp**:
   포켓 place config(`place_ref`, 엘보-다운)를 **사이클 시작에 미리 계산**해
   pregrasp(`move_to_pose_ref`)에 넘긴다. → 박스를 포켓과 같은 family로 잡음.
   (standby가 J3=−105로 이미 엘보-다운이라 이 pregrasp RRT는 오히려 더 쉽다.)

4. **`move_constrained(yaw_free=True)`** (보조, J6 와인딩 차단):
   carry goal을 박스 180° 대칭 `{place, place+180}`(둘 다 tool-down, 같은 footprint)
   중 carry-start 최근접으로 선택. step 7의 J1/J6 사전정렬 해킹은 제거.

### 5-1. place 위치 미세조정 (사용자가 실물 맞춤)
- `pocket_hover_xyz`의 y를 실물 포켓에 맞춰 오프셋(`pocket_y-0.17` = pocket no.3,
  `-0.05` = no.4). POCKET_Y 배열값과 실제 매거진 칸의 정렬차를 흡수.
- 내림 높이 `PLACE_TCP_ABOVE - POCKET_HOVER + 0.01` (1cm 더 높게 release).
이건 carry와 무관한 **배치 정렬 튜닝**이므로 실물/시뮬에서 그때그때 맞춘다.

---

## 6. 검증

오프라인(`.venv`, 결합 충돌모델 + 실제 한계값):
- 포켓 place 브랜치: J3=−134, J5=−90 (엘보-다운)
- 선반 grasp: 엘보-다운 브랜치(J3=−82, J5=−91, J4=−11) **존재·충돌프리** (J4 확장 덕)
- 픽이 엘보-다운 선택(place_ref 최근접) ✓
- **carry grasp→포켓 = 124°, 플립 없음** (per-joint 전부 <80°)

실물/시뮬: **full cycle 성공** (2026-06-13). 기대 로그:
```
[cycle] pocket place branch: J3=-134 J5=-90 deg (pick will match it)
[pick] pre-grasp branch nearest pocket (of N): J3=-82 J5=-91 deg
[CBiRRT] per-joint gap(deg)= J1 -75,J2 -26,J3 -52,J4 +79,J5 +1,J6 +15
```
`per-joint gap`에 ±180°짜리가 없으면 OK.

### 남은 주의 (오프라인 검증 불가였던 곳)
선반 틈 **삽입(step 2)·후퇴(step 6)** 를 엘보-다운 자세로 할 때 팔뚝이 아래
선반판에 닿을 수 있다(오프라인 모델엔 선반판 없음). 이번엔 통과했지만, 선반
높이/틈이 바뀌면 여기부터 의심할 것.

---

## 7. 재발 시 체크리스트

1. carry stall 로그에서 **`per-joint gap`** 확인. ±180° 가까운 관절이 범인.
2. **J3·J5 동시에 큼** → 엘보/리스트 브랜치 불일치(본 문서). 아래로.
3. start/goal config를 FK해 선반·포켓 높이차 확인(높은 곳 vs 낮은 곳).
4. **pick을 place의 family로** 잡게 하라(`move_to_pose_ref` + `place_ref`).
5. 호환 브랜치가 관절제한에 막혀 있나 확인 — 특히 **J4 음수**, J5 범위. 충돌프리면
   푼다(정책: 자가·환경 충돌만 없으면 제한 해제).
6. `deploy_optimizer.py`로 "두 자세가 같은 family에서 충돌프리로 잡히는지" 오프라인
   확인 후 실물 1회 실행.
7. J1만 크면(엘보 동일) → 그건 azimuth/배치 문제(다른 시나리오).
8. J6만 크면 → place yaw / J6 언와인드(`move_constrained` yaw_free).
