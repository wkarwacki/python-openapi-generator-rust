from app.ibis.app_lift_response import AppLiftResponse
from trust.ibis.service import IbisService

class IbisServiceImpl(IbisService):
    def get_lift(self, elephant_id: str) -> AppLiftResponse:
        return AppLiftResponse(buckets=4)
