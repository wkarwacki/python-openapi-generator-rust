from trust.entity_relation import entity_relation_abstraction
from trust.api import duration



class EntityRelationTimeRangeDto(entity_relation_abstraction.EntityRelationAbstractionDto):

    explore_shorter_ranges: bool
    _from: duration.DurationDto
    to: duration.DurationDto | None
