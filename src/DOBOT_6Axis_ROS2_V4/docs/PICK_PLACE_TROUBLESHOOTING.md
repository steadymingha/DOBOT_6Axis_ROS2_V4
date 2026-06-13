# Pick & Place 트러블슈팅 기록 (선반 → 베이스 매거진)

2026-06-12 ~ 06-13, `cbirrt_pick_place.py` 셀프-잼(Blender fixed-jaw 그리퍼) 픽앤플레이스를
처음으로 끝까지 성공시키기까지 발생한 문제와 해결책. 같은 증상이 재발하거나
그리퍼/박스 사양이 바뀔 때 참고.

## 핵심 측정값 (메시 실측, 코드/문서의 근거)

`src/blender/gripper/meshes/{base,finger}.dae`를 직접 파싱해 얻은 값.
gripper_base_link 프레임 기준 (플랜지면 z=0.1401, Link6에 π 뒤집혀 장착, 고정 jaw가 +X쪽):

| 항목 | 값 |
|---|---|
| 고정 패드 안쪽 면 | x = +0.0894 |
| 이동 패드 안쪽 면 (joint q=0) | x = +0.0161 |
| 패드 gap | **gap = 0.0733 + q** (axis -X, q>0 = 열림, 한계 [-0.07, +0.10]) |
| 그립 중심 | x = +0.0528 → **플랜지/툴 z축에서 53 mm 오프셋** |
| 그리퍼 최저점 | 플랜지 아래 85 mm (패드 하단은 82 mm) |
| TCP 관례 (`TCP_OFFSET_M=0.12005`) | OnRobot 2FG7 값 — 이 그리퍼 패드 하단보다 38 mm **아래** 허공 |
| 박스 (box_l1a) | 0.236 × 0.081 × 0.14 m, 0.3 kg, world yaw 90°(짧은 변이 row 방향) |
| wrap 깊이 공식 | wrap = 0.032 − GRASP_TCP_ABOVE (패드 하단이 박스 윗면 아래로 내려가는 깊이) |

---

## 문제 1. 비주얼은 안 닿았는데 descend에서 박스가 튕김

**증상**: 그리퍼를 열고 조금만 내려가면 상자가 튕겨나감. 화면상 접촉 없음.

**원인 (2단계로 발견됨)**:
1. 고정 jaw는 C자형(상부 빔 X[0.009,0.107]·Z[0.105,0.115] + 외측 기둥 X[0.0944,0.1071]·Z[0.055,0.105])
   인데, 충돌박스가 **C자 전체의 union AABB 한 덩어리**로 돼 있어서 패드 사이
   그립 공간이 충돌상 솔리드로 꽉 차 있었음. Gazebo 물리는 충돌지오메트리만 보므로
   "눈에 보이는 빈 공간"으로 내려가는 순간 충돌.
2. 그 이전 버전은 반대로 비주얼에 없는 팬텀 jaw(z=-0.116까지)가 충돌에 있었음.

**해결**: `cr7_on_mpo700.urdf.xacro`의 gripper_base_link 충돌을 빔 + 기둥(패드 포함)
2개 박스로 분리 → 그립 공간 개방. 빔 하단은 플랜지 아래 35 mm이므로 최대 wrap ≈ 55 mm.

**교훈/판별법**: "안 닿았는데 튕김"류는 전부 Gazebo **View → Collisions**를 켜서
주황색 충돌체와 비주얼을 비교하면 즉시 판별됨. 충돌박스를 메시 AABB로 만들 때
오목한(C/L자) 형상은 반드시 분해할 것.

## 문제 2. 그립 중심이 플랜지 축에 없음 (fixed-jaw 정렬 불가)

**증상**: 고정 jaw를 박스에 붙이는 좌표 튜닝이 불가능에 가까움.

**원인**: 패드 사이 그립 중심이 플랜지 축에서 +X로 53 mm 오프셋인데, IK 타깃은
박스 중심을 플랜지 축(TCP) 위에 둠 → 박스가 패드 사이에 비대칭으로 들어가고
고정 jaw 쪽 구조물이 박스 영역을 침범.

**해결**: J6 트위스트 **후에** step 2c "jaw-align" 추가 — 실제 jaw +X축을 TF로 읽어
`-(0.0894 − 0.0405 − FIXED_PAD_CLEARANCE) × jaw_x` 만큼 수평 서보 이동.
트위스트 후 실측 축을 쓰므로 `rotate_j6`가 충돌 회피로 회전 방향을 뒤집어도 자동 보정.

