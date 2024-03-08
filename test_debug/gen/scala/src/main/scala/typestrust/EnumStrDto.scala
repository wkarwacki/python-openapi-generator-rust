package types-trust

import com.github.plokhotnyuk.jsoniter_scala.core._
import com.github.plokhotnyuk.jsoniter_scala.macros._

enum EnumStrDto(val value: String):
    case A extends EnumStrDto("a")
    case B extends EnumStrDto("b")
    case C extends EnumStrDto("c")


