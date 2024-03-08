from trust.experiment import create_experiment_config_request
from trust.experiment import experiment_id
from .service import ExperimentService

from fastapi import APIRouter, Depends, Path, Response
from fastapi.encoders import jsonable_encoder
from fastapi.responses import JSONResponse

from typing import Annotated

experiment_router = APIRouter()


@experiment_router.get(
    "/experiments"
)
def getExperiments(service: ExperimentService = Depends(ExperimentService)) -> JSONResponse:
    content = service.get_experiments()
    content = jsonable_encoder(content)
    response = JSONResponse(content=content)
    return response

@experiment_router.post(
    "/experiments/configs"
)
def createExperimentConfig(request: create_experiment_config_request.CreateExperimentConfigRequestDto, service: ExperimentService = Depends(ExperimentService)) -> JSONResponse:
    content = service.create_experiment_config(request)
    content = jsonable_encoder(content)
    response = JSONResponse(content=content)
    return response

@experiment_router.get(
    "/experiments/configs/{experimentConfigId}"
)
def getExperimentConfig(experiment_config_id: Annotated[experiment_id.ExperimentIdDto, Path(alias = "experimentConfigId")], service: ExperimentService = Depends(ExperimentService)) -> JSONResponse:
    content = service.get_experiment_config(experiment_config_id)
    content = jsonable_encoder(content)
    response = JSONResponse(content=content)
    return response

@experiment_router.get(
    "/experiments/{experimentId}"
)
def getExperiment(experiment_id: Annotated[experiment_id.ExperimentIdDto, Path(alias = "experimentId")], service: ExperimentService = Depends(ExperimentService)) -> JSONResponse:
    content = service.get_experiment(experiment_id)
    content = jsonable_encoder(content)
    response = JSONResponse(content=content)
    return response

@experiment_router.post(
    "/experiments/{experimentId}", status_code=204, response_class=Response
)
def runExperiment(experiment_id: Annotated[experiment_id.ExperimentIdDto, Path(alias = "experimentId")], service: ExperimentService = Depends(ExperimentService)) -> None:
    service.run_experiment(experiment_id)
@experiment_router.get(
    "/experiments/{experimentId}/best-model"
)
def getBestModel(experiment_id: Annotated[experiment_id.ExperimentIdDto, Path(alias = "experimentId")], service: ExperimentService = Depends(ExperimentService)) -> JSONResponse:
    content = service.get_best_model(experiment_id)
    content = jsonable_encoder(content)
    response = JSONResponse(content=content)
    return response

