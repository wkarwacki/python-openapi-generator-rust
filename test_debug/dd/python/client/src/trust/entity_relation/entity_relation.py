from trust.entity_relation import time_resolution
from trust.entity_relation import entity_relation_abstraction
from trust.api import duration

from pydantic import Field
from typing import Annotated, Literal

from trust import Dto

class EntityRelationDtoBase(Dto):
    
    type: Literal["exact-match"] | Literal["time-match"] | Literal["time-range"] | Literal["time-range-auto"]
    
class EntityRelationDtoExactMatch(entity_relation_abstraction.EntityRelationAbstractionDto):
    pass

    type: Literal["exact-match"]


class EntityRelationDtoTimeMatch(entity_relation_abstraction.EntityRelationAbstractionDto):

    resolution: time_resolution.TimeResolutionDto

    type: Literal["time-match"]


class EntityRelationDtoTimeRange(entity_relation_abstraction.EntityRelationAbstractionDto):

    explore_shorter_ranges: bool
    _from: duration.DurationDto
    to: duration.DurationDto | None

    type: Literal["time-range"]


class EntityRelationDtoTimeRangeAuto(entity_relation_abstraction.EntityRelationAbstractionDto):

    explore_shorter_ranges: bool

    type: Literal["time-range-auto"]



EntityRelationDto = Annotated[
     EntityRelationDtoExactMatch | EntityRelationDtoTimeMatch | EntityRelationDtoTimeRange | EntityRelationDtoTimeRangeAuto,
     Field(discriminator="type")
 ]
 
