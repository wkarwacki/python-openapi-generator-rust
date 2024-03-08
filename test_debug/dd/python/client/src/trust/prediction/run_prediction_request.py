from trust.model import model_id


from trust import Dto

class RunPredictionRequestDto(Dto):

    model_id: model_id.ModelIdDto
    name: str
    tables: dict[str | None, str | None]
