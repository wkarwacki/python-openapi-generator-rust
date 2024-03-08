
from .service import LogService

from fastapi import APIRouter, Depends, Path
from fastapi.responses import StreamingResponse

from typing import Annotated

log_router = APIRouter()


@log_router.get(
    "/experiments/{experimentId}/logs"
)
def getExperimentLogs(experiment_id: Annotated[experiment_id.ExperimentIdDto, Path(alias = "experimentId")], service: LogService = Depends(LogService)) -> StreamingResponse:
    content = service.get_experiment_logs(experiment_id)
    
    response = StreamingResponse(content=content, media_type = "text/plain")
    return response

@log_router.get(
    "/predictions/{predictionId}/logs"
)
def getPredictionLogs(prediction_id: Annotated[prediction_id.PredictionIdDto, Path(alias = "predictionId")], service: LogService = Depends(LogService)) -> StreamingResponse:
    content = service.get_prediction_logs(prediction_id)
    
    response = StreamingResponse(content=content, media_type = "text/plain")
    return response

