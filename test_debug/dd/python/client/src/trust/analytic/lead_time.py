from trust.api import duration
from trust.table import column_id



class LeadTimeDto(duration.DurationDto):

    column_id: column_id.ColumnIdDto
