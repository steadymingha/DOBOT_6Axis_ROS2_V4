# 00. 프로젝트 개요 (인수인계)

> 이 handover 문서 세트는 2026-07-20 작성, 2026-08-19 실물 파이프라인 반영 갱신. 문서화 규약: **계약(무엇을 하고 무엇을
> 반환하는가, 안전/튜닝 경고)은 코드 docstring에, 설계 근거·사고 이력은 이 문서들의
> "설계 노트" 섹션에** 있음. 동작 계약이 어긋나 보이면 코드 docstring이 우선.

## 문서 인덱스

| 문서 | 내용 |
|------|------|
| [00_overview.md](00_overview.md) | 이 문서 — 목적, 구성, 저장소 맵, 현재 상태 |
| [01_flow.md](01_flow.md) | 전체 실행 흐름: MCS 명령 → 디스패처 → 시퀀스 → 모션 |
| [02_cr7_pnp.md](02_cr7_pnp.md) | 모션 라이브러리 `cr7_pnp/` 명세 (IK/충돌/플래너/노드) |
| [03_sequences.md](03_sequences.md) | 시퀀스 명세: shelf / wirebonder / main.py 디스패처 |
| [04_vision.md](04_vision.md) | 비전 명세: AprilTag/ArUco 포즈, 포켓 점유 판정 |
| [05_comms.md](05_comms.md) | MCS(관제) 통신 명세: 프로토콜/브리지/서버 스텁 |
| [06_env_tools.md](06_env_tools.md) | 실행 환경, 셸 스크립트, tools/, 시뮬레이션 구성 |
| [07_gotchas.md](07_gotchas.md) | **함정 모음 — 새로 맡으면 이것부터 읽을 것** |

기존 심층 문서(`docs/` 바로 아래)는 개별 이슈의 원인 분석 기록. 대표적으로:
`ARCHITECTURE.md`(라이브러리 상세, 영문), `real_robot_transition.md`(실물 전환 가이드, 진행 중),
`CARRY_BRANCH_STALL.md`(hub-and-spoke가 생긴 이유), `PICK_PLACE_TROUBLESHOOTING.md`.

## 무엇을 하는 시스템인가

**AGV 위에 얹힌 DOBOT CR7 6축 팔**이, 관제(MCS)의 이송 명령을 받아
매거진 박스(81×236×140 mm)를 옮기는 셀. 미션은 두 종류:

1. **shelf**: 선반 1단의 박스 4개를 AGV 베이스의 포켓 4칸으로 옮겨 싣기
2. **wirebonder**: 베이스 포켓 ↔ 와이어본더 장비 매거진 슬롯(A/B/C/D) 간 이송 3종

위치 인식은 눈-손(eye-in-hand) **D405 카메라 + AprilTag/ArUco**로 함. AGV가 대충
주차해도 태그를 읽어 상대 좌표로 동작하므로 정밀 주차가 필요 없음.

## 기술 스택 — 핵심 결정 사항

- **런타임에 MoveIt을 쓰지 않음.** IK/충돌검사/플래닝 전부 in-process
  **pinocchio**(+coal)  (`cr7_pnp/model.py`). MoveIt/RViz는 시각화·디버깅용으로만 띄움.
  결정 배경: `docs/moveit_vs_tcpip_decision.md`.
- 플래너는 자작: **CBiRRT**(기울기 구속 운반) + 자유 **RRT**(이동) + **직선 데카르트 서보**(미세 접근).
- 시뮬레이터는 **Gazebo Classic**, 파지는 물리 대신 **IFRA LinkAttacher**(링크 부착)로 처리.
- sim과 실물은 **카메라 토픽명이 통일**(`/camera/d405/...`)되어 있어, 실물 전환 =
  Gazebo 대신 realsense 드라이버 + 실물 팔 브리지를 띄우는 것 (`docs/real_robot_transition.md`).
- 파이썬은 **환경마다 다름 — 06 문서 필독.** 시뮬 PC는 시스템 python(`/usr/bin/python3`,
  apt pinocchio + cv2 4.5.4; `.venv`는 numpy 2.x가 ROS pinocchio를 segfault). 실물
  Jetson 컨테이너(`ros2_dobot`)는 **반대로 `.venv` python이 정답** — `test/run.sh`·
  `tools/run.sh`가 그걸 씀. `launch/arm.launch.py`는 profile 인자로 둘을 자동 선택.

