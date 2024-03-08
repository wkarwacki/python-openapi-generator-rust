from trust.prediction import prediction_progressed_event_abstraction
from trust.prediction import prediction_progressed_event_abstraction

from pydantic import Field
from typing import Annotated, Literal

from trust import Dto

class PredictionProgressedEventDtoBase(Dto):
    pass
class PredictionProgressedEventDtoFailed(prediction_progressed_event_abstraction.PredictionProgressedEventAbstractionDto):
    pass

    type: Literal["failed"]


class PredictionProgressedEventDtoFeatureValuesGenerated(prediction_progressed_event_abstraction.PredictionProgressedEventAbstractionDto):
    pass

    type: Literal["feature-values-generated"]


class PredictionProgressedEventDtoPredictionDone(prediction_progressed_event_abstraction.PredictionProgressedEventAbstractionDto):
    pass

    type: Literal["prediction-done"]


class PredictionProgressedEventDtoPredictionResultsCollected(prediction_progressed_event_abstraction.PredictionProgressedEventAbstractionDto):
    pass

    type: Literal["prediction-results-collected"]


class PredictionProgressedEventDtoStarted(prediction_progressed_event_abstraction.PredictionProgressedEventAbstractionDto):
    pass

    type: Literal["started"]


class PredictionProgressedEventDtoTablesLoaded(prediction_progressed_event_abstraction.PredictionProgressedEventAbstractionDto):
    pass

    type: Literal["tables-loaded"]



PredictionProgressedEventDto = Annotated[
     PredictionProgressedEventDtoFailed | PredictionProgressedEventDtoFeatureValuesGenerated | PredictionProgressedEventDtoPredictionDone | PredictionProgressedEventDtoPredictionResultsCollected | PredictionProgressedEventDtoStarted | PredictionProgressedEventDtoTablesLoaded,
     Field(discriminator="type")
 ]
 
