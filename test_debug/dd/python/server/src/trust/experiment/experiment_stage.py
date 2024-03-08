
from enum import Enum


class ExperimentStageDto(str, Enum):
    STARTING = "starting"
    ANALYTIC_DB_CREATION = "analytic-db-creation"
    FEATURE_SELECTION = "feature-selection"
    FEATURE_ASSESSMENT = "feature-assessment"
    MACHINE_LEARNING = "machine-learning"
    BEST_MODEL_INTERPRETATION = "best-model-interpretation"
    BEST_MODEL_TEST_PREDICTION_CALCULATON = "best-model-test-prediction-calculaton"
    COMPLETED = "completed"
