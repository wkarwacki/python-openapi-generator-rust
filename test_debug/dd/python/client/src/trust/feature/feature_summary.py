from trust.feature import feature_metrics
from trust.feature import feature_id


from trust import Dto

class FeatureSummaryDto(Dto):

    explanation: str
    group: int
    id: feature_id.FeatureIdDto
    metrics: feature_metrics.FeatureMetricsDto
