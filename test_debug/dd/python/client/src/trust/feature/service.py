from trust.feature import feature_insights
from trust.feature import feature_id
from trust.feature import feature_summary
from trust import client

class FeatureService:

    def get_features(self, experiment_id: experiment_id.ExperimentIdDto) -> list[feature_summary.FeatureSummaryDto]:
        query_params = { "experimentId": experiment_id }
        response = client.get("/features".format(), params = query_params)
        return response

    def get_feature_insights(self, feature_id: feature_id.FeatureIdDto) -> feature_insights.FeatureInsightsDto:
        query_params = {  }
        response = client.get("/features/{featureId}/insights".format(featureId = feature_id), params = query_params)
        return response

