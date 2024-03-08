from trust.table import table_id
import json

from fastapi import File, Form, UploadFile
from typing import Annotated

from trust import Dto

class UploadTableAsFileDto(Dto):
    class ColumnsItem(Dto):
    
        id: str
        name: str

    


    columns: list[ColumnsItem | None] | None
    file: UploadFile
    id: table_id.TableIdDto | None
    table_name: str | None
    @classmethod
    def of_form(
        cls,
        columns: list[str],
        file: Annotated[UploadFile, File()],
        id: Annotated[table_id.TableIdDto | None, Form()],
        table_name: Annotated[str | None, Form()]
    ) -> 'UploadTableAsFileDto':
            return UploadTableAsFileDto(
                columns = [json.loads(item) for item in columns],
                file = file,
                id = id,
                table_name = table_name
            )