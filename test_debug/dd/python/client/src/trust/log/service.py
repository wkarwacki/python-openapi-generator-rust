
from typing import Generator
from trust import client

class LogService:

    def get_experiment_logs(self, experiment_id: experiment_id.ExperimentIdDto) -> Generator[str, None, None]:
        query_params = {  }
        response = client.get("/experiments/{experimentId}/logs".format(experimentId = experiment_id), params = query_params)
        return response

    def get_prediction_logs(self, prediction_id: prediction_id.PredictionIdDto) -> Generator[str, None, None]:
        query_params = {  }
        response = client.get("/predictions/{predictionId}/logs".format(predictionId = prediction_id), params = query_params)
        return response

