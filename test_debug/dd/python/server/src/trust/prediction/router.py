from trust.prediction import prediction_id
from trust.prediction import run_prediction_request
from .service import PredictionService

from fastapi import APIRouter, Depends, Path, Query
from fastapi.encoders import jsonable_encoder
from fastapi.responses import JSONResponse

from typing import Annotated

prediction_router = APIRouter()


@prediction_router.get(
    "/predictions"
)
def getPredictions(experiment_id: Annotated[experiment_id.ExperimentIdDto, Query(alias = "experimentId")], service: PredictionService = Depends(PredictionService)) -> JSONResponse:
    content = service.get_predictions(experiment_id)
    content = jsonable_encoder(content)
    response = JSONResponse(content=content)
    return response

@prediction_router.post(
    "/predictions"
)
def runPrediction(request: run_prediction_request.RunPredictionRequestDto, service: PredictionService = Depends(PredictionService)) -> JSONResponse:
    content = service.run_prediction(request)
    content = jsonable_encoder(content)
    response = JSONResponse(content=content)
    return response

@prediction_router.get(
    "/predictions/{predictionId}"
)
def getPrediction(prediction_id: Annotated[prediction_id.PredictionIdDto, Path(alias = "predictionId")], service: PredictionService = Depends(PredictionService)) -> JSONResponse:
    content = service.get_prediction(prediction_id)
    content = jsonable_encoder(content)
    response = JSONResponse(content=content)
    return response

@prediction_router.get(
    "/predictions/{predictionId}/results"
)
def getPredictionResults(prediction_id: Annotated[prediction_id.PredictionIdDto, Path(alias = "predictionId")], service: PredictionService = Depends(PredictionService)) -> JSONResponse:
    content = service.get_prediction_results(prediction_id)
    content = jsonable_encoder(content)
    response = JSONResponse(content=content)
    return response

