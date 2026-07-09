# TODO

## Goal
D405 depth를 활용해 wirebonder 태그의 **거리축만** 교정하는 depth-우선 하이브리드를
추가한다. 단일 뷰 PnP(회전) + depth(거리)로 device pose를 한 프레임에 잡고, depth가
무효(구멍 많음/범위 밖)일 때는 **기존 two-view 삼각측량 경로로 fallback**한다(코드 유지,
삭제하지 않음). sim에서 검증한 뒤 실물 D405로는 토픽 remap + depth 단위 처리만으로 이전.

## Tasks

### 1. sim 카메라에 depth 출력 추가 (cr7_on_mpo700.urdf.xacro만)
- [ ] `cr7_on_mpo700.urdf.xacro`의 d405 센서 블록(434~458)에서 `type="camera"` →
      `type="depth"`로 변경, `libgazebo_ros_camera.so` 그대로 사용해 depth image +
      depth camera_info를 `d405_optical_frame` 기준으로 publish.
- [ ] `colcon build --packages-select cra_description` 후 sim 띄워
      `ros2 topic list`로 depth 토픽 실제 이름 확인 → 이후 노드 구독에 반영.
- [ ] `ros2 topic echo --once`로 depth 인코딩(32FC1, 미터) 확인.
- [ ] arm_on_mpo700(CR10 jog용) 블록은 건드리지 않음 — 이 태스크 범위 밖.

### 2. depth 단위 정규화 리더
- [ ] `wirebonder_vision_node.py`에 `_read_depth(msg)` 추가: HxW float32 **미터**로 통일.
      encoding 분기 — `32FC1`(sim, 미터) / `16UC1`(실물 D405, 밀리미터 → /1000).
- [ ] 두 인코딩 각각에 대한 assert 자체 점검(작은 합성 배열).

### 3. depth-우선 range 교정 (wirebonder_vision.py)
- [ ] `device_pose_in_base(T_base_optical, solutions, depth=None, K=None, corners=None)`로
      확장: PnP로 회전 그대로 두고, depth 유효 시 **거리축만** 교체.
- [ ] 태그 **본체 안쪽(eroded ROI)** median depth 샘플 → 태그 중심 픽셀 deproject
      (`Z * Kinv @ [u,v,1]`)로 optical 3D 위치 → `T[:3,3]` 교체(테두리 깜빡임 회피).
- [ ] `_depth_valid(...)`: 유효 픽셀 비율 + 예상 작업거리 창(예: 0.07~1.0 m) 밖이면
      reject → PnP `tvec` 유지(무효 fallback). 임계값은 실물 튜닝 노브로 주석.
- [ ] `_demo()`에 케이스 추가: 합성 depth로 range 교정 성공 / 무효 depth로 PnP 유지.

### 4. 노드 배선 (depth = 주 경로, two-view = fallback 유지)
- [ ] `/d405/.../depth/image_raw` + depth camera_info 구독, depth를
      `device_pose_in_base`에 전달해 **depth-교정 pose를 주 경로로 publish**.
- [ ] 기존 two-view(`device_pose_from_two_views`, `/vision/capture` 콜백)는 그대로 두고
      planner가 오케스트레이션하는 fallback으로 유지(삭제하지 않음).

### 5. sim 검증 (vis vs gt, bias 확인)
- [ ] sim 띄우고 AMR을 wb1에 park, 팔 hub로, 노드 실행 → depth-교정 slot 중심이
      `DEVICES_GT`와 수 mm 내 일치하는지 vis vs gt 프린트로 확인.
- [ ] 중심 편향(bias) 유무 기록 — 있으면 실물에서 on-chip self-cal 필요 신호로 남김.

### 6. 실물 이전 문서화 (코드 아님)
- [ ] `docs/`에 remap 절차 기록: 실물은 `realsense2_camera` + `align_depth.enable=true`,
      depth 16UC1(mm) → `_read_depth`가 흡수, 로직 변경 0. 토픽 매핑 표만 남김.

## 참고사항
- sim depth는 노이즈/구멍이 없어 **`_depth_valid` fallback 경로가 sim에선 트리거되지
  않음** — fallback 정확성은 실물에서 별도 검증(또는 센서 플러그인 gaussian noise로 모사).
- ±1cm 랜덤 jitter는 프레임 median으로 mm급으로 줄어듦(노드가 이미 median-of-frames).
  교정이 밀어내는 단안 PnP range 오차보다 낫다는 전제.
- d405 블록이 cr7_on_mpo700 / arm_on_mpo700 두 곳에 복제돼 있음. wirebonder 플로우는
  전자만 사용하므로 전자만 수정. 나중에 CR10에도 depth 필요하면 동일 수정 반복.
- two-view는 실물에서 팔 FK/baseline 오차에 의존 → depth 대비 부정확할 수 있으나,
  검증 전까지 작동하는 fallback을 삭제하지 않는다(삭제는 실물 데이터 후 재결정).
- 실물 이전 접합부는 딱 둘: (1) 토픽 remap, (2) `_read_depth` 단위 정규화.
