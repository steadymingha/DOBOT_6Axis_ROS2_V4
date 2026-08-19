#!/usr/bin/env python3
"""비전으로 매거진을 잡아 그 상방 호버 자세 q 를 만든다. 로봇을 움직이지 않는다.

cbirrt_p1p2_test.py --vision 이 쓰는 계산 전담 모듈. 여기서 하는 일은 넷뿐이다:

    러너 MAGAZINE 스냅샷  ->  xyz_cam
    tool_vector           ->  P_base            (vision_bridge/transform.py 그대로)
    P_base                ->  hover (base_link)  매거진 상면 중심 + 여유
    hover                 ->  q (플랜지 원점 = hover, 툴 수직 아래)

★ 로봇에 명령을 보내지 않는다. 30004 는 읽기 전용, 러너에는 set_mode 만 보낸다.
  이동은 전부 호출부(cbirrt_p1p2_test.py)의 approach()/execute_path() 가 한다.

프레임 (docs/real_robot_joint_convention.md 9.1)
    hover 는 base_link 기준이다 -- 로봇 펜던트의 GetPose(user=0, tool=0) 와 같은 프레임.
    pinocchio 모델의 루트는 base_link 보다 30 mm 아래에 있으므로, IK 에 넣기 전에
    base_link -> 모델 루트로 되돌려야 한다. 이 환산을 빠뜨리면 30 mm 계통 오차가
    조용히 들어간다. base_placement() 가 그 하나의 SE3 이고, 왕복 항등은
    selftest() 에서 검사한다.

플랜지 기준이다
    hover 는 **플랜지 원점**의 위치다. TCP_OFFSET_M(팬텀 그리퍼 120 mm)은 실물에
    달려 있지 않으므로 목표 기준으로 쓰지 않는다 -- solve_hover_q() 는 그 오프셋을
    IK 입력에서 더해 넣어 모델 내부에서 정확히 상쇄시킨다.
    충돌 모델의 팬텀 그리퍼는 hover 아래로 40 mm 쯤 드리우지만, 매거진은 어느 충돌
    모델에도 없으므로 계획에는 영향이 없다. 물리적으로도 플랜지에는 카메라 홀더밖에
    없어 100 mm 여유 안에서 닿는 것이 없다.

자가시험 (로봇도 러너도 필요 없다):
    /root/dobot_ws/.venv/bin/python3 test/vision_target.py
"""

import json
import math
import os
import subprocess
import sys
import time

import numpy as np
import pinocchio as pin

_PKG_ROOT = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
BRIDGE_DIR = os.path.join(_PKG_ROOT, 'vision_bridge')
sys.path.insert(0, _PKG_ROOT)
sys.path.insert(0, BRIDGE_DIR)

try:
    import cv2                                          # noqa: F401
except Exception as _cv2_exc:
    # 이 경로는 pinocchio 가 있는 .venv 로 돌아야 하는데, 그 venv 의 numpy 는 2.x 이고
    # 컨테이너의 cv2(4.5.4)는 numpy 1.x 로 빌드돼 있어 import 가 깨진다.
    # 우리가 쓰는 것은 handeye_calib 의 tool_to_T / STILL_DPS 뿐이고 둘 다 순수 numpy
    # 인데, 그 모듈이 import 시점에 cv2 상수(METHODS)를 참조한다. 남의 모듈을 고치지
    # 않고 지나가려고 더미를 끼운다. 실제로 cv2 를 호출하는 코드가 이 프로세스에
    # 들어오면 MagicMock 이 numpy 연산에서 곧바로 터진다 -- 조용히 틀리지는 않는다.
    import unittest.mock as _mock                       # noqa: E402
    sys.modules['cv2'] = _mock.MagicMock(name='cv2-stub')
    print("[vision_target] cv2 를 더미로 대체 (%s: %s) -- tool_to_T 만 쓰므로 무해"
          % (type(_cv2_exc).__name__, _cv2_exc), file=sys.stderr)

# vision_bridge 는 그대로 import 해서 쓴다 -- 게이트도 곱 순서도 여기서 다시 만들지
# 않는다 (docs/VISION_CHAIN_VERIFICATION.md 에서 p95 1.37 mm 로 검증된 구현).
import config as vcfg                                   # noqa: E402
from gate import Gate                                   # noqa: E402
from transform import Transform                         # noqa: E402
from vision_client import VisionClient                  # noqa: E402

sys.path.insert(0, vcfg.HANDEYE_DIR)
from handeye_calib import STILL_DPS                     # noqa: E402

from cr7_pnp.geometry import TCP_OFFSET_M, XACRO_PATH   # noqa: E402


# --------------------------------------------------------------------------
# 상수 -- 실측으로 채우는 자리
# --------------------------------------------------------------------------

# 비전 점은 매거진 **전면의 중심**이다. 상면까지 올라가려면 높이의 절반이 필요하다.
MAGAZINE_HEIGHT_M = 0.14

# 상면 위로 띄울 높이 -- **플랜지 원점** 기준이다.
#   사람 지정: 100 mm, 그리고 **80 mm 아래로는 내려가지 말 것**(하한).
#   더미 그리퍼의 조가 플랜지 아래로 70 mm 내려오므로, 100 mm 면 조 끝이 상면에서
#   30 mm 뜬다. 선반·매거진을 로봇 쪽으로 당긴 뒤로는 도달 여유도 충분하다
#   (이 자리 최대 도달 z=0.601 = 플랜지 여유 151 mm).
HOVER_CLEARANCE_M = 0.08

JAW_BELOW_FLANGE_M = 0.07

# 전면 중심의 연직선상이 아니라 매거진 **몸통 중심** 위에 서려면, 전면 법선의
# 안쪽으로 깊이의 절반만큼 들어가야 한다.
#   2026-08-11 사람 실측: 상자 길이 236 mm -> 절반 118 mm.
MAGAZINE_INWARD_M = 0.118
# 그 안쪽 방향이 base_link 의 어느 축에 가까운지. 사람이 지정한다 -- 자동 추정 금지.
#   이번 배치: 매거진 전면이 로봇(+y 쪽)을 보고 있고 카메라가 -y 방향으로 본다
#   (obs 플랜지 y=-0.209, 전면 중심 y=-0.53). 따라서 "상자 안쪽" = -y.
INWARD_AXIS = "-y"

# 더미 그리퍼의 조 배치 보정. 플랜지 중심에서 **카메라 방향으로 63 mm 에 고정 조**,
# 반대 방향 27 mm 에 이동 조(더미라 움직이지 않는다)가 있다. 그래서 파지 중심은
# 플랜지 원점이 아니고, 그만큼 목표를 옮겨야 조가 매거진 위에 온다.
#   ★ 값과 방향은 사람이 지정한 것이다 (2026-08-11: base -x 로 25 mm). 여기서
#     조 치수로 다시 유도하지 않는다 -- 실물 배치를 본 사람의 값이 우선이다.
#   ★ 시뮬레이션 쪽 대응 위치는 cr7_pnp/gripper_params.py 지만, 실물에 그리퍼가
#     아직 없으므로 그 파일은 건드리지 않는다. 실물 그리퍼가 오면 그쪽으로 옮길 것.
JAW_OFFSET_M = 0.025
JAW_OFFSET_AXIS = "-x"

