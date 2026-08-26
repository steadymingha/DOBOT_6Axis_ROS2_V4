# 04. 비전 명세 — `vision/`

설계 원칙: **검출/기하 코어는 순수 모듈**(cv2/numpy만, rclpy 없음 → 오프라인 테스트 가능),
**ROS 배관은 노드 파일**에만. 카메라는 눈-손(D405, 플랜지 장착), 토픽명은 sim/실물 동일
(`/camera/d405/...`) — 노드 코드는 양쪽에서 무수정.

```
vision/
├── tag_vision.py         ★ 코어: 태그 검출 → base_link 모델 포즈 (순수, rclpy 없음)
├── tag_vision_node.py    ROS 노드 (sim): 이미지/TF 구독 → 포즈 발행
├── vision_hover_node.py  ROS 노드 (real, 2026-08-17): /vision/capture →
│                         vision_bridge 사슬로 AI 매거진 검출 → 박스 중심을
│                         /vision/device_pose 로 발행. launch profile:=real 이 선택
└── pocket_vision.py      베이스 포켓 점유 판정 (depth 기반, numpy만)
```

## 실물 비전 사슬

```
D405 ─► vision_runner (호스트, ~/robot_vision: YOLOX 매거진 검출 + ArUco)
      ─ZMQ 5555/5556─► vision_bridge (컨테이너: 10프레임 게이트 · hand-eye 합성)
      + robot_feed (30004 tool_vector, read-only)
      ─► vision_hover_node ─/vision/device_pose─► main.locate_box() → 선반 목표
```
- 계산은 전부 `test/vision_target.py`(`--vision --run` 실물 검증본) 재사용 — 노드는 shim.
- 실물 `/vision/pocket_state` 발행자는 아직 없음(3c) → real 프로파일은 포켓 점유 비전을
  끄고 정적 기본 포켓 사용. `/vision/shelf_pose`(ArUco)도 실물 발행자 미구현(3b, 정밀화용).

## `tag_vision.py` — 태그 → 포즈 코어

- 태그: **AprilTag 36h11, 30 mm**, `cv2.aruco`(OpenCV 4.5.4 구 API)로 검출.
  - ID 0/1: **wirebonder 장비** (좌측 컬럼, 슬롯 A/B 위 — 1장으로 6-DoF 포즈)
  - ID 2/3: **선반 tier** (단마다 1장)
- 두 태그군이 **같은 파이프라인**을 공유: `detect_tag → device_pose_in_base`.
  다른 건 태그 id와 모델프레임 내 태그 포즈(`T_model_tag`)뿐.
- 프레임 체인:
  `optical(d405) → tag_cv(OpenCV) → (R_CV_TO_SDF) → tag_sdf → model → base_link`
- **`R_CV_TO_SDF` = Rz(−90°)** — OpenCV 마커 프레임 vs SDF 태그판 프레임의 규약 차이.
  순수 유도가 불가능해서 ground-truth 대조 실험으로 핀 박은 값 (수정 시 반드시
  `tag_vision_node.py` 진단 프린트로 재검증).
- `SLOT_OFFSET`(모델프레임 슬롯 4개 좌표)의 **단일 소스가 이 파일**.

## `tag_vision_node.py` — ROS 노드

`launch/arm.launch.py`가 main.py와 함께 띄움. 단독 실행은 `python3 vision/tag_vision_node.py`.

| 방향 | 토픽 | 타입 | 내용 |
|------|------|------|------|
| 구독 | `/camera/d405/color/image_raw` | Image | 컬러 (rgb8/bgr8, cv_bridge 없이 직접 변환) |
| 구독 | `/camera/d405/color/camera_info` | CameraInfo | 내부 파라미터 |
| 구독 | `/camera/d405/aligned_depth_to_color/image_raw` | Image | depth (포켓 점유용) |
| 구독 | `/vision/capture` | Int32 | 0=리셋, 1=뷰 확보 (시퀀스가 촬영 지시) |
| 구독 | TF | | `base_link ← d405_optical_frame` (팔 FK 경유) |
| 발행 | `/vision/device_pose` | PoseStamped | wirebonder 장비 포즈 (odom) |
| 발행 | `/vision/shelf_pose` | PoseStamped | 선반 포즈 (odom) |
| 발행 | `/vision/pocket_state` | Int32MultiArray | 포켓 4칸 × {-1 미상, 0 빈, 1 박스} |

- 1초마다 진단 프린트 (검출 포즈 vs `DEVICES_GT` 하드코딩 ground truth 대조).
- **주의**: 이 노드의 실제 출력은 `~/.ros/log/python3_<pid>*.log`에 있음 (launch 하).
- "태그가 화면에 있는데 검출 안 됨" = 컬러 구독이 wedge된 것 (stale frame) —
  미러 구독 + staleness 워치독이 들어가 있음 (07 참고).

## `pocket_vision.py` — 베이스 포켓 점유

시퀀스의 하드코딩(포켓 인덱스, 박스 모델명)을 대체: 명령 시점에 팔이 허브에서
**J5 손목만 굽혀** 베이스를 내려다보고 4칸 점유를 읽음.

- **depth 기반, 마커 불필요**: 포켓은 base_link에 강체 고정 → 포켓 중심 광선의 depth가
  이봉(bimodal) — 빈 칸이면 포켓 표면, 박스가 있으면 140 mm 가까움. 그 외(그리퍼 가림,
  depth 구멍)는 UNKNOWN. 실물에서도 무수정 동작.
- 시선 자세는 런마다 **J5 스캔으로 유도** (IK/RRT 없음 → 허브 조인트 패밀리를 벗어나지
  않아 복귀가 안전). 조준 한계 `LOOK_MAX_OFF_DEG=20°`, 안 되면 조그 오버라이드 상수.
