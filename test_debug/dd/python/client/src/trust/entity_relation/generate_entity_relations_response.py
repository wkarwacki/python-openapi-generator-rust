from trust.table import table_column


from trust import Dto

class GenerateEntityRelationsResponseDto(Dto):
    class EntityRelationsItem(Dto):
    
        cardinality: Cardinality
        left_table_column: table_column.TableColumnDto
        right_table_column: table_column.TableColumnDto

    


    entity_relations: list[EntityRelationsItem | None]
