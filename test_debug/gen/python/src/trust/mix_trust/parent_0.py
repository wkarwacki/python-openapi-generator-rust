from trust.mix_trust import mix_with_vars
from trust.mix_trust import parent_2
from trust.mix_trust import mix_mix
from trust.mix_trust import mix
from trust.mix_trust import parent_1
from trust.mix_trust import parent_0
from trust.mix_trust import mix_of_mix_var
from trust.mix_trust import mix_var
import json

from enum import Enum
from fastapi import File, Form, UploadFile
from pydantic import Field
from typing import Annotated, Any, Literal

from trust import Dto

class Parent0Dto(Dto):

    str: str
