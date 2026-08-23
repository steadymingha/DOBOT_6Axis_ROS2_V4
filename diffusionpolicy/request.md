# 요청: lowdim diffusion policy 학습 (로컬 2080)

## 진행 상황 (2026-07-30, 다른 세션의 읽기 전용 진단)

**1차 학습 발산 확정.** `train_lowdim.log`의 epoch별 loss 궤적:

```
ep0 1.08 → ep1 0.142 → ... → ep27 0.019 → ep29 0.024   (정상 하강, 매우 좋음)
ep30 0.157 (튐) → ep31 0.021 → ep32 0.914 (폭발) → ep33~44 0.9~1.1 고착
```

- epoch 30~32에서 **gradient 폭발** → 이후 loss가 ~1.0(랜덤 출력 수준)에 눌어붙어
  회복 못 함. 로그에 리터럴 "nan"은 안 찍혔지만(EMA/부분 클립이 완전 nan 직전을
  붙든 것), 학습은 실질적으로 죽었음. 마지막이 epoch 44에서 wandb 스레드 예외로 끊김.
- **중요**: 폭발 전 30 epoch가 깨끗하게 학습됨 = **진단 실험 자체는 유효**. lowdim이
  이 태스크를 배우기 시작한다는 증거는 이미 있음.

### 처방 (싸고 확실한 것부터)

1. **gradient clipping 추가 (max_norm 1.0)** — config에 clip이 없음. transformer는
   warmup만으로 부족해 ~30 epoch 특정 배치에서 튀면 그대로 폭발. 재발 방지 표준,
   한 줄. (repo 다른 transformer workspace에 `training.grad_clip` 또는 옵티마이저
   스텝 직전 `torch.nn.utils.clip_grad_norm_` 패턴이 있는지 먼저 확인해 맞출 것.)
2. **`object` 입력 점프 점검** — lowdim에만 있는 위험: obs의 박스 pose가 AGV 재파킹/
   박스 텔레포트 순간 급점프하는 프레임이 있으면 큰 gradient 씨앗. npz_qc식으로
   `object` 채널의 틱당 점프(>0.15m)를 스캔해 이상 구간이 폭발 시점과 겹치는지 확인.
3. 위로도 터지면 **lr 1e-4 → 5e-5** (batch 256엔 약간 높을 수 있음).

### 재개 방법

- clipping 적용 후 **처음부터 재학습**(폭발한 가중치는 못 이어씀). checkpoint_every=10.
- 폭발 전 정상 ckpt(epoch 20 또는 30 직전)가 남아 있으면, 재학습을 기다리는 동안
  그걸로 **eval을 먼저 한 번** 돌려 lowdim의 대략적 능력을 미리 볼 수 있음
  (loss 0.02대면 image의 loss 0.004보다 높지만 파지 시도는 나올 수준).

---

## 목적 (왜 하는가)

image-hybrid 정책이 rollout에서 파지 직전 호버링으로 전멸 중 (경과는
`doc/TRAIN.md` 결과 로그 참고 — 겹침 실행으로 close 발화까지는 왔음).
**diffusion 헤드는 동일하고 조건 입력만 다른** lowdim 정책을 학습해서 원인을
분리한다:

- lowdim 성공 + image 실패 → 병목은 시각 인코더(ResNet 조건 벡터 품질)
- lowdim도 동일 실패 → 병목은 action 표현/실행(하네스) 쪽

lowdim은 obs에 `object`(박스 pose, sim 특권 정보)를 쓰므로 **sim 전용
상한선 베이스라인**이다. 배포용 아님.

## 사전 지식 (필독)

- `doc/TRAIN.md` — 지금까지의 학습/eval 전 과정, 지표 읽는 법, 인프라 함정
- `doc/DATASET.md` — 데이터셋 규약 (zarr 변환 섹션, 쿼터니언 주의 섹션)
- 핵심 함정: 저장된 quat는 ±부호 혼재 → **반드시 6D rotation으로 변환해 학습**
  (`diffusion_policy/dataset/isaac_shelf_image_dataset.py`의 numpy 헬퍼
  `pose7_to_pose9`/`action8_to_action10` 재사용)

## 환경

- repo: `~/fun/diffusion_policy` (pip 설치 아님 — cwd를 repo root로)
- python: `~/miniforge3/envs/robodiff/bin/python` (torch 1.12/cu11, 2080용)
- 데이터: `~/dobot_ws/diffusionpolicy/data/isaac_shelf/isaac_shelf_lowdim.zarr`
  (11MB, ep 68–331, 264 에피소드 — 통째로 RAM에 올려도 됨)
