from trust.prediction import prediction_id
from trust.prediction import prediction_results_row
from trust.prediction import prediction
from trust.prediction import run_prediction_request
from trust.prediction import prediction_summary
from abc import ABC, abstractmethod

class PredictionService(ABC):
    

    @abstractmethod
    def get_predictions(self, experiment_id: experiment_id.ExperimentIdDto) -> list[prediction_summary.PredictionSummaryDto]:
        raise NotImplementedError
    

    @abstractmethod
    def run_prediction(self, run_prediction_request_dto: run_prediction_request.RunPredictionRequestDto) -> prediction_id.PredictionIdDto:
        raise NotImplementedError
    

    @abstractmethod
    def get_prediction(self, prediction_id: prediction_id.PredictionIdDto) -> prediction.PredictionDto:
        raise NotImplementedError
    

    @abstractmethod
    def get_prediction_results(self, prediction_id: prediction_id.PredictionIdDto) -> list[prediction_results_row.PredictionResultsRowDto]:
        raise NotImplementedError

