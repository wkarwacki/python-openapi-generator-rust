from trust.table import column_id
from trust.api import duration



class LeadTimeDto(duration.DurationDto):

    column_id: column_id.ColumnIdDto
