from trust.analytic import lead_time
from trust.analytic import analytic_table_id
from trust.analytic import analytic_column
from trust.table import table_id


from trust import Dto

class AnalyticTableDto(Dto):
    

    


    analytic_columns: list[analytic_column.AnalyticColumnDto]
    data_lead_time: lead_time.LeadTimeDto | None
    id: analytic_table_id.AnalyticTableIdDto
    name: str
    table_id: table_id.TableIdDto