## 문제 3. GRIPPER_OPEN/CLOSE 값이 현재 URDF와 불일치

**증상**: "close = -0.036 (light grip)" 이었으나 실제로는 gap 37 mm = 81 mm 박스를
44 mm 관통하는 값.

**원인**: 그리퍼 정의가 세 군데 복붙돼 있고 서로 어긋남. `cr7_robot.xacro`(구버전:
axis +X, mount 0.1443, 팬텀 충돌) 기준의 상수가 `cr7_on_mpo700.urdf.xacro`(현행:
axis -X, mount 0.1401) 기준으로 재보정되지 않음.

**해결**: gap 공식(gap = 0.0733 + q)으로 유도하도록 변경 —
`GRIPPER_CLOSE = BOX_SHORT − JAW_GAP_AT_ZERO − CLOSE_SQUEEZE` (= +0.0057).
근본 대책은 TODO.md "Unify the triplicated gripper URDF definition" 참고.

## 문제 4. 상수 커플링 — 하나 바꾸면 다른 것이 따라 바뀜

**증상 A**: pregrasp에 넣은 디버그 z 오프셋이 grasp 깊이까지 전파 (descend가 상대이동이라).
**증상 B**: grasp 깊이를 줄이려 GRASP_TCP_ABOVE를 올렸더니 **삽입 주행 높이가 같이
올라가** 250 mm 삽입의 246 mm 지점에서 특이점(sigma_min=0.0001) 발생 — 선반 진입
corridor는 팔 리치 한계라 10 mm에도 민감.

**해결**: 좌표가 아닌 "정도" 파라미터로 분리, 파생값은 코드에서 유도.
- `GRASP_TCP_ABOVE` — grasp 깊이만 결정 (wrap = 0.032 − 이 값)
- `INSERT_TCP_ABOVE = 0.105` — 삽입 corridor 높이만 결정 (**박스 기준 절대값**, 검증된 sweet spot)
- descend/ascend 거리 = `INSERT_TCP_ABOVE − GRASP_TCP_ABOVE` (자동)
- `FIXED_PAD_CLEARANCE` — 고정 패드-박스 간격이자 close 시 밀림량
- `CLOSE_SQUEEZE` — 쥐는 압력

## 문제 5. 운반 중 박스 낙하 = attach가 사실 한 번도 성공한 적 없음

**증상**: 들어 올리는 것까지 되고 운반 전/중 낙하.

**원인 (2단계)**:
1. 사이클이 `attach_box()` 반환값을 무시 → 실패해도 진행. gap이 정확히 박스 폭이라
   쥐는 힘 0 → 마찰만으로 잠깐 버티다 가감속에서 빠짐.
2. 게이트를 넣자 진짜 원인이 로그에 노출: `Attach: Failed to find link with name:
   gripper_base_link`. **URDF의 fixed joint는 SDF 변환 때 부모 링크로 병합(lumping)**
   되므로 Gazebo 모델에는 gripper_base_link라는 링크가 존재하지 않음.

**해결**:
- attach 실패 시 그리퍼 열고 사이클 중단 (조용한 실패 금지).
- attach 대상을 `Link6`로 변경 (revolute joint6의 자식이라 병합 안 됨, 그리퍼와 강체).
- 대안: `<gazebo reference="gripper_attach_joint"><preserveFixedJoint>true</preserveFixedJoint></gazebo>`

**교훈**: link attacher가 못 찾는 링크는 대부분 fixed-joint lumping이 원인.
`gz model -m cr7_on_mpo700 --list`(또는 gz topic)로 실제 링크 이름을 확인할 것.

## 문제 6. close(스퀴즈)에서 박스 사출

**증상**: 2 mm 스퀴즈로 닫는 순간 박스가 튕겨나감.

**원인**: 이동 패드(gripper_finger_link)에는 접촉 연화(kp 1e6, minDepth, maxVel 0.1)가
있었지만 **고정 jaw(gripper_base_link)는 Gazebo 기본값(kp=1e12, maxVel=100)** —
박스가 고정 패드에 1 mm 파고드는 순간 초강성 접촉이 거대한 보정 임펄스를 발사.

