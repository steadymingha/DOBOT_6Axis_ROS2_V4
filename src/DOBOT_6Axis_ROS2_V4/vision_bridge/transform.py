"""P_cam → P_base 변환.

    P_base = T_base_flange @ T_flange_cam @ P_cam
             = tool_to_T(tool_vector) @ X @ [x, y, z, 1]

곱 순서의 정답지는 handeye_calib.py cmd_verify L973 그대로다.
X 의 역행렬은 쓰지 않는다.

회전 규약(ZYX intrinsic)은 handeye_calib.tool_to_T 를 그대로 import 해서 쓴다 —
Pinocchio FK 대조로 0.46° 잔차까지 검증된 구현이라 여기서 재유도하지 않는다.

단위는 전 구간 미터. tool_vector 의 mm→m 은 tool_to_T 안에서 이미 처리되므로
호출부에서 다시 나누지 말 것.
"""
import json
import os
import sys

import numpy as np

import config

sys.path.insert(0, config.HANDEYE_DIR)
from handeye_calib import tool_to_T  # noqa: E402  (재구현 금지 — 검증된 구현 재사용)


def load_X(path=None):
    """handeye_result.json → X = T_flange_cam (4x4). 회전이 정규직교가 아니면 예외."""
    path = path or config.HANDEYE_RESULT
    with open(path) as fp:
        X = np.asarray(json.load(fp)["T_flange_cam"], dtype=np.float64)

    if X.shape != (4, 4):
        raise ValueError("T_flange_cam shape %s, 4x4 가 아니다" % (X.shape,))
    R = X[:3, :3]
    det = float(np.linalg.det(R))
    orth = float(np.max(np.abs(R @ R.T - np.eye(3))))
    if abs(det - 1.0) > 1e-6 or orth > 1e-6:
        raise ValueError("T_flange_cam 회전이 정규직교가 아니다: "
                         "det=%.9f (기대 1), max|R@R.T - I|=%.3e" % (det, orth))
    if not np.allclose(X[3], [0, 0, 0, 1], atol=1e-12):
        raise ValueError("T_flange_cam 마지막 행이 [0 0 0 1] 이 아니다: %s" % X[3])
    return X


class Transform:
    def __init__(self, path=None):
        self.X = load_X(path)

    def to_base(self, xyz_cam, tool_vector):
        """카메라 광학 프레임 점 [m] + tool_vector(mm/deg) → base 좌표 [m]."""
        P = np.array([xyz_cam[0], xyz_cam[1], xyz_cam[2], 1.0], dtype=np.float64)
        return (tool_to_T(tool_vector) @ self.X @ P)[:3]

    def flange(self, tool_vector):
        """T_base_flange. 표본 novelty 판정에 쓴다."""
        return tool_to_T(tool_vector)


def selftest():
    """하드웨어 없이 도는 자가시험. 단위와 규약이 어긋나면 여기서 걸린다."""
    ok = True

    # ① tool_to_T — handeye_calib.py cmd_selftest 의 기지값 재사용
    #    (2026-08-07 실측 프레임. mm→m 이 한 번만 일어나는지도 함께 확인)
    tool = np.array([-357.649, 105.967, 297.336, 91.908, -9.589, 93.659])
    T = tool_to_T(tool)
    want = np.array([-0.35765, 0.10597, 0.29734])
    err_mm = float(np.linalg.norm(T[:3, 3] - want) * 1000)
    det = float(np.linalg.det(T[:3, :3]))
    print("① tool_to_T  translation err %.4f mm   det(R) %.9f" % (err_mm, det))
    ok &= err_mm < 0.02 and abs(det - 1.0) < 1e-9

    # ② X 로드 + 정규직교
    X = load_X()
    print("② X 로드 OK   det=%.9f  t=%s m"
          % (np.linalg.det(X[:3, :3]), np.round(X[:3, 3], 6)))

    # ③ 원점 일관성: P_cam = 0 이면 P_base = (T_base_flange @ X) 의 위치
    tf = Transform()
    got = tf.to_base([0.0, 0.0, 0.0], tool)
    exp = (T @ X)[:3, 3]
    print("③ P_cam=0 → P_base %s   (기대 %s)" % (np.round(got, 6), np.round(exp, 6)))
    ok &= np.allclose(got, exp, atol=1e-12)

    # ④ 곱 순서: 카메라 +Z 1m 는 base 에서 X 의 3번째 열 방향으로 1m 떨어져야 한다
    got = tf.to_base([0.0, 0.0, 1.0], tool)
    exp = (T @ X)[:3, 3] + (T[:3, :3] @ X[:3, :3])[:, 2]
    print("④ P_cam=+Z1m 방향 일치: %s" % np.allclose(got, exp, atol=1e-12))
    ok &= np.allclose(got, exp, atol=1e-12)

    # ⑤ 단위: cam z 를 1mm 늘리면 base 도 정확히 1mm 움직여야 한다 (이중 환산 감지)
    d = np.linalg.norm(tf.to_base([0, 0, 0.001], tool) - tf.to_base([0, 0, 0], tool))
    print("⑤ cam +1mm → base %.6f mm" % (d * 1000))
    ok &= abs(d - 0.001) < 1e-12

    print("\nPASS" if ok else "\nFAIL")
    return ok


if __name__ == "__main__":
    sys.exit(0 if selftest() else 1)
