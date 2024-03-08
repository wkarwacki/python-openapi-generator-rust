
from pydantic import Field
from typing import Annotated, Literal

from trust import Dto

class TargetDownsampleConfigDtoBase(Dto):
    pass
class TargetDownsampleConfigDtoNumber(TargetDownsampleConfigDtoBase):

    value: int

    type: Literal["number"]


class TargetDownsampleConfigDtoRatio(TargetDownsampleConfigDtoBase):

    value: float

    type: Literal["ratio"]



TargetDownsampleConfigDto = Annotated[
     TargetDownsampleConfigDtoNumber | TargetDownsampleConfigDtoRatio,
     Field(discriminator="type")
 ]
 
