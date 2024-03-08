

from trust import Dto

class ModelClassificationMetricsDto(Dto):

    accuracy: float
    auc: float
    f_1: float
    kappa: float
    mcc: float
    precision: float
    recall: float
