
from abc import ABC, abstractmethod
from typing import Generator

class LogService(ABC):
    

    @abstractmethod
    def get_experiment_logs(self, experiment_id: experiment_id.ExperimentIdDto) -> Generator[str, None, None]:
        raise NotImplementedError
    

    @abstractmethod
    def get_prediction_logs(self, prediction_id: prediction_id.PredictionIdDto) -> Generator[str, None, None]:
        raise NotImplementedError

