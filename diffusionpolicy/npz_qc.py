# npz quality check for diffusionpolicy/data/isaac_shelf/episodes
# Checks per episode: keys/shapes/dtypes, T consistency, NaN/Inf, quat norms,
# action[t] vs eef[t+1] alignment, per-step eef jumps, gripper range,
# image resolution + black/frozen-frame detection. Writes a summary report
# and a contact sheet of sampled frames.
import glob, os, sys
import numpy as np

EP_DIR = os.path.expanduser("~/dobot_ws/diffusionpolicy/data/isaac_shelf/episodes")
OUT_DIR = os.path.dirname(os.path.abspath(__file__))

EXPECTED_KEYS = ["agentview_image", "robot0_eye_in_hand_image",
                 "robot_eef_pose", "gripper", "object", "action"]
META_KEYS = ["meta_box", "meta_box_idx", "meta_pocket", "meta_station", "meta_agv_xy"]

rows = []
problems = []

files = sorted(glob.glob(os.path.join(EP_DIR, "episode_*.npz")))
for f in files:
    ep = int(os.path.basename(f)[8:12])
    d = np.load(f)
    r = {"ep": ep}
    missing = [k for k in EXPECTED_KEYS if k not in d.files]
    if missing:
        problems.append((ep, "missing keys: %s" % missing))
        rows.append(r)
        continue

    eef = d["robot_eef_pose"]; act = d["action"]; obj = d["object"]; grip = d["gripper"]
    T = eef.shape[0]
    r["T"] = T
    lens = {k: d[k].shape[0] for k in EXPECTED_KEYS}
    if len(set(lens.values())) != 1:
        problems.append((ep, "length mismatch: %s" % lens))

    img = d["agentview_image"]
    r["res"] = "%dx%d" % (img.shape[1], img.shape[2])
    r["meta"] = all(k in d.files for k in META_KEYS)

    # NaN/Inf
    for k in ["robot_eef_pose", "action", "object", "gripper"]:
        a = d[k]
        if not np.isfinite(a).all():
            problems.append((ep, "non-finite values in %s" % k))

    # quaternion norms (xyzw at [3:7])
    for k, a in [("eef", eef), ("object", obj)]:
        n = np.linalg.norm(a[:, 3:7], axis=1)
        r["qerr_" + k] = float(np.abs(n - 1).max())

    # action[t] should be eef pose at t+1 (absolute action convention)
    pos_err = np.linalg.norm(act[:-1, :3] - eef[1:, :3], axis=1)
    # quat distance, sign-invariant
    qdot = np.abs(np.sum(act[:-1, 3:7] * eef[1:, 3:7], axis=1)).clip(0, 1)
    ang_err = 2 * np.arccos(qdot)
    r["act_pos_err_max"] = float(pos_err.max())
    r["act_pos_err_p99"] = float(np.percentile(pos_err, 99))
    r["act_ang_err_max"] = float(ang_err.max())

    # per-step eef jump (10 Hz -> 0.15 m/step = 1.5 m/s, generous bound)
    step = np.linalg.norm(np.diff(eef[:, :3], axis=0), axis=1)
    r["step_max"] = float(step.max())
    r["step_mean"] = float(step.mean())
    if step.max() > 0.15:
        problems.append((ep, "eef jump %.3f m in one tick" % step.max()))

    # gripper range
    r["grip_min"] = float(grip.min()); r["grip_max"] = float(grip.max())
    r["grip_changes"] = int(np.count_nonzero(np.diff(grip[:, 0])))

    # object pose sanity: should move (it gets carried)
    obj_travel = np.linalg.norm(obj[-1, :3] - obj[0, :3])
    r["obj_travel"] = float(obj_travel)
    if obj_travel < 0.05:
        problems.append((ep, "object barely moved (%.3f m) - failed grasp recorded?" % obj_travel))

    # image checks on subsampled frames
    idx = np.linspace(0, T - 1, 8).astype(int)
    for cam in ["agentview_image", "robot0_eye_in_hand_image"]:
        frames = d[cam][idx]
        stds = frames.reshape(len(idx), -1).std(axis=1)
        if (stds < 2.0).any():
            problems.append((ep, "%s has near-constant (black/frozen?) frames" % cam))
        # frozen camera: all sampled frames identical
        if len(idx) > 1 and all((frames[i] == frames[0]).all() for i in range(1, len(idx))):
            problems.append((ep, "%s frozen for whole episode" % cam))
    r["img_mean"] = float(d["agentview_image"][idx].mean())
    rows.append(r)
    d.close()

