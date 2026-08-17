# 실물 CR7 핸드-아이 캘리브레이션 — 절차와 함정

2026-08-07. 손목에 장착한 D405의 **`T_flange_cam`**(플랜지 기준 카메라 위치·자세)을
구하는 절차. 이게 없으면 비전이 내놓는 카메라 좌표를 로봇 좌표로 옮길 수 없다.

```
T_base_타겟 = T_base_플랜지 (FK, 이미 정확)
            × T_플랜지_카메라 (← 이 문서가 구하는 것)
            × T_카메라_타겟 (비전 출력)
```

**함께 읽을 것**

- `real_robot_joint_convention.md` — URDF와 컨트롤러의 J1/J5/J6 부호가 반대다.
  캘리브레이션은 FK가 맞다는 전제 위에서 돌아가므로 이 수정이 선행되어야 한다.
- `real_robot_p1p2_test.md` — 같은 로봇의 모션 쪽 도구.

---

## 0. 현재 결과와 그 한계

`~/realsense-ros/cal/handeye_result.json` (2026-08-07 07:53):

```
method                PARK
target                charuco  11x8, square 15.0 mm, marker 11.0 mm, DICT_4X4_50
n_samples             20
rotation_spread_deg   48.0
spread_pos_mm         2.389
spread_rot_deg        0.702
T_flange_cam 위치     x=-6.49  y=-84.28  z=+23.21  mm
```

**`spread_pos_mm = 2.389`을 정확도로 읽으면 안 된다.** 이 값은 *채택된 20개 샘플*
안에서의 자기일관성이다. 수집한 샘플은 **48개**였고 그중 28개가 버려졌는데, 수집
도중 보드가 움직였기 때문이다. 보드가 고정이라는 전제가 깨진 구간이 있었다는 뜻이고,
남은 20개도 그 영향에서 완전히 자유롭다고 볼 근거가 없다.

**실사용 정확도는 8~15 mm 수준으로 보는 것이 안전하다.** 이 값을 2.4 mm로 알고
쓰면 나중에 "왜 자꾸 빗나가지"로 돌아온다. 정밀도가 필요해지면 **보드를 확실히
고정하고 처음부터 다시 수집**할 것.

## 1. URDF 값은 대체재가 아니다

`cr7_on_mpo700.urdf.xacro`의

```xml
<joint name="d405_joint" type="fixed">
  <origin xyz="0.071 0 0.147" rpy="0 0 0"/>
  <parent link="gripper_base_link"/>
</joint>
```

두 가지 문제가 있다.

1. **명목값(추측)이다.** 실물 장착 오차가 그대로 남는다.
2. **부모 링크가 `gripper_base_link`인데 이 로봇엔 그리퍼가 없다.** 카메라 홀더만
   달려 있다. 즉 존재하지 않는 물건을 기준으로 정의돼 있다.

그래서 캘리브레이션 결과는 **`Link6`(플랜지) 기준**으로 쓴다. `handeye_calib.py`가
내놓는 `T_flange_cam`이 바로 그것이다.

## 2. 도구

| 파일 | 역할 |
|---|---|
| `~/realsense-ros/handeye_calib.py` | 본체. selftest / gen-board / collect / solve / verify |
| `~/realsense-ros/run_handeye.sh` | 컨테이너 안에서 실행해주는 래퍼 |
| `~/realsense-ros/board_preview.py` | 라이브 검출 뷰어 (젯슨 모니터에 창) |
| `~/realsense-ros/board_depth_check.py` | 보드 사양 검증 — 깊이 교차검증 + 재투영 |
| `~/realsense-ros/board_grid_check.py` | 일반 체커보드로 읽히는지 확인 (결과: 안 됨) |

`run_handeye.sh`가 하는 일은 셋뿐이다:

```
1. 스크립트와 상태 파일을 docker cp 로 ros2_dobot 컨테이너에 복사
2. 컨테이너 안에서 ROS 환경 source 하고 실행
3. 결과 파일(JSON/PNG)을 호스트로 다시 복사
```

**아무것도 설치하지 않고, 컨테이너를 만들거나 재시작하거나 설정을 바꾸지 않는다.**
`~/realsense-ros`는 컨테이너에 마운트돼 있지 않아서(`~/dobot_ws`만 마운트됨) 이렇게
넣었다 뺀다. 컨테이너에 이미 cv2 4.5.4, numpy, rclpy, cv_bridge, realsense2_camera가
있으므로 추가 설치가 필요 없다.

첫 인자가 `.py`로 끝나면 그 스크립트를 대신 실행한다. `HANDEYE_PY`로 인터프리터를
바꿀 수 있다(`probe_euler.py`는 `/root/dobot_ws/.venv`의 pinocchio가 필요).

