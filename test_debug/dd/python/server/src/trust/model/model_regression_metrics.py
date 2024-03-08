

from trust import Dto

class ModelRegressionMetricsDto(Dto):

    mae: float
    mape: float
    r_2: float
    rmse: float
    rmsle: float
