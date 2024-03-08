from trust.experiment import experiment_stage
from trust.task import task



class ExperimentDto(task.TaskDto):

    stage: experiment_stage.ExperimentStageDto
