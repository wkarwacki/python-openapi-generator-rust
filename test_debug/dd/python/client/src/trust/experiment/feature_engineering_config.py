

from trust import Dto

class FeatureEngineeringConfigDto(Dto):

    eliminate_outliers: bool | None
    explore_features_from_target: bool
