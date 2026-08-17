# Claude Code 작업 지시: 비전 기반 호버 + J6 정렬 (cbirrt_p1p2 확장)

## 배경

- verify_chain PASS: 정차 오차 포락선에서 P_base 반복정밀도 p95 1.14mm
- cbirrt_p1p2_test.py로 교시점 기반 pick-place 리허설 완료 (부호 규약 수정, 토크 접촉 감지, 안전 게이트 검증됨)
- 이번 단계: **p1을 교시값이 아니라 비전으로 획득**해서, 매거진 상방 10cm에 플랜지를 세우고 J6를 ±90° 돌려 파지 방향을 맞추는 데까지. **하강 없음.** 하강(충돌레벨 반영)은 이 단계가 실측 검증된 뒤 별도 지시한다.
- 이 테스트가 곧 **절대 편향 측정**이다: 도착 후 사람이 줄자로 "플랜지가 정말 매거진 상면 중심 위 10cm인가"를 재면, 비전→변환→IK→실행 전체 사슬의 절대 오차가 처음으로 숫자가 된다.

먼저 읽을 것: `real_robot_joint_convention.md`, `real_robot_p1p2_test.md` (특히 §9.1 프레임, §6 안전 게이트).

## 형태

- `test/vision_target.py` **신규 모듈**: 비전 획득 + 좌표/IK 계산 전담
- `test/cbirrt_p1p2_test.py`에 `--vision` 모드 추가: 기존 approach/settled/안전 게이트 기계를 재사용하고, 변경은 최소로
- vision_bridge 모듈(vision_client, gate, transform, config) import 재사용 — 재구현 금지. sys.path에 `{BRIDGE_DIR}` 추가

## vision_target.py 사양

```python
acquire_hover_q(node, mon) -> (q_urdf, info)
```

1. 러너 set_mode MAGAZINE → gate 스냅샷(10프레임 평균) → xyz_cam
2. RobotFeed(또는 mon)의 tool_vector로 P_base 계산 — vision_bridge의 transform.to_base 그대로 사용. **P_base는 base_link(GetPose user=0 tool=0) 기준이다**
3. 목표점 산출 (전부 base_link 기준):
   ```
   hover = P_base + [0(*), 0(*), MAGAZINE_HEIGHT_M/2 + HOVER_CLEARANCE_M]
   ```
   - `MAGAZINE_HEIGHT_M = 0.14` — 비전 점은 전면 **중심**이므로 상면까지 +높이/2
   - `HOVER_CLEARANCE_M = 0.10`
   - `MAGAZINE_INWARD_M = 0.0(*)` — 전면 중심 연직선상이 아니라 몸통 중심 위에 서려면 전면 법선 안쪽으로 깊이/2가 필요하다. 매거진 깊이 실측 전이므로 **상수 자리만 만들고 0으로**. (*)의 수평 성분은 이 상수가 채워지면 전면 법선 방향으로 적용 — 법선은 이번 배치에서 base 어느 축에 가까운지 사람이 config로 지정 (`INWARD_AXIS = "+x"` 식). 자동 추정하지 말 것
4. **프레임 변환**: IK는 pinocchio 모델 루트 기준이고 base_link는 루트보다 30mm 위다 (§9.1). `flange_in_base()`가 하는 환산의 **역방향**을 적용해 hover를 모델 프레임으로 옮길 것. 이걸 빼먹으면 30mm 계통 오차가 조용히 들어간다 — 단위시험으로 잡아라: 현재 관절각의 FK를 flange_in_base로 base_link 값으로 바꾼 것이 GetPose와 수 mm 이내인지, 그리고 그 역변환의 왕복이 항등인지
5. **자세**: 툴 축 수직 아래(straight-down). `--level`이 쓰는 자세 스냅/IK 기계를 재사용해 "플랜지 원점 = hover, 자세 = 수직"의 q를 푼다. IK 실패 시 명확한 에러로 종료
6. **플랜지 기준이다.** TCP_OFFSET_M(팬텀 그리퍼 120mm)을 쓰지 말 것 — 실물에 없다. hover는 플랜지 원점의 위치다. (팬텀 그리퍼 충돌체는 hover 아래로 40mm쯤 드리우지만 매거진이 충돌 모델에 없으므로 계획엔 영향 없음 — 물리적으로도 카메라 홀더뿐이라 접촉 없음. 이 사실을 주석으로 남길 것)
7. sanity check — 하나라도 걸리면 이동 없이 종료:
   - hover가 config `VISION_WORK_BOX` 안 (사람이 실측해 채우기 전엔 실행 거부)
   - 현재 플랜지→hover 이동량 < 0.40 m
   - xyz_cam의 z가 gate 거리범위 안
   - 등록된 면(surfaces.json)과의 간섭은 기존 플래너가 어차피 검사 — 중복 구현 금지
8. 반환 info에 기록용 전부: xyz_cam, P_base, hover, 스냅샷 상세(bbox, valid, sd, n_frames), req_id, git 해시

## --vision 모드 시퀀스

```
1. 관측 자세 'obs' 접근 (교시값; --teach 선택지에 obs 추가, --level 불필요)
2. settled 확인
3. vision_target.acquire_hover_q()
4. dry면: 계획·목표 전부 출력하고 종료. 기본은 dry — --run일 때만 이동
5. approach(hover q)  — 기존 approach() 그대로 (안전 게이트·면 울타리 포함)
6. settled 확인
7. J6 회전: 현재 q에서 J6만 ± config.J6_ROT_DEG (기본 +90, CLI --j6-deg -90 가능)
   - 회전 전 관절 한계 확인 — 넘으면 반대 부호를 제안하고 중단 (자동 반전 금지)
   - 기존 실행 경로(execute_trajectory)로 2점 궤적 전송 — 안전 게이트 통과
8. settled 확인 → 도착 보고 출력:
   - 명령 hover vs 실제 flange_in_base (컨트롤러 추종 오차)
   - "지금 줄자로 잴 것: ① 플랜지 중심의 매거진 상면 중심 대비 X/Y 편차 ② 상면까지 수직 거리 (기대 100mm)"
   - 사람 실측값 입력받아 (mm, 스킵 가능) info와 함께 json 저장: vision_hover_{ts}.json
9. 종료. 원위치 자동 복귀 없음. 러너 IDLE 복귀
```

p2, carry, station(하강), home 복귀는 --vision 모드에서 **실행하지 않는다**.

## 하지 말 것

- guarded_descend 호출 (하강 전면 금지 — 다음 단계)
- 교시 p1 폴백 (비전 실패 시 명확히 중단; 엉뚱한 좌표보다 정지가 낫다)
- vision_bridge 모듈·러너 코드 수정
- 부호 규약(JOINT_SIGN_REAL) 재적용 — 기존 계층이 처리한다. vision 경로의 P_base는 tool_vector 직접 계산이라 관절 규약과 무관
- 팬텀 TCP 기준 목표, MAGAZINE_INWARD 자동 추정, J6 한계 초과 시 자동 부호 반전

## 진행 순서

1. vision_target.py + 프레임 왕복 단위시험 (로봇 이동 없음)
2. --teach obs 지원 추가 → 사람이 obs 교시
3. --vision --dry: 목표 수치 검토
4. --vision --run (SpeedFactor 낮게, E-stop 대기 하에)
5. 사람 실측 → 절대 편향 기록. 3~5회 반복해 평균이 보정 상수 후보

각 단계 실행 방법 안내할 것.
