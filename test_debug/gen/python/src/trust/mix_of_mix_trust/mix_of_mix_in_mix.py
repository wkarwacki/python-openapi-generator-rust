from trust.mix_of_mix_trust import parent_0
from trust.mix_of_mix_trust import parent_2
from trust.mix_of_mix_trust import mix_of_mix
from trust.mix_of_mix_trust import mix_of_mix_in_mix
from trust.mix_of_mix_trust import parent_1
import json

from enum import Enum
from fastapi import File, Form, UploadFile
from pydantic import Field
from typing import Annotated, Any, Literal

from trust import Dto

class MixOfMixInMixDto(parent_0.Parent0Dto):
    class Mix(parent_1.Parent1Dto):
        

            mix_in_mix: MixInMix

    mix: Mix | None
