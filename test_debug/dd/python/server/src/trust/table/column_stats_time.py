from trust.table import column_stats



class ColumnStatsTimeDto(column_stats.ColumnStatsDto):

    max: str
    mean: str
    median: str
    min: str
