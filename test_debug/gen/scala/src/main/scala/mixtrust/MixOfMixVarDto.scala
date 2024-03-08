package mix-trust

import com.github.plokhotnyuk.jsoniter_scala.core._
import com.github.plokhotnyuk.jsoniter_scala.macros._

trait MixOfMixVarDto extends Parent0Dto {
val mix: Option[MixOfMixVarDto.MixDto]
override val str: String

}
object MixOfMixVarDto {
final case class Impl(
    mix: Option[MixOfMixVarDto.MixDto.Impl]

)
given codec: JsonValueCodec[MixOfMixVarDto.Impl] = JsonCodecMaker.make
    trait MixDto extends Parent1Dto, Parent2Dto {
override val dec: Option[Double]
override val int: Option[Long]

    }
object MixDto {
final case class Impl(
)
given codec: JsonValueCodec[MixDto.Impl] = JsonCodecMaker.make
}

    
}

er.make
}

    
}

 Parent1Dto, Parent2Dto {
override val dec: Option[Double]
override val int: Option[Long]

    }
object MixDto {
final case class Impl(
)
given codec: JsonValueCodec[MixDto.Impl] = JsonCodecMaker.make
}

    
}

