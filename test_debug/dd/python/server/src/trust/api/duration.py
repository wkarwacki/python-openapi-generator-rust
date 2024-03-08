from trust.api import time_unit


from trust import Dto

class DurationDto(Dto):

    unit: time_unit.TimeUnitDto
    value: int
