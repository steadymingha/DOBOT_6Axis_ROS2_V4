# TODO

## Goal
잡고/내리는 위치가 바뀌어도 엘보/리스트 **브랜치 불일치 carry stall**(docs/CARRY_BRANCH_STALL.md)을
다시 겪지 않도록 두 가지 안전장치를 추가한다:
- **(A) 런타임 carry preflight** — 잡기 전에 픽·플레이스가 공통 브랜치로 연결 가능한지
  확인하고, 호환되는 grasp 브랜치를 자동 선택하며, 불가능하면 박스를 잡기 전에 중단.
- **(B) 보강된 1cm reachability 맵** — voxel마다 down-grasp 브랜치 가용성을 저장해,
  저장 파일을 **읽어서** 픽/플레이스 조합의 carry 가능성을 오프라인 예측.

## 배경
원인·해결 기록은 `docs/CARRY_BRANCH_STALL.md`. 현재 이미 들어간 fix: J4 음수 허용,
`move_to_pose_ref`(픽을 포켓 브랜치로 grasp), `move_constrained(yaw_free)`. 이 작업은
그 위에 "위치가 바뀌어도 자동으로 거르는" 게이트/예측을 얹는다.

## Tasks

### 1. 공통 브랜치 유틸 (A·B 공용)
- [ ] `branch_signature(q)` 추가: 관절각 → 엘보(J3 부호)·리스트(J5 부호) [필요시 J4 부호]를
      인코딩한 작은 정수/튜플. 플래너와 맵 양쪽에서 import 가능한 위치(`branch_utils.py`
      신규, 또는 `reachability_map.py` 내 함수)에 둔다.
- [ ] `enumerate_branches(model, pos, R, collision_fn, n_restart, dedup_deg)` 추가:
      한 자세의 **충돌프리 IK 브랜치들**을 관절거리로 dedup해 반환(맵 IK 솔버 재사용).
- [ ] 단위 확인: 알려진 선반 grasp 자세에서 엘보-업/다운 두 브랜치가 잡히는지 출력으로 확인.

### 2. (A) 런타임 carry preflight
- [ ] `CBiRRTPickPlace.preflight_carry(grasp_pose, place_hover_pose, max_joint_gap)` 추가:
      양쪽 브랜치를 열거(plate은 yaw·yaw+180 둘 다), **모든 단일관절 갭 < max_joint_gap**
      이고 충돌프리인 (픽,플레이스) 쌍 중 최소 비용 쌍을 찾아 `(ok, grasp_ref_q, reason)` 반환.
- [ ] 임계값 상수 `CARRY_MAX_JOINT_GAP_DEG`(기본 150) 정의 + 주석.
- [ ] `shelf_to_base_cycle`에 통합: **step 1(잡기) 이전**에 `preflight_carry` 호출.
      - 불가 → 어느 관절이 못 잇는지 로그 + `return False`(박스 미파지).
      - 가능 → 반환된 `grasp_ref_q`를 `move_to_pose_ref`에 넘김(현재 `place_ref` 즉석
        계산을 이 결과로 대체).
- [ ] `move_constrained(yaw_free)` carry는 변경 없음(그대로 사용).
- [ ] 오프라인 검증: 알려진 선반 grasp 자세 + 포켓 hover 자세를 넣어 preflight가 `ok=True`,
      grasp 브랜치 J3<0(엘보-다운)을 반환하는지 `.venv`로 확인.

### 3. (B) 보강된 reachability 맵
- [ ] `reachability_map.py`: voxel마다 **down-grasp 브랜치 가용성 비트마스크**(엘보×리스트
      family별 down-reachable 여부) 기록 → CSV 열 `down_branches` 추가. JSON meta에 설명.
- [ ] `load_csv`가 새 열을 읽도록 갱신(열 없으면 0 = 하위호환).
- [ ] `predict_carry(csv, pick_xyz, place_xyz)` 추가: 최근접 voxel을 찾아 **공통 down-branch가
      있는지** → 가능/불가 예측(+어느 family) 반환.
- [ ] 재생성 명령 문서화: **1cm + 현재 풀어둔 한계(J1[-180,90], J4[-60,120], …)** `--limits-deg`로
      지정 + 작업영역 `--bounds`로 한정. 무거우므로(~30~60분, `.venv`) **사용자가 실행**.

### 4. 문서
- [ ] `docs/CARRY_BRANCH_STALL.md` 체크리스트에 "preflight가 런타임에서 자동 차단; 오프라인
      예측은 보강 맵 `predict_carry`로 읽어서 판정" 한 줄 추가.

## 참고사항 (결정/위험 — 승인 시 확정)
- **결정(확인 요)**: (A) 불가 처리 = **감지+중단/보고**만(staging 재파지 fallback은 후속으로
  분리). (B) voxel 저장 = **브랜치 마스크**(컴팩트). 대안: 풀 관절각 J1–J6(파일 큼, IK 시딩 가능).
- **위험/사실**: 저장된 기존 맵들(reach_*)은 **옛 한계**(J1[-101,10]/J4[0,120])로 계산돼
  새 엘보-다운 브랜치를 "불가"로 **오판**한다. 재생성은 **반드시 현재 한계**로. 해상도(1cm)는
  부차적.
- 맵 재생성은 무거워 **사용자 머신(.venv)에서 실행**; 코드는 여기서 작성, 실행은 사용자.
- preflight = 정확한 **런타임 게이트**(실제 자세+현재 충돌). 맵 = **오프라인 스크린**
  (voxel 해상도 + canonical-down 기준 → 보장 아닌 예측).
- 모든 주석/출력은 영어(CLAUDE.md). 맵 IK/충돌은 기존 `ReachabilityModel` 재사용.
