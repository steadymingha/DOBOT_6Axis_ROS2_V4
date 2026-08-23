#!/bin/bash
# Mirror the newest offline wandb run of the server transformer training and
# sync it to wandb.ai with this machine's login. Exits when no train.py runs
# on the server anymore.
MIRROR=/home/user/dobot_ws/diffusionpolicy/server_run_wandb
WANDB=/home/user/miniforge3/envs/robodiff/bin/wandb
SSH="ssh -p 10023 192.168.2.49"
mkdir -p "$MIRROR"
while true; do
    for RUN_REMOTE in $($SSH 'ls -dt /hdd/workspace/temp/diffusion_policy/data/outputs/*/*transformer*/wandb/offline-run-* 2>/dev/null | head -4'); do
        rsync -az --delete -e "ssh -p 10023" "192.168.2.49:$RUN_REMOTE" "$MIRROR/" || true
    done
    $WANDB sync "$MIRROR"/offline-run-* >/dev/null 2>&1 || true
    if $SSH 'pgrep -f "train[.]py" >/dev/null' 2>/dev/null; then
        MISS=0
    else
        MISS=$((MISS + 1))   # tolerate restart gaps: exit after 3 misses (30 min)
        if [ "$MISS" -ge 3 ]; then echo "server training exited $(date)"; break; fi
    fi
    sleep 600
done