- GPU: 로컬 2080 8GB — Isaac sim(~2.8GB) + policy_server(~0.9GB)와 공유.
  **원칙: eval이 안 도는 시간에 학습한다.** 시작 전 두 가지 확인:
  1. `pgrep -f "isaac_[e]val"` — eval 진행 중이면 대기
  2. `nvidia-smi` — 여유 VRAM 3GB 이상인지
  학습 시작 후에도 eval을 새로 돌리지 말 것 (어느 쪽이든 OOM 즉사 위험 +
  GPU 경쟁으로 eval 청크 타이밍 흔들림). sim/서버를 내려야 하면 pgrep 패턴
  자기매칭 주의 — TRAIN.md 인프라 노트.

## 만들 것

1. **dataset**: `diffusion_policy/dataset/isaac_shelf_lowdim_dataset.py`
   - `isaac_shelf_image_dataset.py`에서 rgb 처리만 뺀 형태 (SequenceSampler/
     normalizer/val split 구조 동일, rot6d 헬퍼 import)
   - obs 키: `robot_eef_pose`(7→9), `gripper`(1), `object`(7→9) — **object도
     rot6d 변환** (이미 `_convert_lowdim`이 'object' 키를 처리함)
   - action: 8→10 변환
   - zarr가 작으니 `ReplayBuffer.copy_from_path`로 메모리 로드 OK
2. **task config**: `diffusion_policy/config/task/isaac_shelf_lowdim.yaml`
   - shape_meta: obs {robot_eef_pose:[9], gripper:[1], object:[9]}, action:[10]
   - env_runner: dummy (`real_pusht_image_runner.RealPushTImageRunner`)
   - val_ratio 0.02
3. **workspace config**: `train_diffusion_transformer_isaac_lowdim_workspace.yaml`
   - base: `train_diffusion_transformer_lowdim_workspace.yaml` (repo 기본)
   - task 교체, To=2 / Tp=10 / Ta=8 (image 런과 동일 조건 유지 — 비교 목적)
   - **checkpoint.topk.monitor_key = train_loss / mode: min** (rollout 없음 —
     test_mean_score 기본값이면 epoch 종료 시 KeyError로 죽음, 이미 밟은 지뢰)
   - logging project: `isaac_shelf_diffusion` (wandb는 로컬이라 online OK)
   - batch: repo lowdim 기본값(보통 256) 그대로 — 2080에서 문제없는 크기

## 실행

```bash
cd ~/fun/diffusion_policy
nohup setsid ~/miniforge3/envs/robodiff/bin/python train.py \
  --config-name=train_diffusion_transformer_isaac_lowdim_workspace \
  training.checkpoint_every=10 > ~/dobot_ws/diffusionpolicy/train_lowdim.log 2>&1 &
# 실제 python pid를 pgrep로 따로 확보할 것 ($!는 setsid 래퍼)
```

- lowdim은 epoch당 이미지 디코딩이 없어 수 분 이내 — 수백 epoch도 몇 시간
- NaN 감시 걸 것 (unet에서 NaN 발산 전례; transformer는 아직 무사고지만
  `loss=nan` 프린트 감시는 공짜)

## 학습 후: rollout eval 연결

- `diffusionpolicy/policy_server.py`는 action 차원(10)으로 rot6d를 자동 감지하고
  'object' 키도 7→9 변환함 — 서버는 그대로 사용 가능
- **`diffusionpolicy/isaac_eval.py` 수정 필요 (1곳)**: run_episode의 obs dict에
  `'object': np.asarray(rec.buf['object'][-To:], np.float32)` 추가
  (Recorder는 이미 매 틱 object를 기록 중). image 정책과 양쪽 호환이 필요하면
  서버가 cfg의 shape_meta obs 키 목록으로 받은 obs를 필터링하게 한 줄 추가
  (lowdim 정책에 image 키가 들어가면 normalizer KeyError)
- eval 실행법/함정은 TRAIN.md의 "Eval 하네스" 섹션 그대로 (겹침 실행 버전 사용)

## 판정

같은 조건(10 에피소드, 점유 랜덤화, DDIM-16, Ta8)에서 image 정책 결과와 비교:
- 성공률/attach율이 유의하게 높으면 → 시각 인코더 쪽 수사 (카메라 구도,
  crop, 해상도, 인코더 용량)
- 동일하게 호버링/실패면 → 실행 하네스·action 표현 쪽 수사 계속
