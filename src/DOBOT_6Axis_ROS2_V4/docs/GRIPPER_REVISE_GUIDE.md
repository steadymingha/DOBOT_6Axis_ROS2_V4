Measured vs current constants — both pads shifted +2cm in +X (the jaw direction), not just the fixed jaw:

┌──────────────────────┬────────┬────────┬─────────────┐
│        const         │  old   │  new   │      Δ      │
│                      │        │  mesh  │             │
├──────────────────────┼────────┼────────┼─────────────┤
│ JAW_FIXED_PAD_X      │ 0.1632 │ 0.1832 │ +0.020      │
│ (pad_fixed Xmin)     │        │        │             │
├──────────────────────┼────────┼────────┼─────────────┤
│ JAW_MOVING_PAD_X0    │ 0.0822 │ 0.1022 │ +0.020      │
│ (pad_moving Xmax)    │        │        │             │
├──────────────────────┼────────┼────────┼─────────────┤
│ JAW_GAP_AT_ZERO      │ 0.081  │ 0.081  │ 0 (gap      │
│                      │        │        │ unchanged)  │
├──────────────────────┼────────┼────────┼─────────────┤
│ GRASP_LATERAL_M      │ 0.1197 │ 0.1397 │ +0.020      │
└──────────────────────┴────────┴────────┴─────────────┘

So the real source isn't a wirebonder variable — it's the two mesh-measured constants in geometry.py. Fixing them recomputes GRASP_LATERAL_M, which is applied through FK in the jaw direction, so every computed grasp (base pockets + the front-load slot grasp) self-corrects. Gap is unchanged, so GRIPPER_CLOSE is untouched.

좌표계는 gripper_base_link 기준, +X = 고정 jaw가 뻗어나가는 방향(tool 축에서 옆으로). 박스는 두 pad 사이에 끼워.

        tool 축(X=0)
          |
          |   [moving pad]......[fixed pad]
          |        ^                 ^
          |   X=0.1022          X=0.1832
          |        |<--- 박스(81mm)--->|
          |        |<------ gap 81mm ->|
          +X →  (jaw가 뻗는 방향)

1. JAW_FIXED_PAD_X = 0.1832
고정 pad의 안쪽 면 X 위치. 박스가 닿아 멈추는 기준면. (mesh pad_fixed.stl의 Xmin = 가장 안쪽). 박스 한쪽 벽.

2. JAW_MOVING_PAD_X0 = 0.1022
움직이는 pad의 안쪽 면 X 위치, finger_joint = 0일 때. (mesh pad_moving.stl의 Xmax). 박스 반대쪽 벽. X0의 0은 "관절값 0에서"라는 뜻 — 관절 움직이면 이 면이 이동.

3. JAW_GAP_AT_ZERO = 0.1832 − 0.1022 = 0.081
두 pad 사이 간격, 관절 0일 때 = 81mm. 박스 짧은 변(BOX_SHORT=0.081)과 똑같게 맞춰놔서, finger_joint=0이면 정확히 박스 폭으로 닫혀. 그래서 GRIPPER_CLOSE가 q=0이 됨. (계산값, 직접 안 적음)

4. GRASP_LATERAL_M = 0.1832 − 0.003 − 0.0405 = 0.1397
박스 중심이 tool 축(X=0)에서 옆으로 얼마나 떨어지는지 = 140mm.
- JAW_FIXED_PAD_X(고정 벽) 0.1832
- − FIXED_PAD_CLEARANCE(0.003): 내려갈 때 박스랑 고정 pad 사이 살짝 띄우는 여유
- − BOX_SHORT/2(0.0405): 박스 벽에서 중심까지

→ jaw가 길어서 박스가 flange 바로 밑이 아니라 옆으로 14cm 떨어진 곳에 잡힘. 그래서 grasp 코드가 flange를 박스 중심에서 이만큼 비켜 세움(grasp_tcp_pose의 nominal − GRASP_LATERAL_M·jaw_x).