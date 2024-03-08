package types-trust

import com.github.plokhotnyuk.jsoniter_scala.core._
import com.github.plokhotnyuk.jsoniter_scala.macros._

enum EnumIntDto(val value: String):
    case `0` extends EnumIntDto("0")
    case `1` extends EnumIntDto("1")
    case `2` extends EnumIntDto("2")


