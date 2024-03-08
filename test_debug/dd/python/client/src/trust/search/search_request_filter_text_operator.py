
from enum import Enum


class SearchRequestFilterTextOperatorDto(str, Enum):
    BLANK = "blank"
    CONTAINS = "contains"
    EMPTY = "empty"
    ENDS_WITH = "endsWith"
    EQUALS = "equals"
    NOT_BLANK = "notBlank"
    NOT_CONTAINS = "notContains"
    NOT_EQUAL = "notEqual"
    STARTS_WITH = "startsWith"
