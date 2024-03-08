from trust.model import model_regression_metrics
from trust.model import model_classification_metrics
from trust.model import model_abstraction

from pydantic import Field
from typing import Annotated, Literal

from trust import Dto

class ModelDtoBase(Dto):
    pass
class ModelDtoClassification(model_abstraction.ModelAbstractionDto):

    metrics: model_classification_metrics.ModelClassificationMetricsDto

    type: Literal["classification"]


class ModelDtoRegression(model_abstraction.ModelAbstractionDto):

    metrics: model_regression_metrics.ModelRegressionMetricsDto

    type: Literal["regression"]



ModelDto = Annotated[
     ModelDtoClassification | ModelDtoRegression,
     Field(discriminator="type")
 ]
 
