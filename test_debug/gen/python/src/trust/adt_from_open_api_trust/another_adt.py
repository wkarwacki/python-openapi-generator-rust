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

class AnotherAdtDtoBase(Dto):
    pass
class AnotherAdtDtoTypeToken0(adt.AdtDto):
    pass

    type: Literal["typeToken0"]


class AnotherAdtDtoTypeToken1(adt.AdtDto):
    pass

    type: Literal["typeToken1"]



AnotherAdtDto = Annotated[
     AnotherAdtDtoTypeToken0 | AnotherAdtDtoTypeToken1,
     Field(discriminator="type")
 ]
 
