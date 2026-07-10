# 실물 로봇 전환 가이드

sim에서 검증된 것을 실제 하드웨어(D405 카메라, 실물 팔/그리퍼)로 옮길 때
손봐야 하는 곳을 모은 문서. 필요한 항목이 생기면 아래에 계속 이어서 추가한다.

---

## 1. Vision (D405 depth 하이브리드) 전환

### A. 안 하면 아예 안 도는 것 (배선)

- [ ] **카메라 드라이버 교체**: sim `gazebo_ros_camera` → 실물 `realsense2_camera`
      노드 실행. 런치에서 `align_depth.enable=true`(depth를 color 픽셀에 정합) 켤 것.
- [ ] **토픽 이름**: `vision/wirebonder_vision_node.py`의 구독 토픽이 하드코딩돼 있음
      (`/d405/color/image_raw`, `/d405/color/camera_info`, `/vision/capture`).
      실물 realsense 토픽명과 다르면 런치 remap 또는 해당 줄 수정. depth 토픽도 구독 추가.
- [ ] **depth 인코딩**: sim 32FC1(미터) ↔ 실물 D405 16UC1(밀리미터).
      `_read_depth()`가 encoding 보고 미터로 통일 (이미 코드에 반영 예정).
- [ ] **OpenCV 버전**: 코드가 4.5.4 구 API 사용(`DetectorParameters_create`,
      `solvePnPGeneric`). 실물 머신 `cv2` 버전이 다르면 이 호출이 깨짐 → 확인/맞출 것.

### B. 정확도 위해 캘리/실측 필요 (핵심 갭)

- [ ] **hand-eye calibration** ⭐: `cra_description/urdf/cr7_on_mpo700.urdf.xacro`의
      `d405_joint origin xyz="0.071 0 0.147" rpy="0 0 0"`은 명목값(추측).
      실물 장착 오차 때문에 아루코 10~20 포즈로 hand-eye cal(AX=XB) 수행 →
      실측값으로 이 origin 교체. 안 하면 depth·two-view 좌표가 **똑같이 편향**됨.
      (two-view는 hand-eye cal이 아님 — 삼각측량은 hand-eye가 이미 맞다고 가정함.)
- [ ] **카메라 왜곡(distortion)**: sim은 왜곡 0이라 `detect_tag`가 `dist=zeros`로 동작.
      실물 D405는 왜곡 있음 → CameraInfo의 D 계수를 읽어 `detect_tag`에 전달.
- [ ] **디바이스 모델 기하 실측**: `vision/wirebonder_vision.py`의 `SLOT_OFFSET`,
      `TAG0_XYZ`, `TAG0_RPY`는 sim `model.sdf` 기준. 실물 wirebonder/매거진의 태그
      부착 위치·슬롯 간격이 다르면 재측정해 이 상수 교체. `R_CV_TO_SDF`(sim SDF 기준
      empirical pin)도 실물 태그 장착 방향이 SDF 가정과 같은지 확인.
- [ ] **물리 태그**: `TAG_SIZE_M=0.03` → 실물 인쇄 태그가 정확히 30mm여야 함(아니면 수정).
      딕셔너리 `DICT_APRILTAG_36h11` 일치, 무광 인쇄, 평면 부착.

### C. 튜닝 (실물서 값만 조정)

- [ ] **`_depth_valid` 임계값**: 유효 픽셀 비율 / 예상 거리 창. sim은 완벽해서 안 걸림
      → 실물서 실측으로 잡음.
- [ ] **realsense 후처리**: temporal/spatial 필터 on. hole-filling은 추정값이라
      정확도용으론 주의.
- [ ] **조명/작업거리**: min range ~7cm 밖, 충분한 조명, 광택 없는 표면.

### D. 실물엔 없어지는 것

- [ ] **`DEVICES_GT` 검증**: sim은 device의 odom 참값을 알아 vis-vs-gt 프린트가 되지만
      (`wirebonder_vision_node.py`), 실물엔 ground truth가 없음. 그 side-by-side 검증은
      sim 전용 → 실물 검증은 "잡은 pose로 실제 집히나"로 대체.

---

## OLD_DEVICE_POSE 재캡처 (파이프라인/센서/capture 포즈 바뀔 때마다)

`wirebonder_pick_place.py`의 waypoint(SLOT_WORLD)는 `OLD_DEVICE_POSE`에 재anchor돼,
런타임에 vision이 읽는 device pose와 합성됨. 상쇄가 성립하려면 **OLD = 그 vision
파이프라인이 고정 capture 포즈(`CAPTURE_A_JOINTS`)에서 읽는 값**이어야 함. 따라서:

- **vision 알고리즘**(예: two-view→depth-upright), **센서**(sim→실물 D405), 또는
  **capture 포즈**를 바꾸면 → 그 조건에서 device를 한 번 캡처해 `OLD_DEVICE_POSE`
  (+ `DEVICES['wb1']`)를 그 read로 **재캘리**. SLOT_WORLD는 건드리지 않음.
- 현재(sim depth-UPRIGHT, 2026-07-10 이후): 위치+yaw 모두 depth에서 직접 구성,
  PnP 회전 미사용 (`docs/vision_viewpoint_dependence_fix.md`). read가 **뷰포인트
  불변**(5개 config에서 spread <1 mm)이므로 캡처 포즈/AGV park가 조금 달라져도
  read는 같은 값 — 재캡처는 파이프라인/센서 변경 때만 실질적으로 필요.
- 실물: hand-eye cal 후 재캡처. `CAPTURE_A_JOINTS`도 실물 팔에서 태그를 잘 보는
  (가급적 fronto-parallel) config로 재조깅 권장. 브링업 검증은
  `tools/diag_camera_geometry.py`의 다중 뷰포인트 일관성 체크(GT 비교 제외)로.

## 환경 노트 (sim/실물 공통)

- **시퀀스 실행 python**: `.venv`(numpy 2.2.6)로 `wirebonder_pick_place.py`를 돌리면
  `/opt/ros/humble`의 pinocchio(numpy 1.x 빌드)와 충돌해 **segfault**. system python3
  (`/usr/bin/python3`, numpy 1.21)로 돌리면 정상. vision 노드도 system python3 사용.
  (conda 활성 시 PATH에서 벗겨야 함 — `run_mpo700_cr7.sh`가 그 처리를 함.)

<!-- 이후: 그리퍼 교체 시 재조정 파라미터 등 이어서 추가 -->
