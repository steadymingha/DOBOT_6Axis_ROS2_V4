#!/usr/bin/env bash
# Wait for the epoch=0010 checkpoint, then stop lowdim training + free the GPU.
# ponytail: one-shot watcher; delete after it fires.
set -u
CKPTS=/home/user/fun/diffusion_policy/data/outputs/2026.07.30/10.39.51_train_diffusion_transformer_isaac_lowdim_isaac_shelf_lowdim/checkpoints
PGID=2607686
TARGET="epoch=0010"

until ls "$CKPTS"/${TARGET}-*.ckpt >/dev/null 2>&1; do sleep 15; done
echo "$TARGET ckpt landed at $(date):"
ls -la "$CKPTS"/${TARGET}-*.ckpt
kill -TERM -"$PGID" 2>/dev/null
sleep 8
kill -KILL -"$PGID" 2>/dev/null || true
sleep 3
echo "training stopped. free VRAM:"
nvidia-smi --query-gpu=memory.free --format=csv,noheader
echo "checkpoints available:"
ls "$CKPTS"/epoch=*.ckpt