- 판정 노브: `Z_TOL=0.035 m`(140 mm 갭의 판별 허용), `ROI_R=6 px`, `ROI_MIN_FRAC=0.3`.

### 시퀀스 쪽 API
```python
pockets.subscribe(node); pockets.subscribe_models(node)   # 시작 시 1회 (main.py가 함)
pockets.check_pockets(node)     # 허브→J5 굽힘→다수결→복귀
pockets.next_free() / next_filled()   # 다음 빈/찬 포켓 (POCKET_ORDER_Y 순)
pockets.model_at(world_xyz)     # 해당 지점의 Gazebo 박스 모델명 (이름 하드코딩 제거)
```
- `model_at`은 `/gazebo/model_states`(gazebo_ros_state 플러그인) 의존 — **sim 전용**.
  실물에서는 모델명 자체가 필요 없어짐 (attach가 없으므로) → 실물 전환 문서 참고.
- 자가 테스트: `python3 vision/pocket_vision.py` (양쪽 env 모두 import 가능하게 설계됨).

## 진단 도구

- `debug/view_d405.py <topic>` — 카메라 뷰어 (run_mpo700_cr7.sh가 자동 실행)
- `tools/diag_camera_geometry.py` — 카메라 외부 파라미터/프레임 체인 검증
- `docs/vision_viewpoint_dependence_fix.md` — 시점 의존성 이슈 분석 기록

## 설계 노트 — 왜 이렇게 생겼나 (docstring에서 이관)

### 태그 포즈 (`tag_vision.py`)
- **IPPE 2해 모호성**: 평면 태그 PnP(IPPE_SQUARE)는 해가 최대 2개이고 정면에 가까울수록
  재투영 오차가 비슷해짐. 첫 해만 취하면 장비 yaw가 ~25° 튀는 플립이 그것.
  `detect_tag`는 후보 **리스트**를 반환하고 선택은 `device_pose_in_base`가 함.
- **depth 축별 융합**: depth가 온전하면 포즈를 depth만으로 구성(수직 구성).
  부분/구멍 depth여도 가용한 측정은 축별로 PnP를 보정함 —
  (a) 태그 평면 법선을 depth로 피팅(`_tag_plane_normal`: 피팅→면외점 기각→재피팅 ×3;
  sim은 정확해서 기각이 안 걸림) → 올바른 IPPE 해 선택 + 그 해의 법선을 스냅
  (면외 방향=depth, 면내=PnP), (b) 거리는 depth 역투영 중심으로 tvec 교체.
  나쁜 depth는 축별로 무시되고 PnP가 남음 — depth가 결과를 악화시키지 못하는 구조.
- **depth 없을 때**: 장비는 수직이 알려져 있으므로 모델 z축이 base z와 가장 정렬된
  해를 선택 (base_link z ≈ 월드 상방 가정 = 평평한 바닥 전제).
- **`plane_scale`**: 평면 피팅 샘플 범위. 기본 ×2 확장은 태그 주변이 큰 평면(wirebonder
  전면)일 때만 유효 — 선반의 소형 플래카드는 **~1.0**을 써야 배경 depth를 쓸어 담아
  피팅이 기울지 않음.
- **`read_depth`가 sim/실물 seam의 절반**: sim Gazebo=32FC1(m), 실물 D405=16UC1(mm).
  이 함수 하나가 흡수 (나머지 절반은 토픽 remap). 무효 픽셀은 NaN.
- **`_lookup_T`는 latest, stamp 아님**: stamp 조회는 타이머 콜백에서 TF가 이미지를
  따라잡길 기다려야 해 데드락 성향. 캡처는 **정지 상태에서만** 읽는 설계라 latest와
  stamp의 차이가 실측 0.0 mm (`tools/diag_camera_geometry.py`). **이동 중 캡처 금지.**

### 포켓 점유 (`pocket_vision.py`)
- **박스-상단 중심 샘플링**: 각 포켓의 판정점은 표면이 아니라 포켓 중심을 BOX_H만큼
  올린 **박스 상단 중심**. 사선 뷰에서도 광선이 목표까지 박스-상단 평면 위에 머물러
  **이웃 포켓 박스를 스칠 수 없다** — 표면점 방식은 정확히 그걸 스쳐서 박스 3개가
  1..3에 있을 때 빈 포켓 0을 false OCCUPIED로 읽었다(실측).
- 판정: ROI 중앙값 depth가 판정점 거리와 같으면 OCCUPIED(박스 상단), 더 멀면 EMPTY
  (광선이 빈 박스 공간 통과), 더 가까우면 UNKNOWN(그리퍼/팔 가림).
- **`check_pockets` 반환 계약**: 전부-UNKNOWN = look/read 실패 → 호출자 abort
  (next_free/next_filled가 아무것도 못 찾음). `None` = vision 미구성(subscribe 안 함,
  --no-vision)일 때만 — 그때만 정적 기본 포켓 폴백.
- **`_look_joints` = J5 스캔 (순수 계산, 팔 안 움직임)**: 고정 외부 파라미터
  (optical 포즈 = FK(q) @ T_tcp_opt)로 J5 델타를 스캔해 조준 오차 최소 config 선택.
  이전의 auto-aim TCP 포즈 방식은 툴을 수평(허브 패밀리에서 90° 이탈)으로 만들어
  IK 브랜치를 뒤틀고 자유 RRT를 스톨시켰음 — 그래서 J5만 굽힘.
- **`model_at`의 tol < 0.118 m**: 포켓 피치보다 작아야 이웃 박스가 매치될 수 없음.