# hover 가 여기 안에 없으면 실행하지 않는다 (base_link, m).
#   ★ 이것은 "매거진 한 대의 자리"가 아니라 **선반의 작업 영역**이다. 선반에는
#     매거진이 여러 개 놓이고 하나씩 집어야 하므로, 한 대의 실측 위치 둘레로
#     좁히면 옆자리 매거진이 전부 거부된다.
#   ★ 탐지값 기준으로 잡으면 안 된다 -- 그러면 자기 자신을 기준으로 자기를 검사하는
#     꼴이라 아무것도 못 막는다. 이 값이 담는 정보는 "매거진이 이 방 어디에 있어야
#     하는가"이고, 그건 사람만 안다.
#   2026-08-11 잠정값: 오늘 상자를 옮겨가며 관측된 hover 가
#     x -0.14~-0.19, y -0.51~-0.65, z 0.53~0.55 였다. 선반 전체를 덮도록 넉넉히
#     잡아 둔다. 배치가 굳으면 실측 station 크기(정차 오차 ±2cm 수준)로 좁힐 것.
#   이 범위여도 hand-eye 틀어짐·프레임 회귀·엉뚱한 물체 같은 수십 cm 급 오류는 잡고,
#   IK·이동량 한도·게이트는 그대로 살아 있다.
VISION_WORK_BOX = {
    "x": (-0.40, 0.05),
    "y": (-0.70, -0.30),
    "z": (0.40, 0.65),
}

# ── 광각 관측(coarse) -> 근접 관측(fine) ──────────────────────────
# 한 자리에서 선반 전체를 보면서 mm 정확도를 내는 것은 불가능하다. 실측(2026-08-12,
# 매거진 3개가 한 화면):
#     0.36~0.43 m 에서 탐지는 score 0.84~0.97 로 멀쩡한데, sd 는 0.28~0.30 (한도
#     0.25) 이고 z 는 게이트 상한 0.30 밖 -> 셋 다 채택 불가.
# 그래서 두 단계로 나눈다:
#   coarse : 지금 자리에서 **게이트 없이** 후보를 읽는다. 조준용이라 ±2 cm 면 족하다
#   fine   : 고른 상자 전면 앞 VIEW_DIST_M 로 카메라를 옮겨, 거기서 게이트 통과
#            스냅샷을 받는다. hover/하강은 **오직 이 값으로만** 만든다
# ★ coarse 좌표로는 절대 하강하지 않는다. 조준에만 쓴다.
VIEW_DIST_M = 0.20          # 정밀 관측 시 카메라 원점 ~ 매거진 전면 거리
COARSE_Z_MIN = 0.15         # 조준용 검출로 인정할 거리 범위 (게이트보다 넓다)
COARSE_Z_MAX = 0.80
TARGET_RULE = "nearest"     # nearest | left | right | id=N   (--target 로 덮어씀)

# 현재 플랜지에서 hover 까지 이 이상 떨어져 있으면 비전이 엉뚱한 것을 봤다고 본다.
#   0.40 -> 0.50 (2026-08-11). 안쪽 118 mm 가 들어가면서 obs(플랜지 y=-0.105)에서
#   목표(y=-0.51)까지 정상적으로 425 mm 가 나온다. 울타리가 정상 동작을 막으면
#   울타리가 틀린 것이다 -- 다만 늘린 만큼 늦게 잡는다는 것도 사실이다.
MAX_MOVE_M = 0.50

# J6 정렬 회전 기본값 (도). CLI --j6-deg 로 덮어쓴다.
J6_ROT_DEG = 90.0

# 스냅샷 하나(게이트 통과 10프레임)를 기다리는 한도.
SNAPSHOT_TIMEOUT_S = 30.0

# 모델 FK(플랜지, base_link)와 로봇 자신의 tool_vector 가 이보다 벌어지면 프레임이나
# 부호 규약이 어긋난 것이다. 실측 잔차는 1.4 mm 수준 (joint_convention 6).
FK_CHECK_TOL_M = 0.010

AXIS_VEC = {'x': np.array([1.0, 0.0, 0.0]),
            'y': np.array([0.0, 1.0, 0.0]),
            'z': np.array([0.0, 0.0, 1.0])}


class VisionTargetError(RuntimeError):
    """이동해서는 안 되는 상태. info 에 그때까지 계산된 값이 전부 들어 있다."""

    def __init__(self, msg, info=None):
        super().__init__(msg)
        self.info = info or {}


# --------------------------------------------------------------------------
# 프레임
# --------------------------------------------------------------------------

def base_placement(node):
    """모델 루트 -> base_link 의 고정 배치 (SE3).

    base_link 는 모든 관절보다 위(루트 쪽)에 있으므로 관절각과 무관하다. 그래서
    q=0 으로 한 번 FK 를 돌려 얻은 값이 항상 옳다.

    cbirrt_p1p2_test.flange_in_base() 가 하는 환산이 이 SE3 의 actInv() 이고,
    IK 로 넣을 때 필요한 것은 그 역방향인 act() 다.
    """
    m = node.ik_model
    pin.forwardKinematics(m.model, m.data, m.pin_q([0.0] * 6))
    pin.updateFramePlacements(m.model, m.data)
    return m.data.oMf[m.model.getFrameId('base_link')].copy()


def base_to_model(node, p_base):
    """base_link 점 -> 모델 루트 점. flange_in_base() 의 역방향."""
    return base_placement(node).act(np.asarray(p_base, dtype=float))


def model_to_base(node, p_model):
    """모델 루트 점 -> base_link 점."""
    return base_placement(node).actInv(np.asarray(p_model, dtype=float))


def flange_pose_in_base(node, q):
    """관절각 q 의 플랜지 자세 (위치, 회전) in base_link.

    위치는 cbirrt_p1p2_test.flange_in_base() 와 같은 값이고, 자세를 수직으로 스냅
    하려면 회전도 같은 프레임에서 필요해서 SE3 째로 환산한다.
    """
    # base_placement() 는 q=0 으로 FK 를 다시 돌려 m.data 를 덮어쓴다. 반드시 이
    # 관절각의 FK **전에** 받아둘 것 -- 순서를 바꾸면 q=0 자세의 플랜지를 읽는다.
    base = base_placement(node)
    m = node.ik_model
    pin.forwardKinematics(m.model, m.data, m.pin_q(list(q)))
    pin.updateFramePlacements(m.model, m.data)
    T = base.actInv(m.data.oMf[m.frame_id])
    return T.translation.copy(), T.rotation.copy()


