package mix-trust

import com.github.plokhotnyuk.jsoniter_scala.core._
import com.github.plokhotnyuk.jsoniter_scala.macros._

trait MixMixDto {
val mix: MixMixDto.MixDto

}
object MixMixDto {
final case class Impl(
    mix: MixMixDto.MixDto.Impl

)
given codec: JsonValueCodec[MixMixDto.Impl] = JsonCodecMaker.make
    trait MixDto extends Parent0Dto, Parent1Dto {
override val str: String
override val dec: Option[Double]

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

 Parent0Dto, Parent1Dto {
override val str: String
override val dec: Option[Double]

    }
object MixDto {
final case class Impl(
)
given codec: JsonValueCodec[MixDto.Impl] = JsonCodecMaker.make
}

    
}

