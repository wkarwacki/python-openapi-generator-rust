
from abc import ABC, abstractmethod

class DevService(ABC):
    

    @abstractmethod
    def get_health(self) -> None:
        raise NotImplementedError

