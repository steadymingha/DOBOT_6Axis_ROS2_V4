# 01. 전체 실행 흐름

## 프로세스 구성

```
[관제 MCS]  ── TCP :9100, 15바이트 바이너리 프레임 ──►  [comms/mcs_bridge.py]
 (개발 중엔 comms/mcs_server.py 스텁)                        │ ROS2 토픽으로 변환
                                                             ▼
                                    /mcs/command (START, JSON)   /mcs/stop (즉시)
                                                             │
                                                             ▼
[비전 노드]                ──/vision/device_pose──►  [main.py  디스패처]
 sim: tag_vision_node.py       /vision/shelf_pose        │ REGISTRY로 라우팅
 real: vision_hover_node.py    /vision/pocket_state(sim) │ (in-process 함수 호출)
 (launch profile 인자가 선택)
        ▲                                                ▼
        │ /vision/capture (촬영 지시)        [sequences/shelf_pick_place.py]
        └────────────────────────────────  [sequences/wirebonder_pick_place.py]
                                                             │ cr7_pnp 프리미티브 조합
                                                             ▼
                                              [cr7_pnp.HubPickPlace 노드]
                                        pinocchio IK/충돌 + CBiRRT/RRT/서보
                                                             │ FollowJointTrajectory
                                                             ▼
                              [Gazebo Classic + ros2_control]  또는  [실물 팔 브리지]
                              /joint_states, /ATTACHLINK·/DETACHLINK, TF
```

프로세스는 실제로 3~4개:
1. **시뮬레이터** (`./run_mpo700_cr7.sh`) 또는 실물 (realsense 드라이버 + 팔 브리지)
2. **arm 측** (`ros2 launch launch/arm.launch.py [profile:=real]`) = 비전 노드 + main.py.
   sim은 tag_vision_node + `/usr/bin/python3`; real은 vision_hover_node + 정적 TF
   (`odom≡base_link`) + `.venv` python — profile 인자 하나가 전부 교체
3. **MCS 브리지** (`comms/mcs_bridge.py`) — 실전에서는 통신 담당 팀 노드로 교체
4. (개발) **MCS 서버 스텁** (`comms/mcs_server.py`) — 관제 대신 명령을 타이핑

시퀀스는 별도 프로세스가 아니라 main.py가 **라이브러리로 import해서 같은 프로세스에서
블로킹 호출**함. 노드는 `HubPickPlace` 하나, executor는 MultiThreaded + 별도 spin 스레드.

## 미션 FSM (main.py)

dict-router 방식의 단순 블로킹 FSM. `run_mission()`:

```
IDLE ─(location id 수신)─► [충돌월드 갱신] ─► LOCATE ─► PICK/PLACE ─► REPORT ─► IDLE
```

1. **REGISTRY 조회**: `'wb1'/'wb2'/'wb3'` → wirebonder 이송 1/2/3, `'shelf'` → 선반.
   MCS TargetID → location id 매핑은 `comms/mcs_protocol.TARGET_LOCATION`.
2. **`refresh_collision_world(node)`** — 모든 world-고정 팬텀(선반 재고, 장비 본체)을
   **미션 시작마다 라이브 TF 기준으로 재배치**. AGV가 다른 위치로 이동했으면 팬텀이
   구좌표에 떠서 1 m 밖 물체와 가짜 충돌을 냄 (07_gotchas 참고). 새 스테이션 팬텀을
   추가하면 이 함수에 한 줄 추가하는 게 규약.
3. **LOCATE** — 타깃 프레임 확보. 팔이 촬영 자세로 가서 읽고 허브로 복귀:
   - wirebonder: `wb.capture_device()` — AprilTag(ID 0)로 장비 포즈 갱신
   - shelf(sim): `shelf.locate_shelf()` — 단(tier) ArUco로 `node.shelf_pose` 갱신 + 충돌모델 재배치
   - shelf(**real**): `main.locate_box()` — obs 자세 → `/vision/capture` → AI 매거진 검출
     박스 중심 15샘플 중앙값 → `node.vision_box`. **실물 선반의 목표 박스는 비전이 결정**;
     레이아웃(`box_xs`)은 이웃 팬텀 + 비전 불량 시 조그 폴백. ArUco는 나중 정밀화용(3b)
   - 실패 시 미션 중단 (기본 포즈로 대충 진행하지 않음)
