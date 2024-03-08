

from trust import Dto

class FeatureMetricsDto(Dto):

    confidence: float
    feature_auc: float | None
    max: float | None
    mean: float | None
    min: float | None
    null_ratio: float
    stddev: float | None
    zeros_ratio: float
