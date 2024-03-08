from trust.prediction import predicted_class
from trust.prediction import predicted_class


from trust import Dto

class PredictionResultsRowDto(Dto):

    predicted_class: predicted_class.PredictedClassDto
    prediction_score: float
    primary_key: dict[str | None, str | None]