## 3. 보드 — legacy 배치 함정

### 3.1 사양

calib.io 생성기 기준:

```
8 x 11,  Checker Size 15 mm,  Marker Size 11 mm,  Dictionary ArUco DICT_4X4
start id 0,  charuco legacy  ← 반드시 체크
```

D405는 근거리 카메라(최적 10~50 cm)라 큰 보드는 화각에 안 들어온다. 위 크기는
120 × 165 mm로 25~30 cm 거리에서 적당하다.

인쇄·부착:

- **100% 배율**로 인쇄 ("용지에 맞춤" 끌 것)
- **무광** 용지 (광택은 반사로 코너가 뭉갠다)
- **평평하고 단단한 판에** 부착. 종이가 조금만 휘어도 그대로 오차
- 인쇄 후 **캘리퍼로 실측**. 여러 칸에 걸쳐 재서 나누는 편이 정확하다

하단 캡션(`www.calib.io | 8x11 | ...`)은 패턴 바깥 여백이라 무해하다. 오히려
나중에 보드 사양을 되짚을 때 유용하다.

### 3.2 legacy를 반드시 체크해야 하는 이유

**OpenCV 4.6에서 ChArUco의 마커 배치 규약이 바뀌었다.** 컨테이너의 cv2는 **4.5.4**로
그 이전 버전이라 **legacy 배치만** 안다. 새 배치로 뽑은 보드를 넣으면:

```
마커 44/44 검출        ← 전부 찾음
코너 70/70 검출        ← 전부 찾음
재투영 오차 24 px      ← 1 px 미만이어야 정상
PnP 거리 21.5 cm       vs  깊이 실측 25.8 cm
```

**검출은 "성공"하는데 자세가 틀린다.** 코너 개수만 보면 완벽해 보이므로 이 실패는
눈에 안 띈다. 2026-08-06에 실제로 이 상태로 진행할 뻔했다.

호스트의 cv2는 4.13이라 새 배치를 알지만, 그건 **다른 담당자의 `venv_ammr`** 이고
ROS도 없다. 컨테이너에 최신 OpenCV를 설치하는 것도 "설치 금지" 원칙에 걸린다.
그래서 **보드를 legacy로 맞추는 것**이 옳은 해결이다.

### 3.3 보드가 맞는지 확인

```bash
cd ~/realsense-ros
./run_handeye.sh board_depth_check.py
```

**통과 기준**

- **재투영 오차 1 px 미만**
- **PnP 거리 ≈ 깊이 실측** (몇 mm 이내)

두 값이 어긋나면 배치나 사각 크기가 틀린 것이다. `reproj_8x11.png` /
`reproj_11x8.png`가 함께 저장되니 열어보면 검출(빨강)과 재투영(녹색)이 어떻게
어긋나는지 바로 보인다.

위치·조명을 잡을 때는 라이브 창을 쓴다:

```bash
./run_handeye.sh board_preview.py --squares 8x11 --square-mm 15 --marker-mm 11
```

`q`/`Esc`로 종료. 마커 수·코너 수·보드 거리가 실시간으로 표시되고, 코너가 절반
이상이면 초록으로 바뀐다.

## 4. 절차

### 4.0 사전 확인

```bash
./run_handeye.sh selftest        # 하드웨어 없이 도는 오프라인 점검
```

### 4.1 카메라 드라이버 (전용 터미널)

```bash
docker exec -it ros2_dobot bash -lc "source /opt/ros/humble/setup.bash && \
  ros2 launch realsense2_camera rs_launch.py camera_name:=d405 align_depth.enable:=true"
```

### 4.2 수집

```bash
./run_handeye.sh collect --target charuco --squares 8x11 --square-mm 15 --marker-mm 11
```

`SPACE` 캡처, `U` 직전 취소, `Q`/`ESC` 종료.

**로봇에 명령을 보내지 않는다.** 손으로(또는 조그로) 팔을 옮기면 스크립트는 지켜보다
기록만 한다. 플랜지 자세는 컨트롤러 실시간 피드백(포트 30004)에서 읽는다.

**자세를 10~20개, 손목 회전을 크게 섞어서** 잡을 것. 핸드-아이는 순수 병진만으로는
수학적으로 풀리지 않는다(회전 다양성이 없으면 자신 있게 틀린 답이 나온다). `solve`가
`rotation_spread_deg`로 이를 검사한다.

**보드는 절대 움직이면 안 된다.** 한 번이라도 움직이면 그 전후 샘플이 서로 모순되고,
어느 쪽이 맞는지 사후에 알 수 없다. §0의 48개 중 20개만 살아남은 사고가 이것이다.

### 4.3 풀이

```bash
./run_handeye.sh solve            # -> handeye_result.json
```

