
from .service import DevService

from fastapi import APIRouter, Depends, Response


dev_router = APIRouter()


@dev_router.get(
    "/health", status_code=204, response_class=Response
)
def getHealth(service: DevService = Depends(DevService)) -> None:
    service.get_health()
