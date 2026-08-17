# 파일 정리 인수인계 — 2026-08-11/12 세션

이 세션에서 **만든 것**과, 정리 판단에 필요한데 **다른 데 안 적혀 있는 사실**만 적는다.
"지울지 / `test/`로 옮길지"는 다음 세션이 정하도록 근거만 남긴다.

## 0. 먼저 알아야 할 것

**`test/` 디렉터리 전체가 git 미추적이다** (`git status` → `?? test/`). 이 세션에서 만든
`test/vision_target.py`, 고친 `test/cbirrt_p1p2_test.py`, 교시값 `points.json`,
측정 기록 `vision_hover_*.json` 이 **전부 커밋되지 않은 상태**다. 정리 전에 이것부터
결정할 것 — 지우는 것보다 잃어버리는 쪽이 쉽다.

컨테이너가 root 로 도는 탓에 워크스페이스에 쓰인 파일은 root 소유가 됐었다. 지금은
`cbirrt_p1p2_test.own_like_dir()` 가 쓰기 직후 디렉터리 소유자로 맞추고, 기존 파일은
`ammrdev:ammrdev 644` 로 되돌려 놨다.

## 1. 이 세션이 만든 파일

| 파일 | 상태 | 정리 판단에 필요한 사실 |
|---|---|---|
| `test/vision_target.py` | **신규.** 비전 획득 + 좌표/IK 계산 전담. 로봇 명령 없음 | 유지해야 함. `--vision` 이 의존한다. 자가시험 8항목 PASS(인자 없이 실행). `--probe`(게이트 뷰파인더) · `--tilt`(툴 기울기) 도 이 파일에 있다 |
| `test/cbirrt_p1p2_test.py` | **대폭 수정.** `--vision / --goto / --j6-only / --descend / --no-descend / --j6-deg`, `own_like_dir`, `arrived`, `retrace`, `warn_descend_speed` 추가 | 유지. 노드 클래스를 `CBiRRTPickPlace` → `HubPickPlace` 로 올렸다(녹화·역재생을 모듈 것으로 쓰기 위해). 팬텀 박스는 충돌 쌍이 0이라 무해함을 실측 확인(`41 active pairs` 로 동일), 다만 면 울타리에서 `carried_box` 를 제외하는 한 줄이 있다 |
| `docs/vision_hover_test.md` | **신규.** 사용법·방어층·CBiRRT 전용 이유·복귀 지점 | §8 의 obs 관절값은 **선반을 당기기 전** 것이다. 지금 obs 와 다르다 → 갱신 필요 |
| `docs/VISION_CHAIN_VERIFICATION.md` | §6-8(cv2 더미 부채) + §8 표 한 줄 추가 | 나머지는 손대지 않았다 |
| `test/points.json` | `obs` 추가. 이 세션에서 **세 번** 재교시됨 | 마지막 것이 유효(선반·매거진을 로봇 쪽으로 당긴 뒤). 백업 `points_backup_MMDD_HHMM.json` 이 세 개 있다 — 서로 다른 배치의 obs 라 헷갈리기 쉽다 |
| `test/vision_hover_*.json` | 측정 기록 4개 | **앞의 3개는 평균 내면 안 된다.** 매거진이 회차 사이에 22~34 mm 밀렸다(깊이·겉보기 크기 두 채널로 확인). 그 안의 `measured` dx = 10 / 1 / 0 mm 는 서로 다른 장면의 값이다 |

## 2. 이 세션이 만들었지만 **호스트에 없는** 것

아래는 전부 `docker exec ... python3 -` 로 **stdin 파이프로만** 실행했다. 파일로 남아
있지 않으니 다음 세션이 찾아도 없다. 필요하면 다시 만들어야 한다.

| 무엇 | 하던 일 | 지금 대체물 |
|---|---|---|
| `level_wrist.py` | 교시된 obs 로 짧게(≤10°) 이동, 12점 충돌검사 | `--goto obs` 로 대체됨 |
| `goto_obs.py` | hover→obs CBiRRT 복귀 | `--goto` |
| `probe/streak/edgechk` | 게이트 통과율·연속 통과·bbox 여백 진단 | `vision_target.py --probe` 가 흡수(스냅샷 카운터 포함). 다만 **통과율·탈락 사유 집계**는 probe 에 없다 |
| `reach.py / reach2.py / reach3.py` | 목표점의 툴 수직 최대 도달 높이 이분탐색 (씨앗 25개) | **없음.** 배치가 바뀔 때마다 유용하다 — `vision_target.py --reach` 같은 형태로 넣을 가치가 있다 |
| `predict.py / analyze.py / analyze2.py` | 기록 json 들에서 P_base 산포·bbox·자세 비교 | 없음 |

## 3. 이전 세션이 남긴 것 (내가 만든 것 아님)

