from trust.varsTrust import objectWithOptString
from trust.varsTrust import objectWithOptBoolean
from trust.varsTrust import objectWithDouble
from trust.varsTrust import objectWithLong
from trust.varsTrust import objectWithOptLong
from trust.varsTrust import objectWithObjectInlinedWithAnotherObjectInlined
from trust.varsTrust import objectWithOptObjectInlined
from trust.varsTrust import objectWithBoolean
from trust.varsTrust import objectWithOptFloat
from trust.varsTrust import objectWithOptDouble
from trust.varsTrust import objectWithFloat
from trust.varsTrust import objectWithString
from trust.varsTrust import objectWithOptInteger
from trust.varsTrust import objectWithInteger
from enum import Enum
from pydantic import Field
from typing import Annotated, Any

from trust import Dto

class ObjectWithOptFloatDto(Dto):

    float: number | None