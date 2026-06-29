## 시스템 구성 요소 파악

먼저 관여하는 주체들을 정리하면:

- **Camera** → 이미지 수집 + 객체 좌표 추출
- **Robot Controller (Dobot CR7V)** → 좌표 받아서 pick-and-place 실행
- **AMR** → TCP/IP로 상태 송수신
- **MCS (Mission Control System)** → MQTT로 임무 완료 수신

---

## 아키텍처 제안

**NVIDIA Orin을 중앙 허브**로 두고, ROS2 노드들이 각 인터페이스를 담당하는 구조입니다.

```

┌─────────────────────────────────────────────────────┐

│                  NVIDIA Orin                        │

│                                                     │

│  \\\[Camera Node]                                      │

│      │ sensor\\\_msgs/Image                            │

│      ▼                                              │

│  \\\[Vision Node]  ──→ /object\\\_pose (geometry\\\_msgs)   │

│                          │                          │

│                          ▼                          │

│              \\\[BT / FSM Mission Node]  ←── 핵심     │

│               (행동 결정 + 시퀀스 관리)              │

│                  │           │                      │

│                  ▼           ▼                      │

│        \\\[Robot Control    \\\[AMR Bridge Node]          │

│           Node]           TCP/IP ↔ ROS2 변환        │

│        ROS2 action/srv        │                     │

│             │            AMR와 통신                  │

│             ▼                                       │

│        Dobot CR7V                                   │

│                                                     │

│       \\\[MQTT Bridge Node]                            │

│        임무완료 → MCS                               │

└─────────────────────────────────────────────────────┘

```

---

## 각 레이어별 설계 포인트

### 1. Vision Layer
- 카메라 드라이버 노드 (RealSense 등) → raw image publish
- Vision 노드가 subscribe해서 6D pose 추출 후 `/object\\\_pose` publish
- Vision 노드는 ROS2와 무관하게 독립적으로 개발 가능 → **인터페이스만 ROS2 topic**

### 2. Mission Layer (BT vs FSM 선택 문제)
이게 핵심 설계 결정입니다.

| 항목 | BT | FSM |
|---|---|---|
| 적합한 경우 | 조건 분기/재시도 복잡할 때 | 상태 전이가 명확하고 단순할 때 |
| 구현 복잡도 | 높음 (BehaviorTree.CPP 등) | 낮음 |
| 유지보수 | 트리 구조로 직관적 | 상태 많아지면 복잡 |
| POC 적합성 | 과할 수 있음 | 충분할 가능성 높음 |

pick-and-place 고정 시퀀스라면 **FSM이 현실적**입니다. 나중에 BT로 확장 가능한 구조로 잡으면 됩니다.

FSM 상태 예시:
```

IDLE → WAITING\\\_FOR\\\_POSE → MOVING\\\_TO\\\_PICK → PICKING → 

MOVING\\\_TO\\\_PLACE → PLACING → REPORTING → IDLE

```

### 3. Robot Control Layer
- Mission 노드로부터 목표 pose 수신
- Dobot TCP/IP 또는 ROS2 action server로 명령 전달
- 실행 결과(성공/실패/오류)를 Mission 노드로 피드백

### 4. AMR Bridge Layer
- TCP/IP 소켓 ↔ ROS2 topic/service 변환 담당
- AMR 상태 수신 → `/amr\\\_status` publish
- Mission 노드의 명령 → AMR으로 TCP 전송
- 독립 노드로 분리해야 Mission 로직이 깔끔해집니다

### 5. MQTT Bridge Layer
- `rclpy` + `paho-mqtt` 조합이 일반적
- Mission 완료 이벤트를 subscribe해서 MCS로 publish
- 별도 노드로 분리 → MCS 프로토콜 변경에도 Mission 로직 불변

---

## 노드 구성 요약

| 노드 | 역할 | 통신 |
|---|---|---|
| `camera\\\_node` | 이미지 수집 | ROS2 topic |
| `vision\\\_node` | 객체 좌표 추출 | ROS2 topic |
| `mission\\\_node` | BT/FSM 실행, 전체 조율 | ROS2 action/service |
| `robot\\\_control\\\_node` | Dobot 제어 | ROS2 action server + TCP/IP |
| `amr\\\_bridge\\\_node` | AMR 통신 변환 | TCP/IP ↔ ROS2 |
| `mqtt\\\_bridge\\\_node` | MCS 보고 | MQTT |

---

## 추가로 고려할 것

- **에러 핸들링**: 각 노드 실패 시 Mission 노드가 어떻게 대응할지 (재시도? 알림?)
- **타임아웃 관리**: 로봇 응답, AMR 응답 모두 타임아웃 필요
- **상태 로깅**: `ros2 bag` 또는 별도 로그 노드로 디버깅 대비

---
