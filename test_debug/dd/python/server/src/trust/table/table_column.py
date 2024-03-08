from trust.table import table_id


from trust import Dto

class TableColumnDto(Dto):

    column: str
    table: table_id.TableIdDto
