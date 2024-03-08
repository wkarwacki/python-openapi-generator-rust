
from enum import Enum


class SearchRequestFilterNumberOperatorDto(str, Enum):
    BLANK = "blank"
    EQUALS = "equals"
    GREATER_THAN = "greaterThan"
    GREATER_THAN_OR_EQUAL = "greaterThanOrEqual"
    IN_RANGE = "inRange"
    LESS_THAN = "lessThan"
    LESS_THAN_OR_EQUAL = "lessThanOrEqual"
    NOT_BLANK = "notBlank"
    NOT_EQUAL = "notEqual"
