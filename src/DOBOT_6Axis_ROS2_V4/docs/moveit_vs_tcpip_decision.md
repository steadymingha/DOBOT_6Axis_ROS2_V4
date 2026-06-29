# 모션 제어 아키텍처 결정 정리 — MoveIt vs TCP/IP Wrapper

> 실로봇(Dobot CR7V) 제어를 위해 **MoveIt을 쓸지 / TCP/IP wrapper(dobot_msgs_v4)로 갈지** 고민했던 내용 정리.
> 대상 코드: `hub_pick_place.py`, `cbirrt_pick_place.py`, `constrained_cbirrt.py`

---

## 0. TL;DR (3줄 요약)

1. **"MoveIt vs TCP/IP wrapper"는 사실 양자택일이 아니다.** 현재 계획(planning)은 이미 MoveIt이 아니라 **pinocchio**가 100% 하고 있다.
2. **MoveIt을 써도 `dobot_msgs_v4` 명령에서 해방되지 않는다.** 팔 궤적 dispatch만 한 층 아래로 숨을 뿐, 실제 바닥은 여전히 `ServoJ`이고 그리퍼·lifecycle·안전은 어차피 직접 호출해야 한다.
3. **권고: pinocchio 유지 + 실행은 ServoJ 직접 스트리밍.** MoveIt 도입의 기대효과(명령 해방·TrajOpt 자동 제공)는 실제론 거의 없고, 원래 피했던 무게/불확실성만 되돌아온다.

---

## 1. 현재 코드는 누가 무엇을 하나

| 기능 | 담당 | 비고 |
|---|---|---|
| **IK** | pinocchio (`compute_ik_ordered` → reachability map 자체 솔버) | MoveIt KDL은 map과 불일치해서 **의도적으로 버림** |
| **충돌검사** | pinocchio/coal (`is_state_valid` → 전체 로봇 모델) | MoveIt `/check_state_validity` 오버라이드함 |
| **모션플래닝(스포크)** | pinocchio (`cbirrt.plan`, CBiRRT) | 샘플링 + 제약 매니폴드 투영 |
| **직선 서보** | pinocchio (`cbirrt.linear_path`, 자코비안 DLS) | RRT 아님, 결정론적 |
| **특이점 판별** | pinocchio (`linear_path`의 `sigma_min`) | |
| **궤적 실행** | `/cr7_group_controller/follow_joint_trajectory` 액션 | ros2_control 컨트롤러 (MoveIt config가 띄움) |

**결론: 모션플래닝의 두뇌는 전부 pinocchio.** MoveIt은 이 파이프라인에서 사실상 **우회(bypass)** 되어 있고, 남은 역할은 (1) 궤적 실행 컨트롤러를 띄우는 것, (2) 로봇 모델(URDF/SRDF) 소스 정도다.

> 처음부터 pinocchio로 짠 이유: MoveIt이 무겁고 불확실해서, TCP/IP wrapper로 가볍게 배포하기 위해. 그리고 실로봇 명령으론 시뮬을 못 돌리니까 시뮬 가능한 구조로 짠 것. → **이 논리는 지금도 유효하다.**

---

## 2. 결정적 발견: 다리(bridge)는 이미 있고, 그건 MoveIt이 아니다

`dobot_moveit/action_move_server.py` (78줄)가 실기 연결 다리:

```
hub_pick_place.py (pinocchio 계획)
      │  execute_path → FollowJointTrajectory.Goal
      ▼
action_move_server.py        ← 이미 존재, MoveIt 아님, 78줄
      │  웨이포인트마다 rad→deg 변환 후 ServoJ 서비스 호출
      ▼
dobot_bringup_ros2 (TCP/IP) → 실제 CR7
```

- 이 노드는 `/{DOBOT_TYPE}_group_controller/follow_joint_trajectory` **액션 서버**를 띄운다 → `DOBOT_TYPE=cr7`이면 **당신 `execute_path`가 보내는 바로 그 액션 이름**.
- move_group·planning scene·OMPL **안 씀**. `dobot_bringup` + 이 노드 하나면 끝.

→ 즉 **"TCP/IP 포팅"은 이미 90% 되어 있고, MoveIt과 무관**하다.

---

## 3. "MoveIt 쓰면 dobot 명령 안 써도 된다" → 반은 틀림

