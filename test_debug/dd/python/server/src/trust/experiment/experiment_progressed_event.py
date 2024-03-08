from trust.experiment import experiment_progressed_event_abstraction

from pydantic import Field
from typing import Annotated, Literal

from trust import Dto

class ExperimentProgressedEventDtoBase(Dto):
    pass
class ExperimentProgressedEventDtoAnalyticDbCreated(experiment_progressed_event_abstraction.ExperimentProgressedEventAbstractionDto):
    pass

    type: Literal["analytic-db-created"]


class ExperimentProgressedEventDtoBestModelInterpreted(experiment_progressed_event_abstraction.ExperimentProgressedEventAbstractionDto):
    pass

    type: Literal["best-model-interpreted"]


class ExperimentProgressedEventDtoBestModelTestPredictionCalculated(experiment_progressed_event_abstraction.ExperimentProgressedEventAbstractionDto):
    pass

    type: Literal["best-model-test-prediction-calculated"]


class ExperimentProgressedEventDtoFailed(experiment_progressed_event_abstraction.ExperimentProgressedEventAbstractionDto):
    pass

    type: Literal["failed"]


class ExperimentProgressedEventDtoFeaturesAssessed(experiment_progressed_event_abstraction.ExperimentProgressedEventAbstractionDto):
    pass

    type: Literal["features-assessed"]


class ExperimentProgressedEventDtoFeaturesSelected(experiment_progressed_event_abstraction.ExperimentProgressedEventAbstractionDto):
    pass

    type: Literal["features-selected"]


class ExperimentProgressedEventDtoMlDone(experiment_progressed_event_abstraction.ExperimentProgressedEventAbstractionDto):
    pass

    type: Literal["ml-done"]


class ExperimentProgressedEventDtoStarted(experiment_progressed_event_abstraction.ExperimentProgressedEventAbstractionDto):
    pass

    type: Literal["started"]



ExperimentProgressedEventDto = Annotated[
     ExperimentProgressedEventDtoAnalyticDbCreated | ExperimentProgressedEventDtoBestModelInterpreted | ExperimentProgressedEventDtoBestModelTestPredictionCalculated | ExperimentProgressedEventDtoFailed | ExperimentProgressedEventDtoFeaturesAssessed | ExperimentProgressedEventDtoFeaturesSelected | ExperimentProgressedEventDtoMlDone | ExperimentProgressedEventDtoStarted,
     Field(discriminator="type")
 ]
 
