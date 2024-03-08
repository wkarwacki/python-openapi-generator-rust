from trust.adt_to_open_api_trust import adt
import json

from enum import Enum
from fastapi import File, Form, UploadFile
from pydantic import Field
from typing import Annotated, Any, Literal

from trust import Dto

class AdtDtoBase(Dto):
    
    bool: bool
    dec: float | None
class AdtDtoTypeToken0(AdtDtoBase):

    str: str

    type: Literal["typeToken0"]


class AdtDtoTypeToken1(AdtDtoBase):

    int: int
    str: str | None

    type: Literal["typeToken1"]


class AdtDtoTypeToken2(AdtDtoBase):
    pass

    type: Literal["typeToken2"]



AdtDto = Annotated[
     AdtDtoTypeToken0 | AdtDtoTypeToken1 | AdtDtoTypeToken2,
     Field(discriminator="type")
 ]
 