MoveIt은 **하드웨어와 직접 말하지 않는다.** 계획·궤적생성 후 **컨트롤러에게 FollowJointTrajectory로 넘길 뿐**이고, 실기에서 그 컨트롤러가 곧 `action_move_server` → `ServoJ`다.

- **팔 모션 명령(ServoJ)**: 손으로 안 써도 됨 ✓ — 하지만 "안 쓰는" 게 아니라 **한 층 아래로 숨은 것**. 바닥은 여전히 ServoJ.
- **모션 외 전부는 MoveIt이 못 준다** → `dobot_msgs_v4`로 직접 호출 필수:
  - `EnableRobot` / `DisableRobot` / `ClearError` / `EmergencyStop` / `ResetRobot`
  - `SpeedFactor` (초기 저속)
  - **그리퍼** (`ToolDO` 또는 RS485 Modbus)
  - 피드백 (`GetPose` / `GetAngle` / `RobotStatus` 토픽)
  - 안전 (`SetCollisionLevel` 등)

**결론: MoveIt을 써도 dobot 명령에서 해방되지 않는다. 오히려 그 위에 한 층 더 얹혀 무거워진다.**

---

## 4. 어떤 모션 명령을 쓰나 — ServoJ가 정답, MovJ/MovL은 오답

| 명령 | 입력 | 성격 | 적합? |
|---|---|---|---|
| **`ServoJ(a~f, param)`** | **관절각 6개(deg)** | 스트리밍 서보 (`t=` 블렌드) | ✅ **정답** |
| `MovJ(mode, a~f)` | mode=True면 관절각 | 블로킹 점대점(PTP) | △ gross 이동 1회용만 |
| `MovL(mode, a~f)` | Cartesian 포즈 | 직선 PTP | ❌ |

**demo.py(MovJ/MovL)를 모델로 삼으면 안 되는 이유:**
1. **MovL은 Cartesian 포즈**를 받음 → Dobot이 *자기 IK*를 다시 풂 → 피노키오로 고른 elbow/wrist 분기가 날아가고, MoveIt KDL을 버린 이유였던 **IK 불일치 재발**.
2. **MovJ/MovL은 호출마다 블로킹** → dense한 경로(CBiRRT 에지 + 3mm 직선서보)에서 **stop-and-go**라 jerky·느림. ServoJ는 고정 주기로 흘려 dense 경로를 매끄럽게 따르도록 설계됨.

> 참고: `action_move_server.py`의 `execution_trajectory()`가 ServoJ 직접 호출의 **레퍼런스 구현**이다 (rad→deg, `param_value=["t=0.2"]`). 단 흠은 `time.sleep(0.18)`로 **타이밍을 버리는 것** → 직접 포팅 시 개선 포인트.

---

## 5. TrajOpt 관련

- **"TrajOpt이 MoveIt에 포함"은 위험한 기대.** MoveIt2(Humble)의 `moveit_planners_trajopt` 플러그인은 **실험적·사실상 방치**. 안정적인 최적화 플래너는 CHOMP/STOMP. 진짜 프로덕션 TrajOpt은 **Tesseract**(MoveIt과 별개 생태계).
- **TrajOpt은 MoveIt을 요구하지 않는다.** 필요한 건 충돌 **거리+그래디언트**인데, 지금 쓰는 **coal이 signed distance 질의를 줄 수 있다.** → MoveIt 없이 pinocchio 위에서 TrajOpt 가능.
- **hub 문제는 플래너가 아니라 *설계*가 푼다.** `HUB_TCP`을 pocket 분기로 잡아서 hub↔shelf, hub↔pocket이 **같은 elbow/wrist family**가 되도록 해둠. shelf→pocket 직접 carry는 아예 계획하지 않음. 따라서 플래너에 남는 건 **"같은 family 안 짧은 스포크"** 하나뿐 → 국소 옵티마이저(TrajOpt)가 잘 푸는 종류.
- **유일한 체크포인트:** 스포크의 `hub_q→goal_q` 직선보간 seed가 선반/cube를 **관통하느냐**. 관통 안 하면 TrajOpt 단독으로 충분(CBiRRT 완전 제거 가능), 관통하면 그 스포크만 CBiRRT seed + TrajOpt smoothing. → **보간 경로 collision-check 한 번으로 확정 가능.**

