from trust.task import task_summary
from trust.prediction import prediction_id
from trust.prediction import prediction_id



class PredictionSummaryDto(task_summary.TaskSummaryDto):

    id: prediction_id.PredictionIdDto
    model_name: str
    name: str
    target_table_name: str
