
from typing import Any

from trust import Dto

class TablePreviewResponseDto(Dto):
    




    


    last_row: int
    row_count: int
    row_data: list[dict[str | None, dict[Any, Any] | None] | None]
