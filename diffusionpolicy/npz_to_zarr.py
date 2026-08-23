#!/usr/bin/env python
"""Convert isaac_shelf npz episodes to diffusion_policy zarr replay buffers.

Run with the robodiff env:
  ~/miniforge3/envs/robodiff/bin/python diffusionpolicy/npz_to_zarr.py
"""
import sys
import numpy as np

sys.path.insert(0, '/home/user/fun/diffusion_policy')
from diffusion_policy.common.replay_buffer import ReplayBuffer
from diffusion_policy.codecs.imagecodecs_numcodecs import register_codecs, Jpeg2k

register_codecs()

EP_DIR = '/home/user/dobot_ws/diffusionpolicy/data/isaac_shelf/episodes'
OUT_DIR = '/home/user/dobot_ws/diffusionpolicy/data/isaac_shelf'
EP_RANGE = range(68, 332)
IMAGE_EXCLUDE = {74, 192}  # recording tick drops (see doc/DATASET.md QC section)

LOWDIM_KEYS = ['robot_eef_pose', 'gripper', 'object', 'action']
IMAGE_KEYS = ['agentview_image', 'robot0_eye_in_hand_image']


def convert(out_path, keys, episodes, image_keys=()):
    buf = ReplayBuffer.create_from_path(out_path, mode='w')
    chunks = {k: (1, 240, 320, 3) for k in image_keys}
    compressors = {k: Jpeg2k(level=50) for k in image_keys}
    for i, ep in enumerate(episodes):
        with np.load(f'{EP_DIR}/episode_{ep:04d}.npz') as f:
            data = {k: f[k] for k in keys}
        buf.add_episode(data, chunks=chunks, compressors=compressors)
        if i % 20 == 0 or i == len(episodes) - 1:
            print(f'{out_path}: {i + 1}/{len(episodes)} (ep {ep}, T={buf.n_steps})', flush=True)
    print(f'done: {out_path} episodes={buf.n_episodes} steps={buf.n_steps}')


if __name__ == '__main__':
    convert(f'{OUT_DIR}/isaac_shelf_lowdim.zarr', LOWDIM_KEYS, list(EP_RANGE))
    convert(f'{OUT_DIR}/isaac_shelf_image.zarr', LOWDIM_KEYS + IMAGE_KEYS,
            [e for e in EP_RANGE if e not in IMAGE_EXCLUDE], image_keys=IMAGE_KEYS)