4. **PICK/PLACE** — 해당 이송 실행. shelf는 `node.box_idx`를 걸어가며 트리거 1회 = 박스 1개.
5. **REPORT** — 현재는 콘솔 프린트만. MCS 보고 채널은 미구현 seam
   (`node.last_error` / `last_error_detail`에 에러코드는 채워짐).

**STOP**: `/mcs/stop`은 파라미터 없는 별도 채널 — 수신 즉시 `node.abort=True`.
단, 체크포인트(LOCATE 전 / PICK-PLACE 전)에서만 확인하는 **coarse abort**.
모션 중 즉시 정지가 필요해지면 시퀀스 내부에 훅을 추가해야 함.
새 START 명령이 오면 abort 플래그는 자동 해제.

## 시퀀스 공통 패턴 (반드시 이해할 것)

세 가지 안전 패턴이 모든 시퀀스의 뼈대다:

1. **Hub-and-spoke**: 모든 모션이 tool-down **허브 자세**(공유 `HUB_TCP` = (0.33, 0, 0.32))를
   경유함. 선반→포켓을 직행하면 파지/적재 자세가 다른 팔꿈치/손목 브랜치에 떨어져
   CBiRRT가 5 rad 재구성을 못 뚫고 스톨함 (`docs/CARRY_BRANCH_STALL.md`).
2. **Preflight (무동작 사전검증)**: 스포크 전 구간을 박스-부착 충돌모델로 **팔을 움직이지 않고**
   먼저 풀어봄. 불가능하면 출발 전에 abort — 허공에서 멈춘 채 실패하는 일이 없음.
   (`preflight_linear`, `plan_spoke`, wirebonder의 `preflight_transfer`,
   회귀 체크는 `tools/preflight_check.py`)
3. **Reverse-replay (역재생 복귀)**: 전진 경로의 조인트 웨이포인트를 기록(`capture`)했다가
   그대로 역재생(`replay_reverse`)해서 허브로 돌아옴. 방금 실행한 경로는 역방향으로도
   실행 가능하므로 복귀는 IK 실패가 없음. 단 **J6 오프셋을 준 변형 복귀는 동일 경로가
   아니므로 preflight로 별도 검증**해야 함 (07 참고).

## 비전 캡처 흐름 (LOCATE 내부)

```
시퀀스: 팔을 태그 조준 자세로 이동 (자동 조준: aim_pose_at_tag / CAPTURE_FLANGE)
   → /vision/capture 발행 (0=리셋, 1=뷰 확보; 2뷰 캡처)
   → tag_vision_node가 검출 → /vision/{device,shelf}_pose 발행
   → 시퀀스가 n샘플 수집, 산포/타당성 검사 (spread ↑ → CAPTURE_SPREAD_HIGH 등)
   → node.shelf_pose / DEVICES[dev] 갱신 + 충돌 팬텀 재배치 → 허브 복귀
```

이후 모든 박스 좌표·충돌 보드는 이 태그 포즈에 **상대적으로 합성**되므로, AGV 재주차나
선반 이동에 코드 수정이 필요 없음. `--no-vision`이면 스폰 기본값(`SHELF_WORLD_POSE`,
`DEVICES` 하드코딩)을 사용 — 정밀 주차 전제.

## 실행 방법 요약

```bash
# 1) 시뮬 기동
cd ~/dobot_ws && ./kill_sim.sh && ./run_mpo700_cr7.sh

# 2) 통합 테스트 (bridge + vision + dispatcher)
./run_test.sh
# 별도 터미널에서 관제 스텁:
/usr/bin/python3 src/DOBOT_6Axis_ROS2_V4/comms/mcs_server.py
#   프롬프트에 "IN START"(선반) / "A START"(wirebonder 이송1) 입력

# 시퀀스 단독 실행 (디스패처 없이):
source /opt/ros/humble/setup.bash && source ~/dobot_ws/install/setup.bash
cd ~/dobot_ws/src/DOBOT_6Axis_ROS2_V4
python3 vision/tag_vision_node.py                     # 터미널 A
/usr/bin/python3 sequences/shelf_pick_place.py        # 터미널 B (--no-vision 가능)
```

```bash
# 실물 (Jetson 호스트): bringup + dobot_joint 기동 후
~/dobot_ws/run_real.sh          # 브리지 + arm.launch.py profile:=real (컨테이너 안)
```
실물 절차 상세는 `docs/manual.md` 5장 (사전 조건 · preflight · 실패 로그 표),
설계 근거는 `docs/real_robot_pipeline_plan.md`.
