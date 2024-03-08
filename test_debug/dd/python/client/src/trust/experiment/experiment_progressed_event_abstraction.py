from trust.experiment import experiment_id
from trust.task import progress


from trust import Dto

class ExperimentProgressedEventAbstractionDto(Dto):

    id: experiment_id.ExperimentIdDto
    progress: progress.ProgressDto