def straight_down(R):
    """툴 축(z)을 정확히 연직 아래로 보내는 **최소 회전**을 R 에 적용한다.

    --level(level_config)이 쓰는 것과 같은 보정이다: 툴 축 둘레 회전(yaw)은 건드리지
    않으므로 J6 가 튀지 않는다. 이미 수직이면 그대로 돌려준다.
    """
    tz = np.asarray(R)[:, 2]
    down = np.array([0.0, 0.0, -1.0])
    v = np.cross(tz, down)
    s = float(np.linalg.norm(v))
    c = float(np.dot(tz, down))
    if s < 1e-9:
        if c > 0:
            return np.array(R, dtype=float)
        raise VisionTargetError(
            "툴이 정확히 위를 향하고 있다 -- 최소 회전이 정의되지 않는다. "
            "관측 자세 obs 를 툴이 아래를 향하도록 다시 교시할 것")
    vx = np.array([[0.0, -v[2], v[1]], [v[2], 0.0, -v[0]], [-v[1], v[0], 0.0]])
    return (np.eye(3) + vx + vx @ vx * ((1.0 - c) / s ** 2)) @ np.asarray(R)


def tilt_from_down(R):
    """툴 축이 연직 아래에서 몇 도 벗어나 있는지."""
    c = float(np.dot(np.asarray(R)[:, 2], [0.0, 0.0, -1.0]))
    return math.degrees(math.acos(max(-1.0, min(1.0, c))))


# --------------------------------------------------------------------------
# IK
# --------------------------------------------------------------------------

def solve_flange_q(node, p_base, R_base, q_seed, pos_tol=2e-4,
                   rot_tol=math.radians(0.05), what="목표"):
    """플랜지 원점 = p_base, 플랜지 회전 = R_base (둘 다 base_link) 인 q 를 푼다.

    solve_hover_q() 와 viewpoint_q() 가 공유하는 알맹이. IK 는 목표를 TCP 로 받아
    내부에서 TCP_OFFSET_M 을 다시 빼므로, 미리 더해 넣어 정확히 상쇄시킨다 --
    팬텀 그리퍼 길이가 목표에 섞이지 않는다.
    """
    base = base_placement(node)
    R_model = base.rotation @ np.asarray(R_base, dtype=float)
    p_model = base.act(np.asarray(p_base, dtype=float))

    m = node.ik_model
    tcp_target = p_model + TCP_OFFSET_M * R_model[:, 2]
    rng = np.random.default_rng(0)
    q0 = np.asarray(q_seed, dtype=float)
    seeds = [q0]
    for spread in (0.10, 0.20, 0.35, 0.60):
        seeds += [q0 + rng.uniform(-spread, spread, 6) for _ in range(6)]
    qp = None
    for k, sd in enumerate(seeds):
        qp = m.inverse_kinematics(tcp_target, R_model, seeds=[m.pin_q(list(sd))],
                                  pos_tol=pos_tol, rot_tol=rot_tol)
        if qp is not None:
            break
    if qp is None:
        raise VisionTargetError(
            "IK 실패: %s [%.4f %.4f %.4f] (base_link) 에 플랜지를 세우는 해가 "
            "씨앗 %d개로도 없다. 도달 범위 밖이거나 자기충돌이다"
            % (what, *p_base, len(seeds)))
    q = [float(qp[i]) for i in m.q_index]

    got, R_got = flange_pose_in_base(node, q)
    err_mm = float(np.linalg.norm(got - np.asarray(p_base)) * 1000)
    rot_err = math.degrees(math.acos(max(-1.0, min(1.0,
              (np.trace(np.asarray(R_base).T @ R_got) - 1.0) / 2.0))))
    if err_mm > 1.0 or rot_err > 0.3:
        raise VisionTargetError(
            "IK 해가 %s 를 벗어난다: 위치 %.2f mm, 자세 %.2f deg. "
            "프레임 환산을 의심할 것" % (what, err_mm, rot_err))
    return q, dict(ik_pos_err_mm=err_mm, ik_rot_err_deg=rot_err,
                   ik_seed_index=k, ik_seeds_tried=len(seeds))


def solve_hover_q(node, hover_base, q_seed, pos_tol=2e-4,
                  rot_tol=math.radians(0.05)):
    """플랜지 원점 = hover_base, 툴 축 = 수직 아래인 관절각을 푼다.

    IK 는 --level 이 쓰는 것과 같은 기계다 -- ReachabilityModel.inverse_kinematics
    (감쇠 최소자승), 씨앗은 **현재 자세 하나뿐**이다. 씨앗을 하나로 두는 이유는
    level_config() 의 주석 그대로다: 전역 재해석은 팔꿈치/손목 분기를 바꿔버리는데,
    관측 자세는 주변 지그를 피해 사람이 고른 자세이기 때문이다.

    inverse_kinematics 는 목표를 **TCP**(= 플랜지 + TCP_OFFSET_M * 툴축)로 받아
    내부에서 다시 그 오프셋을 뺀다. 우리 목표는 플랜지 원점이므로 미리 더해 넣어
    정확히 상쇄시킨다 -- 팬텀 그리퍼 길이가 목표에 섞이지 않는다.
    """
    _, R_now = flange_pose_in_base(node, q_seed)
    R_down = straight_down(R_now)          # 툴 축 수직, yaw 는 현재 것 유지
    q, ik = solve_flange_q(node, hover_base, R_down, q_seed,
                           pos_tol=pos_tol, rot_tol=rot_tol, what="hover")
    ik['ik_tilt_deg'] = tilt_from_down(flange_pose_in_base(node, q)[1])
    return q, ik


# --------------------------------------------------------------------------
# 비전 획득
# --------------------------------------------------------------------------

def _axis_vector(spec, length, what):
    s = str(spec).strip().lower()
    sign = -1.0 if s.startswith('-') else 1.0
    axis = s.lstrip('+-')
    if axis not in AXIS_VEC:
        raise VisionTargetError("%s 가 잘못됐다: %r (+x/-x/+y/... 중 하나)"
                                % (what, spec))
    return sign * AXIS_VEC[axis] * length


def _inward_vector():
    return _axis_vector(INWARD_AXIS, MAGAZINE_INWARD_M, "INWARD_AXIS")


def _jaw_vector():
    return _axis_vector(JAW_OFFSET_AXIS, JAW_OFFSET_M, "JAW_OFFSET_AXIS")


def _front_normal():
    """매거진 전면이 향하는 방향(단위벡터). 안쪽의 반대다 -- 상수를 둘로 나누면
    서로 어긋날 수 있으므로 INWARD_AXIS 하나에서 끌어낸다."""
    return -_axis_vector(INWARD_AXIS, 1.0, "INWARD_AXIS")


def git_hash(path=_PKG_ROOT):
    try:
        return subprocess.check_output(
            ['git', '-C', path, 'rev-parse', '--short', 'HEAD'],
            stderr=subprocess.DEVNULL).decode().strip()
    except Exception:
        return None


