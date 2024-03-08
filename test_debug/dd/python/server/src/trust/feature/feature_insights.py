from trust.feature import feature_id


from trust import Dto

class FeatureInsightsDto(Dto):
    class CorrelationsItem(Dto):
    
        correlation: float
        feature_id: feature_id.FeatureIdDto

    


    correlations: list[CorrelationsItem | None]
