from trust.table import column_stats_time
from trust.table import column_chart_time
from trust.table import column_profile_abstraction



class ColumnProfileTimeDto(column_profile_abstraction.ColumnProfileAbstractionDto):

    histogram: column_chart_time.ColumnChartTimeDto
    stats: column_stats_time.ColumnStatsTimeDto