다섯 가지 OpenCV 방법(Tsai/Park/Horaud/Andreff/Daniilidis)을 모두 풀어
**보드 위치의 흩어짐(mm)** 으로 순위를 매긴다. 대수적 잔차가 아니라 mm인 이유는
그게 실제로 겪게 될 오차이기 때문이다.

사각 크기를 잘못 넣었더라도 **원시 코너가 저장돼 있으므로 재수집 없이** 다시 풀 수
있다:

```bash
./run_handeye.sh solve --square-mm 14.8
```

### 4.4 검증 — 이걸 건너뛰지 말 것

```bash
./run_handeye.sh verify
```

`handeye_result.json`에서 `T_flange_cam`과 보드 사양을 **알아서 읽는다.**

화면에 보드의 base_link 좌표가 실시간으로 뜬다:

```
base_link  x=+0.3421  y=-0.1055  z=+0.0287
```

**팔을 조그하면서 이 숫자가 움직이지 않아야 한다.** 보드는 실제로 고정돼 있으므로,
숫자가 흔들리는 폭이 곧 `T_flange_cam`의 오차이고 **실제 타겟에서 그만큼 빗나간다.**
글씨가 초록일 때가 팔이 멈춘 상태이니 그때 값을 비교한다.

이 검사는 참값 없이도 정확도를 재는 유일한 수단이다. 통과 못 하면 그 결과는 못 쓴다.

## 5. 함정 모음

### 5.1 결과 파일 위치

`run_handeye.sh`가 컨테이너로 넣었다 빼는 파일은 **`~/realsense-ros` 바로 아래**에
있는 것뿐이다:

```bash
STATE="handeye_samples.json handeye_result.json board.png preview.png"
```

`cal/` 같은 하위 폴더로 옮기면 `verify`가 파일을 못 찾는다. 보관은 `cal/`에 하되
작업할 때는 루트에 복사해 둘 것:

```bash
cp ~/realsense-ros/cal/handeye_result.json ~/realsense-ros/
```

### 5.2 창이 안 뜬다

컨테이너는 `DISPLAY=:1`로 만들어졌는데 호스트 X 소켓은 `/tmp/.X11-unix/X0`이다.
`:1`에는 아무것도 없어서 `cv2.imshow`가 `Can't initialize GTK backend`로 죽는다.
`run_handeye.sh`가 실행 시 `DISPLAY=:0`으로 덮어쓴다(`HANDEYE_DISPLAY`로 변경 가능).
컨테이너 재생성은 필요 없다.

### 5.3 끝나면 카메라 드라이버를 끌 것

**D405는 한 번에 한 프로세스만 열 수 있다.** 켜둔 채로 두면 비전 담당자의 YOLOX
작업이 막힌다.

```bash
docker exec ros2_dobot bash -lc "pkill -f '[r]s_launch'; pkill -f '[r]ealsense2_camera_node'"
```

`pkill` 패턴을 대괄호로 감싸는 이유는, 그러지 않으면 이 명령을 실행하는 셸 자신의
커맨드라인이 패턴에 걸려 스스로 죽기 때문이다.

### 5.4 `tool_vector_actual`은 활성 좌표계 기준

플랜지 자세는 RT 피드백 offset 624에서 읽고, 회전은 **intrinsic ZYX**
(`Rz@Ry@Rx`)다. 이는 실측으로 확정한 것으로, 2등 후보가 14° 어긋났으므로 추측했다면
조용히 크게 틀렸을 값이다.

단 이 필드는 **펜던트에 설정된 user/tool 좌표계 기준**으로 보고된다. `user=0`,
`tool=0`이 아니면 플랜지 자세가 아니다. 그래서 `collect`가 `q_actual`을 함께
저장한다 — 그 전제가 깨져도 FK로 다시 계산할 수 있게.

### 5.5 `collect`가 캡처를 거부할 때

정상이다. 세 가지를 확인한다:

- 팔이 움직이는 중 → 영상과 플랜지 자세가 **서로 다른 순간**의 것이 된다
- 영상이 1초 이상 오래됨
- 보드 검출 실패

특히 첫 번째는 사후에 티가 나지 않는 오류라 아예 못 찍게 막는다.

## 6. 다음에 할 것

- [ ] **보드를 확실히 고정하고 재수집** — §0의 정확도 한계를 없애려면 이게 우선이다.
      48개 중 28개가 버려진 원인이 보드 이동이었다
- [ ] `verify`로 실제 흔들림 폭을 mm 단위로 기록하고 이 문서 §0에 반영
- [ ] `T_flange_cam`을 비전 파이프라인에 연결 (카메라 좌표 → base_link 변환)
- [ ] 그리퍼 장착 후 카메라 위치가 바뀌면 **전부 다시** 할 것
