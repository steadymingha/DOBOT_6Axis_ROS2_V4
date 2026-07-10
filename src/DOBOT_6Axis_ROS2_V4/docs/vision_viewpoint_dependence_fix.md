# Vision 뷰포인트 의존성 디버깅 기록 (2026-07-10)

단안 + depth 하이브리드로 바꾼 뒤 "잘 안 되던" 원인을 계측으로 확정하고 고친 기록.
증상 → 진단 방법 → 측정 결과 → 근본 원인 → 해결 → 검증 순서.

## 증상

- depth 하이브리드(단일뷰 PnP + depth Z + depth 법선)의 device pose가 **캡처
  뷰포인트(팔 config)에 따라 x/z로 수 mm~수십 mm 널뜀**. 같은 뷰에서는 spread
  0(결정론적)이라 기존 gate로는 안 잡힘.
- 그래서 `CAPTURE_A_JOINTS`로 뷰포인트를 못 박고, `OLD_DEVICE_POSE` 상쇄(anchor)로
  가려 놓은 상태였음 — AGV 재주차/실물 전환 시 상쇄 조건이 깨지는 구조.

## 진단 방법 — `tools/diag_camera_geometry.py`

오차가 **어느 단계에서** 들어오는지 뷰포인트 5곳(A, A±관절 섭동 3, 원거리 B)에서 분리 계측:

1. **reproj**: GT 태그 코너(cr.world pose, odom==world 확인됨)를 TF(image stamp)+K로
   투영한 픽셀 vs 검출 픽셀 → 카메라 TF 체인/K/검출 오차 (depth 무관)
2. **depth**: 태그 ROI 평면 피팅 + 코너 픽셀 deproject → odom에서 GT 코너와 비교
   → depth 정합 오차
3. **centre**: 태그 중심 추정 2방식(현행 corner-mean+median-Z vs 대각교점+평면교차-Z)
4. **device**: 파이프라인 최종 device pose vs GT (x, y, z, yaw)
5. **tf-skew**: TF(latest) vs TF(image stamp) — 이동 중 캡처 시 오염 크기

실행: `/usr/bin/python3 tools/diag_camera_geometry.py` (sim 가동 + 팔 이동 가능 상태.
conda python에는 numpy가 없으니 반드시 `/usr/bin/python3`.)

## 측정 결과 (수정 전)

| 계측 | 결과 | 판정 |
|---|---|---|
| 코너 reproj | ~1 px, 전 뷰 일정 | 카메라 TF/K/검출 **무죄** (상수 바이어스만) |
| 코너 depth→odom | ~1.5 mm, 전 뷰 일정 | depth 정합 **무죄** |
| 태그 중심 위치 | ~1.4 mm, 전 뷰 일정 | 중심 계측 **무죄** |
| corner-mean vs 대각교점 | ≤0.2 px | 근사 오차 **무시 가능** (수정 불필요) |
| median-Z vs 평면교차-Z | ≤0.4 mm | 〃 |
| TF latest vs stamp (정지) | 0.0 mm | 정지 캡처면 **무죄** |
| **device pose dx** | **+0.8 ~ +25.4 mm, 뷰마다 다름** | **범인은 pose 합성 단계** |

태그 중심은 전 뷰에서 1.4 mm로 정확한데 device pose만 뷰마다 널뜀 → 오차는
**회전**에서 온다. 수치가 정확히 맞아떨어짐:

- dx ≈ θ × 1.2 m, dz ≈ θ × 0.35 m (태그(model z=1.2) → 디바이스 원점의 레버암)
- θ = **PnP의 in-plane 회전(태그 법선 축 스핀) 오차 0.1~0.4°**, 뷰포인트 의존
- depth 법선이 yaw는 잡아주지만(오차 ≤0.02°) in-plane 스핀은 PnP 몫이었음
- 30 mm 태그 하나의 PnP로는 이 축을 이보다 잘 잡을 수 없음 (구조적 한계)

## 해결 — depth-UPRIGHT pose 구성 (`vision/wirebonder_vision.py`)

디바이스는 어차피 수직 가정(시퀀스가 4-DOF (x,y,z,yaw)만 사용)이므로, **회전에서
PnP를 완전히 제거**:

```
yaw  = depth 평면 법선의 방위각 (태그의 model-frame 법선과의 차)
R    = Rz(yaw)                        ← 수직 가정으로 나머지 2 DOF 확정
pos  = depth-deproject된 태그 중심 − R @ TAG0_XYZ
```

