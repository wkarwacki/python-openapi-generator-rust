
from enum import Enum


class AvailableModelsDtoItem(str, Enum):
    LR = "lr"
    RIDGE = "ridge"
    SVM = "svm"
    LASSO = "lasso"
    EN = "en"
    LAR = "lar"
    LLAR = "llar"
    OMP = "omp"
    BR = "br"
    ARD = "ard"
    PAR = "par"
    RANSAC = "ransac"
    TR = "tr"
    HUBER = "huber"
    KR = "kr"
    DT = "dt"
    RF = "rf"
    ET = "et"
    GBC = "gbc"
    GBR = "gbr"
    XGBOOST = "xgboost"
    LIGHTGBM = "lightgbm"
    CATBOOST = "catboost"
    ADA = "ada"
    KNN = "knn"
    NB = "nb"
    RBFSVM = "rbfsvm"
    GPC = "gpc"
    MLP = "mlp"
    QDA = "qda"
    LDA = "lda"


AvailableModelsDto = list[AvailableModelsDtoItem | None]
