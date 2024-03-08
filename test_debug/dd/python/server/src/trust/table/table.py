from trust.table import table_id
from trust.table import table_source


from trust import Dto

class TableDto(Dto):

    column_count: int
    id: table_id.TableIdDto
    name: str
    row_count: int
    size_in_bytes: int
    source: table_source.TableSourceDto
