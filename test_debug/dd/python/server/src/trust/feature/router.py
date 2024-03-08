from trust.feature import feature_id
from .service import FeatureService

from fastapi import APIRouter, Depends, Path, Query
from fastapi.encoders import jsonable_encoder
from fastapi.responses import JSONResponse

from typing import Annotated

feature_router = APIRouter()


@feature_router.get(
    "/features"
)
def getFeatures(experiment_id: Annotated[experiment_id.ExperimentIdDto, Query(alias = "experimentId")], service: FeatureService = Depends(FeatureService)) -> JSONResponse:
    content = service.get_features(experiment_id)
    content = jsonable_encoder(content)
    response = JSONResponse(content=content)
    return response

@feature_router.get(
    "/features/{featureId}/insights"
)
def getFeatureInsights(feature_id: Annotated[feature_id.FeatureIdDto, Path(alias = "featureId")], service: FeatureService = Depends(FeatureService)) -> JSONResponse:
    content = service.get_feature_insights(feature_id)
    content = jsonable_encoder(content)
    response = JSONResponse(content=content)
    return response

