# 비전 → 로봇 base 변환 사슬 구축 및 검증

D405 카메라가 본 매거진 위치를 로봇 base 좌표로 옮기는 전체 사슬을 만들고, 그 사슬이
옳은지 · 얼마나 정확한지 실측으로 확인한 기록.

- **결론: PASS.** AMR 정차 오차 범위(±2cm / ±3°) 안에서 **vision→base 재현성 p95 1.37 mm**
- 작성 시점 기준 러너 커밋 `864875f`, 컨테이너 `vision_bridge/` 는 아직 untracked
- 이번 작업 범위에서 **로봇에 명령을 보낸 적이 없다.** 30004 실시간 피드백 읽기 전용

---

## 1. 왜 이 구조인가

RealSense D405 는 프로세스 간 공유가 되지 않는다. 카메라를 쓰려는 주체가 셋이었다.

| 주체 | 하는 일 |
|---|---|
| 비전담당자 하네스 `realtime_app_jetson.py` | TensorRT YOLOX 매거진 탐지 (호스트 venv) |
| ArUco 자세추정 | 마커 기반 위치 (원래 ROS2 노드) |
| 로봇제어 컨테이너 | 좌표를 받아 pick-and-place |

셋이 동시에 카메라를 열 수 없으므로, **호스트의 단일 프로세스가 카메라를 독점**하고
결과만 ZMQ 로 내보내는 구조를 택했다.

```
┌─ 호스트 (Ubuntu 20.04, venv_ammr) ─────────────┐
│  D405 ──▶ vision_runner (~/robot_vision)       │
│            · 카메라 단독 소유, 단일 스레드      │
│            · MAGAZINE: YOLOX+ring depth+역투영  │
│            · ARUCO   : PnP+depth 융합           │
│            · 좌표는 카메라 광학 프레임까지만    │
└───────────┬─────────────────────────────────────┘
            │ ZMQ (컨테이너가 --network host 라 127.0.0.1)
            │  5555 PUB 결과+하트비트 / 5556 REP 커맨드 / 5557 미리보기(옵션)
┌───────────▼─ 컨테이너 ros2_dobot (Ubuntu 22.04) ┐
│  vision_bridge                                  │
│    · P_base = T_base_flange @ T_flange_cam @ P_cam │
│    · 채택 게이트, 검증 도구                      │
│    · 30004 read-only (로봇 명령 없음)            │
└─────────────────────────────────────────────────┘
```

**단일 스레드인 이유**: `YOLOXInferencerTRT.infer()` 가 `self.ratio` 를 인스턴스 상태로
보관하고, `pycuda.autoinit` 이 CUDA 컨텍스트를 import 스레드에 묶는다. 추론·ArUco·소켓을
한 루프에서 돌린다.

**비전담당자 코드 수정 금지**가 대전제였다. 러너는 그의 모듈을 import 만 하고 복사하지
않는다. 유일한 예외가 `GrabberEx`(상속 1개).

---

## 2. 만든 것

### 2-1. 호스트: `~/robot_vision` (git 저장소)

| 파일 | 역할 |
|---|---|
| `config.py` | 경로·포트·타임아웃. 하네스 값은 두지 않는다(전부 import) |
| `runner.py` | 진입점. 단일 루프 + IDLE/MAGAZINE/ARUCO 모드 분기 + 소켓 |
| `grabber_ex.py` | `RealSenseGrabber` 상속. intrinsics/depth_scale 노출 + depth_units 정렬 |
| `aruco_core.py` | `aruco_test_gemini.py` 순수 계산부 이식 (ROS 의존 제거) |
| `ipc.py` | ZMQ 소켓 + msgpack 페이로드 스키마 |
| `replay_grabber.py` | npz 재생. 카메라 없이 전체 파이프라인 구동 |
| `tests/test_aruco_equivalence.py` | 테스트 0 — 카메라 불필요 |
| `tests/standalone_aruco.py` | 테스트 1 — 카메라+창. 줄자 대조/정확도 측정 |
| `tests/cmd_client.py` | FSM 대역 디버그 클라이언트 |
| `tests/preview.py` | 미리보기 뷰어 (러너는 창을 띄우지 않는다) |

### 2-2. 컨테이너: `~/dobot_ws/src/DOBOT_6Axis_ROS2_V4/vision_bridge`

