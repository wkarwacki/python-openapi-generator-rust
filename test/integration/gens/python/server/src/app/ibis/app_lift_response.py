from pydantic import BaseModel
from trust import TypeMapping
from trust.ibis.lift_bucket import LiftBucketDto
from trust.ibis.lift_response import LiftResponseDto

class AppLiftResponse(BaseModel, TypeMapping[LiftResponseDto, 'AppLiftResponse']):
    buckets: int

    @classmethod
    def of(cls, dto: LiftResponseDto) -> 'AppLiftResponse':
        return AppLiftResponse(
            buckets=len(dto.buckets)
        )

    def to(self) -> LiftResponseDto:
        return LiftResponseDto(
            buckets=[LiftBucketDto.get() for _ in range(self.buckets)]
        )

