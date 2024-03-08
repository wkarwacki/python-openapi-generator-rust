from .service import ApiService

from fastapi import APIRouter, Depends
from fastapi.encoders import jsonable_encoder
from fastapi.responses import JSONResponse


api_router = APIRouter()


@api_router.get(
    "/sse"
)
def subscribeToSse(service: ApiService = Depends(ApiService)) -> JSONResponse:
    content = service.subscribe_to_sse()
    content = jsonable_encoder(content)
    response = JSONResponse(content=content, media_type = "text/event-stream")
    return response

