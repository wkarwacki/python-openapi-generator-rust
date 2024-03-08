from trust.feature import feature_id
from trust.feature import feature_insights
from trust.feature import feature_summary
from abc import ABC, abstractmethod

class FeatureService(ABC):
    

    @abstractmethod
    def get_features(self, experiment_id: experiment_id.ExperimentIdDto) -> list[feature_summary.FeatureSummaryDto]:
        raise NotImplementedError
    

    @abstractmethod
    def get_feature_insights(self, feature_id: feature_id.FeatureIdDto) -> feature_insights.FeatureInsightsDto:
        raise NotImplementedError

