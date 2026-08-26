# 06. 실행 환경 · 스크립트 · 도구 · 시뮬레이션 구성

## 파이썬 환경 — 가장 먼저 알아야 할 것 (머신마다 반대)

| 머신 | 인터프리터 | 이유 |
|------|-----------|------|
| **시뮬 PC** | `/usr/bin/python3` | apt ros-humble-pinocchio + 시스템 cv2 4.5.4. `.venv`(uv, numpy 2.x)는 ROS pinocchio를 **segfault** |
| **실물 Jetson** (컨테이너 `ros2_dobot`) | `/root/dobot_ws/.venv/bin/python3` | pinocchio가 venv에만 있음. `test/run.sh`·`tools/run.sh`가 자동으로 이걸 씀 |

`launch/arm.launch.py`는 profile 인자에 따라 둘을 자동 선택 (sim=`PY`, real=`PY_REAL`).
어느 문서든 python 경로 지시를 보면 **어느 머신 기준인지 먼저 확인**할 것.
- 예외: `jog.sh`/`teleop_agv.sh`는 `uv run`을 씀 (pinocchio를 안 타거나 별도 검증됨).
- conda가 활성화돼 있으면 xacro가 죽어 로봇이 안 뜸 — `run_mpo700_cr7.sh`가
  자동으로 PATH에서 제거해 줌 (직접 띄울 땐 `conda deactivate`).

## 환경 변수 / DDS 설정 (전 터미널 공통, ~/.bashrc에도 있음)

```bash
export ROS_LOCALHOST_ONLY=1                                    # 단일 머신 셀
export FASTRTPS_DEFAULT_PROFILES_FILE=~/dobot_ws/fastdds_localhost.xml  # 참가자 64슬롯 프로브
export DOBOT_TYPE=cr7
export GAZEBO_MODEL_PATH=$GAZEBO_MODEL_PATH:~/dobot_ws/src/blender
```
이 둘(LOCALHOST_ONLY + 프로파일)이 안 맞으면 "goal send timed out" 지옥이 열림
(07 참고). MCS는 순수 TCP라 LOCALHOST_ONLY와 무관.

## 워크스페이스 스크립트 (`~/dobot_ws/`)

| 스크립트 | 역할 |
|----------|------|
| `run_mpo700_cr7.sh` | ★ sim 기동: Gazebo+컨트롤러 → MoveIt/RViz → d405 뷰어. conda 제거, DDS 설정 포함 |
| `run_mpo700_cr10.sh` | CR10 버전 (동일 구조) |
| `run_simulation.sh` | 구버전 팔-단독 sim (AGV 없음) |
| `run_test.sh` | 통합 테스트(sim): mcs_bridge + arm.launch.py. 관제 스텁은 별도 터미널에서 |
| `run_real.sh` | **실물 등가물** (Jetson): 컨테이너 안에서 mcs_bridge + arm.launch.py profile:=real. `--host/--port`로 관제 지정 |
| `kill_sim.sh` | ★ 시뮬/암/비전/브리지 전부 정리. **재기동 전 반드시 실행** (07 참고) |
| `teleop_agv.sh` | AGV 키보드 주행 (터미널 포커스 유지 필요) |
| `jog.sh` | TCP 조그 (`ARM_TYPE` 환경변수로 모델 선택 — sim과 일치시킬 것) |

## `tools/` — 검증·진단·오프라인 분석