| 파일 | 역할 |
|---|---|
| `config.py` | 러너 주소, hand-eye 경로, 게이트 임계, 검증 포락선 |
| `vision_client.py` | ZMQ SUB(CONFLATE)/REQ 클라이언트 |
| `transform.py` | `P_base = tool_to_T(tool) @ X @ P_cam`. 자가시험 포함 |
| `gate.py` | 채택 게이트 + 스냅샷(10프레임 평균) |
| `verify_chain.py` | 검증 진입점 |
| `analysis/` | 사후 분석 스크립트 5개 (축별 분해, 카메라 프레임 환산, what-if) |

`handeye_calib/` 에 `handeye_calib.py` 와 `handeye_result.json` 을 복사해 두었다
(원본은 호스트 `~/realsense-ros/` 에 있어 컨테이너에서 안 보였다).
`RobotFeed`, `tool_to_T`, `STILL_DPS`, `rot_angle` 은 **재구현하지 않고 import 해서 쓴다** —
회전 규약(ZYX intrinsic)이 Pinocchio FK 대조로 검증돼 있으므로 재유도 금지.

---

## 3. 실행 방법

### 러너 (호스트)

```bash
source ~/venv_ammr/bin/activate
cd ~/robot_vision
python runner.py                 # 기본
python runner.py --preview       # + 미리보기 발행(5557)
python runner.py --replay dumps  # npz 재생, 카메라 불필요
```

> **카메라 점유 주의**: 컨테이너의 `realsense2_camera` 노드가 떠 있으면
> `VIDIOC_S_FMT ... Device or resource busy` 로 죽는다. 먼저 내려야 한다.
> ```bash
> docker exec ros2_dobot bash -lc "kill -INT <ros2 launch PID>"
> # 되살릴 때
> docker exec -d ros2_dobot bash -lc \
>   "ros2 launch realsense2_camera rs_launch.py camera_name:=d405 align_depth.enable:=true"
> ```
> 비전담당자 하네스와도 배타 실행이다.

### 검증 (컨테이너)

```bash
docker exec -it ros2_dobot bash -lc \
  'cd /root/dobot_ws/src/DOBOT_6Axis_ROS2_V4/vision_bridge && python3 -u verify_chain.py'
# 기존 표본에 이어붙이려면
  ... python3 -u verify_chain.py --append
```

### 하드웨어 없이 도는 자가시험

```bash
# 호스트
python tests/test_aruco_equivalence.py     # ArUco 이식 등가성, 역투영, median 추종
# 컨테이너
python3 transform.py                       # 단위·규약·정규직교
python3 gate.py                            # 게이트 조건별 차단, 스냅샷 평균
```

### 미리보기 / 관측

```bash
DISPLAY=:0 python tests/preview.py                    # 카메라 화면 + bbox
python tests/cmd_client.py run MAGAZINE 120           # 모드 전환 + 5초 ping + 결과 요약
```

**FSM 전제**: 러너는 마지막 유효 커맨드(`set_mode`/`ping`) 후 10초가 지나면 IDLE 로
강등하고 `degraded_to_idle` 을 1회 발행한다. FSM 이 5초 주기 ping 을 유지해야 한다.

---

## 4. 검증이 재는 것

### 원리

매거진은 고정이고, 카메라는 플랜지에 붙어 팔과 함께 움직인다. 매 표본마다:

```
YOLOX bbox → ring depth(Z) → 역투영(xyz_cam) → T_base_flange @ T_flange_cam → P_base
```

물체가 안 움직였으니 **어느 자세에서 봐도 `P_base` 는 같은 값**이어야 한다. 어긋난 만큼이
사슬 전체의 오차다. 한 단계를 따로 재는 게 아니라 끝에서 끝까지 합산된 값을 본다.
여기 섞이는 것: hand-eye `T_flange_cam`, 로봇 기구학/`tool_to_T`, YOLOX bbox 안정성,
ring depth, 역투영 대표픽셀 가정.

### 판정 규칙

`p95` = 각 표본이 평균점에서 떨어진 거리의 95퍼센타일.

| 조건 | 판정 |
|---|---|
| travel < 기준 **또는** 회전 < 기준 | **INCONCLUSIVE** (시험 무효) |
| p95 < 10 mm | **PASS** |
| 그 외 | **FAIL** |

**INCONCLUSIVE 가 따로 있는 이유**: 팔을 조금만 움직이면 사슬이 틀려도 `P_base` 가 나란히
나온다. 오차가 자세에 비례하는 종류라 자세를 안 바꾸면 드러나지 않는다. 특히 **회전**이
hand-eye 오류를 드러내는 축이다.

### 스냅샷 정의 (검증 = 운용)

