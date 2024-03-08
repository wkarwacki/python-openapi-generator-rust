from trust.experiment import target_downsample_config
from trust.table import column_id
from trust.analytic import analytic_table_id


from trust import Dto

class TargetConfigDto(Dto):
    

    


    analytic_table_id: analytic_table_id.AnalyticTableIdDto
    column_id: column_id.ColumnIdDto
    downsample: target_downsample_config.TargetDownsampleConfigDto | None
    unique_constraint: list[column_id.ColumnIdDto]
