from trust.table import column_id
from trust.api import sort_type
from trust.search import search_request_filter


from trust import Dto

class TablePreviewRequestDto(Dto):
    class SortModelItem(Dto):
    
        col_id: column_id.ColumnIdDto
        sort: sort_type.SortTypeDto

    


    end_row: int
    filter_model: dict[str | None, search_request_filter.SearchRequestFilterDto] | None
    sort_model: list[SortModelItem | None] | None
    start_row: int
