

from trust import Dto

class ColumnChartTimeDtoItem(Dto):

    count: int
    _from: int
    to: int


ColumnChartTimeDto = list[ColumnChartTimeDtoItem | None]
