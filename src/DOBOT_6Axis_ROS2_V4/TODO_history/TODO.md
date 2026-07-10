# TODO

## Goal
D405 depth를 활용해 wirebonder 태그의 **거리축만** 교정하는 depth-우선 하이브리드를
추가한다. 단일 뷰 PnP(회전) + depth(거리)로 device pose를 한 프레임에 잡고, depth가
무효(구멍 많음/범위 밖)일 때는 **기존 two-view 삼각측량 경로로 fallback**한다(코드 유지,
삭제하지 않음). sim에서 검증한 뒤 실물 D405로는 토픽 remap + depth 단위 처리만으로 이전.

## Tasks

### 1. sim 카메라에 depth 출력 추가 (cr7_on_mpo700.urdf.xacro만)
- [x] `cr7_on_mpo700.urdf.xacro`의 d405 센서 블록에서 `type="camera"` →
      `type="depth"`로 변경, `libgazebo_ros_camera.so` 그대로 사용해 depth image +
      depth camera_info를 `d405_optical_frame` 기준으로 publish (min/max_depth 0.07/5.0).
- [x] `colcon build --packages-select cra_description` 후 sim 띄워 토픽 확인.
      확정: depth=`/d405/color/depth/image_raw`, info=`/d405/color/depth/camera_info`,
      points=`/d405/color/points`. color 토픽(`/d405/color/image_raw`,`.../camera_info`) 불변.
- [x] depth 인코딩 확인: **32FC1, 848x480, 미터**.
- [x] arm_on_mpo700(CR10 jog용) 블록은 건드리지 않음 — 이 태스크 범위 밖.

### 2. depth 단위 정규화 리더
- [x] `wirebonder_vision.py`에 순수 함수 `read_depth(data,h,w,encoding)` 추가(노드가
      아니라 offline 모듈에 둬서 `_demo()` 자체점검이 붙음). HxW float32 **미터**로 통일,
      encoding 분기 `32FC1`(sim, 미터)/`16UC1`(실물 D405, mm→/1000), 0/non-finite→NaN.
- [x] `_demo()`에 두 인코딩 + NaN 처리 assert 추가 → self-check OK.

### 3. depth-우선 range 교정 (wirebonder_vision.py)
- [x] `device_pose_in_base(T_base_optical, solutions, depth=None, K=None, corners=None)`로
      확장: PnP로 회전 그대로 두고, depth 유효 시 tvec을 depth-deproject로 교체.
- [x] 태그 **본체 안쪽(eroded ROI, `_tag_roi`)** median depth 샘플 → 태그 중심 픽셀
      deproject(`_deproject`)로 optical 3D 위치 → tvec 교체(테두리 깜빡임 회피).
      ray 따라 교정하므로 X/Y도 비례 보정(거리축뿐 아니라 translation 전체 정합).
- [x] `_depth_corrected_tvec`: 유효 픽셀 비율(`DEPTH_MIN_FRAC`) + 거리 창
      (`DEPTH_MIN_Z/MAX_Z`) 밖이면 None → PnP `tvec` 유지(무효 fallback). 상수는 실물 노브 주석.
- [x] `_demo()` 케이스 7: 합성 depth로 wrong-range 교정 성공 / all-NaN depth로 PnP 유지 → OK.

### 4. 노드 배선 (depth = 주 경로, two-view = fallback 유지)
- [x] `/d405/color/depth/image_raw` 구독(+`_depth_cb`/`_depth_m`), depth를
      `device_pose_in_base`에 전달해 **depth-교정 pose(단일 뷰)를 odom 주 경로로 publish**.
      depth camera_info는 불필요 — aligned depth라 color K로 deproject.
- [x] `_tick` 재작성: 라이브 태그 검출 시 depth-교정 pose publish, 미검출 시에만
      마지막 two-view solve republish(fallback). 진단 프린트도 depth-교정 vis로.
- [x] 기존 two-view(`device_pose_from_two_views`, `/vision/capture` 콜백) 삭제 없이 유지.
- [x] node py_compile + wirebonder_vision self-check 통과.

### 5. sim 검증 (vis vs gt, bias 확인)
- [x] sim 띄우고 시퀀스로 팔을 capture 뷰포인트 서보 → depth-교정 slot 중심이
      `DEVICES_GT`와 일치: A=0.4 / B=1.8 / C=3.7 / D=3.3 mm (팔 정지 시). 검출 1115/미검출 9.
      시퀀스가 depth 주 경로 소비: wb1 x=2.347 y=0.508 z=0.001 yaw=0.005 (gt 2.35/0.5/0/0).
- [x] 편향(bias) 없음 — 차이 sub-mm~3mm 대칭(sim 정확). two-view fallback도 동작 확인.
      (팔 이동 중 단일 프레임만 transient 오차, median-of-frames가 걸러냄.)
- [x] (발견) 환경: 시퀀스/pinocchio가 .venv numpy 2.2.6과 충돌해 segfault →
      system python3(numpy 1.21)로 실행하면 정상. real_robot_transition 문서에 기록.

### 6. 실물 이전 문서화 (코드 아님)
- [x] `docs/real_robot_transition.md` 작성: A.배선(드라이버/토픽/인코딩/cv2),
      B.캘리(hand-eye/왜곡/모델기하/태그), C.튜닝, D.sim전용 검증. 체크리스트 + 파일 참조.
      remap 절차·align_depth·16UC1 흡수 포함. 이후 그리퍼 등 이어붙일 자리 마련.

### 7. (발견/수정) seq 2/3 under-reach — depth 회전 안정화 + 재anchoring
- [x] 원인 1: 단일뷰 PnP **yaw이 뷰포인트에 민감**(작은 평면 태그 약관측) → view A를 RRT로
      가니 run마다 yaw이 20°까지 요동 → 먼 슬롯 C/D가 120mm 밀림. depth Z는 회전 안 고침.
- [x] (b) **depth 평면 법선으로 회전 안정화**: `_tag_plane_normal`(태그 depth 3D 점 →
      SVD 평면 fit → 법선)로 IPPE flip disambiguate + `_align_normal`로 normal snap.
      yaw 요동 20°→**0.000**(뷰포인트 불변). in-plane은 PnP, range는 depth Z 유지.
- [x] 원인 2: view A RRT 착지가 run마다 달라 x/z deproject ~30mm 흔들림 →
      **`CAPTURE_A_JOINTS` 고정 joint로 `joint_move`**(결정론적 뷰포인트). view B 모션 제거.
- [x] 옵션 A: 고정 뷰포인트의 depth read로 `OLD_DEVICE_POSE`+`DEVICES['wb1']` 재캘리 →
      SLOT_WORLD(조깅 좌표) 불변, bias 양쪽 상쇄. **slot drift = (0,0,0)mm, dyaw 0** 확인.
- [x] y(range)/yaw 완벽 재현(뷰포인트 불변). x/z는 AGV park 따라감(재anchoring 전제).
- [ ] seq 2/3 물리 배치는 메뉴가 termios(대화형)라 파이프 구동 불가 → 사용자가 직접 실행.
      단 slot drift=(0,0,0)=SLOT_WORLD 일치이므로 --no-vision과 동일 목표 → 동일 배치 보장.
- [ ] real_robot_transition.md에 "파이프라인/센서/capture포즈 바뀌면 OLD_DEVICE_POSE 재캡처" 기록.

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
