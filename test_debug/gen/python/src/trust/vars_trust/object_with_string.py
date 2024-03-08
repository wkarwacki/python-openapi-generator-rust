from trust.vars_trust import object_with_opt_long
from trust.vars_trust import object_with_opt_object_inlined
from trust.vars_trust import object_with_integer
from trust.vars_trust import object_with_opt_float
from trust.vars_trust import object_with_opt_string
from trust.vars_trust import object_with_string
from trust.vars_trust import object_with_opt_double
from trust.vars_trust import object_with_long
from trust.vars_trust import object_with_boolean
from trust.vars_trust import object_with_double
from trust.vars_trust import object_with_opt_boolean
from trust.vars_trust import object_with_object_inlined_with_another_object_inlined
from trust.vars_trust import object_with_float
from trust.vars_trust import object_with_opt_integer
import json

from enum import Enum
from fastapi import File, Form, UploadFile
from pydantic import Field
from typing import Annotated, Any, Literal

from trust import Dto

class ObjectWithStringDto(Dto):

    string: str
