from pydantic import BaseModel
from trust import TypeMapping
from trust.transformer.transformer_preview_request import TransformerPreviewRequestDto


class AppTransformerPreviewRequest(BaseModel, TypeMapping[TransformerPreviewRequestDto, 'AppTransformerPreviewRequest']):
    start: int
    end: int

    @classmethod
    def of(cls, dto: TransformerPreviewRequestDto) -> 'AppTransformerPreviewRequest':
        return AppTransformerPreviewRequest(
            start=dto.start_row,
            end=dto.end_row
        )

    def to(self) -> TransformerPreviewRequestDto:
        return TransformerPreviewRequestDto(
            start_row=self.start,
            end_row=self.end,
            filter_masterpiece={},
            sort_masterpiece=[]
        )
