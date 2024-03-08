from trust.experiment import confusion_matrix
from trust.experiment import best_model_abstraction


from trust import Dto

class BestModelClassificationDto(best_model_abstraction.BestModelAbstractionDto):
    class ThresholdToConfusionMatrixItem(Dto):
    
        threshold: float
        value: confusion_matrix.ConfusionMatrixDto

    


    threshold_to_confusion_matrix: list[ThresholdToConfusionMatrixItem | None]