> **스냅샷 = 게이트를 연속 통과한 같은 track_id 의 프레임 10장 평균**

검증과 FSM 이 같은 정의를 써야 검증이 운용을 대변한다. 평균은 카메라 프레임에서 이뤄지므로
**그 10장 동안 팔이 정지해 있어야 한다** — 호출부가 정지 판정을 하고, 움직이면
`Gate.reset()` 으로 누적을 버린다.

### 채택 게이트 (최종값)

| 항목 | 값 | 근거 |
|---|---|---|
| `state` | `TRACKING` | LOST 는 depth 재계산이 없다 |
| `stale_frames` | 0 | |
| `valid_pct` | ≥ 70 | 실측 근거는 §6-4 |
| `sd_cm` | ≤ 0.25 | ring 평면 피팅 잔차. 통과율 98% |
| bbox 경계 여백 | ≥ 10 px | 잘리면 ring ROI 가 비대칭이 되어 대표픽셀 가정이 깨짐 |
| `xyz_cam[2]` | 0.10 ~ 0.30 m | 그 너머는 depth 정확도·모델 confidence 동반 하락 |
| 스냅샷 | 연속 10프레임 | |
| 크기 교차검증 | **비활성** | §6-3 참조 |

---

## 5. 결과

### 세션 요약

| 세션 | 자세 범위 | 표본 | p95 | 답한 질문 |
|---|---|---|---|---|
| run1 | travel 291 mm / 66° | 28 | 10.54 mm | 사슬 정합성 |
| run2 | travel 270 mm / 79° | 18 | 12.18 mm | 사슬 정합성 |
| **run3** | **±2 cm / ±3° (AMR 정차)** | **20** | **1.37 mm** | **운용 정확도** |

### 사슬은 옳다 — 독립 세션 평균이 2.45 mm 안에서 일치

```
run1 mean [+0.4545 -0.3644 +0.4850] m   (28표본, 66° 회전 포함)
run2 mean [+0.4557 -0.3625 +0.4858] m   (18표본, 79° 회전 포함)
차이       [+1.15   +1.99   +0.83  ] mm  →  2.45 mm
```

서로 다른 자세 집합으로 독립 측정한 두 평균이 2.45 mm 안에서 만난다. 곱 순서·회전 규약·
hand-eye 중 하나라도 틀렸다면 66°/79° 회전을 섞은 두 집합의 평균이 이렇게 붙을 수 없다.
(측정 사이 매거진은 움직이지 않았음을 확인)

### run3 최종 (PASS)

```
표본 20개   travel 22.88 mm ✓   wrist 5.05° ✓
p95 1.37 mm   중앙값 0.79   최대 1.50
축별 p95   X 0.60   Y 1.08   Z 1.04 mm
P_base 평균 [+0.4703 -0.3702 +0.4870] m
cam z 0.202~0.218 m   valid_pct 81~93%   대상 front_grill1
```

**정확도 예산 기록용 한 줄**

> vision→base 재현성 **p95 1.37 mm** (축별 X 0.60 / Y 1.08 / Z 1.04 mm), 표본 20개
> 조건: 포락선 ±20 mm / ±3.0°, 작업거리 0.20~0.22 m, 대상 `front_grill1`(grill형),
> 표본 = 게이트 통과 10프레임 평균, 커버리지 travel 22.9 mm / wrist 5.05°

### 왜 넓은 자세는 10~12 mm 인데 좁은 범위는 1.4 mm 인가

오차가 **시점 의존**이기 때문이다. 자세가 크게 변하면 그만큼 오차도 변하고, 정차 오차
수준으로 좁히면 시점이 거의 안 변해 오차도 거의 안 변한다. 두 숫자는 모순이 아니라
서로 다른 질문의 답이다.

- **run1/run2 (넓은 자세)** = "사슬이 맞나" → 맞다
- **run3 (좁은 포락선)** = "실제 쓰는 범위에서 얼마나 정확한가" → 1.4 mm

포락선이 커지면 p95 도 커진다. ±2 cm 에서 1.4 mm, ±29 cm 에서 11 mm 였으니 그 사이
어딘가다. **Hils 스펙을 받으면 `config.ENVELOPE_*` 를 갱신하고 재측정할 것.**

### 오차의 정체 (넓은 자세 46표본 분해)

base 좌표에서는 X 가 지배적으로 보이지만, 광축(cam +Z)의 base 성분이 `[+0.88 -0.37 -0.02]`
로 base X 와 거의 나란해서 그렇다. **카메라 프레임으로 되돌리면**:

