

from trust import Dto

class ColumnChartCategoryDtoItem(Dto):

    count: int
    value: str


ColumnChartCategoryDto = list[ColumnChartCategoryDtoItem | None]
