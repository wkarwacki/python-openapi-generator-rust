from trust.api import server_event
from abc import ABC, abstractmethod

class ApiService(ABC):
    

    @abstractmethod
    def subscribe_to_sse(self) -> server_event.ServerEventDto:
        raise NotImplementedError