### 효율 비교 (짧은 동일-family 스포크 기준)
| 축 | CBiRRT(현재) | TrajOpt | 승자 |
|---|---|---|---|
| 계획 시간 | 샘플마다 manifold Newton 투영 | seed에서 SQP 수렴 | TrajOpt |
| 경로 품질/실행 | 울퉁불퉁(smoothing 없음) | 매끈·짧음 | TrajOpt |
| 결정론 | 난수, 매번 다름 | seed 같으면 결정론 | TrajOpt |
| 막힌 경우 강건성 | 확률적 완전 | 국소 → seed 나쁘면 실패 | CBiRRT |

→ **런타임 효율은 TrajOpt 우세.** 단 이건 *실행* 효율이지 *개발* 효율이 아니다(coal에 distance/gradient 붙이는 구현 비용). 현재 CBiRRT가 충분히 빠르면 ROI 낮을 수도 → **계측으로 확정 권장.**

---

## 6. 컨트롤러가 못 주는 것 vs 줄 수 있는 것

### 위임 불가 (당신 코드의 본질, 펌웨어에 없음)
- CBiRRT 스포크 계획 (씬 우회)
- 씬 충돌검사 (선반·cube·AGV·박스 phantom) — Dobot `SetCollisionLevel`은 **토크 기반 반응형**(부딪힌 후 정지)이지 예측형 플래닝 충돌이 아님
- IK 분기 선택 (pocket family 고정)
- 무모션 pre-flight, 역재생, hub 라우팅

### 위임 가능 (네이티브가 동등하거나 더 나음)
- 직선 구간 → `MovL`(+`CheckOddMovL` 특이점) — *단, 두 운동학 모델 발산 주의*
- 그리퍼 → `ToolDO` / RS485
- 조인트 리밋, 에러복구, EStop, 자세 피드백
- (추가) 하드웨어 안전 backstop → `SetCollisionLevel` + `SetSafeWallEnable` + `SetWorkZoneEnable`

> 위임의 비용: 컨트롤러 운동학 ≠ pinocchio 운동학 → MoveIt KDL을 버린 그 발산이 재발. 인계점마다 `GetPose` 검증 필요.

---

## 7. 최종 권고

```
계획:  CBiRRT(전역) — 필요시 TrajOpt smoothing   ← MoveIt 불필요, pinocchio 위에서
충돌:  pinocchio/coal 유지 (distance 질의만 추가)  ← move_group 안 들임
배포:  ServoJ 직접 스트리밍 + 그리퍼(ToolDO/RS485) + lifecycle/안전(dobot_msgs_v4 직접)
```

**이유:**
1. MoveIt은 dobot 명령에서 해방시키지 못함 (lifecycle·그리퍼·안전은 어차피 직접) → 도입 기대효과의 핵심이 실제론 없음.
2. TrajOpt은 MoveIt 없이 pinocchio 위에서 가능, Humble의 MoveIt-TrajOpt 플러그인은 불안정.
3. 충돌 오써링 편의 하나 때문에 원래 피한 무게/불확실성을 되안을 가치 낮음.
4. 원래 설계 철학(가볍게·TCP/IP 배포·시뮬 가능)과 가장 일관됨.

**MoveIt-only가 맞는 경우:** 분기 제어 요구 없고, 제약 carry 없고, 새 프로젝트일 때. → 당신 시퀀스는 정확히 그 반대 조건이라 안 맞음.

---

## 8. 관련 명령 빠른 참조 (dobot_msgs_v4)

- **모션**: `ServoJ`(관절 스트리밍·권장), `MovJ`(관절 PTP), `MovL`(직선)
- **그리퍼**: `ToolDO`(바이너리), RS485 체인 `SetTool485`→`ModbusRTUCreate`→`SetHoldRegs`/`GetHoldRegs`→`ModbusClose`
- **lifecycle**: `EnableRobot`, `DisableRobot`, `ClearError`, `EmergencyStop`, `ResetRobot`, `SpeedFactor`
- **피드백**: `GetPose`, `GetAngle`, 토픽 `ToolVectorActual`/`RobotStatus`
- **안전**: `SetCollisionLevel`, `SetSafeWallEnable`, `SetWorkZoneEnable`, `CheckOddMovL`/`CheckOddMovJ`(특이점 사전판별)
