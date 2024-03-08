from trust.prediction import prediction
from trust.prediction import prediction_id
from trust.prediction import prediction_results_row
from trust.prediction import run_prediction_request
from trust.prediction import prediction_summary
from trust import client

class PredictionService:

    def get_predictions(self, experiment_id: experiment_id.ExperimentIdDto) -> list[prediction_summary.PredictionSummaryDto]:
        query_params = { "experimentId": experiment_id }
        response = client.get("/predictions".format(), params = query_params)
        return response

    def run_prediction(self, run_prediction_request_dto: run_prediction_request.RunPredictionRequestDto) -> prediction_id.PredictionIdDto:
        query_params = {  }
        response = client.post("/predictions".format(), params = query_params, )
        return response

    def get_prediction(self, prediction_id: prediction_id.PredictionIdDto) -> prediction.PredictionDto:
        query_params = {  }
        response = client.get("/predictions/{predictionId}".format(predictionId = prediction_id), params = query_params)
        return response

    def get_prediction_results(self, prediction_id: prediction_id.PredictionIdDto) -> list[prediction_results_row.PredictionResultsRowDto]:
        query_params = {  }
        response = client.get("/predictions/{predictionId}/results".format(predictionId = prediction_id), params = query_params)
        return response