| 카메라 축 | p95 | 에너지 기여율 |
|---|---|---|
| X (가로) | 8.18 mm | 37% |
| Y (세로) | 5.56 mm | 17% |
| Z (깊이) | 7.73 mm | **45%** |

깊이와 가로가 거의 반반이다. 한 놈이 범인이 아니다.

- **깊이 45%** — ring depth 가 시점에 따라 ±8 mm 흔들림
- **가로 37%** — bbox 중심 이동으로 환산하면 중앙값 6 px, p95 18~28 px

둘 다 "물체를 정면 아닌 각도에서 볼 때" 생기며, hand-eye 나 곱 순서와는 무관하다.

---

## 6. 작업 중 발견한 것 (인수인계 핵심)

### 6-1. depth_units 가 10배 어긋나 있었다 ★

D405 가 내보내는 depth 는 정수(uint16)이고, 미터로 바꾸는 계수 `depth_units` 는
**카메라 펌웨어에 남는 장치 옵션**이다. 다른 프로그램(컨테이너의 `realsense2_camera` 등)이
바꿔놓으면 그대로 남는다. 실제로 **0.001 로 오염돼 있었다.**

비전담당자 `depth_calculator.py:41` 은 `DEPTH_SCALE = 0.0001` 하드코딩이고 수정 금지다.
따라서 **코드를 장치에 맞추는 게 아니라 장치를 코드에 맞춘다** — `GrabberEx.start()` 가
매번 `depth_units` 를 `depth_calculator.DEPTH_SCALE` 로 써넣고, 되읽어 확인하고, 어긋나면
`RuntimeError` 로 중단한다.

> **★ 반드시 `pipeline.start()` 前에 설정해야 한다.**
> 스트리밍 시작 후에 바꾸면 `get_depth_scale()` 보고값만 바뀌고 실제 양자화는 그대로다.
> 즉 "0.0001 이라고 말하면서 0.001 로 양자화된 raw" 를 받게 되어 **거리가 조용히 10배**
> 틀어지고, 되읽기 검증도 통과해 버린다. 실측:
>
> | 설정 시점 | 보고값 | raw median | 환산 |
> |---|---|---|---|
> | pre-set 0.001 | 0.001 | 1396 | 1.40 m |
> | pre-set 0.0001 | 0.0001 | 12353 | 1.24 m |
> | **post-set 0.0001** | **0.0001** | **1336** | **0.13 m ← 틀림** |

부수 효과: 러너를 한 번 띄우면 그 뒤에 실행하는 하네스도 0.0001 로 정렬된 상태에서 돈다.

### 6-2. 검출 confidence 가 거리에 따라 무너진다

| 거리 | bbox | score | 결과 |
|---|---|---|---|
| 14.6 cm | 366x618 | 0.90 | 검출 |
| 17.7 cm | 287x513 | 0.96 | 검출 |
| 37.1 cm | 140x253 | **0.66** | **CONF DROP** (`CONF_BY_CLASS["front_grill1"] = 0.80`) |

depth 자체는 37 cm 에서도 valid 90% 로 멀쩡하다. **모델이 그 거리에서 자신 없어할 뿐**이다.
학습 데이터가 근거리 위주로 보인다. 게이트의 `z ≤ 0.30 m` 상한은 이 관측에서 나왔다.

`config.CONF_OFFSET` 노브를 두었다. **기본 0 이고, 0 이면 하네스와 완전히 동일하게 동작하며
페이로드 스키마도 그대로다.** 0 이 아니면 부팅 시 경고를 찍고 모든 페이로드에 `conf_offset`
필드가 추가로 실린다 — 그 상태에서 잰 수치는 하네스와 등가가 아니므로 따로 기록해야 한다.
비전담당자가 "임계 낮춰서 써봐라" 라고 결정했을 때만 건드릴 자리다.

### 6-3. 비전담당자 SIZE_K 가 25% 어긋나 있다 (전달 필요)

```
front_grill1 관측  K_w ≈ 51~52,  K_h ≈ 91~94   (17.7cm / 20cm 두 거리에서 일치)
설정값             K_w = 69.0,   K_h = 121.0   (realtime_app_jetson.py SIZE_K["front"])
→ 기대 bbox 대비 -26% / -25%
```

`SIZE_TOL = 0.3` 이라 지금은 통과하지만 **여유가 4~5% 뿐**이다. 조금만 틀어지면 정상 검출이
`[SIZE DROP]` 으로 떨어진다.

