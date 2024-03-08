from trust.model import model_id


from trust import Dto

class ModelAbstractionDto(Dto):

    id: model_id.ModelIdDto
    name: str
