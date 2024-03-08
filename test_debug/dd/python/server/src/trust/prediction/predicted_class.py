
from enum import Enum


class PredictedClassDto(str, Enum):
    POSITIVE = "positive"
    NEGATIVE = "negative"
