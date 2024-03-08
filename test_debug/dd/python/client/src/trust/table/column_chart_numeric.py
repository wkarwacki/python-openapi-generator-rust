

from trust import Dto

class ColumnChartNumericDtoItem(Dto):

    count: int
    _from: float
    to: float


ColumnChartNumericDto = list[ColumnChartNumericDtoItem | None]
