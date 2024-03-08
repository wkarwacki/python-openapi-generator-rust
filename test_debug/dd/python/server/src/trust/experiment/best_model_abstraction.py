from trust.experiment import best_model_feature
from trust.model import model


from trust import Dto

class BestModelAbstractionDto(Dto):
    

    


    baseline_model: model.ModelDto
    features: list[best_model_feature.BestModelFeatureDto]
    model: model.ModelDto
