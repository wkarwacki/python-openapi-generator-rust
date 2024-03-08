from trust.experiment import experiment_config
from trust.experiment import experiment
from trust.experiment import create_experiment_config_request
from trust.experiment import experiment_id
from trust.experiment import best_model
from trust.experiment import experiment_summary
from trust import client

class ExperimentService:

    def get_experiments(self) -> list[experiment_summary.ExperimentSummaryDto]:
        query_params = {  }
        response = client.get("/experiments".format(), params = query_params)
        return response

    def create_experiment_config(self, create_experiment_config_request_dto: create_experiment_config_request.CreateExperimentConfigRequestDto) -> experiment_id.ExperimentIdDto:
        query_params = {  }
        response = client.post("/experiments/configs".format(), params = query_params, )
        return response

    def get_experiment_config(self, experiment_config_id: experiment_id.ExperimentIdDto) -> experiment_config.ExperimentConfigDto:
        query_params = {  }
        response = client.get("/experiments/configs/{experimentConfigId}".format(experimentConfigId = experiment_config_id), params = query_params)
        return response

    def get_experiment(self, experiment_id: experiment_id.ExperimentIdDto) -> experiment.ExperimentDto:
        query_params = {  }
        response = client.get("/experiments/{experimentId}".format(experimentId = experiment_id), params = query_params)
        return response

    def run_experiment(self, experiment_id: experiment_id.ExperimentIdDto) -> None:
        query_params = {  }
        client.post("/experiments/{experimentId}".format(experimentId = experiment_id), params = query_params)

    def get_best_model(self, experiment_id: experiment_id.ExperimentIdDto) -> best_model.BestModelDto:
        query_params = {  }
        response = client.get("/experiments/{experimentId}/best-model".format(experimentId = experiment_id), params = query_params)
        return response

