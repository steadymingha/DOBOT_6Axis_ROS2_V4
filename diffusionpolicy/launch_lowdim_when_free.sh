#!/usr/bin/env bash
# Wait until the Isaac eval frees the GPU, then launch lowdim training.
# ponytail: single-purpose gate script; delete after the run starts.
set -u
LOG=/home/user/dobot_ws/diffusionpolicy/train_lowdim.log
PY=/home/user/miniforge3/envs/robodiff/bin/python

# bracket trick so this script never matches its own pgrep
until ! pgrep -f "isaac_[e]val" >/dev/null; do sleep 30; done
echo "eval finished at $(date), free VRAM:"
nvidia-smi --query-gpu=memory.free --format=csv,noheader
sleep 5  # let the server/sim release VRAM

cd /home/user/fun/diffusion_policy || exit 1
nohup setsid "$PY" train.py \
  --config-name=train_diffusion_transformer_isaac_lowdim_workspace \
  training.checkpoint_every=10 > "$LOG" 2>&1 &
sleep 15
# real python pid (not the setsid wrapper)
pgrep -f "train.py.*isaac_lowdim" | tee /home/user/dobot_ws/diffusionpolicy/train_lowdim.pid
echo "launched; log -> $LOG"
