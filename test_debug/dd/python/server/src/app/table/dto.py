from trust import Dto
from trust.table.table import TableDto

class Xxx(Dto):

    column_count: str
    id: str
    size_in_bytes: str
    name: str
    row_count: str

    @classmethod
    def of(cls, table_dto: TableDto) -> 'Xxx':
        return Xxx(
            column_count=str(table_dto.column_count),
            id=table_dto.id,
            size_in_bytes=str(table_dto.size_in_bytes),
            name=table_dto.name,
            row_count=str(table_dto.row_count)
        )
