from trust.experiment import create_machine_learning_config
from trust.experiment import time_series_config
from trust.experiment import feature_engineering_config
from trust.experiment import experiment_mode
from trust.experiment import target_config
from trust.analytic import analytic_table
from trust.entity_relation import entity_relation


from trust import Dto

class CreateExperimentConfigRequestDto(Dto):
    

    

    

    


    analytic_tables: list[analytic_table.AnalyticTableDto]
    entity_relations: list[entity_relation.EntityRelationDto]
    feature_engineering_config: feature_engineering_config.FeatureEngineeringConfigDto
    machine_learning_config: create_machine_learning_config.CreateMachineLearningConfigDto
    mode: experiment_mode.ExperimentModeDto
    name: str
    resource_units: int | None
    target_config: target_config.TargetConfigDto
    time_series_config: time_series_config.TimeSeriesConfigDto | None
