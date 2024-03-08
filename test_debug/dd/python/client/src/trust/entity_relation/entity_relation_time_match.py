from trust.entity_relation import time_resolution
from trust.entity_relation import entity_relation_abstraction



class EntityRelationTimeMatchDto(entity_relation_abstraction.EntityRelationAbstractionDto):

    resolution: time_resolution.TimeResolutionDto
