package mix-trust

import com.github.plokhotnyuk.jsoniter_scala.core._
import com.github.plokhotnyuk.jsoniter_scala.macros._

trait MixVarDto {
val `var`: MixVarDto.VarDto

}
object MixVarDto {
final case class Impl(
    `var`: MixVarDto.VarDto.Impl

)
given codec: JsonValueCodec[MixVarDto.Impl] = JsonCodecMaker.make
    trait VarDto extends Parent0Dto, Parent1Dto, MixDto {
override val str: String
override val dec: Option[Double]

    }
object VarDto {
final case class Impl(
)
given codec: JsonValueCodec[VarDto.Impl] = JsonCodecMaker.make
}

    
}

er.make
}

    
}

 Parent0Dto, Parent1Dto, MixDto {
override val str: String
override val dec: Option[Double]

    }
object VarDto {
final case class Impl(
)
given codec: JsonValueCodec[VarDto.Impl] = JsonCodecMaker.make
}

    
}

