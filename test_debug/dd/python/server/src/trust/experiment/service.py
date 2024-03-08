from trust.experiment import experiment_config
from trust.experiment import create_experiment_config_request
from trust.experiment import best_model
from trust.experiment import experiment_summary
from trust.experiment import experiment_id
from trust.experiment import experiment
from abc import ABC, abstractmethod

class ExperimentService(ABC):
    

    @abstractmethod
    def get_experiments(self) -> list[experiment_summary.ExperimentSummaryDto]:
        raise NotImplementedError
    

    @abstractmethod
    def create_experiment_config(self, create_experiment_config_request_dto: create_experiment_config_request.CreateExperimentConfigRequestDto) -> experiment_id.ExperimentIdDto:
        raise NotImplementedError
    

    @abstractmethod
    def get_experiment_config(self, experiment_config_id: experiment_id.ExperimentIdDto) -> experiment_config.ExperimentConfigDto:
        raise NotImplementedError
    

    @abstractmethod
    def get_experiment(self, experiment_id: experiment_id.ExperimentIdDto) -> experiment.ExperimentDto:
        raise NotImplementedError
    

    @abstractmethod
    def run_experiment(self, experiment_id: experiment_id.ExperimentIdDto) -> None:
        raise NotImplementedError
    

    @abstractmethod
    def get_best_model(self, experiment_id: experiment_id.ExperimentIdDto) -> best_model.BestModelDto:
        raise NotImplementedError