def snapshot(mon, timeout_s=SNAPSHOT_TIMEOUT_S, host=None, verbose=True):
    """러너를 MAGAZINE 으로 돌려 게이트 스냅샷 하나와 그때의 tool_vector 를 얻는다.

    스냅샷은 카메라 프레임에서 10장을 평균한 값이므로 그동안 팔이 정지해 있어야
    한다 (gate.py 주석). 움직임이 보이면 누적을 버린다 -- verify_chain 과 같은 규약.

    끝나면 반드시 러너를 IDLE 로 되돌린다. 성공/실패/예외 어느 쪽이든.
    """
    client = VisionClient(host=host)
    try:
        try:
            accepted, resp = client.set_mode("MAGAZINE")
        except Exception as e:
            raise VisionTargetError(
                "러너 REP 응답 없음 (%s:%d): %s -- 호스트에서 vision_runner 를 "
                "먼저 띄울 것" % (host or vcfg.RUNNER_HOST, vcfg.REP_PORT, e))
        if not accepted:
            raise VisionTargetError("러너가 set_mode MAGAZINE 을 거부했다: %s" % resp)

        gate = Gate()
        t0 = time.time()
        last_ping = time.time()
        last_msg = None
        while time.time() - t0 < timeout_s:
            # 러너는 10초간 커맨드가 없으면 IDLE 로 강등된다.
            if time.time() - last_ping >= 5.0:
                last_ping = time.time()
                try:
                    client.ping()
                except Exception as e:
                    raise VisionTargetError("러너 ping 실패: %s" % e)

            payload = client.poll(50)
            st, stamp = mon.state()
            if st is None or time.time() - stamp > vcfg.FEED_MAX_AGE_S:
                gate.reset()
                continue
            if float(np.max(np.abs(st['qd_actual']))) >= STILL_DPS:
                gate.reset()
                if verbose and last_msg != 'moving':
                    last_msg = 'moving'
                    print("      ... 팔이 아직 움직인다 -- 스냅샷 누적 버림")
                continue
            if payload is None:
                if client.silent and verbose and last_msg != 'silent':
                    last_msg = 'silent'
                    print("      ... 러너 무소식 (%.1fs)" % client.age_s)
                continue

            snap, why = gate.select(payload)
            if snap is None:
                if verbose and why and why != last_msg:
                    last_msg = why
                    print("      ... %s" % why)
                continue
            return snap, np.asarray(st['tool_vector'], dtype=float), payload

        raise VisionTargetError(
            "%.0f초 안에 게이트 통과 스냅샷이 없다 (마지막 사유: %s). 매거진이 화면에 "
            "들어와 있는지, 조명/거리(%.2f~%.2f m)를 볼 것"
            % (timeout_s, last_msg, vcfg.GATE_Z_MIN, vcfg.GATE_Z_MAX))
    finally:
        try:
            client.set_mode("IDLE")
        except Exception as e:
            print("      !! 러너 IDLE 복귀 실패: %s -- 호스트에서 확인할 것" % e)
        client.close()


def coarse_detections(mon, timeout_s=10.0, host=None, verbose=True):
    """지금 화면의 매거진 후보를 **게이트 없이** 읽는다 -- 조준 전용.

    ★ 이 좌표로 하강하지 않는다. 여기서 나오는 P_base 는 "어느 상자를 고를까"와
      "카메라를 어디로 데려갈까"에만 쓴다. 실측(2026-08-12) 0.36~0.43 m 에서 sd 가
      0.28~0.30 (게이트 한도 0.25) 이었다 -- 즉 cm 급이지 mm 급이 아니다.
      정밀 좌표는 근접 관측 자세로 옮긴 뒤 게이트를 통과한 스냅샷에서만 나온다.

    최소한의 위생 조건만 본다: TRACKING, stale 0, xyz_cam 존재, 거리 범위.
    (팔이 움직이는 중이면 프레임과 자세가 어긋나므로 정지 상태에서만 읽는다.)
    """
    tf = Transform()
    client = VisionClient(host=host)
    try:
        accepted, resp = client.set_mode("MAGAZINE")
        if not accepted:
            raise VisionTargetError("러너가 set_mode MAGAZINE 을 거부했다: %s" % resp)
        t0 = time.time()
        last_ping = time.time()
        while time.time() - t0 < timeout_s:
            if time.time() - last_ping >= 5.0:
                last_ping = time.time()
                client.ping()
            payload = client.poll(50)
            st, stamp = mon.state()
            if st is None or time.time() - stamp > vcfg.FEED_MAX_AGE_S:
                continue
            if float(np.max(np.abs(st['qd_actual']))) >= STILL_DPS:
                continue
            if payload is None:
                continue
            tool = np.asarray(st['tool_vector'], dtype=float)
            out = []
            for d in payload.get("detections") or []:
                if d.get("state") != "TRACKING" or d.get("stale_frames", 0):
                    continue
                xyz = d.get("xyz_cam")
                if not xyz or not (COARSE_Z_MIN <= xyz[2] <= COARSE_Z_MAX):
                    continue
                out.append(dict(track_id=d.get("track_id"), cls=d.get("cls"),
                                score=float(d.get("score", 0.0)),
                                xyz_cam=[float(v) for v in xyz],
                                valid_pct=d.get("valid_pct"), sd_cm=d.get("sd_cm"),
                                bbox=d.get("bbox"),
                                P_base=[float(v) for v in
                                        tf.to_base(xyz, tool)]))
            if out:
                if verbose:
                    print("  [coarse] 후보 %d개 (게이트 없이 읽음 -- 조준 전용)" % len(out))
                    for c in out:
                        print("     ID%-3s %-13s cam z=%.3f  P_base [%+.3f %+.3f %+.3f]"
                              "  score %.2f  sd %.2f"
                              % (c['track_id'], c['cls'], c['xyz_cam'][2],
                                 *c['P_base'], c['score'], c['sd_cm'] or float('nan')))
                return out, tool
        raise VisionTargetError(
            "%.0f초 안에 조준용 검출이 없다. 매거진이 화면에 들어와 있는지 볼 것"
            % timeout_s)
    finally:
        try:
            client.set_mode("IDLE")
        except Exception:
            pass
        client.close()


def pick_target(cands, rule=None):
    """후보 중 하나를 **결정적으로** 고른다. gate.py 는 score 최대를 고르는데,
    여러 대가 보이면 그 승자가 프레임마다 바뀌어 10연속 누적이 안 된다."""
    rule = (rule or TARGET_RULE).strip().lower()
    if not cands:
        raise VisionTargetError("고를 후보가 없다")
    if rule.startswith("id="):
        want = int(rule[3:])
        for c in cands:
            if c['track_id'] == want:
                return c, rule
        raise VisionTargetError("track_id %d 가 후보에 없다 (있는 것: %s)"
                                % (want, [c['track_id'] for c in cands]))
    if rule == "nearest":
        return min(cands, key=lambda c: c['xyz_cam'][2]), rule
    if rule == "left":
        return min(cands, key=lambda c: c['P_base'][0]), rule
    if rule == "right":
        return max(cands, key=lambda c: c['P_base'][0]), rule
    raise VisionTargetError("--target 는 nearest|left|right|id=N 중 하나여야 한다: %r"
                            % rule)


