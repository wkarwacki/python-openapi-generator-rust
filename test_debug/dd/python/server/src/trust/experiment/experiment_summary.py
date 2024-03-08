from trust.task import task_summary
from trust.experiment import experiment_id



class ExperimentSummaryDto(task_summary.TaskSummaryDto):

    id: experiment_id.ExperimentIdDto
