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

class AdtTypeToken0Dto(adt.AdtDto):

    bool: bool
    dec: float | None