# ---- report ----
def fmt(v):
    return "%.4f" % v if isinstance(v, float) else str(v)

rep = []
rep.append("episodes: %d" % len(rows))
Ts = [r["T"] for r in rows if "T" in r]
rep.append("T: min=%d max=%d mean=%.0f" % (min(Ts), max(Ts), np.mean(Ts)))

res_groups = {}
for r in rows:
    res_groups.setdefault(r.get("res"), []).append(r["ep"])
for res, eps in sorted(res_groups.items()):
    rep.append("resolution %s: %d eps (ep %d..%d)" % (res, len(eps), min(eps), max(eps)))

meta_eps = [r["ep"] for r in rows if r.get("meta")]
rep.append("meta keys present: %d eps (from ep %d)" % (len(meta_eps), min(meta_eps) if meta_eps else -1))

for key, label in [("qerr_eef", "quat norm err eef"), ("qerr_object", "quat norm err object"),
                   ("act_pos_err_max", "action-vs-eef[t+1] pos err max (m)"),
                   ("act_ang_err_max", "action-vs-eef[t+1] ang err max (rad)"),
                   ("step_max", "eef per-tick step max (m)"),
                   ("obj_travel", "object travel (m)")]:
    vals = np.array([r[key] for r in rows if key in r])
    rep.append("%s: max=%.4f p99=%.4f median=%.4f" % (label, vals.max(), np.percentile(vals, 99), np.median(vals)))

g = np.array([[r["grip_min"], r["grip_max"]] for r in rows if "grip_min" in r])
rep.append("gripper range: [%.3f, %.3f], changes/ep median=%d" %
           (g[:, 0].min(), g[:, 1].max(), int(np.median([r["grip_changes"] for r in rows if "grip_changes" in r]))))

rep.append("")
if problems:
    rep.append("PROBLEMS (%d):" % len(problems))
    for ep, msg in problems:
        rep.append("  ep %04d: %s" % (ep, msg))
else:
    rep.append("PROBLEMS: none")

# worst episodes by action alignment error
worst = sorted([r for r in rows if "act_pos_err_max" in r], key=lambda r: -r["act_pos_err_max"])[:5]
rep.append("")
rep.append("worst action-alignment episodes: " +
           ", ".join("ep%d=%.3fm" % (r["ep"], r["act_pos_err_max"]) for r in worst))

txt = "\n".join(rep)
print(txt)
with open(os.path.join(OUT_DIR, "qc_report.txt"), "w") as fh:
    fh.write(txt + "\n")

# ---- contact sheet: first/mid/last frames of 4 episodes across lineage ----
try:
    import cv2
    sheet_eps = [0, 67, 68, 199]
    tiles = []
    for ep in sheet_eps:
        d = np.load(os.path.join(EP_DIR, "episode_%04d.npz" % ep))
        T = d["agentview_image"].shape[0]
        row_tiles = []
        for t in [0, T // 2, T - 1]:
            for cam in ["agentview_image", "robot0_eye_in_hand_image"]:
                im = d[cam][t]
                im = cv2.resize(im, (320, 240))
                cv2.putText(im, "ep%d t%d %s" % (ep, t, cam[:5]), (5, 20),
                            cv2.FONT_HERSHEY_SIMPLEX, 0.5, (0, 255, 0), 1)
                row_tiles.append(im)
        tiles.append(np.hstack(row_tiles))
        d.close()
    sheet = np.vstack(tiles)
    out = os.path.join(OUT_DIR, "qc_contact_sheet.png")
    cv2.imwrite(out, cv2.cvtColor(sheet, cv2.COLOR_RGB2BGR))
    print("contact sheet: %s" % out)
except Exception as e:
    print("contact sheet skipped: %s" % e)
