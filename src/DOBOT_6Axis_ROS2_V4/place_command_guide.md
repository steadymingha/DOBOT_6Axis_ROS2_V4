## A. 관절 토크 내장 충돌 감지 touch-off

관절 토크 내장 충돌 감지 기반. 손목 F/T 센서 불필요, CR7V 내장 기능 사용. 표면 높이·물린 매거진 높이 몰라도 됨.

**사전 설정 (큐 명령, 하강 전 1회)**

- `SetPayload(...)` — 잡은 매거진 상태로 갱신. 미갱신 시 예측 토크 vs 실측 편차가 어긋나 오검(헛트립)/미검 발생.
- `SetCollisionLevel(level)` — level [0~5], 0=off, 클수록 민감. N 단위 아니라 감도 레벨이라 실기 튜닝.
- `SetPostCollisionMode(1)` — 1=pause. stop보다 복구 부드러움.
- `SetBackDistance(distance)` — 충돌 후 원경로 후퇴 거리 [0~50]mm. 0이나 소량. 후퇴 시 릴리스 직전 박스 살짝 뜸.

**시퀀스**

1. -Z(툴프레임)로 표면보다 아래 지점 목표로 아주 느린 MovL 하강. 접촉이 목표 도달 전 트립되게 over-travel.
2. RT 피드백에서 `RobotMode`==11(COLLISION) 또는 `CollisionState`(1038)==1 감지.
3. 그리퍼 열기.
4. `ClearError`로 충돌 상태 해제.
5. 상승.

**함정**

- SetPayload 정확도에 민감 — 3kg 매거진 물린 걸 안 넣으면 모델 불일치.
- 충돌 트립 = protective stop 감속이라 급함. 접촉 순간 오버슈트만큼 박스가 표면 누름. 접근 속도 최대한 낮춰야 힘 스파이크 작아짐.
- 본질이 "닿을 때까지 눌러 트립"이라 접촉힘 0 아님. 놓기엔 무방하나 제로힘 착지 아님.