def viewpoint_q(node, P_box, q_seed, view_dist=None, tf=None):
    """매거진 전면 앞 view_dist 에 **카메라 원점**을 두고 전면을 마주보는 q.

    툴은 수직 아래를 유지하고(우리 규약), 카메라 방위는 **J6 를 포함한 z 축 둘레
    회전**으로 맞춘다 -- 카메라가 플랜지 옆(광축 = 플랜지 -y)을 보므로 손목을 돌리면
    시선이 그대로 돈다. 실제로 조그로 J6 를 17도 틀었더니 대상이 화면에서 사라졌다.

        u = R0 @ X[:3,2]              yaw 0 일 때의 카메라 광축 (base)
        d = -법선                      카메라가 향해야 할 방향
        psi = atan2(d) - atan2(u)      z 축 둘레로 그만큼 돌리면 u 가 d 에 겹친다
        p_flange = (P_box + 법선*view_dist) - R @ X[:3,3]
    """
    view_dist = VIEW_DIST_M if view_dist is None else float(view_dist)
    tf = tf or Transform()
    X = tf.X
    t_cam = np.asarray(X[:3, 3], dtype=float)     # 플랜지 기준 카메라 위치
    a_cam = np.asarray(X[:3, 2], dtype=float)     # 플랜지 기준 카메라 광축

    _, R_now = flange_pose_in_base(node, q_seed)
    R0 = straight_down(R_now)
    u = R0 @ a_cam
    if abs(u[2]) > 0.35:
        raise VisionTargetError(
            "툴을 수직으로 두면 카메라 광축이 수평이어야 하는데 z 성분이 %.2f 다. "
            "hand-eye(X) 나 카메라 장착이 예상과 다르다" % u[2])
    n = _front_normal()
    d = -n
    psi = math.atan2(d[1], d[0]) - math.atan2(u[1], u[0])
    c, s_ = math.cos(psi), math.sin(psi)
    Rz = np.array([[c, -s_, 0.0], [s_, c, 0.0], [0.0, 0.0, 1.0]])
    R = Rz @ R0
    cam_target = np.asarray(P_box, dtype=float) + n * view_dist
    p_flange = cam_target - R @ t_cam

    q, ik = solve_flange_q(node, p_flange, R, q_seed, what="관측 자세")
    ik['viewpoint_yaw_deg'] = math.degrees(psi)
    ik['cam_target'] = [float(v) for v in cam_target]
    ik['view_dist_m'] = view_dist
    return q, ik


def camera_pose_in_base(node, q, tf=None):
    """관절각 q 일 때 카메라 원점과 광축 (base_link). 검증·자가시험용."""
    tf = tf or Transform()
    p_f, R_f = flange_pose_in_base(node, q)
    return p_f + R_f @ np.asarray(tf.X[:3, 3]), R_f @ np.asarray(tf.X[:3, 2])


def hover_from_snapshot(snap, tool, tf=None):
    """게이트 통과 스냅샷 + 그때의 tool_vector -> (P_base, hover, inward, jaw), base_link.

    acquire_hover_q 와 vision/vision_hover_node.py 가 같은 식을 쓴다 (재구현 금지).
    """
    tf = tf or Transform()
    P_base = np.asarray(tf.to_base(snap['xyz_cam'], tool), dtype=float)
    inward = _inward_vector()
    jaw = _jaw_vector()
    hover = (P_base + inward + jaw
             + np.array([0.0, 0.0, MAGAZINE_HEIGHT_M / 2.0 + HOVER_CLEARANCE_M]))
    return P_base, hover, inward, jaw