**해결**:
- gripper_base_link `<gazebo>` 태그에 핑거와 동일한 mu1/mu2=1.2, kp=1e6, kd=1,
  minDepth=0.001, maxVel=0.1 추가 (스폰 시점에 박히므로 Gazebo 재시작 필요).
- `GRIPPER_OPEN` 0.07 → 0.03: 고정 2초 close 트래젝토리에서 패드 충돌 속도 32 → 12 mm/s.

**관련 지식**: `minDepth`는 그 깊이까지의 침투에 보정력을 안 만드는 데드존.
→ **스퀴즈는 minDepth(1 mm)를 넘어야 실제 그립력이 생긴다**. gap=박스폭(스퀴즈 0)은
그립력 0이며, 0.5 mm 스퀴즈도 데드존 안이라 못 잡는다. 마찰 자체는 0.3 kg 박스에
~1.2 N이면 충분하므로(패드 μ≈1.2) 2 mm면 충분하고 과하지 않음.

## 문제 7. move_group 에러 스팸 ("Joint 'gripper_finger_joint' not found")

**증상**: 실행 내내 같은 에러가 /joint_states 주기(≈30 Hz)로 도배. 기능엔 무영향.

**원인**: move_group의 로봇 모델은 `cr7_moveit/config/cr7_robot.urdf.xacro` →
`dobot_rviz/urdf/cr7_robot.urdf`(그리퍼 없는 순정 팔)로 만들어지는데, Gazebo의
joint_state_broadcaster는 gripper_finger_joint를 포함해 발행 → 상태 모니터가
매 메시지마다 미지의 조인트 에러.

**해결**: cr7_moveit URDF에 그리퍼 링크/조인트 추가(기하는 cr7_on_mpo700과 동일,
gazebo 태그 제외) + SRDF에 `<passive_joint name="gripper_finger_joint"/>`.
SRDF에는 그리퍼 disable_collisions가 이미 있었음(반쯤 하다 만 흔적).

## 일반 교훈

1. **로봇 기술(description)의 사본이 4개** 돌아다닌다: Gazebo 스폰(cr7_on_mpo700),
   move_group(cr7_moveit), IK/서보(cr7_robot.xacro), 플래닝 충돌(cr7_on_mpo700).
   이들이 어긋나면 "모델은 된다는데 시뮬은 안 되는" 류의 버그가 됨. 그리퍼 매크로
   통일 과제가 TODO.md Follow-up에 있음.
2. install이 **colcon symlink-install**이라 xacro/py 수정은 빌드 불필요.
   단, **Gazebo 스폰 모델(충돌·접촉 파라미터)은 재시작해야 반영**.
3. 튜닝 파라미터는 "좌표"가 아니라 물리적 의미가 있는 "정도"로 정의하고
   나머지는 유도할 것 (문제 4).
4. 서비스/액션 반환값을 버리지 말 것 — attach 실패 무시가 "운반 중 낙하"라는
   엉뚱한 증상으로 둔갑했음 (문제 5).

## 미해결 (다음 작업)

- [x] **close 시 미세 진동·박스 살짝 이탈** (06-13 해결): CLOSE_SQUEEZE 2 → 0 mm.
      스퀴즈 접촉 진동이 원인이었음. attach(Link6 강체 조인트)가 잡아주므로 마찰
      그립이 필요 없고, gap=박스 폭으로 닿기만 하는 게 가장 부드러움. 단 attach가
      실패하는 환경에서는 스퀴즈(>minDepth 1 mm)가 다시 필요해짐 — 문제 6 참고.
- [ ] **carry 시작 시 CBiRRT 정체**: 목표 IK 분기가 관절공간에서 ~5 rad 멀어
      (J6 와인드업 + J1 스윙) 제약 플래닝이 느림/실패. 06-13 해결 시도: 7a/7b
      단일관절 정렬(J6→J1, 둘 다 "아래 향함"을 정확히 보존) 후 7c CBiRRT는
      J2~J5 잔여 갭만. 검증 대기.
- [x] **carry(step 7)가 자세 비유지 RRT** (06-13 해결): `move_to_pose`(자유 RRT) 대신
      `move_constrained`(tilt 고정·yaw 자유 CBiRRT)로 교체 — 적재 상태에서는 그리퍼가
      항상 아래를 향함. TODO.md의 "delete move_constrained" 항목은 KEEP으로 정정됨.
- [ ] 그리퍼 URDF 3중 복붙 통일 (TODO.md Follow-up 참조).
