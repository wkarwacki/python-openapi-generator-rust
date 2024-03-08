from trust.table import column_stats_category
from trust.table import column_profile_abstraction
from trust.table import column_chart_category
from trust.table import column_chart_time
from trust.table import column_stats_numeric
from trust.table import column_chart_numeric
from trust.table import column_stats_time

from pydantic import Field
from typing import Annotated, Literal

from trust import Dto

class ColumnProfileDtoBase(Dto):
    pass
class ColumnProfileDtoCategory(column_profile_abstraction.ColumnProfileAbstractionDto):

    frequent_values: column_chart_category.ColumnChartCategoryDto
    stats: column_stats_category.ColumnStatsCategoryDto

    type: Literal["category"]


class ColumnProfileDtoNumeric(column_profile_abstraction.ColumnProfileAbstractionDto):

    frequent_values: column_chart_category.ColumnChartCategoryDto
    histogram: column_chart_numeric.ColumnChartNumericDto
    stats: column_stats_numeric.ColumnStatsNumericDto

    type: Literal["numeric"]


class ColumnProfileDtoTime(column_profile_abstraction.ColumnProfileAbstractionDto):

    histogram: column_chart_time.ColumnChartTimeDto
    stats: column_stats_time.ColumnStatsTimeDto

    type: Literal["time"]



ColumnProfileDto = Annotated[
     ColumnProfileDtoCategory | ColumnProfileDtoNumeric | ColumnProfileDtoTime,
     Field(discriminator="type")
 ]
 