def acquire_hover_q(node, mon, timeout_s=SNAPSHOT_TIMEOUT_S, host=None):
    """비전 -> hover -> q. (q_urdf, info) 를 돌려주거나 VisionTargetError.

    로봇을 움직이지 않는다. 호출부는 팔을 관측 자세에 세워 두고 정지시킨 뒤 부른다.
    """
    q_now = node.current_joints.tolist()
    info = dict(
        wall=time.time(),
        stamp=time.strftime('%Y-%m-%d %H:%M:%S'),
        git=git_hash(),
        constants=dict(MAGAZINE_HEIGHT_M=MAGAZINE_HEIGHT_M,
                       HOVER_CLEARANCE_M=HOVER_CLEARANCE_M,
                       MAGAZINE_INWARD_M=MAGAZINE_INWARD_M,
                       INWARD_AXIS=INWARD_AXIS,
                       JAW_OFFSET_M=JAW_OFFSET_M,
                       JAW_OFFSET_AXIS=JAW_OFFSET_AXIS,
                       JAW_BELOW_FLANGE_M=JAW_BELOW_FLANGE_M,
                       VISION_WORK_BOX=VISION_WORK_BOX,
                       MAX_MOVE_M=MAX_MOVE_M,
                       TCP_OFFSET_M_unused=TCP_OFFSET_M),
        obs_joints=[float(v) for v in q_now],
        obs_joints_deg=[round(math.degrees(v), 3) for v in q_now],
    )

    tf = Transform()
    info['handeye_t_flange_cam_m'] = [round(float(v), 6) for v in tf.X[:3, 3]]

    print("  [vision] 러너 MAGAZINE -- 스냅샷 %d프레임 평균을 기다린다"
          % vcfg.GATE_SNAPSHOT_N)
    snap, tool, payload = snapshot(mon, timeout_s=timeout_s, host=host)
    info['detection'] = snap
    info['tool_vector'] = tool.tolist()
    info['payload'] = {k: payload[k] for k in
                       ("req_id", "mode", "seq", "status", "depth_scale",
                        "git_vision", "git_runner") if k in payload}
    info['req_id'] = payload.get('req_id')

    # 프레임 단위시험 (실시간판): 모델 FK 로 계산한 플랜지 위치가 로봇 자신이 보고하는
    # tool_vector 와 맞는지. 어긋나면 부호 규약이 꺼졌거나 프레임이 바뀐 것이고,
    # 그 상태의 P_base 는 믿을 수 없다.
    fl_fk = flange_pose_in_base(node, q_now)[0]
    fl_robot = tool[:3] / 1000.0
    d_mm = float(np.linalg.norm(fl_fk - fl_robot) * 1000)
    info['fk_vs_robot_mm'] = d_mm
    print("  [frame] 모델 FK 플랜지 [%+.4f %+.4f %+.4f] vs 로봇 보고 "
          "[%+.4f %+.4f %+.4f] m -> %.1f mm" % (*fl_fk, *fl_robot, d_mm))
    if d_mm > FK_CHECK_TOL_M * 1000:
        raise VisionTargetError(
            "모델 FK 와 로봇 tool_vector 가 %.1f mm 어긋난다 (한도 %.0f). 관절 부호 "
            "규약이나 프레임이 틀렸다는 뜻이므로 좌표를 믿을 수 없다 -- "
            "docs/real_robot_joint_convention.md 11절" % (d_mm, FK_CHECK_TOL_M * 1000),
            info)

    P_base, hover, inward, jaw = hover_from_snapshot(snap, tool, tf)
    info['xyz_cam'] = list(snap['xyz_cam'])
    info['P_base'] = [float(v) for v in P_base]
    info['inward'] = [float(v) for v in inward]
    info['jaw_offset'] = [float(v) for v in jaw]
    info['hover'] = [float(v) for v in hover]
    info['flange_now'] = [float(v) for v in fl_fk]
    move_m = float(np.linalg.norm(hover - fl_fk))
    info['move_m'] = move_m

    print("  [vision] xyz_cam  [%+.4f %+.4f %+.4f] m  (track %s, %s, %d프레임, "
          "valid %.0f%%, sd %.2f cm)"
          % (*snap['xyz_cam'], snap['track_id'], snap['cls'], snap['n_frames'],
             snap['valid_pct'], snap['sd_cm']))
    print("  [vision] P_base   [%+.4f %+.4f %+.4f] m  (매거진 전면 중심)" % tuple(P_base))
    print("  [vision] hover    [%+.4f %+.4f %+.4f] m  (= P_base + 안쪽 %.3f m %s "
          "+ 조 %.3f m %s + z %.3f m)"
          % (*hover, MAGAZINE_INWARD_M, INWARD_AXIS, JAW_OFFSET_M,
             JAW_OFFSET_AXIS, MAGAZINE_HEIGHT_M / 2.0 + HOVER_CLEARANCE_M))
    print("  [vision] 현재 플랜지에서 %.0f mm 이동" % (move_m * 1000))

    # ── sanity -- 하나라도 걸리면 이동 없이 끝난다 ──────────────────────
    if VISION_WORK_BOX is None:
        raise VisionTargetError(
            "VISION_WORK_BOX 가 비어 있다. 작업 영역을 실측해 test/vision_target.py "
            "에 채우기 전에는 실행하지 않는다 -- 비전이 엉뚱한 점을 줘도 걸러낼 것이 "
            "없기 때문이다. 위 hover 값이 그 상자를 정하는 출발점이다.", info)
    for k, ax in (('x', 0), ('y', 1), ('z', 2)):
        lo, hi = VISION_WORK_BOX[k]
        if not (lo <= hover[ax] <= hi):
            raise VisionTargetError(
                "hover %s=%+.4f m 가 VISION_WORK_BOX %s [%+.3f, %+.3f] 밖이다"
                % (k, hover[ax], k, lo, hi), info)
    if move_m > MAX_MOVE_M:
        raise VisionTargetError(
            "현재 플랜지에서 hover 까지 %.0f mm -- 한도 %.0f mm 를 넘는다. 비전이 다른 "
            "것을 봤거나 변환이 틀렸다" % (move_m * 1000, MAX_MOVE_M * 1000), info)
    z_cam = float(snap['xyz_cam'][2])
    if not (vcfg.GATE_Z_MIN <= z_cam <= vcfg.GATE_Z_MAX):
        # gate.check() 가 이미 막는 조건이다. 게이트 설정이 바뀌어도 이 경로만은
        # 거리 범위를 벗어난 점으로 팔을 보내지 않도록 한 번 더 확인한다.
        raise VisionTargetError(
            "cam z=%.3f m 가 게이트 거리범위 %.2f~%.2f 밖이다"
            % (z_cam, vcfg.GATE_Z_MIN, vcfg.GATE_Z_MAX), info)

    q_hover, ik_info = solve_hover_q(node, hover, q_now)
    info.update(ik_info)
    info['hover_joints'] = [float(v) for v in q_hover]
    info['hover_joints_deg'] = [round(math.degrees(v), 3) for v in q_hover]
    valid = bool(node.is_state_valid(list(q_hover)))
    info['collision_free'] = valid
    print("  [ik]     hover 관절 " + " ".join(f"{math.degrees(v):+8.2f}" for v in q_hover)
          + "   (되돌린 오차 %.2f mm, 기울기 %.2f deg)"
          % (ik_info['ik_pos_err_mm'], ik_info['ik_tilt_deg']))
    if not valid:
        raise VisionTargetError(
            "hover 자세가 계획 충돌 모델에서 충돌 상태다 (자기충돌이거나 교시된 면 "
            "너머). 이동하지 않는다", info)
    return q_hover, info


def dump_info(info, path=None):
    """info 를 사람이 읽을 수 있게 (그리고 파일로) 남긴다."""
    text = json.dumps(info, indent=2, ensure_ascii=False, default=float)
    if path:
        with open(path, 'w') as f:
            f.write(text)
        print("기록 저장: %s" % path)
    return text


# --------------------------------------------------------------------------
# 자가시험 -- 로봇도 러너도 필요 없다
# --------------------------------------------------------------------------

class _ModelOnly:
    """selftest 전용 껍데기. ROS 노드 없이 프레임 계산만 하려고 ik_model 만 갖는다."""

    def __init__(self):
        from cr7_pnp.model import ReachabilityModel
        self.ik_model = ReachabilityModel(xacro_path=XACRO_PATH)
        self.ik_model.set_joint_limits([(-6.27, 6.27)] * 6)

    def is_state_valid(self, q):
        return self.ik_model.is_collision_free(list(q))


