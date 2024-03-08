from trust.model import model_classification_metrics
from trust.model import model_abstraction



class ModelClassificationDto(model_abstraction.ModelAbstractionDto):

    metrics: model_classification_metrics.ModelClassificationMetricsDto
