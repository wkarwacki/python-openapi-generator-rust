from trust.type_params_trust import type
from trust.type_params_trust import param_type
from trust.type_params_trust import subtype
import json

from enum import Enum
from fastapi import File, Form, UploadFile
from pydantic import Field
from typing import Annotated, Any, Literal

from trust import Dto

class TypeDto(Dto):

    dec: float
    str: str | None
