from trust.entity_relation import generate_entity_relations_request_abstraction
from trust.table import table_id



class GenerateEntityRelationsRequestForTablesDto(generate_entity_relations_request_abstraction.GenerateEntityRelationsRequestAbstractionDto):
    

    


    tables: list[table_id.TableIdDto]
