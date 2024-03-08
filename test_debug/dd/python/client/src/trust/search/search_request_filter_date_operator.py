
from enum import Enum


class SearchRequestFilterDateOperatorDto(str, Enum):
    BLANK = "blank"
    EQUALS = "equals"
    GREATER_THAN = "greaterThan"
    IN_RANGE = "inRange"
    LESS_THAN = "lessThan"
    NOT_BLANK = "notBlank"
    NOT_EQUAL = "notEqual"
