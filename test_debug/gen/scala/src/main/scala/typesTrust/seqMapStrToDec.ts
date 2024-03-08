from trust.typesTrust import seqDec
from trust.typesTrust import mapStrToDec
from trust.typesTrust import seqRefBool
from trust.typesTrust import seqSeqBool
from trust.typesTrust import obj
from trust.typesTrust import mapStrToMapStrToObj
from trust.typesTrust import objWithSeq
from trust.typesTrust import seqObj
from trust.typesTrust import mapStrToStruct
from trust.typesTrust import seqBool
from trust.typesTrust import seqInt
from trust.typesTrust import mapStrToStr
from trust.typesTrust import dec
from trust.typesTrust import mapStrToBool
from trust.typesTrust import mapStrToRefObj
from trust.typesTrust import enumInt
from trust.typesTrust import mapStrToRefInt
from trust.typesTrust import seqRefDec
from trust.typesTrust import enumStr
from trust.typesTrust import bool
from trust.typesTrust import seqMapStrToDec
from trust.typesTrust import seqStr
from trust.typesTrust import mapStrToSeqInt
from trust.typesTrust import seqSeqStruct
from trust.typesTrust import str
from trust.typesTrust import mapStrToInt
from trust.typesTrust import int
from enum import Enum
from pydantic import Field
from typing import Annotated, Any

from trust import Dto





SeqMapStrToDecDto = Array<{ [key: string]: number | None; ]>