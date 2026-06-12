# reachability_map.py 사용 설명서

CR7 6축 팔의 **IK 기반 도달성(reachability) 맵**을 계산·저장·시각화하는 스크립트.
작업공간을 voxel 격자로 나누고, 각 voxel에서 여러 tool orientation에 대해 수치 IK를
풀어 **Reachability Index(RI)** 를 구한다.

---

## 1. 개념

### Method C — Voxel Map
작업공간을 균일 voxel(기본 50mm)로 나눈다. 한 voxel은 **테스트한 orientation 중 하나라도
IK 해(자기충돌 없는)가 있으면** "도달 가능"이다.

### Method D — 6D Reachability Map
각 voxel 중심에서 여러 tool orientation(접근축 방향 × roll)을 테스트한다.

```
RI(voxel) = (IK가 풀린 orientation 수) / (테스트한 전체 orientation 수)
  RI == 1.0  → 모든 자세에서 도달 (Dexterous Workspace)
  0 < RI < 1 → 일부 자세만 도달
  RI == 0    → 도달 불가
```

추가로 **"아래(top-down) 접근이 되는가"** 를 `down` 플래그로 따로 기록한다.

---

## 2. 좌표·TCP·충돌 기준 (필수 이해)

- **프레임**: 모든 좌표는 팔 베이스 **`base_link`** 기준. (+x 정면, +y 왼쪽, +z 위)
- **TCP 위치**: `Link6 원점 + TCP_OFFSET_M × (Link6 z축)`.
  - `TCP_OFFSET_M = 0.12005 m` (OnRobot 2FG7 그리퍼 몸체 길이).
  - **그리퍼 교체 시 이 값만 바꾸면** FK/IK/도달 판정이 일관되게 따라감(z축 일직선 장착 가정).
    충돌 형상까지 정확히 하려면 그리퍼 URDF도 교체해야 함.
- **자기충돌**: IK가 수렴해도 **자기충돌이면 미도달로 처리**(보고되는 점은 관절한계+충돌없음 통과).
  - 그리퍼 **형상은 충돌 검사에 포함**(관절은 neutral 고정 → 손가락 동작은 미고려).
  - 마운트 겹침·SRDF 인접쌍은 허위 충돌 방지로 비활성.
- **"down"의 의미**: 접근축(tool z)이 월드 **-Z(±`down_tol_deg`)** → 그리퍼가 바닥을 향함.
  → 탑다운 grasp 가능 여부는 **`/reach_down` / CSV `down` 열**로 확인.

### orientation 집합
기본은 **전방위**(Fibonacci sphere로 `n_dir`개 접근 방향 × `n_roll` roll). 아래 방향은 그중
하나로 스냅되어 항상 포함되고, `down` flag로 표시될 뿐 — **아래 방향만 쓰는 게 아님**.
탑다운만 보려면 `--n-dir 1`(정확히 -Z 한 방향)로 좁힌다.

---

## 3. 실행

항상 **venv 파이썬**으로 실행(시스템 python엔 pinocchio 없음):

```bash
cd /home/user/dobot_ws/src/DOBOT_6Axis_ROS2_V4
/home/user/dobot_ws/.venv/bin/python reachability_map.py [옵션]
```

### 예시
```bash
# 전체 도달 영역 자동 계산(FK 엔벨로프로 후보 voxel 선정) + 시각화
/home/user/dobot_ws/.venv/bin/python reachability_map.py

# 특정 박스만 고해상도, 헤드리스(계산+저장만)
/home/user/dobot_ws/.venv/bin/python reachability_map.py --no-viz \
  --voxel 0.025 --bounds 0,0.556,-0.8,0,-0.05,0.05

# 탑다운 grasp만(그리퍼 똑바로 아래) roll 4가지
/home/user/dobot_ws/.venv/bin/python reachability_map.py --n-dir 1 --n-roll 4

# 저장된 결과를 재계산 없이 다시 publish
/home/user/dobot_ws/.venv/bin/python reachability_map.py --load latest
```

---

## 4. 옵션 (CLI)

