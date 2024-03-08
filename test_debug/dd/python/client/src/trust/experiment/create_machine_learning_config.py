from trust.experiment import available_models


from trust import Dto

class CreateMachineLearningConfigDto(Dto):

    categorical_imputation: CategoricalImputation | None
    fix_imbalance: bool
    impute_missing_values: bool
    models: available_models.AvailableModelsDto | None
    normalize: bool
    numeric_imputation: NumericImputation | None
    remove_outliers: bool
