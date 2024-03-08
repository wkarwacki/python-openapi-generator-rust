from trust.table import column_stats



class ColumnStatsNumericDto(column_stats.ColumnStatsDto):

    max: float
    mean: float
    median: float
    min: float
    stddev: float
    zero_count: int
