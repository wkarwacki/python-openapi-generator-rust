from trust.types_trust import seq_seq_struct
from trust.types_trust import str
from trust.types_trust import seq_ref_dec
from trust.types_trust import map_str_to_dec
from trust.types_trust import seq_ref_bool
from trust.types_trust import seq_seq_bool
from trust.types_trust import bool
from trust.types_trust import enum_str
from trust.types_trust import dec
from trust.types_trust import seq_int
from trust.types_trust import map_str_to_bool
from trust.types_trust import seq_bool
from trust.types_trust import seq_obj
from trust.types_trust import obj_with_seq
from trust.types_trust import seq_dec
from trust.types_trust import seq_str
from trust.types_trust import seq_map_str_to_dec
from trust.types_trust import map_str_to_struct
from trust.types_trust import alias
from trust.types_trust import obj
from trust.types_trust import map_str_to_str
from trust.types_trust import map_str_to_map_str_to_obj
from trust.types_trust import map_str_to_ref_int
from trust.types_trust import map_str_to_ref_obj
from trust.types_trust import int
from trust.types_trust import enum_int
from trust.types_trust import map_str_to_int
from trust.types_trust import map_str_to_seq_int
import json

from enum import Enum
from fastapi import File, Form, UploadFile
from pydantic import Field
from typing import Annotated, Any, Literal

from trust import Dto




MapStrToRefObjDto = dict[str | None, obj.ObjDto]
