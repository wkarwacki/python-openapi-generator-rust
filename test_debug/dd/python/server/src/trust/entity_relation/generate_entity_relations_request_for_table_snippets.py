from trust.entity_relation import generate_entity_relations_request_abstraction
from trust.entity_relation import table_snippet



class GenerateEntityRelationsRequestForTableSnippetsDto(generate_entity_relations_request_abstraction.GenerateEntityRelationsRequestAbstractionDto):
    

    


    table_snippets: list[table_snippet.TableSnippetDto]