def selftest():
    ok = True
    node = _ModelOnly()

    # ① base_link 는 모델 루트보다 30 mm 위에 있고 회전은 단위행렬이다 (9.1)
    base = base_placement(node)
    dz = float(base.translation[2])
    rot_err = float(np.max(np.abs(base.rotation - np.eye(3))))
    print("① base_link 배치  t=%s m   |R-I|=%.2e" % (np.round(base.translation, 5), rot_err))
    ok &= abs(dz - 0.030) < 0.005 and rot_err < 1e-9
    if abs(dz - 0.030) >= 0.005:
        print("   !! 30 mm 가 아니다 -- URDF 가 바뀌었으면 문서 9.1 도 갱신할 것")

    # ② 왕복 항등: base -> model -> base
    rng = np.random.default_rng(0)
    pts = rng.uniform(-1.0, 1.0, size=(20, 3))
    err = max(float(np.linalg.norm(model_to_base(node, base_to_model(node, p)) - p))
              for p in pts)
    print("② 왕복 항등 base->model->base  최대 오차 %.3e m" % err)
    ok &= err < 1e-12

    # ③ 두 방향이 서로의 역이고, 차이가 정확히 그 30 mm 다.
    #    base_link 가 루트보다 위에 있으므로 base_link 값을 모델로 옮기면 z 가 **오른다**
    #    (거꾸로 하면 30 mm 계통 오차가 그대로 두 배가 된다).
    p = np.array([0.3, -0.1, 0.2])
    d = base_to_model(node, p) - p
    print("③ base->model 이동량 %s m (기대 [0 0 +%.3f])" % (np.round(d, 6), dz))
    ok &= np.allclose(d, [0.0, 0.0, dz], atol=1e-12)

    # ④ straight_down: 최소 회전이라 툴 축은 정확히 아래, 회전각은 원래 기울기와 같다
    q_tilt = [0.1, -0.6, -2.0, 1.0, -1.55, -0.2]
    _, R = flange_pose_in_base(node, q_tilt)
    R_dn = straight_down(R)
    turned = math.degrees(math.acos(
        max(-1.0, min(1.0, (np.trace(R_dn @ R.T) - 1.0) / 2.0))))
    print("④ straight_down  기울기 %.2f -> %.2f deg   회전량 %.2f deg (같아야 최소회전)"
          % (tilt_from_down(R), tilt_from_down(R_dn), turned))
    ok &= tilt_from_down(R_dn) < 1e-6 and abs(turned - tilt_from_down(R)) < 1e-6

    # ⑤ 축 파서 -- 부호나 축을 잘못 읽으면 상자 안쪽 대신 옆으로 간다.
    #    (오프셋이 118 mm 라 축 하나만 틀려도 12 cm 어긋난다)
    for spec, want in (("+x", [1, 0, 0]), ("-x", [-1, 0, 0]),
                       ("+y", [0, 1, 0]), ("-y", [0, -1, 0]),
                       ("+z", [0, 0, 1]), ("-z", [0, 0, -1])):
        got = _axis_vector(spec, 0.5, "test")
        ok &= np.allclose(got, np.array(want) * 0.5, atol=1e-12)
    print("⑤ 축 파서 6방향 OK   안쪽 %s %.3f m -> %s   조 %s %.3f m -> %s"
          % (INWARD_AXIS, MAGAZINE_INWARD_M, np.round(_inward_vector(), 4),
             JAW_OFFSET_AXIS, JAW_OFFSET_M, np.round(_jaw_vector(), 4)))

    # ⑥ hover 산술 -- 수직 상승분은 높이/2 + 여유, 수평은 안쪽 + 조 오프셋뿐
    P = np.array([0.30, -0.05, 0.10])
    hover = P + _inward_vector() + _jaw_vector() + np.array(
        [0.0, 0.0, MAGAZINE_HEIGHT_M / 2.0 + HOVER_CLEARANCE_M])
    rise = MAGAZINE_HEIGHT_M / 2.0 + HOVER_CLEARANCE_M
    horiz = (_inward_vector() + _jaw_vector())[:2]
    print("⑥ hover 산술  P_base %s -> %s   (z +%.3f, 수평 %s)"
          % (P, np.round(hover, 4), rise, np.round(horiz, 4)))
    ok &= abs(hover[2] - (P[2] + rise)) < 1e-12
    ok &= np.allclose(hover[:2] - P[:2], horiz, atol=1e-12)
    ok &= abs(rise - (0.07 + HOVER_CLEARANCE_M)) < 1e-12

    # ⑦ 사슬 전체: 어떤 자세의 플랜지 위치를 목표로 주고 IK 를 풀면 그 자리로
    #    돌아와야 한다. 프레임 환산이 한쪽만 적용되면 여기서 30 mm 로 터진다.
    pts_file = os.path.join(os.path.dirname(os.path.abspath(__file__)), 'points.json')
    q_seed = [0.1, -0.6, -2.0, 1.0, -1.55, -0.2]
    if os.path.exists(pts_file):
        with open(pts_file) as f:
            saved = json.load(f)
        if 'p1' in saved:
            q_seed = saved['p1']['joints']
    target, _ = flange_pose_in_base(node, q_seed)
    q_sol, ik = solve_hover_q(node, target, q_seed)
    got, R_got = flange_pose_in_base(node, q_sol)
    err_mm = float(np.linalg.norm(got - target) * 1000)
    print("⑦ IK 왕복  목표 %s -> 되돌린 오차 %.3f mm, 툴 기울기 %.3f deg"
          % (np.round(target, 4), err_mm, tilt_from_down(R_got)))
    ok &= err_mm < 1.0 and tilt_from_down(R_got) < 0.2

    # ⑧ 30 mm 를 빼먹은 잘못된 환산이었다면 반드시 걸렸을 것 -- 시험이 무딘지 확인
    bad, _ = solve_hover_q(node, target + np.array([0.0, 0.0, 0.030]), q_seed)
    moved_mm = float(np.linalg.norm(
        flange_pose_in_base(node, bad)[0] - target) * 1000)
    print("⑧ 민감도  목표를 30 mm 올리면 플랜지도 %.1f mm 움직인다 (시험이 무디지 않다)"
          % moved_mm)
    ok &= 29.0 < moved_mm < 31.0

    # ⑨ 관측 자세: 가짜 매거진 전면 중심을 주고 q 를 푼 뒤, 그 q 에서 카메라가
    #    정말 전면 앞 VIEW_DIST 에서 전면을 마주보는지 FK 로 되돌려 확인한다.
    #    (틀리면 팔이 엉뚱한 곳을 보러 간다 -- 로봇 없이 잡을 수 있는 오류다)
    try:
        tf = Transform()
    except Exception as e:
        print("⑨ 관측 자세  건너뜀 (hand-eye 로드 실패: %s)" % e)
        tf = None
    if tf is not None:
        P_box = np.array([-0.16, -0.50, 0.38])
        try:
            q_v, ikv = viewpoint_q(node, P_box, q_seed, tf=tf)
            cam_p, cam_a = camera_pose_in_base(node, q_v, tf=tf)
            want_p = P_box + _front_normal() * VIEW_DIST_M
            want_a = -_front_normal()
            dp = float(np.linalg.norm(cam_p - want_p) * 1000)
            da = math.degrees(math.acos(max(-1.0, min(1.0, float(np.dot(
                cam_a / np.linalg.norm(cam_a), want_a))))))
            tilt_v = tilt_from_down(flange_pose_in_base(node, q_v)[1])
            print("⑨ 관측 자세  카메라 위치오차 %.2f mm, 광축오차 %.2f deg, "
                  "툴 기울기 %.3f deg  (yaw %+.1f deg)"
                  % (dp, da, tilt_v, ikv['viewpoint_yaw_deg']))
            ok &= dp < 1.0 and da < 0.5 and tilt_v < 0.2
        except VisionTargetError as e:
            print("⑨ 관측 자세  실패: %s" % str(e)[:90])
            ok = False

    print("\nPASS" if ok else "\nFAIL")
    return ok


