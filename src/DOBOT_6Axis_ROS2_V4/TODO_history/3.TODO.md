# TODO

> 이전 작업(세그먼트 pick-and-place)은 완료되어 `cbirrt_pick_place.py`에 있음(놓기 높이 미세조정은 보류). 본 TODO는 새 활성 작업.

## Goal
로봇 팔끝(Link6) 도달 영역(reachability map)을 pinocchio FK로 계산해 (1) RViz `PointCloud2`로 시각화하고 (2) 오프라인 분석용 파일로 저장하는 검증 스크립트 `reachability_map.py`를 만든다. 관절 범위를 인자화하여 실로봇 관절한계각 조정·파지/놓기 위치 결정에 활용한다. 한 번의 샘플링 패스로 "방향 무관 도달 영역"과 "그리퍼 다운 도달 영역" 두 클라우드를 생성한다.

## Tasks

### 1. 준비
- [x] 새 스크립트 `reachability_map.py` 골격: rclpy 노드, pinocchio 모델 재사용(ConstrainedPlanner 또는 직접 로드)
- [x] 파라미터화: 관절 범위(기본 = test_w_gripper의 J1~J6 한계각), 샘플 수 N(기본 300000), voxel 크기(기본 0.02 m), 다운 판정 틸트 허용오차(기본 15°)
- [x] pinocchio collision 셋업: URDF mesh 경로 resolve해 GeometryModel(COLLISION) 로드, 인접/항상충돌 쌍 오탐 방지를 위해 SRDF(disabled collision pairs) 적용

### 2. 샘플링 + FK + self-collision 필터
- [x] J1~J5 Monte-Carlo 균일 샘플(J6는 위치·다운성에 무영향이라 제외; 그리퍼 관절은 neutral 고정)
- [x] 각 샘플 FK로 Link6 위치 p와 회전 R 계산
- [x] pinocchio `computeCollisions`로 self-collision 검사 → **충돌나는 자세는 버림**(관절한계 + 무충돌 자세만 도달로 인정)
- [x] 무충돌 샘플만 all 집합에 p 기록 / 접근축(tool z = R[:,2])이 −Z와 허용오차 내면 down 집합에 p 기록

### 3. Voxel dedupe
- [x] all·down 각각 voxel 그리드(기본 2cm)로 점유 dedupe → 점 수 bounded

### 4. 시각화 publish
- [x] `/reach_all`, `/reach_down` 두 토픽에 `PointCloud2`(QoS transient_local, frame_id=base_link) latched publish
- [x] RViz에 PointCloud2 Display 추가해 확인(고정 프레임 base_link) — publish/latched 수신 검증 완료, RViz 육안 확인은 사용자 몫

### 5. 오프라인 분석용 파일 저장
- [x] all/down 점 구름을 파일로 저장: CSV(x, y, z, down 플래그)로 저장(어디서든 로드 가능)
- [x] PCD로도 저장(`pcl_viewer`/CloudCompare/open3d로 바로 열람)
- [x] 저장 위치·파일명(타임스탬프 + 사용한 관절범위 메타) 정해서 재현 가능하게

### 6. 검증
- [x] sanity check: 마커 (0.2,0.35) 부근이 down 집합에 포함되고, (0.08,0.08)은 down 집합에서 희박/없음(앞서 IK 도달 불가와 일치)
- [x] self-collision 필터 on/off 비교로 걸러진 자세 수·영역 차이 확인(필터 동작 검증)
- [x] 관절 범위를 바꿔 재실행 시 도달 영역이 그에 맞게 변하는지 확인

## 참고사항
- **J6 제외**: 손목 롤은 Link6 원점 위치도, 접근축의 −Z 정렬(다운성)도 바꾸지 않음(롤=툴축 회전). → 5D 샘플.
- **시각화는 RViz PointCloud2**: Gazebo에 점마다 모델 spawn은 대량에서 크래시 → 사용 안 함.
- **저장**: CSV(범용) + PCD(점구름 표준) 둘 다. 오프라인에서 도달 부피·z-슬라이스·한계각 비교 분석에 사용.
- **down 기준이 핵심**: 파지·놓기 모두 그리퍼 다운 필요. all은 전체 엔벨로프 참고용. (접근/이동은 방향 무관이나 보통 제약 아님.)
- **샘플링=Monte-Carlo**: 비용/밀도를 N으로 단순 제어, voxel化와 호환. 한계각 변경 시 N 그대로 재실행.
- **계산은 sim 불필요**(순수 FK + 충돌체크). 단 RViz로 보려면 RViz·robot_state_publisher가 떠 있어야 base_link TF 표시됨.
- **self-collision 필터 포함(v1, 선택 B)**: pinocchio GeometryModel + `computeCollisions`. URDF mesh 경로 resolve와 SRDF disabled-pair 적용 필요. 샘플당 충돌체크라 느려짐(그래서 기본 N=300000; 필요시 조정). 환경(테이블/마커) 충돌은 여전히 미고려(순수 self-collision).
- pinocchio reduced model(그리퍼 관절 lock)·Link6 프레임은 기존 constrained_cbirrt.py와 동일하게 사용. 단 충돌 geometry는 full 모델 기준(그리퍼 neutral 고정)으로 로드.
