from trust.experiment import experiment_id
from trust.task import task_summary



class ExperimentSummaryDto(task_summary.TaskSummaryDto):

    id: experiment_id.ExperimentIdDto