## 저장소 맵

```
~/dobot_ws/
├── run_mpo700_cr7.sh        ← sim 기동 (Gazebo+컨트롤러 → MoveIt/RViz → d405 뷰어)
├── run_test.sh              ← 로컬 통합 테스트 (bridge + arm.launch.py)
├── kill_sim.sh              ← 시뮬/노드 정리 (반드시 이걸로 죽일 것)
├── teleop_agv.sh / jog.sh   ← AGV 키보드 주행 / TCP 조그
├── fastdds_localhost.xml    ← DDS localhost 광역 discovery 프로파일 (필수, 07 참고)
└── src/
    ├── DOBOT_6Axis_ROS2_V4/        ← ★ 메인 패키지 (아래)
    ├── blender/                    ← Blender 제작 모델 (shelf, wirebonder, box, gripper_long …)
    ├── IFRA_LinkAttacher/          ← 파지용 링크 부착 플러그인 (per-pair 패치됨)
    ├── neo_simulation2/            ← MPO-700 AGV 모델/설정
    ├── realsense-ros/              ← D405 드라이버 (실물용)
    └── gazebo_ros2_control_patched/

src/DOBOT_6Axis_ROS2_V4/
├── main.py                  ← ★ 미션 디스패처 (MCS 명령 → 시퀀스 라우팅)
├── cr7_pnp/                 ← ★ 모션 라이브러리 (02 문서)
├── sequences/               ← ★ shelf / wirebonder 시퀀스 (03 문서)
├── vision/                  ← ★ 태그·포켓 비전 (04 문서)
├── comms/                   ← ★ MCS 프로토콜/브리지/서버 스텁 (05 문서)
├── launch/arm.launch.py     ← vision + dispatcher 통합 기동
├── tools/                   ← 프리플라이트/감시/조그/최적화 (06 문서)
├── debug/                   ← view_d405.py 등 뷰어
├── cra_description/         ← URDF/xacro (cr7_robot, cr7_on_mpo700)
├── dobot_gazebo/            ← Gazebo 런치 + cr.world
├── dobot_moveit/, cr7_moveit/ ← MoveIt 설정 (시각화·실물 브리지용)
├── dobot_bringup_v4/, dobot_msgs_v4/ ← Dobot 벤더 제공 실물 브리지/메시지
└── docs/                    ← 이슈별 심층 문서 + 이 handover 세트
```

## 현재 상태 (2026-08-19)

- **sim에서 3개 시퀀스(shelf, wirebonder 이송 1/2/3) 전부 성공** — 커밋 `ee7e7e9` (07-16).
  비전 포함/미포함(`--no-vision`) 모두, MCS 스텁 명령 구동까지 확인.
- **실물 파이프라인 코드 완성 (Jetson `~/dobot_ws`)** — 작업 문서는
  `docs/real_robot_pipeline_plan.md`(왜 이 순서인가) + `docs/manual.md`(운용 절차).
  `--vision --run` 실물 단일 사이클(탐지→hover→접촉정지 하강, 그리퍼 없음)은 검증 완료.
  main.py 경로에 반영된 것: 공용 30004 파서 `cr7_pnp/robot_feed.py`, 접촉정지 모듈
  `cr7_pnp/contact.py`, 실측 상수 `cr7_pnp/env/{sim,real}.json`(`DOBOT_ENV` 선택),
  표면 등록 `cr7_pnp/collision_env.py` + 교시 `tools/teach_env.py`, 프로파일 스위치
  (`main.py --profile sim|real [--preflight] [--gripper]`), 실물 비전
  `vision/vision_hover_node.py`(AI 매거진 검출 → 선반 목표), fail-closed 충돌 모델.
  선반 실측(`--teach-shelf`)까지 완료; **로봇 부재로 실물 미션 통주행은 미검증.**
- 미구현: MCS로의 결과 REPORT 채널(에러코드는 채워짐), RelPos/Gripper 필드 소비,
  TargetID `OUT` 시퀀스, 실물 그리퍼 서버, 시퀀스 하강의 접촉정지 교체(4단계). (05 참고)
