from trust.experiment import confusion_matrix
from trust.experiment import best_model_abstraction

from pydantic import Field
from typing import Annotated, Literal

from trust import Dto

class BestModelDtoBase(Dto):
    pass
class BestModelDtoClassification(best_model_abstraction.BestModelAbstractionDto):
    class ThresholdToConfusionMatrixItem(Dto):
    
        threshold: float
        value: confusion_matrix.ConfusionMatrixDto

    


    threshold_to_confusion_matrix: list[ThresholdToConfusionMatrixItem | None]

    type: Literal["classification"]


class BestModelDtoRegression(best_model_abstraction.BestModelAbstractionDto):
    pass

    type: Literal["regression"]



BestModelDto = Annotated[
     BestModelDtoClassification | BestModelDtoRegression,
     Field(discriminator="type")
 ]
 
