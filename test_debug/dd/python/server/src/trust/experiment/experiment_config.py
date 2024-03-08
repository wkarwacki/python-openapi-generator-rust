from trust.experiment import time_series_config
from trust.experiment import experiment_id
from trust.experiment import machine_learning_config
from trust.experiment import feature_engineering_config
from trust.experiment import experiment_mode
from trust.experiment import target_config
from trust.analytic import analytic_table
from trust.entity_relation import entity_relation


from trust import Dto

class ExperimentConfigDto(Dto):
    

    

    

    


    analytic_tables: list[analytic_table.AnalyticTableDto]
    entity_relations: list[entity_relation.EntityRelationDto]
    feature_engineering_config: feature_engineering_config.FeatureEngineeringConfigDto
    id: experiment_id.ExperimentIdDto
    machine_learning_config: machine_learning_config.MachineLearningConfigDto
    mode: experiment_mode.ExperimentModeDto
    name: str
    resource_units: int | None
    target_config: target_config.TargetConfigDto
    time_series_config: time_series_config.TimeSeriesConfigDto | None
