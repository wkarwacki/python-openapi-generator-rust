from trust.table import column_stats_numeric
from trust.table import column_chart_category
from trust.table import column_chart_numeric
from trust.table import column_profile_abstraction



class ColumnProfileNumericDto(column_profile_abstraction.ColumnProfileAbstractionDto):

    frequent_values: column_chart_category.ColumnChartCategoryDto
    histogram: column_chart_numeric.ColumnChartNumericDto
    stats: column_stats_numeric.ColumnStatsNumericDto
