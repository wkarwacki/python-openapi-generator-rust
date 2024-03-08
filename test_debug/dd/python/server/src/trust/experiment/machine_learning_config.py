from trust.experiment import current_and_previous_available_models


from trust import Dto

class MachineLearningConfigDto(Dto):

    categorical_imputation: CategoricalImputation | None
    fix_imbalance: bool
    impute_missing_values: bool
    models: current_and_previous_available_models.CurrentAndPreviousAvailableModelsDto
    normalize: bool
    numeric_imputation: NumericImputation | None
    remove_outliers: bool
