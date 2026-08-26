# 05. MCS(관제) 통신 명세 — `comms/`

```
[관제 MCS 서버] ──TCP :9100──► [mcs_bridge.py] ──ROS 토픽──► [main.py]
 (스텁: mcs_server.py)           (통신팀 노드로 교체 예정)
```

```
comms/
├── mcs_protocol.py   ★ 와이어 프로토콜 단일 소스 (프레임/enum) — 어디서든 이걸 import
├── mcs_bridge.py     TCP 클라이언트 → /mcs/command, /mcs/stop (임시 브리지)
└── mcs_server.py     관제 스텁: 타이핑한 명령을 프레임으로 브로드캐스트
```

## 프로토콜 (`mcs_protocol.py`) — 관제 스펙 그대로

**15바이트 고정 프레임, 리틀엔디언, 패딩 없음** (`struct '<BfffBB'`).
엔디언은 관제 측과 최종 확인 필요 (코드 주석에도 CONFIRM 표시).

| 필드 | 타입 | 값 |
|------|------|----|
| TargetID | uint8 | IN=0(선반), OUT=1(**시퀀스 미구현**), A=2, B=3, C=4 |
| TargetRelPosX/Y/Z | float×3 | ArUco 상대 좌표 — **현재 팔 쪽에서 미사용** |
| Command | uint8 | START=0, STOP=1, PAUSE=2, RESUME=3 |
| Gripper | uint8 | OPEN=0, CLOSE=1 — **현재 미사용** |

- `TARGET_LOCATION`: TargetID → main.py REGISTRY id (A→wb1, B→wb2, C→wb3, IN→shelf).
- `take_frames(buf)`: TCP는 메시지 경계가 없으므로 프레임 조립은 반드시 이걸로.
- `pack`/`unpack` + `--selftest`(`python3 comms/mcs_protocol.py`) 있음.

### ErrorCode (팔 → MCS 보고용, 정의만 완료)
OK=0, TAG_NOT_DETECTED=1, CAPTURE_SPREAD_HIGH=2, CAPTURE_IMPLAUSIBLE=3,
UNREACHABLE=4, TF_UNAVAILABLE=5, PLAN_FAILED=6, COLLISION=7, EXEC_FAILED=8,
ATTACH_FAILED=9, GRIPPER_FAULT=10, INIT_FAILED=11, NO_POCKET=12.

시퀀스와 `main.locate_box()`가 실패 시 `node.last_error`(코드) + `node.last_error_detail`
(문자열)을 채우고, `[REPORT]` 프린트에 코드명이 같이 출력됨 (2026-08-19).
**MCS로 되돌려 보내는 채널은 미구현** — 그 프린트 자리가 seam.

## ROS 측 채널 (bridge → main.py)

| 토픽 | 타입 | 내용 |
|------|------|------|
| `/mcs/command` | std_msgs/String | START만. 스펙 필드 그대로의 **JSON 1건 원자 전달** (필드가 반쯤 적용되는 일 방지). 통신팀 커스텀 msg가 준비되면 이 타입과 필드 읽기만 교체 |
| `/mcs/stop` | std_msgs/String | STOP/PAUSE/RESUME → 즉시 abort 플래그 (파라미터 없는 별도 채널이라 명령 데이터를 기다리지 않음) |

현재 PAUSE/RESUME도 전부 STOP 취급(coarse abort).

## 브리지/서버 스텁

- `mcs_bridge.py --host 127.0.0.1 --port 9100`: 접속 재시도 루프 내장 (기동 순서 무관).
  프레임 수신 → route → 토픽 발행. `--selftest` 있음.
- `mcs_server.py --port 9100`: 프롬프트에 `<target> <command> [gripper] [x y z]`.
  예: `A START`, `IN START`, `A STOP`, `A START OPEN 0.1 0.2 0.3`. `--selftest` 있음.

## 남은 일 (통신 관점)

1. 통신팀의 실제 ROS2 클라이언트/디스패처 노드로 `mcs_bridge.py` 교체
   (main.py 쪽은 `/mcs/command` JSON 파싱 부분만 커스텀 msg로 바꾸면 됨).
2. REPORT(결과/에러코드) 채널 정의 + 배선.
3. TargetID `OUT` 시퀀스 구현 (베이스 → 선반 되돌리기로 추정, 스펙 확인 필요).
4. RelPos/Gripper 필드 소비 여부를 관제와 합의.
5. 프레임 엔디언 최종 확인.