원인은 주석에 있다 — `front_solid 38cm 182x320` 으로 잰 값(K_w=69)인데, `CLASS_GROUP` 이
`front_grill1` 과 `front_solid` 를 같은 `front` 그룹으로 묶어 SIZE_K 를 공유한다.
"그릴/솔리드는 매거진 사이즈 동일"이라는 전제가 실제로는 25% 어긋나 있다.
**핀홀 모델 자체는 성립한다** (두 거리에서 K 가 일치). 설정값만 틀렸다.

`vision_bridge` 의 크기 교차검증(`GATE_SIZE_CHECK`)은 이 때문에 **기본 비활성**이다.
클래스 분리가 끝나면 `True` 로 켜되, `config.SIZE_K`/`CLASS_GROUP` 이 하네스와 같은지
반드시 대조할 것 (컨테이너에서 그 모듈을 import 할 수 없어 값을 옮겨 적어 두었다).

참고로 이 매거진 전면 실물 크기는 bbox+depth 로 **7.89 x 14.14 cm** (왜곡 보정 기준)이다.

### 6-4. valid_pct 는 거리가 아니라 "보는 각도"에 좌우된다

| 자세 | cam z | valid_pct |
|---|---|---|
| A | 18.2 cm | 89~92% |
| B | 25.4 cm | 94% |
| C | 20.0 cm | **68~80%** |

단조롭지 않다. D405 는 프로젝터 없는 수동 스테레오라 **조명·텍스처가 곧 이 값**이고,
매거진 전면 그릴을 어떤 각도로 보느냐에 따라 74~94% 로 출렁인다.

게이트 임계를 70 → 80 으로 올렸다가 되돌렸다. 이유는 통계가 아니라 **실현 가능성**이다:
작업 자세 C 에서 747프레임을 재보니 80 기준 통과율이 **0%** 였다. 도달 불가능한 값은
운용 게이트가 될 수 없다.

물리적 개선책은 **매거진 전면 조명 보강**이다. 근본 대응이고 실제 운용에도 도움이 된다.

### 6-5. 이식 중 잡은 버그 두 개

- **median 필터가 첫 값에 얼어붙었다.** deque 에 넣은 것이 `T_cam_marker[:3,3]` 의 **view**
  라, 바로 다음 줄에서 median 을 그 자리에 써넣을 때 히스토리 자체가 덮어써졌다.
  자기 자신을 먹는 필터가 된다. `np.asarray` 는 같은 dtype 이면 복사하지 않는 게 함정.
  → 복사본 저장으로 수정 + 추종 회귀 테스트 추가.
- **OpenCV 4.13 이 `cv2.aruco.detectMarkers` 자유함수를 제거**했다 (원본이 돌던 ROS2 Humble
  의 4.5 에는 있었다). `ArucoDetector` 분기 추가. 기존 `Dictionary_get` 분기는 원본대로 유지.

### 6-6. 역투영 대표픽셀의 근거

대표 픽셀 = `smooth_bbox` 중심, Z = `avg_dist_cm`(ring 평균).

`depth_calculator.fit_surface_plane` 이 픽셀 좌표를 중심화한 뒤 **절편**(`coef[2]`)을
반환하므로, `avg_dist_cm` 은 **샘플 픽셀 중심에서의 평면 깊이**다. ring ROI 는 bbox 기준
균일 마진(`uniform=True`)이라 그 중심이 곧 bbox 중심 → 평면이 기울어 있어도 대응이 성립한다.

**단, bbox 가 화면 경계에서 잘리면** ROI 가 비대칭이 되어 샘플 중심이 bbox 중심에서 밀리고
`기울기 × 밀린 거리` 만큼 Z 에 오차가 남는다. 게이트의 경계 여백 10px 조건이 이것을 막는다.

### 6-7. 프레임 평균은 자세 의존 오차를 줄이지 못한다

고정 자세에서 10프레임의 흔들림은 **0.07~0.2 mm** 였다. 즉 러너 출력은 한 자세에서 이미
극도로 안정적이고, run1/run2 의 4~12 mm 는 **전부 자세 의존 성분**이다. 10프레임 평균은
검증과 운용의 정의를 맞추는 의미는 있지만 정확도를 올리지는 않는다.

---

## 7. 방법론 — 사후 조건 선택을 피한 기록

측정값을 보고 조건을 고르면 노이즈에 맞추게 된다. 이 작업에서 두 번 그 유혹이 있었고,
두 번 다 새 데이터로 검정했다.

