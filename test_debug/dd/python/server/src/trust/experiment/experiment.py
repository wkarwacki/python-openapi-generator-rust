from trust.task import task
from trust.experiment import experiment_stage



class ExperimentDto(task.TaskDto):

    stage: experiment_stage.ExperimentStageDto
