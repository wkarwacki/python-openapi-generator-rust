from trust.table import column_stats



class ColumnStatsCategoryDto(column_stats.ColumnStatsDto):

    distinct_count: int
