# 하강 경로 단독 점검 — `tools/check_descend.py`

2026-08-17. 하강(descend) 동작만 본 시퀀스에서 떼어내 검사하는 도구. 원래
`~/dobot_ws/check_descend.py`에 있던 일회용 스크립트였으나, **Part 1이
`JOINT_SIGN_REAL` 회귀 검출기 역할**을 하게 되어 `tools/`로 승격했다.

**먼저 읽을 것**: `real_robot_joint_convention.md` — URDF와 컨트롤러의 관절 부호가
다르고, 그게 어긋나면 팔이 거울 반전된 자세로 간다. 이 도구는 그 어긋남을 **팔이 튀기
전에** 잡는 것이 주 목적이다.

관련 파일:

| 경로 | 역할 |
|---|---|
| `tools/check_descend.py` | 본체 (껍데기) |
| `test/cbirrt_p1p2_test.py` | 실제 로직 — `RealtimeMonitor`, `Dashboard`, `ContactDetector`, `guarded_descend`, `joint_gap_deg` |
| `cr7_pnp/node.py` | `JOINT_SIGN_REAL`, `joint_sign` |

---

## 1. 실행 모드

```bash
# 컨테이너 안에서
/root/dobot_ws/.venv/bin/python3 tools/check_descend.py            # 로봇 안 움직임
/root/dobot_ws/.venv/bin/python3 tools/check_descend.py --move     # 30 mm 실제 하강
/root/dobot_ws/.venv/bin/python3 tools/check_descend.py --mm 10 --move
/root/dobot_ws/.venv/bin/python3 tools/check_descend.py --profile  # 20 mm, 토크 로깅
```

| 모드 | 로봇 | 확인 대상 |
|---|---|---|
| (플래그 없음) | 안 움직임 | 관절각 변환 · 경로 확보 · 첫 명령 크기 · 안전게이트 작동 |
| `--move` | 30 mm 하강 | 접촉 감지가 실제로 멈추는지 |
| `--profile` | 20 mm 하강 | 토크 실측 (임계값 재산출용) |

`--move`는 로봇이 `mode=5`(ENABLE, 대기)이고 enable이 켜져 있어야 진행한다. 아니면
상태만 찍고 아무것도 하지 않는다.

---

## 2. Part 1이 확인하는 4가지 (명령 전송 없음)

### ① 관절각 변환

```
robot q_actual (deg, controller) : [274.9, -68.5, ...]
node.current_joints (deg, URDF)  : [-274.9, -68.5, ...]
```

컨트롤러 각도와 모델 각도를 나란히 출력. `JOINT_SIGN_REAL = [-1,+1,+1,+1,-1,-1]`이
적용된 모습이 보여야 한다.

### ② 하강 경로 확보

```
linear_path: 30.0 mm of 30 mm, <reason>
```

아래로 30 mm 직선이 충돌/관절한계 없이 뚫리는지. 못 가면 몇 mm에서 왜 막혔는지.

### ③ 첫 ServoJ 틱의 gap ← 핵심

```
first ServoJ target would be (deg, controller) : [...]
gap from where the arm actually is : 0.012 deg   OK
```

첫 명령의 목표 각도가 **지금 팔 위치에서 몇 도 떨어져 있는지**. 3 mm/s × 8 ms이므로
거의 0이어야 정상. `1.0°` 이상이면 `<<< WRONG, do not run --move`가 뜨며, 그 상태로
`--move`를 하면 팔이 튄다. 부호·프레임 어긋남이 여기서 즉시 드러난다.

### ④ 안전 게이트 미러 테스트

```
gate test, deliberately mirrored target : 340.2 deg   gate would ABORT (good)
```

목표 각도에 일부러 `-1`을 곱해 게이트에 먹인다. 5° 초과를 감지해 "중단시킬 것"이라고
나와야 통과. 안전장치는 평소엔 통과만 시키므로, 일부러 틀린 값을 넣어보지 않으면
고장 나 있어도 모른다.

---

## 3. 언제 다시 돌리나

Part 1(플래그 없음)은 30초, 로봇을 안 움직인다. 아래 상황에서는 그냥 돌리는 게 싸다.

- **URDF / xacro 수정** — 프레임이나 관절 정의가 바뀌면 ③에 바로 뜬다
- **`cr7_pnp/node.py`의 `joint_sign` 관련 코드 수정**
- **그리퍼 교체, TCP 오프셋 변경** (`gripper_change_checklist.md` 이후)
- **컨트롤러 펌웨어 업데이트, 로봇 본체 교체** — 관절 컨벤션이 그대로인지
- **한동안 실물을 안 쓰다가 다시 시작할 때** — `--run` 전 워밍업

`--profile`은 **파지 대상 무게나 하강 속도가 바뀔 때만**. 그 외에는 이미 임무가 끝났다.

`tools/preflight_check.py`와 헷갈리지 말 것 — 그쪽은 시퀀스/기하 회귀(팔 안 움직임,
sim 기준)를 보고, 관절 부호 변환과 첫 명령 크기는 보지 않는다. 겹치지 않는다.

---

## 4. 알아둘 것

**하강 거리가 30 mm인 이유.** 소프트 접촉 감지는 출발 후 **1.5초 동안 눈을 감는다**
(정지→운동 전환 시 나오는 토크 튐을 무시하려고). 3 mm/s면 그게 4.5 mm다. 5 mm만
내려가면 감지가 켜지기도 전에 끝나서 아무것도 검증되지 않는다. 그래서 기본값이
30 mm다. (`--mm N`으로 바꿀 수 있으나, 5 mm 부근은 의미 없음을 알고 쓸 것.)

**`--profile`이 왜 있었나.** 첫 시도에서 출발 0.2초 만에 J3가 7~8 N·m로 튀며 정지했는데,
그건 접촉이 아니라 정지→운동 전환의 정상 토크였다. 임계값을 감으로 찍지 않고 실측해서
정하려고 만든 모드다. 소프트 채널만 끄고 컨트롤러 자체 충돌감지는 켠 채로 내려가며,
0.35초 전 기준선 대비 관절별 토크 변화를 로그로 찍는다.

**`--move`는 사실상 잉여.** 본 시퀀스가 같은 `guarded_descend`를 어차피 호출한다.
남겨둔 건 하강만 격리해서 보고 싶을 때를 위한 것.

---

## 5. 이력

- 2026-08-06 작성. docstring에 "Delete this file once the descend is trusted" —
  하강 로직이 못 미덥던 시점의 임시 도구였다.
- 2026-08-17 `tools/`로 이동. 용도가 "하강 신뢰 확보"에서 "관절 컨벤션 회귀 검출"로
  바뀌었으므로 삭제 지시 문구 제거. docstring과 안내문의 `5 mm`가 실제 `DROP = 0.030`
  (30 mm)과 어긋나 있던 것도 함께 수정.
