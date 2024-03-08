from trust.entity_relation import generate_entity_relations_response
from trust.entity_relation import generate_entity_relations_request
from trust import client

class EntityRelationService:

    def generate_entity_relations(self, generate_entity_relations_request_dto: generate_entity_relations_request.GenerateEntityRelationsRequestDto) -> generate_entity_relations_response.GenerateEntityRelationsResponseDto:
        query_params = {  }
        response = client.post("/entity-relations/generations".format(), params = query_params, )
        return response

