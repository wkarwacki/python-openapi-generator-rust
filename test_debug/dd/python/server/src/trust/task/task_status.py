
from enum import Enum


class TaskStatusDto(str, Enum):
    FAILED = "failed"
    RUNNING = "running"
    SUCCEEDED = "succeeded"
    UNKNOWN = "unknown"
