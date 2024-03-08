from trust.table import column_id
from trust.table import data_type


from trust import Dto

class TypedColumnDto(Dto):

    data_type: data_type.DataTypeDto
    id: column_id.ColumnIdDto
    name: str