| 도구 | 역할 |
|------|------|
| `preflight_check.py` | ★ **회귀 체크**: 모든 wirebonder 이송의 preflight가 무동작으로 통과해야 함. 웨이포인트/기하 상수/포즈 파이프라인 수정 후 실행 (exit 0 = 전부 PASS) |
| `run.sh` (실물) | tools/*.py·main.py를 컨테이너+venv+`DOBOT_ENV=real`로 실행하는 래퍼. `tools/run.sh check_real_robot.py` 식 |
| `teach_env.py` (실물) | 워크스페이스 교시: `--teach-shelf`(두 점+줄자값 → real.json 선반 블록), `--teach-surface z-`(테이블/벽 → real_surfaces.json), `--set-surface`, `--show`. 팔은 사람이 조그, 도구는 안 움직임 |
| `check_real_robot.py` (실물) | 로봇 연결·모드·플랜지 pose 확인 (ROS 불필요, 움직임 없음) |
| `jog_real.py` (실물) | 컨트롤러 MoveJog 키보드/게임패드 조그 (`--joint`), ClearError 포함 |
| `world_watchdog.py` | ★ sim ground-truth 감시: 시퀀스가 못 보는 물리 사고(박스 낙하 FLOOR/전도 TIPPED/밀림 MOVED, 로봇 TILT)를 타임스탬프로 프린트. **테스트 돌릴 때 항상 옆에 띄워둘 것** |
| `diag_seq_dryrun.py` | 시퀀스 드라이런 진단 |
| `diag_camera_geometry.py` | 카메라 프레임 체인 검증 |
| `jog_tcp.py` / `jog_joint.py` | TCP/조인트 조그 (촬영 자세 찾기 등) |
| `teleop_agv.py` | AGV 텔레옵 노드 (teleop_agv.sh가 래핑) |
| `spawn_device_markers.py` | 장비 매거진 슬롯에 반투명 마커 스폰 (`--gap/--up/--delete`), 슬롯 월드좌표 프린트 |
| `reachability_map.py` | 오프라인 도달성 맵 빌더 (sim 불필요) → `reachability_out/` |
| `deploy_optimizer.py` | 배치(설치) 방향 최적화: 선반↔포켓 왕복이 5 rad 재구성이 되지 않는 팔 장착각/접근 조합 탐색. hub-and-spoke 결정의 정량 근거 |

## 시뮬레이션 구성

- **월드**: `dobot_gazebo/worlds/cr.world` — 선반 + wirebonder + AGV 스폰.
- **로봇**: `cra_description/urdf/cr7_on_mpo700.urdf.xacro` (팔+큐브+AGV+그리퍼+D405 플러그인).
  D405 gazebo 플러그인이 실물과 **같은 토픽명**으로 발행 (depth는 remap).
- **모델**: `src/blender/` — shelf, wirebonder(+충돌 STL, AprilTag 텍스처), box, aruco_box,
  gripper_long(패드 STL — 조 기하 측정의 원본), post_wb.
- **파지**: `src/IFRA_LinkAttacher` — `/ATTACHLINK` `/DETACHLINK`. **per-pair 패치 적용됨**
  (원본은 전역 1회 attach 버그). 재빌드 시 Gazebo 재시작 필요.
- **AGV**: `src/neo_simulation2` (MPO-700).
- **launch 체인**: `dobot_gazebo/launch/gazebo_mpo700_cr7.launch.py`(Gazebo+컨트롤러:
  joint_state_broadcaster, cr7_group_controller, gripper_controller) →
  `dobot_moveit/launch/moveit_gazebo.launch.py`(move_group+RViz, 시각화용).

## 빌드

표준 colcon 워크스페이스:
```bash
cd ~/dobot_ws && colcon build --symlink-install   # 특정 패키지: --packages-select ...
source install/local_setup.bash
```
시퀀스/비전/디스패처는 **빌드 대상이 아니다** — 소스에서 직접 실행하는 스크립트.
빌드가 필요한 건 C++/모델 패키지(IFRA_LinkAttacher, description, gazebo 등)뿐.

## 실물 관련 (요약 — 상세는 `docs/manual.md` + `docs/real_robot_pipeline_plan.md`)

- 실물은 **Jetson**의 `~/dobot_ws`, 컨테이너 `ros2_dobot` 안에서 실행. 팔 브리지:
  `dobot_bringup_ros2.launch.py` + `dobot_joint.launch.py` (sim 컨트롤러와 **같은 액션
  이름** 서빙 — 시뮬/실물 전환점이 액션 이름 하나라는 구조는 그대로).
- 충돌 상수/표면은 `cr7_pnp/env/real.json`·`real_surfaces.json` (`DOBOT_ENV=real`),
  Jetson엔 `neo_simulation2`가 없어 **팔 단독 충돌모델 + 교시 표면**이 AGV 몸체를 대신.
- 기동: `~/dobot_ws/run_real.sh` (manual.md 5장). 무동작 검증:
  `tools/run.sh main.py --profile real --preflight`.
- sim과 실물 카메라를 동시에 띄우지 말 것 (같은 토픽) — 시뮬 PC에서의 주의.
