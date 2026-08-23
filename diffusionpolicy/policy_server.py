#!/usr/bin/env python
"""Diffusion-policy inference server for Isaac rollout eval.

Runs in the robodiff conda env (torch + diffusion_policy repo); the ROS-side
eval client (isaac_eval.py, system python) talks to it over a localhost TCP
socket, because rclpy (py3.10 system) and torch (robodiff py3.9) cannot share
a process.

    ~/miniforge3/envs/robodiff/bin/python diffusionpolicy/policy_server.py \
        --ckpt <path.ckpt> [--port 5556] [--num-inference-steps 16] [--n-action-steps 8]

Protocol: 4-byte big-endian length + pickle, request/response.
  {"cmd": "meta"}                  -> {"n_obs_steps", "n_action_steps"}
  {"cmd": "act", "obs": {k: np}}   -> {"action": (Ta,8) float32}
Obs arrays come in raw recorder layout: images (To,H,W,3) uint8, lowdim (To,D).
"""
import argparse
import pickle
import socket
import struct
import sys

import numpy as np
import torch
import dill
import hydra

sys.path.insert(0, '/home/user/fun/diffusion_policy')
from diffusion_policy.dataset.isaac_shelf_image_dataset import (  # noqa: E402
    pose7_to_pose9, rot6d_to_R, R_to_quat_xyzw)
from diffusion_policy.policy.base_lowdim_policy import BaseLowdimPolicy  # noqa: E402

IMG_KEYS = ('agentview_image', 'robot0_eye_in_hand_image')


def recv_msg(conn):
    hdr = b''
    while len(hdr) < 4:
        c = conn.recv(4 - len(hdr))
        if not c:
            return None
        hdr += c
    n = struct.unpack('>I', hdr)[0]
    buf = b''
    while len(buf) < n:
        c = conn.recv(min(1 << 20, n - len(buf)))
        if not c:
            return None
        buf += c
    return pickle.loads(buf)


def send_msg(conn, obj):
    b = pickle.dumps(obj, protocol=4)
    conn.sendall(struct.pack('>I', len(b)) + b)


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument('--ckpt', required=True)
    ap.add_argument('--port', type=int, default=5556)
    ap.add_argument('--device', default='cuda')
    ap.add_argument('--num-inference-steps', type=int, default=None)
    ap.add_argument('--ddim', action='store_true',
                    help='swap the sampler to DDIM at inference (paper: '
                         'real-time control); pair with --num-inference-steps 8..16')
    ap.add_argument('--n-action-steps', type=int, default=None,
                    help='T_a override (inference-time knob, <= horizon-To+1)')
    args = ap.parse_args()

    payload = torch.load(open(args.ckpt, 'rb'), pickle_module=dill,
                         map_location='cpu')
    cfg = payload['cfg']
    cls = hydra.utils.get_class(cfg._target_)
    workspace = cls(cfg, output_dir='/tmp/claude-1000/policy_server_out')
    workspace.load_payload(payload, exclude_keys=None, include_keys=None)
    policy = workspace.ema_model if cfg.training.use_ema else workspace.model
    if args.ddim:
        from diffusers.schedulers.scheduling_ddim import DDIMScheduler
        # same beta schedule/prediction type as trained; only the sampler changes
        policy.noise_scheduler = DDIMScheduler.from_config(
            policy.noise_scheduler.config)
    if args.num_inference_steps is not None:
        policy.num_inference_steps = args.num_inference_steps
    if args.n_action_steps is not None:
        policy.n_action_steps = args.n_action_steps
    policy.to(args.device)
    policy.eval()
    dev = args.device
    # 6d-rotation policies (action dim 10): convert obs quat->6d on the way in
    # and action 6d->quat on the way out, so the ROS client always speaks quat.
    rot6d = int(cfg.shape_meta.action['shape'][0]) == 10
    # obs keys this ckpt was trained on (in config order); the client sends a
    # superset (e.g. 'object' for lowdim, images for hybrid) so we filter to
    # avoid a normalizer KeyError. lowdim policies want ONE flat 'obs' tensor.
    obs_keys = list(cfg.shape_meta.obs.keys())
    is_lowdim = isinstance(policy, BaseLowdimPolicy)
    try:        # obs frame spacing the ckpt was trained with (ticks)
        obs_stride = int(cfg.task.dataset.get('obs_stride', 1))
    except Exception:
        obs_stride = 1
    print(f'loaded {args.ckpt}: epoch={payload["state_dicts"].get("epoch", "?") if isinstance(payload.get("state_dicts"), dict) else "?"} '
          f'To={policy.n_obs_steps} Ta={policy.n_action_steps}', flush=True)

    srv = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
    srv.setsockopt(socket.SOL_SOCKET, socket.SO_REUSEADDR, 1)
    srv.bind(('127.0.0.1', args.port))
    srv.listen(1)
    print(f'listening on 127.0.0.1:{args.port}', flush=True)
    while True:
        conn, _ = srv.accept()
        print('client connected', flush=True)
        while True:
            req = recv_msg(conn)
            if req is None:
                break
            if req['cmd'] == 'meta':
                send_msg(conn, {'n_obs_steps': policy.n_obs_steps,
                                'n_action_steps': policy.n_action_steps,
                                'obs_stride': obs_stride})
                continue
            per_key = {}
            for k in obs_keys:                       # only the trained keys
                v = req['obs'][k]
                if k in IMG_KEYS:
                    t = torch.from_numpy(
                        np.moveaxis(v, -1, 1).astype(np.float32) / 255.)
                else:
                    v = v.astype(np.float32)
                    if rot6d and (k.endswith('_pose') or k == 'object'):
                        v = pose7_to_pose9(v)
                    t = torch.from_numpy(v)
                per_key[k] = t
            if is_lowdim:
                # concat in shape_meta order -> single (1,To,Do) 'obs' tensor
                obs = {'obs': torch.cat([per_key[k] for k in obs_keys], dim=-1
                                        ).unsqueeze(0).to(dev)}
            else:
                obs = {k: t.unsqueeze(0).to(dev) for k, t in per_key.items()}
            with torch.no_grad():
                out = policy.predict_action(obs)
            act = out['action'][0].detach().cpu().numpy().astype(np.float32)
            if rot6d:                       # (Ta,10) -> (Ta,8) pos+quat+grip
                quat = R_to_quat_xyzw(rot6d_to_R(act[:, 3:9]))
                act = np.concatenate(
                    [act[:, :3], quat.astype(np.float32), act[:, 9:10]], axis=1)
            send_msg(conn, {'action': act})
        conn.close()
        print('client disconnected', flush=True)


if __name__ == '__main__':
    main()
