from trust.model import model
from abc import ABC, abstractmethod

class ModelService(ABC):
    

    @abstractmethod
    def get_models(self, experiment_id: experiment_id.ExperimentIdDto) -> list[model.ModelDto]:
        raise NotImplementedError

