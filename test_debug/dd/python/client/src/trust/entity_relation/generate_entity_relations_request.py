from trust.entity_relation import table_snippet
from trust.entity_relation import generate_entity_relations_request_abstraction
from trust.table import table_id

from pydantic import Field
from typing import Annotated, Literal

from trust import Dto

class GenerateEntityRelationsRequestDtoBase(Dto):
    pass
class GenerateEntityRelationsRequestDtoTableSnippets(generate_entity_relations_request_abstraction.GenerateEntityRelationsRequestAbstractionDto):
    

    


    table_snippets: list[table_snippet.TableSnippetDto]

    type: Literal["table-snippets"]


class GenerateEntityRelationsRequestDtoTables(generate_entity_relations_request_abstraction.GenerateEntityRelationsRequestAbstractionDto):
    

    


    tables: list[table_id.TableIdDto]

    type: Literal["tables"]



GenerateEntityRelationsRequestDto = Annotated[
     GenerateEntityRelationsRequestDtoTableSnippets | GenerateEntityRelationsRequestDtoTables,
     Field(discriminator="type")
 ]
 
