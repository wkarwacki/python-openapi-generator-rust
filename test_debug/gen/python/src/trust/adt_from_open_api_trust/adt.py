from trust.adt_from_open_api_trust import adt_type_token_1
from trust.adt_from_open_api_trust import another_adt_type_token_0
from trust.adt_from_open_api_trust import adt
from trust.adt_from_open_api_trust import another_adt_type_token_1
from trust.adt_from_open_api_trust import another_adt
from trust.adt_from_open_api_trust import adt_type_token_0
import json

from enum import Enum
from fastapi import File, Form, UploadFile
from pydantic import Field
from typing import Annotated, Any, Literal

from trust import Dto

class AdtDtoBase(Dto):
    pass
class AdtDtoTypeToken0(AdtDtoBase):

    bool: bool
    dec: float | None

    type: Literal["typeToken0"]


class AdtDtoTypeToken1(AdtDtoBase):

    int: int
    str: str | None

    type: Literal["typeToken1"]



AdtDto = Annotated[
     AdtDtoTypeToken0 | AdtDtoTypeToken1,
     Field(discriminator="type")
 ]
 
