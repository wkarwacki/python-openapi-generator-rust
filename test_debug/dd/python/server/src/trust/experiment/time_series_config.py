from trust.analytic import lead_time


from trust import Dto

class TimeSeriesConfigDto(Dto):

    prediction_target_time: lead_time.LeadTimeDto