| 옵션 | 기본값 | 설명 |
|---|---|---|
| `--voxel` | `0.05` | voxel 한 변(m) = 3D 샘플 간격(GSD). 작게 = 정밀·느림(절반→~8배 점) |
| `--bounds` | (없음) | `xmin,xmax,ymin,ymax,zmin,zmax`(m, base_link). 생략 시 FK 엔벨로프 자동 |
| `--n-dir` | `12` | 접근축(tool z) 방향 수(구 전체 분포). `1`이면 똑바로 아래만 |
| `--n-roll` | `2` | 방향당 roll 각도 수. 총 orientation = n_dir × n_roll |
| `--down-tol-deg` | `15` | "아래(down)"로 칠 기울기 허용 각도 |
| `--seed-restarts` | `4` | orientation당 랜덤 IK 시드 재시작 수(↑ = recall↑, 느림) |
| `--ik-pos-tol` | `0.005` | IK 위치 허용오차(m) |
| `--ik-rot-tol-deg` | `5` | IK 자세 허용오차(deg) |
| `--ik-max-iter` | `100` | 시드당 최대 IK 반복 |
| `--envelope-samples` | `200000` | `--bounds` 없을 때 후보 voxel용 FK 샘플 수 |
| `--jobs` | CPU 코어 수 | IK 병렬 프로세스 수. `1`=직렬(재현/디버그) |
| `--max-reach` | 자동 | reach 가지치기 반경(m). 기본=모델 링크 길이 상한(보수적, 안전) |
| `--limits-deg` | 내장값 | 관절한계 12값 `j1lo,j1hi,...,j6lo,j6hi`(deg) |
| `--seed` | `1` | RNG 시드 |
| `--out-dir` | `reachability_out/` | CSV/PCD/JSON 출력 폴더 |
| `--load` | (없음) | 저장 CSV 재-publish(`경로` 또는 `latest`). 모델/계산 생략 |
| `--no-viz` | off | ROS publish 생략(계산+저장만) |
| `--frame-id` | `base_link` | publish 프레임 |

---

## 5. 출력물 (`--out-dir`, 타임스탬프)

- **CSV** `reach_*.csv`: `x,y,z,ri,n_ok,n_total,down` + 하단에 높이별 최원거리 요약(`# z_m,max_dist...`).
- **PCD**: `_all`(RI>0), `_down`(아래 도달), `_dex`(RI==1).
- **JSON** `_meta.json`: 한계·voxel·orientation·IK tol·RI 통계.

### 시각화 토픽 (3초마다 republish)
| 토픽 | 타입 | 색 |
|---|---|---|
| `/reach_all` | Marker(CUBE_LIST) | RI 색(파랑→빨강) |
| `/reach_down` | Marker(CUBE_LIST) | 단색 초록 |
| `/reach_all_cloud` | PointCloud2 | intensity=RI + baked rgb |
| `/reach_down_cloud` | PointCloud2 | baked 초록 |

---

## 6. ⚠️ RViz 필수 주의

- **Fixed Frame을 반드시 `base_link`로** 설정. `dummy_link`(존재X) 등으로 두면
  **Status는 OK인데 아무것도 안 보임**. (상세: `RVIZ_REACHABILITY_TROUBLESHOOTING.md`)
- Marker는 Color Transformer 불필요(색 내장). PointCloud2는 Intensity 또는 RGB8 선택 가능.
- Gazebo(sim time) 동반 시 publisher가 `use_sim_time=True`로 stamp를 맞춤(코드 반영됨).
- 계산만 할 땐 시뮬레이터 불필요. 보기만 하려면 robot TF(base_link) 제공하는 RViz면 됨.

---

## 7. 정확도/속도 손잡이

| 목적 | 조절 |
|---|---|
| 공간 해상도(경계 정밀도) | `--voxel` ↓ |
| 자세 해상도(RI 값) | `--n-dir`, `--n-roll` ↑ |
| IK 누락(false negative) 감소 | `--seed-restarts` ↑ |
| 속도 | `--jobs`(멀티코어), `--bounds`로 영역 축소, `--max-reach`로 가지치기 강화 |

### 성능 메모
- voxel 루프는 `--jobs` 프로세스로 병렬화(워커당 모델 1회 빌드). 직렬과 **결과 동일**(voxel별
  결정적 시드 + voxel 내 warm-start만 사용 → 코어 수 무관 재현).
- Reach 가지치기: 모델 링크 길이 합(삼각부등식 상한)+여유 밖 voxel은 IK 없이 RI=0
  (도달 가능 voxel은 수학적으로 안 잘림). 기본 상한은 보수적이라 좁은 박스엔 효과가 적으며,
  실제 도달(~0.92m)을 알면 `--max-reach 0.95`로 안전하게 더 쳐낼 수 있음.
- 진행 로그에 `elapsed / ETA`, 완료 시 `Elapsed Time`(voxel/s) 출력.

---

## 8. 한계/주의

- 수치 IK(지역 최적화)라 어려운 자세를 놓칠 수 있어 **RI는 약간 보수적**(하한). seed restart로 완화.
- 그리퍼를 포함해 계산하므로 "팔끝만"보다 보수적·현실적. 그리퍼 교체 시 `TCP_OFFSET_M`(필요시 URDF)
  갱신.
- 결과 좌표는 `base_link` 기준. 모바일 베이스 위에 장착돼 있으므로 월드 해석 시 TF 변환 유의.