**① 게이트 사후 필터링** — run1/run2 에서 `valid ≥ 80` 으로 거르면 FAIL 이 PASS 로 바뀌었다.
같은 데이터로 재채점한 것이라 근거가 못 된다고 보고, 사전 기준으로 채택해 **새로 측정**하기로
했다. (그 결과가 6-4 의 통과율 0%)

**② valid_pct ↔ 잔차 가설** — "낮은 valid 가 잔차 꼬리를 만든다"는 run1/run2 를 보고 세운
가설이므로, run3 새 데이터로 검정했다.

```
run3 (n=20): valid_pct 81~91%
  잔차 vs valid_pct : r = -0.25      ← 약함
  하위군 잔차 중앙값 0.68 mm  vs  상위군 0.54 mm
판정: 재현 안 됨 → 문턱 70 유지
```

단서: run3 의 valid 범위가 좁아(81~91%) 검정력이 약하다. 70~75% 구간은 여전히 미확인.

**③ 기각된 가설** — "화면 중심에서 벗어난 정도(off-axis)가 오차를 만든다" 는 상관 r=+0.29 로
기각. 434 px 벗어난 표본이 2.18 mm, 382 px 벗어난 표본이 11.19 mm 로 순서가 뒤죽박죽이었다.

**④ 커버리지 문턱** — run3 이 travel 19.5 mm 로 기준 20 mm 에 0.5 mm 미달했다. 문턱을 낮추는
대신, 커버리지를 줄여가며 p95 변화를 확인했다: 8.4 mm→1.03, 13.4→0.98, 17.3→1.09, 19.5→1.14.
**커버리지를 2.3배 늘려도 p95 는 10% 변화** — 커버리지가 결과를 좌우하지 않음을 확인한 뒤,
그래도 형식을 맞추려 표본을 추가해 22.88 mm 로 닫았다.

---

## 8. 남은 일

| 항목 | 내용 |
|---|---|
| **Hils 스펙 반영** | `vision_bridge/config.py` 의 `ENVELOPE_*` 갱신 후 재측정. 포락선이 커지면 p95 도 커진다 |
| **SIZE_K 전달** | 비전담당자에게 6-3 전달. 클래스 분리 후 `GATE_SIZE_CHECK = True` |
| **조명 보강** | 6-4. valid_pct 를 올리면 게이트 여유가 생긴다 |
| **원거리 검출** | 6-2. 30 cm 너머 confidence 하락. 재학습 또는 작업거리 제약으로 대응 |
| **대표픽셀 개선** | 미착수. ring 유효픽셀 중심으로 바꾸면 가로 성분(37%)은 줄지만 조명·반사에 따라 기준점이 물리적으로 이동해 **편향**이 생길 수 있다. 깊이 성분(45%)은 그대로다. 1.4 mm 로 충분하지 않을 때만 검토 |
| **git** | `vision_bridge/`, `handeye_calib/` 가 untracked. `~/dobot_ws` 에 다른 작업이 섞여 있어 커밋하지 않았다 |

### 표본 파일

| 파일 | 내용 |
|---|---|
| `verify_samples_run1.json` | 넓은 자세 28표본 — 사슬 정합성 기준선 |
| `verify_samples_run2.json` | 넓은 자세 18표본 — 〃 |
| `verify_samples_run3_final.json` | **포락선 20표본, PASS — 정확도 예산 근거** |
| `verify_samples_aborted_n1.json` | valid≥80 에 막힌 세션 (참고) |

`verify_chain.py` 는 실행 시 기존 `verify_samples.json` 을 수정시각을 붙여 옆으로 밀어둔다.
표본 json 은 재분석·기준선·가설검정에 쓰이므로 지우지 말 것.

---

## 9. 하지 말 것 (설계 제약)

- 비전담당자 파일 수정 (`GrabberEx` 상속 예외)
- `~/robot_vision` 안에 그의 코드 사본 생성
- 신규 venv, 허용 목록(`pyzmq`, `msgpack`) 외 패키지 설치
- 스레드 / asyncio / 멀티프로세싱
- 러너에 `cv2.imshow` (`tests/standalone_aruco.py`, `tests/preview.py` 예외)
- `handeye_result.json` 수정, `T_flange_cam` 역행렬 사용, 회전 규약(ZYX) 재유도
- 로봇으로의 명령 전송 (29999 접속 금지, 30004 read-only)
- depth scale · K · 해상도 · 엔진 경로 · 마커 파라미터의 러너 내 하드코딩
