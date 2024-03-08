from .service import ModelService

from fastapi import APIRouter, Depends, Query
from fastapi.encoders import jsonable_encoder
from fastapi.responses import JSONResponse

from typing import Annotated

model_router = APIRouter()


@model_router.get(
    "/models"
)
def getModels(experiment_id: Annotated[experiment_id.ExperimentIdDto, Query(alias = "experimentId")], service: ModelService = Depends(ModelService)) -> JSONResponse:
    content = service.get_models(experiment_id)
    content = jsonable_encoder(content)
    response = JSONResponse(content=content)
    return response

