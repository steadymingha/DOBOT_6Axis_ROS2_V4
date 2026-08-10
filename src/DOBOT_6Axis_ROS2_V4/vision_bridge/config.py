"""vision_bridge 설정 — 컨테이너(ros2_dobot) 쪽.

호스트의 vision_runner 가 카메라를 소유하고 ZMQ 로 결과를 낸다. 여기서는 그 결과를
로봇 base 좌표로 변환하고 검증만 한다. 로봇에는 아무것도 보내지 않는다 (30004 read-only).
"""
import os

HERE = os.path.dirname(os.path.abspath(__file__))
HANDEYE_DIR = os.path.abspath(os.path.join(HERE, "..", "handeye_calib"))
HANDEYE_RESULT = os.path.join(HANDEYE_DIR, "handeye_result.json")

# ── 러너 ──────────────────────────────────────────────────────────
# 컨테이너가 --network host 로 떠 있어 호스트 프로세스가 loopback 으로 보인다.
RUNNER_HOST = os.environ.get("RUNNER_HOST", "127.0.0.1")
PUB_PORT = 5555            # 결과 + 하트비트
REP_PORT = 5556            # 모드 커맨드
REQ_TIMEOUT_MS = 3000
SILENT_AFTER_S = 3.0       # 마지막 PUB 수신 후 이만큼이면 "runner silent"

# ── 로봇 ──────────────────────────────────────────────────────────
ROBOT_IP = "192.168.5.1"   # 30004 실시간 피드백만. 29999(명령 포트) 접속 금지
FEED_MAX_AGE_S = 0.5       # 피드가 이보다 오래되면 없는 것으로 취급

# ── 프레임 크기 ───────────────────────────────────────────────────
# 러너 해상도. source: realtime_app_jetson.py IMG_W/IMG_H (1280x720)
# 페이로드에 해상도 필드가 없으므로 여기 값을 쓴다.
FRAME_W = 1280
FRAME_H = 720

# ── 채택 게이트 ───────────────────────────────────────────────────
GATE_VALID_PCT_MIN = 70.0
# 70 → 80 으로 올렸다가 되돌렸다. 이유는 통계가 아니라 실현 가능성이다:
#   실제 작업 자세(cam z 0.20m)에서 747프레임을 재보니 valid_pct 가 68~80% 에 머물러
#   80 기준 통과율이 0% 였다. 도달 불가능한 값은 운용 게이트가 될 수 없다.
#   valid_pct 는 거리보다 "보는 각도"에 좌우된다 (같은 매거진이 18cm 89~92%,
#   25cm 94%, 20cm 68~80%). D405 는 프로젝터 없는 수동 스테레오라 조명·텍스처가 곧 이 값이다.
# ★ "낮은 valid 가 잔차 꼬리를 만든다"는 가설은 run1/run2 를 보고 세운 것이므로
#   그 데이터로 재채점하면 안 된다. run3(이 설정으로 새로 수집)에서
#   valid_pct ↔ 잔차 상관을 검정해 재현되면 그때 문턱을 올릴 것.
GATE_SD_CM_MAX = 0.25      # ring 평면 피팅 잔차. 0.5 는 첫 어림값이었고,
                           #   이 값이 클수록 전면 평면 추정이 흔들려 Z 가 직접 틀어진다
GATE_EDGE_MARGIN_PX = 10   # bbox 네 변이 프레임 경계에서 최소 이만큼
GATE_Z_MIN = 0.10          # 예상 거리 범위 (m, 카메라 광학 프레임 Z)
#   상한 0.30: 그 너머는 D405 depth 정확도와 모델 confidence 가 함께 떨어진다
#   (37cm 실측에서 score 0.66 < 임계 0.80 으로 검출 자체가 끊겼다)
GATE_Z_MAX = 0.30
# 스냅샷 = 게이트를 연속으로 통과한 같은 track_id 의 프레임 GATE_SNAPSHOT_N 장 평균.
#   검증(verify_chain)과 운용(FSM)이 같은 정의를 써야 검증이 의미를 가진다.
#   ★ 평균은 카메라 프레임에서 이뤄지므로 그 10장 동안 팔이 정지해 있어야 한다.
#     호출부가 정지 판정을 하고, 움직이면 Gate.reset() 을 불러 누적을 버려야 한다.
#   연속성 요구(초안 5)는 이 값에 흡수됐다 — 10 연속이면 5 연속을 포함한다.
GATE_SNAPSHOT_N = 10

# 크기 교차검증 — 기본 비활성.
#   비전담당자 쪽 SIZE_K 가 front_grill1/front_solid 를 한 그룹으로 묶고 있는데
#   실측이 25% 어긋난다 (front_grill1 관측 K_w≈51 vs 설정 69). 클래스 분리가
#   끝나면 True 로 켤 것.
GATE_SIZE_CHECK = False
GATE_SIZE_TOL = 0.10       # |K/px - z| / z 허용치

# source: realtime_app_jetson.py SIZE_K / CLASS_GROUP (1280x720 기준 실측)
#   컨테이너에서는 그 모듈을 import 할 수 없어(호스트 전용 의존) 값을 옮겨 적었다.
#   ★ 저쪽이 바뀌면 여기도 같이 고쳐야 한다. GATE_SIZE_CHECK 를 켤 때 반드시 대조할 것.
SIZE_K = {
    "front": {"w": 69.0, "h": 121.0},
    "side": {"w": 161.0, "h": 95.6},
}
CLASS_GROUP = {
    "front_grill1": "front", "front_grill2": "front", "front_solid": "front",
    "side_grill1": "side", "side_grill2": "side", "side_solid": "side",
}

# ── 검증 세션 포락선 ──────────────────────────────────────────────
# AMR 정차 오차 수준으로 자세 변동을 제한한다. ★ Hils 스펙 수령 후 갱신할 것.
#
# ★ 이 좁은 포락선에서는 사슬 오류가 드러나지 않는다 — 자세가 거의 안 바뀌면
#   곱 순서나 hand-eye 가 틀려도 P_base 가 나란히 나온다.
#   사슬 정합성은 넓은 자세 세션에서 이미 확인됐다:
#     run1 travel 291mm/66°, run2 270mm/79° → 두 세션 평균이 2.45mm 안에서 일치.
#   따라서 이 세션이 재는 것은 "사슬이 맞는가" 가 아니라
#   "실제 정차 오차 범위 안에서 얼마나 재현되는가" — 정확도 예산용 숫자다.
ENVELOPE_POS_MM = 20.0          # 기준 자세 대비 ±2cm
ENVELOPE_ROT_DEG = 3.0          # ±3°
ENVELOPE_Z_MIN = 0.20           # 정면 부근 작업거리 (m)
ENVELOPE_Z_MAX = 0.25

# 표본 novelty. 포락선이 ±2cm 라 넓은 세션의 3cm/8° 를 그대로 쓰면
# 두 번째 표본이 영원히 안 잡힌다.
NOVELTY_TRANS_M = 0.005         # 5mm
NOVELTY_ROT_DEG = 1.0

# ── 판정 ──────────────────────────────────────────────────────────
# 포락선을 실제로 훑었는지 확인하는 최소치 (포락선 크기에 맞춰 낮춤).
VERIFY_MIN_TRAVEL_MM = 20.0
VERIFY_MIN_ROT_DEG = 2.0
VERIFY_PASS_P95_MM = 10.0       # 산포 p95 가 이보다 작으면 PASS
VERIFY_WANT_SAMPLES = 12        # 판정 조건은 아니지만, 이보다 적으면 p95 가 불안정하다
SAMPLES_FILE = os.path.join(HERE, "verify_samples.json")
