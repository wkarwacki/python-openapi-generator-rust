from trust.model import model_regression_metrics
from trust.model import model_abstraction



class ModelRegressionDto(model_abstraction.ModelAbstractionDto):

    metrics: model_regression_metrics.ModelRegressionMetricsDto
