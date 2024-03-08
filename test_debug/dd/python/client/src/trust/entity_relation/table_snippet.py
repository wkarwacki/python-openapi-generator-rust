

from trust import Dto

class TableSnippetDto(Dto):
    class SnippetsItem(Dto):
        

    


        column: str
        vals: list[str | None]

    


    snippets: list[SnippetsItem | None]
    table_name: str
