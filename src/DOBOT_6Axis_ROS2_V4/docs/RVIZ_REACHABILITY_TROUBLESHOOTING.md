# RViz Reachability 시각화 트러블슈팅 기록

`reachability_map.py`로 publish한 reachability 결과가 **RViz에 전혀 보이지 않던 문제**의
원인과 해결 과정을 정리한다. (작성: 2026-06)

---

## 1. 증상

- `reachability_map.py`(또는 `--load`)를 실행하면 토픽은 정상 생성됨.
- RViz에서 디스플레이를 추가하면 **Status = OK** 인데 **3D 화면에는 아무것도 안 뜸**.
- 다음을 모두 바꿔봐도 동일하게 안 보였다:
  - PointCloud2 + `intensity`(Color Transformer = Intensity)
  - PointCloud2 + `rgb` baking (Color Transformer = RGB8)
  - Marker `POINTS`
  - Marker `CUBE_LIST` (40 cm짜리 큰 큐브도 안 보임)
- 반면 **로봇 모델과 그리드는 정상적으로 보였다.**

## 2. 확인한 사실 (정상이었던 것들)

조사 결과 데이터 경로는 전부 정상이었다.

| 점검 | 결과 |
|---|---|
| 토픽 존재 / `topic hz` | 정상 (3초마다 publish) |
| wire 상의 메시지 내용 | 정상 (점 개수·좌표·색·scale·frame 모두 OK) |
| RViz가 토픽을 **구독** 중인지 (`ros2 topic info -v`) | **구독 중** (Node name: rviz) |
| 좌표 데이터 프레임 | `base_link` (TF 트리에 정상 연결) |
| Marker `color.a` | 1.0로 보정 |

즉 **publish 쪽에는 문제가 없었다.**

## 3. 진짜 원인

**RViz의 Global Options → Fixed Frame이 `dummy_link`로 되어 있었다.**
`dummy_link`는 현재 TF 트리에 **존재하지 않는/연결되지 않은** 프레임이다.

- 데이터는 `base_link` 좌표인데, Fixed Frame이 유효하지 않으니
  RViz가 `base_link → (Fixed Frame)` 변환을 할 수 없어 **모든 디스플레이를 조용히 드롭**했다.
- 이때 디스플레이별 Status는 **OK처럼** 보여서 더 헷갈렸다.
- 로봇 모델/그리드가 보였던 것은 별개 경로(RobotModel/Grid)였기 때문.

> 핵심: **"Status OK인데 아무것도 안 보인다" → 거의 항상 Fixed Frame 문제다.**

### 부수적 요인 (두 번째 원인)

Gazebo가 함께 떠 있어서 RViz는 **`use_sim_time = True`** 로 동작했고,
`/clock`(시뮬레이션 시간, 예: ~180초)을 따랐다.
반면 publisher는 일반 노드라 **벽시계 시간(≈17억 초, Unix epoch)** 으로 stamp를 찍었다.

- Fixed Frame이 **동적 TF 변환을 포함하는 프레임**이면, RViz는 메시지 stamp 시점의 변환을
  찾으려다 **미래 타임스탬프 외삽 실패**로 메시지를 드롭한다.
- Fixed Frame이 `base_link`(데이터와 동일, identity 변환)이면 TF 조회가 필요 없어
  stamp가 무엇이든 그냥 렌더된다 → 그래서 **`base_link`로 바꾸자 즉시 다 보였다.**

## 4. 해결

### (1) RViz Fixed Frame을 `base_link`로 설정  ← 가장 핵심

- Global Options → Fixed Frame = **`base_link`**
- 데이터가 `base_link` 좌표로 계산되므로 좌표 해석도 그대로 맞는다.
- `cube_link`도 가능하지만(정적 변환) 굳이 쓸 이유 없음. `dummy_link`/동적 world 프레임만 피한다.

### (2) publisher를 sim-time으로 생성 (sim 환경 대비, 코드 반영 완료)

```python
from rclpy.parameter import Parameter
node = Node('reachability_map', parameter_overrides=[
    Parameter('use_sim_time', Parameter.Type.BOOL, True)])
```

→ stamp가 RViz의 sim 시계와 일치하여, 동적 TF 프레임을 Fixed Frame으로 써도 안전.

### (3) Marker와 PointCloud2 동시 publish (비교용, 코드 반영 완료)

| 토픽 | 타입 | 색 |
|---|---|---|
| `/reach_all` | Marker (CUBE_LIST) | per-cube RI 색 (파랑→빨강) |
| `/reach_down` | Marker (CUBE_LIST) | 단색 초록 |
| `/reach_all_cloud` | PointCloud2 | `intensity`=RI + baked `rgb` |
| `/reach_down_cloud` | PointCloud2 | baked 초록 `rgb` |

## 5. 다음에 같은 증상이 보이면 (체크리스트)

1. **RViz Fixed Frame부터 확인** (Global Options). 데이터 프레임(`base_link`)과 맞는가?
   존재하지 않는/오래된 프레임(`dummy_link` 등)으로 되어 있지 않은가?
2. 데이터 프레임이 TF 트리에 연결돼 있는가? (`ros2 run tf2_ros tf2_echo <fixed> <data_frame>`)
3. `ros2 topic info -v <topic>` 로 RViz가 실제 **구독** 중인지 확인.
4. Gazebo(sim time) 동반 시: publisher stamp가 sim 시간과 맞는가? (`use_sim_time`)
5. 그래도 안 보이면 그때 색/타입(PointCloud2 vs Marker, transformer) 같은 코드 요소를 의심.

> 교훈: 색·Marker 타입·rgb baking 같은 **코드 요소를 먼저 의심하지 말 것.**
> "Status OK인데 안 보임"의 1순위 용의자는 항상 **Fixed Frame**이다.

## 6. 실행

```bash
pkill -f reachability_map.py
cd /home/user/dobot_ws/src/DOBOT_6Axis_ROS2_V4
/home/user/dobot_ws/.venv/bin/python tools/reachability_map.py --load latest
# RViz: Fixed Frame = base_link, 4개 토픽을 Add → By topic 으로 추가해 비교
```
