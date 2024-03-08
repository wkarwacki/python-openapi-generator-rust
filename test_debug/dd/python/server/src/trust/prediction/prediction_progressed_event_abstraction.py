from trust.task import progress
from trust.prediction import prediction_id
from trust.prediction import prediction_id


from trust import Dto

class PredictionProgressedEventAbstractionDto(Dto):

    id: prediction_id.PredictionIdDto
    progress: progress.ProgressDto
