from trust.table import table_id


from trust import Dto

class UploadTableAsFileDto(Dto):
    class ColumnsItem(Dto):
    
        id: str
        name: str

    


    columns: list[ColumnsItem | None] | None
    id: table_id.TableIdDto | None
    table_name: str | None
