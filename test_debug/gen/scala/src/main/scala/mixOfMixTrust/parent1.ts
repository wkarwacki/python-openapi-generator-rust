from trust.mixOfMixTrust import mixOfMix
from trust.mixOfMixTrust import mixOfMixInMix
from trust.mixOfMixTrust import parent2
from trust.mixOfMixTrust import parent1
from trust.mixOfMixTrust import parent0
from enum import Enum
from pydantic import Field
from typing import Annotated, Any

from trust import Dto

class Parent1Dto(Dto):

    dec: number | None