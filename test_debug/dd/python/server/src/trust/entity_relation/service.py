from trust.entity_relation import generate_entity_relations_request
from trust.entity_relation import generate_entity_relations_response
from abc import ABC, abstractmethod

class EntityRelationService(ABC):
    

    @abstractmethod
    def generate_entity_relations(self, generate_entity_relations_request_dto: generate_entity_relations_request.GenerateEntityRelationsRequestDto) -> generate_entity_relations_response.GenerateEntityRelationsResponseDto:
        raise NotImplementedError

