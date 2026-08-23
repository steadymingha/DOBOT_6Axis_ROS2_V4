#!/usr/bin/env bash
# Wait for the parallel image eval to free :5557, then resume lowdim training
# from epoch 10 into the SAME run dir and stop at epoch 150.
# ponytail: one-shot; delete after it fires.
set -u
RUN=/home/user/fun/diffusion_policy/data/outputs/2026.07.30/10.39.51_train_diffusion_transformer_isaac_lowdim_isaac_shelf_lowdim
CKPTS="$RUN/checkpoints"
PY=/home/user/miniforge3/envs/robodiff/bin/python
LOG=/home/user/dobot_ws/diffusionpolicy/train_lowdim.log

# 1. wait until :5557 is free AND no eval client — require 2 consecutive clear
#    checks (60s) so a brief gap between back-to-back evals doesn't false-trigger.
free_checks=0
until [ "$free_checks" -ge 2 ]; do
  if ! ss -tln 2>/dev/null | grep -qE '127.0.0.1:5557\b' \
     && ! pgrep -f "isaac_[e]val.py" >/dev/null; then
    free_checks=$((free_checks + 1))
  else
    free_checks=0
  fi
  sleep 30
done
echo "image eval done (:5557 free) at $(date). GPU free:"
nvidia-smi --query-gpu=memory.free --format=csv,noheader

# 2. resume into the same run dir (finds latest.ckpt @ epoch 10)
cd /home/user/fun/diffusion_policy || exit 1
nohup setsid "$PY" train.py \
  --config-name=train_diffusion_transformer_isaac_lowdim_workspace \
  training.checkpoint_every=10 \
  hydra.run.dir="$RUN" >> "$LOG" 2>&1 &
sleep 25
MAINPID=$(pgrep -f "train.py.*isaac_lowdim" | head -1)
PGID=$(ps -o pgid= -p "$MAINPID" 2>/dev/null | tr -d ' ')
echo "resumed training, pid $MAINPID pgid $PGID"
grep -m1 "Resuming from checkpoint" "$LOG" || echo "WARN: no 'Resuming' line — check it did not restart from epoch 0"

# 3. stop at epoch=0150
until ls "$CKPTS"/epoch=0150-*.ckpt >/dev/null 2>&1; do sleep 20; done
echo "epoch=0150 landed at $(date):"
ls -la "$CKPTS"/epoch=0150-*.ckpt
kill -TERM -"$PGID" 2>/dev/null; sleep 8; kill -KILL -"$PGID" 2>/dev/null || true
sleep 3
echo "training stopped. free VRAM:"
nvidia-smi --query-gpu=memory.free --format=csv,noheader
echo "checkpoints:"; ls "$CKPTS"/epoch=*.ckpt
