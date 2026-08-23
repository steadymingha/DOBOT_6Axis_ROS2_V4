# Diffusion Policy 학습 기록 (isaac_shelf)

데이터셋: `doc/DATASET.md` (ep 68–331, zarr 변환본). 이 문서는 **학습 쪽** 기록 —
무엇을 어떤 설정으로 돌렸고, 무엇이 실패했고 왜였는지, 지표를 어떻게 읽는지.

## 실행 환경

| | 로컬 (RTX 2080 8GB) | GPU 서버 (RTX Pro 6000 Blackwell 97GB ×4) |
|---|---|---|
| 용도 | Isaac eval + policy 추론 서버 | 본 학습 (GPU 2번만 사용) |
| repo | `~/fun/diffusion_policy` (sys.path 삽입) | `/hdd/workspace/temp/diffusion_policy` |
| python | robodiff conda env (torch 1.12/cu11) | `/hdd/workspace/temp/env` (torch 2.11/**cu128** — Blackwell 필수) |
| 접속 | – | `ssh -p 10023 192.168.2.49`, 전부 `/hdd/workspace/temp/` 안에 격리 |

- 서버 env 호환 핀: huggingface-hub 0.11.1(diffusers 0.11.1용), protobuf 3.19.6 +
  tensorboard 2.10.1, robomimic은 `--no-deps`(egl_probe 회피), train.py의 mujoco import는 try/except.
- **체크포인트는 버전 교차 호환**: torch 2.11 저장 → 1.12 로드 실측 통과 (zip 직렬화 동일).
- wandb: 서버가 공용 계정이라 API 키를 두지 않고 **offline 학습 + 로컬에서 10분마다
  rsync + sync** (`diffusionpolicy/wandb_sync_loop.sh`). 프로젝트 `isaac_shelf_diffusion`.

## 학습 실행법 (서버)

```bash
# 데이터: 단일 zip(zarr ZipStore 레이아웃)으로 전송해 두었음 — 31만개 소파일 배포 금지(아래 인프라 노트)
ssh -p 10023 192.168.2.49
cd /hdd/workspace/temp/diffusion_policy && CUDA_VISIBLE_DEVICES=2 \
  XDG_CACHE_HOME=/hdd/workspace/temp/.cache TORCH_HOME=/hdd/workspace/temp/.cache/torch \
  WANDB_DIR=/hdd/workspace/temp/.wandb WANDB_MODE=offline \
  nohup setsid /hdd/workspace/temp/env/bin/python train.py \
    --config-name=train_diffusion_transformer_isaac_hybrid_workspace \
    task.dataset_path=/hdd/workspace/temp/data/isaac_shelf_image.zarr.zip \
    task.dataset.store_in_memory=True logging.mode=offline \
    dataloader.num_workers=16 val_dataloader.num_workers=16 \
    > /hdd/workspace/temp/train_transformer.log 2>&1 &
```

주의: `nohup setsid ... &`의 `$!`는 **래퍼 pid** — 죽일 때는 실제 python pid의
**프로세스 그룹**(`kill -TERM -$(ps -o pgid= -p <python_pid>)`)으로. (래퍼만 죽이고
"껐다"고 착각한 사고 2회.)

## 아키텍처: unet 실패 → transformer

- **unet-hybrid (CNN)**: 로컬 2080, batch 32, lr 1e-4 — **epoch 6에서 loss NaN 발산**.
  robomimic can 때 겪은 "unet gradient 폭발"이 이 데이터(240×320, 절대 pose action)에서도
  재현. 논문은 unet을 robust한 기본값이라 하지만 우리 세팅에선 두 번 다 터짐.
  폭발 **직전**(epoch 5) 체크포인트도 이미 망가져 있었음: 학습 obs에 대해서조차
  위치 오차 ~0.4m, quat 노름 0.2~0.9. → 교훈: 발산 런의 직전 체크포인트는 쓰지 말 것.
- **transformer-hybrid (채택)**: n_layer 8 / n_head 4 / n_emb 256, lr warmup 1000스텝,
  transformer_weight_decay 1e-3. can 학습 때 검증된 조합. 현 본학습 런.

## 회전 표현: quat ±부호 문제 → 6D rotation (핵심 교훈)

npz/zarr에는 pose가 quat(xyzw)로 저장돼 있는데, **같은 회전이 q와 −q 두 표현을 가짐**
(쿼터니언은 반각 θ/2 기반이라 θ+360°가 부호 반전). 우리 그리퍼 tool-down은 180° 회전
= **qw≈0 경계**에 정확히 앉아 있어, FK의 행렬→quat 변환이 틱마다 ±를 오락가락:
33 ep 샘플에서 에피소드 내 부호 점프 206회, 시작 부호도 23:10으로 혼재.

- 실행에는 무해 (q ≡ −q). **회귀/생성 타깃으로는 치명적**: 같은 obs에 정답이
  +q/−q로 모순 → MSE 최소화 = 조건부 평균 ≈ 0 → 노름<1의 무의미한 출력으로 붕괴.
  (epoch-5 unet에서 실측된 바로 그 증상.)
- **해결**: 논문 관례대로 **6D rotation representation**(회전행렬 앞 두 열; 부호
  모호성·불연속 원천 부재). **zarr는 그대로 두고** dataset 클래스가 로드 시점 변환:
  obs pose 7→9D, action 8→10D (`IsaacShelfImageDataset`, `rotation_6d: True`;
  numpy 헬퍼 내장이라 pytorch3d 불필요). eval 쪽은 policy_server가 action 차원(10)을
  보고 6d→quat 복원 — ROS 클라이언트는 계속 pos+quat만 다룸.
- quat 시절 transformer 런(실학습 ~45분)은 이 전환으로 폐기 (차원이 바뀌어 이어받기 불가).

## 학습 설정 (현 런: train_diffusion_transformer_isaac_hybrid_workspace)

| 항목 | 값 | 비고 |
|---|---|---|
| obs | 카메라 2대 (3,240,320) + eef pose 9D + gripper 1D | `object`(특권 sim 상태) **제외** — 배포 불가 정보 |
| action | 10D 절대 pose (pos3+rot6d) + gripper | robomimic abs 관례 |
| T_o / T_p / T_a | 2 / 10 / 8 | @10Hz. T_a는 **추론 노브**(재학습 불필요), T_p만 학습 고정 |
| crop | 216×288 (240×320의 90%) | CropRandomizer |
| batch / lr | 64 / 1e-4 (warmup 1000) | 검증된 조합 — batch 키우면 lr 재튜닝 필요, 안 건드림 |
| scheduler | DDPM 100 (학습) | 추론은 DDIM 스왑 가능 (아래) |
| checkpoint | 10 epoch마다, train_loss top-5 + latest | dummy runner라 `test_mean_score` 없음 → monitor_key=train_loss 필수 |
| val | 2% 홀드아웃 | |
| 속도 | **8.5~8.8 it/s, ~5.5분/epoch** (GPU util ~90%) | 600 epoch ≈ 2.3일; VRAM 11GB만 쓰는 건 정상(용량≠속도) |

loss 경과 (6D 런): epoch 0 평균 0.199 → epoch 40 평균 0.012.

## 학습 지표 읽는 법

**train_loss (diffusion 학습 목적함수)** — 매 스텝. expert action에 랜덤 세기의
노이즈를 섞고 그 **노이즈를 맞추는** 오차(epsilon-prediction MSE). 정규화 공간 +
무작위 timestep 기준이라 "expert처럼 행동하는가"를 직접 말하지 않음 — 디노이징
한 스텝의 품질일 뿐.

**train_action_mse_error (모방 품질 프록시)** — sample_every(5 epoch)마다, 학습
배치 하나에 **실제 inference 전체**(순수 노이즈→디노이징 완주)를 돌려 생성 action과
expert action의 MSE (역정규화 실단위). "배포 때 하는 그 연산"의 출력 오차라 정책
품질에 훨씬 가까움.

실용 수칙:
- 둘은 따로 놀 수 있다 — loss 낮은데 action_mse 높으면 디노이징 체인 붕괴 (NaN 직전 unet).
- action_mse는 5 epoch당 배치 1개 샘플이라 노이즈 큼 — 추세만.
- 수렴 판단: val_loss + action_mse 평평 / 최종 판단: **rollout 성공률**.

**지표 서열: 성공률 > action_mse > loss.** expert action이 다봉일 수 있어(같은 상황,
여러 유효 행동) MSE는 "다른 유효한 모드"에도 벌점을 준다 — diffusion policy가 애초에
그 문제의식에서 나온 방법. 논문도 loss 최저 체크포인트 ≠ 성공률 최고를 명시, 그래서
여러 체크포인트를 rollout으로 평가해 고른다. (우리 데이터는 스크립트 전문가라 다봉성이
약한 편 — MSE 왜곡이 덜함. 그래도 서열은 동일.)

## 추론 (배포/eval)

- **batch는 학습 전용** — 추론은 batch 1. 메모리 ~2-3GB (가중치 ~90M 파라미터 +
  no_grad). 2080에서 Isaac과 동시 구동 문제없음.
- **DDIM 스왑**: DDPM으로 학습해도 노이즈 예측 네트워크는 동일 — 추론 때 샘플러만
  교체 가능. `policy_server.py --ddim --num-inference-steps 16`:
  **0.7s → 82ms/추론** (2080 실측, 품질 영향은 eval에서 DDPM-100과 비교 예정).
- **T_a 스윕은 공짜**: 정책은 항상 T_p 스텝을 예측, 몇 개 실행할지는 추론 파라미터
  (`--n-action-steps`). 재학습 없이 성공률로 튜닝.

## Eval 하네스 (성공률 측정)

학습 중 rollout이 없는 이유: env가 Isaac+ROS 스택이라 gym처럼 내장 불가, 서버엔
Isaac 없음. 대신 로컬에서:

- 2-프로세스: `policy_server.py`(robodiff env, GPU 추론, localhost 소켓) +
  `isaac_eval.py`(시스템 python, ROS/sim 구동) — rclpy(py3.10)와 torch(py3.9)는 한
  프로세스에 못 섞음.
- 에피소드: 수집기와 동일한 리셋/파킹 노이즈 분포 → 정책이 T_a 청크(절대 pose)를
  출력 → seeded DLS IK + 충돌 게이트 → FollowJointTrajectory (0.1s/waypoint) →
  gripper close 명령 + 박스 근접 시 ATTACHLINK(데이터 규약과 동일), open 시 detach.
- 성공 판정 = 수집 게이트 재사용: `box_in_pocket`(±6cm/±5cm) + `shelf_undisturbed`.
  결과는 `data/isaac_shelf/eval/*.jsonl`.
- 하네스에서 밟은 지뢰들(재발 방지):
  1. 네트워크 출력 quat은 비정규 — 정규화 후 회전행렬로.
  2. **대상 박스의 stock 팬텀은 에피소드 시작에 absent** (전문가도 그랬음) — 안 하면
     파지 접근이 전부 chunk_collision.
  3. 에피소드 정리는 **recover(팔 이동) 먼저 → 팬텀 복원 나중** — 순서 바꾸면 팬텀이
     그리퍼를 감싸 "start state in collision"으로 이후 모든 플래닝 불능.
  4. rollout은 충돌 상태에서 끝날 수 있음 — recover 실패 시 무검사 hub 직행 폴백
     (sim 한정; 다음 에피소드가 전체 리셋).

### 결과 로그

| 시점 | 체크포인트 | 설정 | 결과 |
|---|---|---|---|
| 07-29 | unet epoch 5 (발산 직전) | DDPM-100, Ta8 | 파이프라인 검증용 — 즉사(chunk_ik), 게이트가 쓰레기 action을 올바르게 거부 |
| 07-29 | transformer epoch 40 | DDIM-16, Ta8, 10ep | **0/10** — chunk_collision 7(그리퍼 vs **이웃** 박스 팬텀, ~150스텝=파지 접근부), traj_fail 3(외곽 스테이션, 물리 접촉 추정) |
| 07-29 | transformer epoch 40 | DDIM-16, **Ta4**, 10ep | **0/10, 동일 프로필** → 병목은 open-loop 드리프트가 아니라 **정책 정밀도 자체** |
| 07-29 | transformer epoch 80 | DDIM-16, Ta8, 10ep ×4회 반복 (하네스 개선 반복) | **0/10** — 단, 실패 국면이 좁혀짐 (아래) |
| 07-30 | transformer epoch 150 (loss 0.005) | DDIM-16, Ta8, 10ep 중 6 완료(서버 소켓 단절로 중단→재연결 패치) | **0/6, epoch 80과 동일한 호버링 실패** → "더 학습"으로 안 풀림 확정. T_p=16 비교 런 결과 대기 |
| 07-30 | **tp16** epoch 60 (T_p=16, loss 0.009) | DDIM-16, Ta8, 10ep | **0/10, 동일 호버링** → T_p 연장도 답 아님 |
| 07-30 | tp16 epoch 60 | DDIM-16, Ta8, **겹침 실행**(PRED_LEAD 0.3s/SKIP_TICKS 3), 10ep | **0/10이지만 국면 전환**: 그리퍼 close가 발화하기 시작, **첫 ATTACH 성공**(ep5, 11.4cm), 에피소드 278~903스텝으로 연장. 타임아웃 4 등장 |

| 07-30 | transformer epoch 250 (loss 0.004) | **stepped eval**(시뮬 일시정지=지연 0, robomimic 의미론), DDIM-16, Ta8, 10ep | **0/10** {collision 6, stuck 2, timeout 1, ik 1}. 단 **5.3cm 정상 파지 attach 1회**(→운반 실패 타임아웃). **지연을 완전 제거해도 접근 정밀도 부족 = 정책 자체의 한계 확정.** 다음 결정 실험: lowdim (request.md) |
| 07-30 | transformer epoch 230 (loss 0.004, md5 무결 확인) | 겹침 실행 + 동적 스플라이스, DDIM-16, Ta8, 10ep | **0/10 전부 chunk_ik** (63~109스텝): 타깃이 eef 1~8cm 옆인데 IK 불능 = 회전 출력 폭주. in-dist 지표는 최상(action_mse 1.5e-4) — 늦은 에폭 + 겹침 실행 조합에서만 터짐 (ep250 stepped는 정상 프로필 → 순수 과적합이라기보다 실행 타이밍과의 상호작용) |
| 07-30 | **ft150_grasp6 epoch 180** (ep150 리줌 + 파지 구간 ×6 오버샘플 +30ep) | 겹침 실행, DDIM-16, Ta8, 10ep | **0/10** {collision 7, ik 2, timeout 1}. 최근접 close **0.100m** (ep80 기준선 0.144 대비 개선, attach 게이트 0.09에는 미달). 원거리 close 난사는 잔존 |
| 07-30 | ft150_grasp6 epoch 180 | **stepped**, DDIM-16, Ta8, 10ep | **0/10** {collision 7, stuck 1, timeout 1, ik 1}, attach 0회 — ep250 stepped와 구분 불가. **오버샘플 30 epoch로는 질적 변화 없음** → ep220에서 재판정 1회 후 기각/유지 결정 |
| 07-30 | ft150_grasp6 epoch 220 | **stepped**, DDIM-16, Ta8, 10ep | **0/10** {collision 5, stuck 4, ik 1}, **attach 0회**. ep180 대비 개선 없음(오히려 stuck 증가). **→ 파지 구간 오버샘플 처방 기각**: +70 epoch(150→220)를 더 태워도 attach조차 안 나옴. 데이터 재가중만으로는 안 풀림 확정 |
| 07-31 | **lowdim ep590** (특권 박스 pose 입력, grad_clip=1.0로 발산 해결·loss 0.004) | **stepped**, DDIM-16, Ta8, 10ep | **0/10** {stuck 8, timeout 1, ik 1}, attach 0회, close는 박스 **13~14cm**에서 발화(문턱 9cm 미달=맴돌기). **오프라인 검증: 학습 obs에선 전문가 action mm 재현(위치오차 1~11mm)** → eval 버그 아님, 순수 rollout 실패. **★결정 결과: 박스 위치를 정확한 숫자로 줘도 image와 동일 실패 = 병목은 인지(시각)가 아니라 닫힌 루프 강건성(compounding error)** |

## ★ 진단 결론 (2026-07-31): 병목은 시각이 아니라 데이터/닫힌 루프

lowdim(특권 박스 pose 직접 입력) 실험이 원인을 갈랐다:
- lowdim은 **in-distribution에서 mm 완벽**(오프라인 검증: 전문가 action 1~11mm 재현)
- 그런데도 rollout 0/10, image와 **동일하게 ~13cm에서 맴돌다 stuck**
- 카메라 추정 오차 0인데도 실패 → **인지 문제 아님**. 시각 처방(카메라/해상도/
  인코더)은 헛수고로 판명.

진짜 원인 = **compounding error / 자기 드리프트 복구 불능**:
- 스크립트 전문가는 실수를 안 하니 "약간 틀어진 상태에서 복귀"를 시연에 안 남김
- 정책은 in-dist는 완벽하나 rollout에서 미세 드리프트 → 전문가 미시연 상태 →
  하강 지속 대신 정지. loss/action_mse는 이걸 못 봄(전부 in-dist 지표).

**다음 처방 (데이터 쪽, 우선순위)**:
1. **corrective/noisy 시연 수집** — 수집기의 파지 접근에 섭동 주입(파킹 노이즈처럼)
   해서 "틀어진 상태→복귀" 궤적을 데이터에 넣기. DAgger 계열의 값싼 근사.
2. 또는 전문가 궤적 다양화(접근 각도/속도 랜덤화) — 현재 궤적이 너무 좁음.
3. (병행 가능) action 표현 재검토: 절대 pose 대신 상대 변위(delta) action이 드리프트에
   더 강건한지 — 논문도 delta가 일부 태스크서 유리.
- 보류: 시각 인코더 처방 전부(원인 아님), 체크포인트/T_a 스윕(성공 나오기 전엔 무의미).

**stepped eval 도입 (2026-07-30)**: 알고리즘 검증은 이제 `--stepped`가 기본 —
isaac_sim.py `/sim_pause` 서비스(SetBool)로 추론 중 world.step을 멈춰 sim time을
얼림(컨트롤러·TF·카메라 일관 동결). 겹침 실행 경로는 배포 리허설용으로 유지.
(교훈: 검증 목적이면 처음부터 스텝 방식으로 갔어야 했다 — 지연 삽질 하루는
배포 때 어차피 필요한 작업이었다는 것만이 위안.)

**타이밍 가설 (부분 확정)**: 청크 사이 추론 정지(~150ms)가 T_o=2 속도 신호를 얼려 정책이
호버링했던 것. 겹침 실행(청크 종료 0.3s 전 obs 캡처→추론→종료 시점에 다음 goal
스플라이스)으로 연속 동작을 만들자 하강·닫기 행동이 즉시 해금됨. 남은 문제:
① 일부 에피소드에서 박스 0.5m 거리(허브 부근)에서 close 남발 — 스플라이스 정렬
(고정 SKIP_TICKS vs 실제 lead 변동)의 그리퍼 채널 타이밍 오차 의심
② 첫 attach가 11.4cm(경계 스레스홀드)에서 걸려 어긋난 파지 → 이후 OOD 표류.
attach 거리 기준(현 0.12m)이 TCP-그리퍼 파지중심 오프셋(~53mm) 대비 너무 관대 —
0.08~0.09m로 조이고, 스플라이스를 실측 경과시간 기반 동적 skip으로 바꾸는 게 다음 수.

epoch 80 심층 진단 (하네스 개선과 병행):
- 선반 **점유 분포 불일치** 발견·수정: 기존 eval은 tier-2 만재(학습 데이터에서 ~0.1%
  빈도의 상태)로 돌았음 → 수집기와 같은 50% 스태시 랜덤화 적용
- 게이트 정당성 검증: **전문가 기록 궤적을 게이트에 리플레이 → 무결점 통과** (게이트가
  과하게 엄격한 것 아님)
- 오프라인 검증: 학습 obs의 파지 순간에서는 그리퍼 닫기 시퀀스를 GT와 거의 일치하게
  예측 (닫기 능력 자체는 학습됨)
- 롤아웃 실패 형태 (영상 확인): 접근·조준(y오차 2~3cm)은 정상, **박스 위에서 호버링**
  — 최종 정밀 하강(straddle) 단계가 재현 안 됨 → 그리퍼 close 트리거 상태에 도달
  못 함 → 이웃 팬텀 클립으로 종료. 6연속 재예측 기회(strike 방식)에도 동일 지점 반복
  = epoch 80의 체계적 한계로 판정. **다음: epoch 150+에서 재평가** (자동 예약)

해석: 접근 궤적·자세는 형성(에피소드당 13~53초 제어 지속), 마지막 몇 cm가 부족.
박스 피치 0.181m + 그리퍼 개방 침범 52mm 기하에서는 cm급 정밀도가 필요. epoch 40의
한계로 판단, 추가 학습 대기. **다음: epoch 80+에서 재평가** (자동 모니터 예약).

## 인프라 노트 (삽질 기록)

- **공용 HDD에 zarr 소파일 31만 개 = 재앙**: 다른 사용자 작업이 디스크를 점유하면
  (r_await 590ms) 랜덤 읽기 dataloader가 D-state로 죽는다. 해법 2단:
  ① zarr를 **단일 zip**(ZipStore 레이아웃, 무압축)으로 배포 — 순차 읽기,
  ② `store_in_memory=True` — 압축된 채 ~6GB를 프로세스 RAM에 상주(워커는 fork COW 공유).
  적용 후 8.8 it/s 안정. python import도 같은 HDD라 프로세스 기동에 10~40분 걸릴 수
  있음(1회 비용) — 재시작 최소화.
- **wandb 정리**: 실패 런은 wandb Api `run.delete()`로 삭제 가능. 단, 삭제하면 비교
  베이스라인도 사라진다 — 삭제 전에 비교 가치 여부 확인 (quat vs 6d 곡선 비교 기회를
  이렇게 날림; 다만 액션 차원이 달라 어차피 공정 비교는 아니었음).
- **pkill/pgrep 자기매칭**: 패턴 문자열이 자기 명령줄에 있으면 자기 셸을 죽인다 —
  `pgrep -f "isaac_[s]im"`처럼 브래킷으로 리터럴 매칭을 깨되, **같은 명령 안에 브래킷
  없는 실제 경로가 또 있으면 무효**. kill과 launch는 별도 명령으로.

**ft150_stride3 트랙 종료 (2026-07-31 15시경)**: 다른 세션에서 띄운 obs_stride=3 리줌 런
(ep150→258, GPU 2, `logging.mode=disabled`라 wandb에는 안 올라감 — 지표는 run 디렉토리
`logs.json.txt`). 중단 판단 근거: ① val_loss가 ep150부터 108 epoch 내내 ~0.007 정체
(train_loss만 0.0058→0.0036 하강 = 암기), action_mse 1~3e-4 평평 — in-dist 수렴 완료,
② lowdim 결정 실험이 병목=닫힌 루프/데이터 커버리지로 확정해 stride 처방 자체가 기대값
상실, ③ "더 학습"이 rollout을 바꾼 전례 없음(ep80→150→250, grasp6 +70ep 모두 동일).
체크포인트 ep160–250은 run 디렉토리에 보존 — stride3 가지를 굳이 실측 기각하려면 ep230
(val_loss 최저 0.0067) stepped eval 1회면 충분. GPU 2 반납.

## 현황 (2026-07-30 저녁)

- 서버 GPU 2: 본런(T_p=10) 학습 계속 (wandb `m9lmewr1`, epoch 250+)
- tp16 비교런: **종료** (가설 기각 — 호버링 진범은 실행 타이밍이었음). 체크포인트
  (서버 `23.59.50_*/checkpoints/` ep30–70 + 로컬 `server_ckpts/tp16_ep60.ckpt`)와
  wandb 로그(`0xdfwpdp`)만 보존. GPU 3 반납.
- 로컬 2080: Isaac(fab, headless, **`/sim_pause` 패치 적용본**) + policy_server
  (main ep250, DDIM-16, :5556) 가동 중
- eval 하네스 현행: **stepped(`--stepped`)가 알고리즘 검증 기본**, 겹침 실행은 배포
  리허설용. 하네스 요인(지연·게이트·분포·attach 거리)은 모두 처리·배제 완료.

**완료된 것 (오전 "즉시 할 일" 3개 + α)**:
- 스플라이스 동적 정렬(실측 경과시간 기반 skip), ATTACH_RANGE 0.12→0.09 적용됨

**ft150_grasp6 트랙 종료 (2026-07-30 22:00)**: ep220 stepped 기각 판정에 따라 학습
중단, GPU 2 반납. 체크포인트는 서버 run 디렉토리(ep160–220) + 로컬
`server_ckpts/ft_grasp_ep180/220.ckpt` 보존. 재현 방법: dataset
`grasp_oversample`/`grasp_window` 파라미터(서버·로컬 dataset 클래스에 반영됨) +
ep150을 latest.ckpt로 심은 run 디렉토리에 `hydra.run.dir` 지정 + `resume=True`.
다음 갈래(② obs stride 재학습)는 **lowdim 진단 결과로 게이트**: lowdim(특권 상태)이
파지를 성공시키면 병목은 시각 인코더의 공간 정밀도(→ stride보다 해상도/crop이 정답일
수 있음), lowdim도 같은 지점에서 막히면 시각이 아니라 정책 정식화/데이터 커버리지
문제라 ② 기대값이 낮음.
- ep250 stepped 평가까지 완료 → **결론: 지연 0에서도 0/10, 정상 파지 1회(5.3cm)**.
  하네스가 아니라 **정책/데이터 정밀도가 태스크 요구치(cm급) 미달**로 확정.
- 데이터 처방 1차 시도: 파지 구간 ×6 오버샘플(`grasp_oversample`) +30ep →
  질적 변화 없음 (결과 표 ft150_grasp6). ep220에서 재판정 1회 예정.

**다음 (결정 실험 순)**:
1. **lowdim 진단** (`diffusionpolicy/request.md`, 별도 세션·2080·eval 안 도는 시간):
   박스 pose를 숫자로 직접 주면 정밀도가 나오나? → 시각 인코더 vs action/데이터 분리
2. lowdim 성공 시: 시각 쪽 처방 (crop/해상도/인코더 용량, grasp 오버샘플 재튜닝)
3. lowdim도 실패 시: action 표현/T_o/데이터 밀도 수사
- 보류(원인 분리 전엔 무의미): 체크포인트 스윕, T_a 스윕, DDIM-16 vs DDPM-100 품질
  비교(아직 미실측), 저층 보드 0.72m eval, 실기 검토
- 서버 정리 약속: 전부 끝나면 checkpoint 회수 후 `/hdd/workspace/temp` 삭제
