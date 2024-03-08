from trust.analytic import analytic_table_id
from trust.entity_relation import entity_relation_id
from trust.table import column_id


from trust import Dto

class EntityRelationAbstractionDto(Dto):
    class ColumnMappingItem(Dto):
    
        base_column_id: column_id.ColumnIdDto
        related_column_id: column_id.ColumnIdDto

    


    base_analytic_table_id: analytic_table_id.AnalyticTableIdDto
    column_mapping: list[ColumnMappingItem | None]
    id: entity_relation_id.EntityRelationIdDto
    related_analytic_table_id: analytic_table_id.AnalyticTableIdDto
