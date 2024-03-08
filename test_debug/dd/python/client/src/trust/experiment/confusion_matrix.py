

from trust import Dto

class ConfusionMatrixDto(Dto):

    false_neg: int
    false_pos: int
    true_neg: int
    true_pos: int
