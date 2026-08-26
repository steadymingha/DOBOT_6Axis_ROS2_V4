# 07. 함정 모음 — 인수인계 핵심

전부 실제로 한 번 이상 (일부는 두 번) 밟은 지뢰. 증상 → 원인 → 대처 순.

## 프로세스 / 환경

### 시뮬 종료는 반드시 `./kill_sim.sh`
ad-hoc pkill은 stale gzserver/robot_state_publisher를 남기고, 이게 옛 스폰 충돌체와
**옛 DDS 참가자**를 유지함. 특히 옛 `ROS_LOCALHOST_ONLY` 값으로 뜬 생존 노드 하나가
있으면 새 노드 전부의 discovery가 걸려 "goal send timed out"이 복불복이 됨
(실측: 하루 묵은 octomap_server 하나 죽이니 3/3 성공).
- **주의**: kill_sim.sh는 실물 realsense **드라이버를 고아로 만든다** (launch 래퍼만 죽음).
  고아가 `/camera/d405/*`를 계속 발행해 재기동 후 비전을 오염시킴 — 이 상태가 되면 재부팅이 답.
- pkill 패턴('ros2' 등)이 자기 셸 커맨드라인과 매치되면 자기 자신을 죽임 — 스크립트
  호출을 격리해서 실행할 것.

### python 경로는 머신마다 반대 (06 필독)
**시뮬 PC**: `.venv`(numpy 2.x)가 ROS pinocchio를 segfault → `/usr/bin/python3`.
**실물 Jetson 컨테이너**: pinocchio가 `.venv`에만 있음 → `.venv/bin/python3`
(`test/run.sh`·`tools/run.sh`가 자동). 문서의 python 지시는 어느 머신인지 먼저 확인.

### "goal send timed out (20s)" — 원인이 3층
1. localhost discovery가 참가자 4슬롯만 프로브 → `fastdds_localhost.xml`(64슬롯) 필수
2. 커널 UDP rmem 208KB가 버스트 드롭 → sysctl로 상향 적용됨
3. 옛 환경값의 좀비 참가자 → kill_sim 패턴으로 정리
격리 진단: `ros2 action send_goal`로 goal만 따로 쏘기, `ss -uampe`로 드롭 소켓 찾기.

### sim/실물 카메라 동시 실행 금지
2026-07-11부터 토픽명 통일(`/camera/d405/...`) — 둘 다 켜면 발행자 2개.

### XML 주석에 `--` 금지
xacro/world 주석 안의 이중 하이픈은 파스를 죽여 **로봇이 조용히 안 뜬다**. 주석 수정
후엔 파스 체크 (`xmllint` 또는 xacro 실행). 두 번 밟은 지뢰.

### RViz에 아무것도 안 보이면
Status OK인데 빈 화면 → Fixed Frame을 `base_link`로 먼저 바꿀 것 (기본값 dummy_link).

## 플래닝 / 충돌 모델

### AGV 주행 후 팬텀은 전부 stale
world-고정 팬텀(선반 재고, 장비 본체)은 base_link 기준 **정적 지오메트리**. AGV가
움직이면 1 m 밖 물체와 가짜 충돌 (실측: shelf_stock_t12가 slot-A 적재를 블록).
→ **모든 미션 진입 시 라이브 TF로 재배치** — main.py `refresh_collision_world()`가 그
seam. 새 팬텀 추가 시 거기 등록.

### IK "collision-free=0"이 재실행하면 성공
근방 시딩이 직전 자세 잔재에 낭비된 것 — retries를 200→600으로 올려 해결됨.
600에서도 실패할 때만 "월드가 잘못 놓였나"를 의심할 것. **retries를 되돌리지 말 것.**

### 회피 팬텀은 절대 줄이지 말 것 + 튜닝 상수 변경은 승인 후
마진이 필요하면 **팬텀을 키우는** 방향. 예외적으로 sim `STOCK_SHRINK=-0.006`
(측면 3 mm, 파지 안착 편차용)은 2026-07-15 사용자 승인된 값. 교훈: 튜닝된 상수는
제안하고 **승인을 기다린 뒤** 변경.

### 역재생 복귀는 "동일 경로"에만 보장
기록-역재생은 방금 실행한 그 조인트 경로에만 성립. **J6 오프셋을 준 변형 복귀는 다른
경로**이므로 박스-vs-재고 충돌쌍을 켠 상태로 preflight에 포함해야 함 (선반 스치던
버그의 검증된 수정).

