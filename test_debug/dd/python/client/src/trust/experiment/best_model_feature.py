from trust.feature import feature_summary


from trust import Dto

class BestModelFeatureDto(Dto):

    feature: feature_summary.FeatureSummaryDto
    weight: float
