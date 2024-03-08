from trust.task import progress
from trust.task import task_status
from trust.task import progress
from trust.task import task_status


from trust import Dto

class TaskSummaryDto(Dto):

    name: str
    progress: progress.ProgressDto
    status: task_status.TaskStatusDto