### 배치(설치) 기하 문제는 코드로 못 고침
선반→포켓 5 rad 스톨의 근본 원인은 pick/place **방위각 불일치**. 팔 재장착으로는 해결
안 되고, 포켓을 선반 쪽에 같이 두거나 hub-and-spoke로 우회함.
`tools/deploy_optimizer.py`가 정량화 도구.

## 비전 / 파지

### "태그가 보이는데 검출 안 됨"
= 컬러 구독 wedge (stale frame에서 검출 중; pocket/depth는 정상 흐름이라 더 헷갈림).
미러 구독 + staleness 워치독이 들어가 있음. 노드 실제 로그는
`~/.ros/log/python3_<pid>*.log`.

### 그리퍼 기하는 측정값 기준 — 감으로 만지지 말 것
파지 중심은 툴축 위가 아니라 **고정조 쪽으로 크게(롱 그리퍼 ≈0.14 m) 매달린다** —
값은 Blender 패드 STL 실측에서 유도된 `cr7_pnp/gripper_params.py`의 `GRASP_LATERAL_M`이
유일한 소스. 옛 문서/docstring의 "≈46–48 mm"나 "53 mm"는 **구형 숏 그리퍼** 수치.
과거 xacro 두 벌의 기하가 달랐던 것이 box-eject 버그의 근원. 기하 변경은
`gripper_params.py` 한 곳 + `docs/gripper_change_checklist.md` 절차를 따를 것.

### 적재한 박스가 AGV를 안 따라옴 (sim)
마찰만으로는 안 됨 (질량 상쇄) → `mpo_base_link`에 link-attach로 고정
(`attach_box_to_magazine`). 큐브가 AGV에 붙어 한 덩어리가 됨.

### IFRA LinkAttacher는 패치본
원본은 전역 1회 attach ("Both links have already been attached" 버그) → per-pair로
패치됨. 업스트림으로 갈아엎으면 재발함. 재빌드 시 Gazebo 재시작.

## 코드 수정 시 절차

1. 웨이포인트/기하 상수/포즈 파이프라인 수정 → `tools/preflight_check.py` (exit 0 확인)
2. 시퀀스 테스트 실행 → `tools/world_watchdog.py`를 옆에 띄워 물리 사고 감시
3. xacro/world 주석 수정 → XML 파스 체크
4. "호출되는 곳이 없는" 함수 발견 → 지우기 전에 docstring/설계 문서 확인
   (`move_constrained`처럼 설계상 seam인 경우가 있음)

## 실물 (Jetson) 전용 지뢰 — 2026-08 추가

### `--profile`과 `DOBOT_ENV`가 어긋나면 기동 거부 — 의도된 것
"실물인데 시뮬 충돌 상수로 움직이는" 최악 상태를 막는 보험. `real.json`이
`"measured": false`여도 거부(`--preflight`만 허용). launch/`run_real.sh`/`tools/run.sh`를
쓰면 자동으로 맞음 — 손으로 `python3 main.py --profile real`만 치면 걸림.

### J5 ≈ 0°(손목 특이점)에서 조그 진입 즉시 에러 76
직교(TCP) 조그를 특이점 자세에서 걸면 컨트롤러가 반복해서 알람. ClearError로 풀리지만
자세를 안 바꾸면 재발. **관절 조그(`jog_real.py --joint`)로 J5부터 ±30° 밖으로** 뺀 뒤
TCP 조그. J4가 크게 접혀 있으면(>140°) 한쪽 방향이 막힐 수 있음 — J4 먼저 펴기.

### TF 실패 → 모션 잠금 (fail-closed, 2026-08-17)
`odom→base_link` 정적 TF(launch가 발행)가 없으면 선반 팬텀 배치가 실패하고
`execute_path/trajectory`가 **모든 모션을 거부**함 (`motion locked` 로그). 과거처럼
경고 후 진행하지 않음 — 충돌 모델이 꺼진 채 움직이는 경로를 없앤 것.

### 컨테이너에서 ros2 service call 이 첫 회 무응답
DDS 웜업으로 첫 호출이 타임아웃 나기도 함 — timeout을 15초로 주고 한 번 더.
