from trust.model import model
from trust import client

class ModelService:

    def get_models(self, experiment_id: experiment_id.ExperimentIdDto) -> list[model.ModelDto]:
        query_params = { "experimentId": experiment_id }
        response = client.get("/models".format(), params = query_params)
        return response