def probe(host=None, period_s=1.0):
    """조그하며 obs 자세를 찾는 뷰파인더. 게이트가 왜 떨어지는지 1초마다 찍는다.

    로봇에는 아무것도 보내지 않는다 (러너 set_mode 만). Ctrl+C 로 끝내면 IDLE 복귀.
    """
    # 한 프레임의 통과/탈락만으로는 부족하다: 스냅샷은 **연속 10장**이라, 통과율이
    # 높아도 임계 근처에서 깜빡이면 영영 안 모인다. 그래서 실제 Gate 를 같이 돌려
    # "지금 이 자세로 --vision 이 되는가"를 그대로 보여준다.
    from gate import check as gate_check
    g = Gate()
    n_snap = 0
    client = VisionClient(host=host)
    print("러너 MAGAZINE -> %s" % (client.set_mode("MAGAZINE")[1],))
    print("게이트 기준: valid >= %.0f%%, sd <= %.2fcm, 경계여백 >= %dpx, "
          "cam z %.2f~%.2fm   (Ctrl+C 로 종료)"
          % (vcfg.GATE_VALID_PCT_MIN, vcfg.GATE_SD_CM_MAX,
             vcfg.GATE_EDGE_MARGIN_PX, vcfg.GATE_Z_MIN, vcfg.GATE_Z_MAX))
    last = 0.0
    last_ping = time.time()
    try:
        while True:
            # 러너는 10초간 커맨드가 없으면 스스로 IDLE 로 떨어지고, IDLE 에서는 탐지를
            # 하지 않는다. 이 ping 이 없으면 10초 뒤부터 "검출 없음"만 나와서 자세
            # 탓을 하게 된다 (실제로 그랬다). snapshot() 과 같은 주기.
            if time.time() - last_ping >= 5.0:
                last_ping = time.time()
                try:
                    client.ping()
                except Exception as e:
                    print("  !! 러너 ping 실패: %s" % e)
            payload = client.poll(200)
            if payload is None:
                # 아무 말도 없는 것과 "러너가 죽었다"를 구분해 준다. 이 줄이 없으면
                # 화면이 그냥 멈춘 것처럼 보여서 조그하는 사람이 자세 탓을 하게 된다.
                if client.silent and time.time() - last >= period_s:
                    last = time.time()
                    print("  러너 무소식 %s -- 호스트에서 runner.py 가 프레임을 내고 "
                          "있는지 볼 것 (tail /tmp/runner_cam.log)"
                          % ("(첫 메시지 아직 없음)" if client.age_s == float('inf')
                             else "%.1fs" % client.age_s))
                continue
            snap, _ = g.select(payload)
            if snap is not None:
                n_snap += 1
                print("  >> 스냅샷 #%d  xyz_cam=[%+.4f %+.4f %+.4f] m  "
                      "valid=%.0f%%  sd=%.2fcm  프레임내 흔들림 %.2f/%.2f/%.2f mm"
                      % (n_snap, *snap['xyz_cam'], snap['valid_pct'],
                         snap['sd_cm'], *snap['xyz_std_mm']))
            if time.time() - last < period_s:
                continue
            last = time.time()
            dets = payload.get("detections") or []
            if not dets:
                # 모드를 같이 찍는다: MAGAZINE 이 아니면 그건 "안 보인다"가 아니라
                # "안 보고 있다"는 뜻이라 대응이 정반대다.
                print("  mode=%s status=%s  검출 없음%s"
                      % (payload.get("mode"), payload.get("status"),
                         "  <- 러너가 MAGAZINE 이 아니다"
                         if payload.get("mode") != "MAGAZINE" else ""))
                continue
            for d in dets:
                ok, why = gate_check(d)
                xyz = d.get("xyz_cam") or [float('nan')] * 3
                print("  %s ID%-3s  %s  z=%.3fm  valid=%4.1f%%  sd=%.2fcm  "
                      "score=%.2f  %s"
                      % ("PASS" if ok else "fail", d.get("track_id"),
                         d.get("cls"), xyz[2], d.get("valid_pct") or float('nan'),
                         d.get("sd_cm") or float('nan'), d.get("score") or 0.0,
                         "연속 %d/%d, 스냅샷 %d개" % (g.n_buffered, g.snapshot_n,
                                                    n_snap)
                         if ok else "<- " + why))
    except KeyboardInterrupt:
        print("")
    finally:
        try:
            print("러너 IDLE -> %s" % (client.set_mode("IDLE")[1],))
        except Exception as e:
            print("!! IDLE 복귀 실패: %s" % e)
        client.close()
    return True


def tilt_monitor(ip=None, hz=5.0):
    """툴이 연직 아래에서 몇 도 벗어났는지 실시간 표시. 조그하면서 보라고 있다.

    CBiRRT 는 툴 **기울기**만 붙들고 툴축 둘레 회전은 자유로 두므로, obs 를 수직으로
    교시했다면 팔의 기울기만 0 에 맞추면 된다 -- 여섯 관절을 다 맞출 필요가 없다.

    로봇이 스스로 보고하는 tool_vector(GetPose 와 같은 값)만 읽는다. URDF 도 관절
    부호 규약도 타지 않으므로, 모델이 어떻든 이 숫자는 로봇의 진실이다. 읽기 전용.
    """
    from handeye_calib import RobotFeed
    from handeye_calib import tool_to_T as _tool_to_T
    feed = RobotFeed(ip or vcfg.ROBOT_IP)
    feed.start()
    print("툴 기울기 실시간 (로봇 tool_vector 기준, 읽기 전용). Ctrl+C 로 종료")
    print("0.5 deg 아래로 내려가면 CBiRRT 가 obs 로 바로 풀린다\n")
    try:
        while True:
            f = feed.latest(vcfg.FEED_MAX_AGE_S)
            if f is None:
                print("\r로봇 피드 없음/오래됨 (%s)   " % (feed.error or "stale"),
                      end='', flush=True)
                time.sleep(1.0 / hz)
                continue
            tz = _tool_to_T(f['tool'])[:3, 2]
            tilt = math.degrees(math.acos(max(-1.0, min(1.0, -float(tz[2])))))
            lean = ("툴 끝이 %s%s 쪽으로 기울어 있다"
                    % ("+x" if tz[0] > 0 else "-x", "/+y" if tz[1] > 0 else "/-y")
                    if tilt > 0.05 else "수직")
            print("\r기울기 %5.2f deg   툴축 [%+.4f %+.4f %+.4f]   %s        %s"
                  % (tilt, tz[0], tz[1], tz[2], lean,
                     "<< OK" if tilt < 0.5 else ""),
                  end='', flush=True)
            time.sleep(1.0 / hz)
    except KeyboardInterrupt:
        print("")
    finally:
        feed.stop()
    return True


if __name__ == '__main__':
    if '--tilt' in sys.argv:
        sys.exit(0 if tilt_monitor() else 1)
    if '--probe' in sys.argv:
        sys.exit(0 if probe() else 1)
    sys.exit(0 if selftest() else 1)
