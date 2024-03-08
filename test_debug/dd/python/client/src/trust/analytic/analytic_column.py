from trust.table import column_id
from trust.analytic import analytic_type


from trust import Dto

class AnalyticColumnDto(Dto):

    analytic_type: analytic_type.AnalyticTypeDto
    column_id: column_id.ColumnIdDto