| 파일 | 사실 |
|---|---|
| `~/dobot_ws/check_descend.py` | **저장소 밖, git 미추적.** 파일 첫 줄이 `"""Throwaway: ... Delete this file once the descend is trusted."""` 인데, `docs/real_robot_p1p2_test.md` §5.5 가 **절대경로로 참조**한다(토크 임계값 재측정 절차). 즉 문서화된 절차가 미추적 임시파일에 의존한다 → `test/` 로 옮기고 문서 경로를 고치거나, 문서에서 참조를 끊거나 둘 중 하나. 내부 `SPEED = 0.003` 상수를 고쳐야 다른 속도로 잴 수 있다 |
| `~/dobot_ws/` 의 `fit_convention.py`, `solve_convention.py`, `verify_convention.py`, `capture_frame.py`, `view_d405.py` | 전부 미추적 일회용. 관절 부호 규약 조사(문서 §6)와 카메라 확인에 쓰인 것들. 그 조사 결과는 `real_robot_joint_convention.md` 에 이미 남아 있으므로 파일 자체는 없어도 된다 |
| `~/dobot_ws/*.log`, `frame.jpg` | bringup/cam/joint/view 로그. 진단 흔적 |
| `/tmp/runner_cam.log`, `/tmp/runner_*.log` | 러너 로그. `/tmp/runner_cam_frozen_0811_1419.log` 는 **얼어붙은 러너의 증거**로 내가 옆으로 밀어둔 것 (USB 리셋 진단에 썼다) |

## 4. 코드에 남은 미해결 항목

- **`HOVER_CLEARANCE_M` 이 0.10 이다.** 사용자가 0.08 로 바꿔둔 것을 내 `scp` 푸시가
  덮어썼다(로컬 사본을 올리는 방식이었다). 어느 값이 맞는지 확인 필요. 조가 플랜지
  아래로 70 mm 내려오므로 0.10 → 조 끝 30 mm, 0.08 → 10 mm.
- **매거진 yaw θ = −4.03°** (실측: 정렬된 J6 +85.75 vs hover 도착 J6 +89.78).
  지금은 `--j6-deg 86` 으로 매번 주고 있고, `MAGAZINE_YAW_DEG` 상수로는 **안 들어갔다.**
  넣을 때 안쪽 벡터·조 오프셋·J6 를 **한꺼번에** 회전시켜야 한다(J6 만 맞추면
  안쪽 118 mm 가 도당 2 mm 씩 틀어진다).
- **`VISION_WORK_BOX` 가 잠정값**이다 (`x -0.40~0.05, y -0.70~-0.30, z 0.40~0.65`).
  선반 전체를 덮으라고 넉넉히 잡았다. 배치가 굳으면 실측 station 크기로 좁힐 것.
- **여러 매거진 중 선택 규칙이 없다.** `vision_bridge/gate.py` 는 통과한 검출 중
  `score` 최대를 고른다("매거진이 한 대씩 오는 전제"라고 주석에 적혀 있다). 선반에
  여러 개가 보이면 매 회차 어느 것이 뽑힐지 사실상 임의다. gate.py 는 수정 금지이므로
  선택은 `vision_target` 쪽에서 `detections` 를 직접 보고 하면 된다.
- **`main.py` 에 `CR7_REAL_ROBOT=1` 이 없다.** 실물에서 그대로 돌리면 J1·J5·J6 가
  거울 반전된다. 선언 대신 **기동 시 확인**(모델 FK vs 로봇 `tool_vector`, 실측 2.5 mm)
  으로 막자고 제안했으나 아직 안 넣었다.

## 5. 실물에서만 드러난 것 (다음 세션이 알아야 할)

- **`execute_path` 는 `_wait_settled` 가 실패해도 `True` 를 돌려준다.** 시뮬에서는
  맞는 설계지만 실물에서는 **보호정지가 성공으로 보인다.** 실제로 한 사이클이
  `mode 11` 로 멈췄는데 json 까지 저장되고 정상 종료했다. `arrived()` 를 넣어 막았다.
- **`replay_reverse` 는 뒤집은 경로의 첫 점을 `time_from_start = 0` 으로 보낸다.**
  정방향 직후에 부르는 시뮬 시퀀스에서는 무해하지만, 사이에 하강·상승이 끼면 몇 도가
  벌어지고 그게 **0초 안에 가라는 명령**이 되어 추종 오차 → 충돌 트립을 부른다.
  해법 둘: 상승을 `dropped` 만큼만 해서 간극을 0 으로 만들기(`shelf_pick_place.py`
  방식), 그리고 뒤집기 전에 현재 자세를 붙이기. 둘 다 적용했다.
- **D405 가 하루에 세 번 죽었다** — 얼어붙음 2회(프로세스는 살아 있는데 seq 정지),
  프레임 0 1회. 커널 로그에 `Disable of device-initiated U1/U2 failed` → `reset
  SuperSpeed ... using tegra-xusb`. `power/control = auto`(자동 서스펜드)가 유력한
  원인이고, 물리적 재연결로만 완전히 복구됐다. 측정 중 재발하면 그 회차가 날아간다.
- **hand-eye 캘리브의 `realsense2_camera` 노드가 떠 있으면** 러너가
  `Device or resource busy` 로 죽는다. 캘리브 후 반드시 내릴 것.
