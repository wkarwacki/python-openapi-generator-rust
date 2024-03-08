
from enum import Enum


class DataTypeDto(str, Enum):
    BOOL = "bool"
    DATE = "date"
    DECIMAL = "decimal"
    INT = "int"
    STR = "str"
    TIMESTAMP = "timestamp"
    UNKNOWN = "unknown"