`device_pose_in_base()`에서 depth 중심+법선이 모두 유효하고 법선이 base에서
수평(|n_z|<0.2)이면 이 경로가 우선. 아니면 기존 PnP 하이브리드로 폴백(불변).
PnP는 검출 이후 pose에 전혀 기여하지 않게 됨 → in-plane 스핀 오차가 원리적으로 0.

self-check 케이스 9 추가: 수직 디바이스 + 합성 depth + **엉터리 PnP 해**를 넣어도
참 pose 복원 (PnP 독립성 증명). `/usr/bin/python3 vision/wirebonder_vision.py`.

## 검증 (수정 후)

- diag 재실행, 뷰포인트 5곳: device pose 오차 **전 뷰 (-1.3, -0.5, +0.4) mm 부근으로
  일정**, cross-view spread **x 0.7 / y 0.2 / z 0.5 mm, yaw 0.02°**
  (수정 전 spread: x 24.7 / z 7.9 mm). 원거리 뷰 B와 사시 뷰 A_j5까지 일치.
- end-to-end(실제 노드 재시작 후 `/vision/device_pose`, view A):
  (2.3487, 0.4995, 0.0002, yaw 0.000), spread 0.2/0.2/1.1 mm.
- 잔여 상수 바이어스 ~1.4 mm: 태그 플레이트가 기둥 면에서 0.5 mm 돌출(SDF) + 검출
  ~1 px 바이어스로 추정. **전 뷰포인트에서 상수**이므로 anchor가 흡수 — 추적 불필요.
- `OLD_DEVICE_POSE` / `DEVICES['wb1']`를 새 read (2.3487, 0.4995, 0, 0)으로 재캡처
  완료 (파이프라인이 바뀌었으므로 필수 절차).

## 부수 결정 사항

- **corner-mean/median-Z 근사 교체 안 함**: 계측상 ≤0.2 px / ≤0.4 mm — 무죄 확정.
- **노드 TF를 image stamp로 조회하는 변경은 되돌림**: stamp 조회는 timer 콜백에서
  TF 수신을 기다리다 막히는 구조(단일 스레드) + `spin_thread=True`는 메인 spin과
  executor 충돌로 TF가 얼어붙음. 정지 캡처 설계에서 latest-vs-stamp 오차는 실측
  0.0 mm이므로 latest 유지. **단, 팔이 움직이는 중에는 절대 캡처하지 말 것**
  (`_lookup_T` docstring에 명시).
- 두-뷰 삼각측량 코드는 fallback으로 유지 (변경 없음).

## 실물(D405) 전환에 주는 의미

- 뷰포인트 의존이 사라졌으므로 "고정 캡처 포즈 + AGV 동일 주차" 강제는 더 이상
  정확도 조건이 아님 (CAPTURE_A_JOINTS 핀은 도달성/FOV 보장용으로만 유지).
- 실물에서 이 경로의 정확도는 **depth 품질**에 직결됨: 태그 ROI의 평면 피팅
  (법선→yaw)과 median Z(위치)가 전부다. D405 depth 노이즈/홀 대응은
  `real_robot_transition.md` C절의 `_depth_valid` 튜닝 + temporal 필터로.
- hand-eye cal(B절 ⭐)은 여전히 필수: sim에선 TF가 참이라 코너 오차 1.5 mm였지만
  실물 장착 오차는 그대로 pose 바이어스가 된다. cal 후 `OLD_DEVICE_POSE` 재캡처.
- 검증 절차도 이식 가능: 실물에는 GT가 없지만 diag의 **다중 뷰포인트 일관성**
  (팔 config 2~3곳에서 device pose 일치 확인)은 GT 없이 돌아간다 — 실물 브링업 때
  같은 스크립트에서 GT 비교만 빼고 사용.

## 남은 일

- [ ] 시퀀스 1/2/3 회귀 확인 (vision 모드로 한 사이클; anchor 재캡처가 맞으면
      waypoint는 SLOT_WORLD를 그대로 재현해야 함)
- [ ] (선택) anchor/SLOT_WORLD 구조 정리 — 이제 vision이 뷰포인트 불변이므로
      디바이스 프레임 직접 티칭(SLOT_LOCAL을 1차 데이터로)으로 단순화 가능.
      `wirebonder_refactor_plan.md`와 함께 진행.
