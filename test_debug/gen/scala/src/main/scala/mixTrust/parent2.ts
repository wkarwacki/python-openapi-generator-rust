from trust.mixTrust import parent2
from trust.mixTrust import parent0
from trust.mixTrust import mix
from trust.mixTrust import mixMix
from trust.mixTrust import mixOfMixVar
from trust.mixTrust import parent1
from trust.mixTrust import mixVar
from trust.mixTrust import mixWithVars
from enum import Enum
from pydantic import Field
from typing import Annotated, Any

from trust import Dto

class Parent2Dto(Dto):

    int: number | None