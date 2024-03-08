package mix-of-mix-trust

import com.github.plokhotnyuk.jsoniter_scala.core._
import com.github.plokhotnyuk.jsoniter_scala.macros._

trait MixOfMixInMixDto extends Parent0Dto {
val mix: Option[MixOfMixInMixDto.MixDto]
override val str: String

}
object MixOfMixInMixDto {
final case class Impl(
    mix: Option[MixOfMixInMixDto.MixDto.Impl]

)
given codec: JsonValueCodec[MixOfMixInMixDto.Impl] = JsonCodecMaker.make
    trait MixDto extends Parent1Dto {
val mixInMix: MixDto.MixInMixDto
override val dec: Option[Double]

    }
object MixDto {
final case class Impl(
    mixInMix: MixDto.MixInMixDto.Impl

)
given codec: JsonValueCodec[MixDto.Impl] = JsonCodecMaker.make
    trait MixInMixDto extends Parent2Dto {
override val int: Option[Long]

    }
object MixInMixDto {
final case class Impl(
)
given codec: JsonValueCodec[MixInMixDto.Impl] = JsonCodecMaker.make
}

    
}

    
}

.Impl] = JsonCodecMaker.make
}

    
}

    
}

xDto {
final case class Impl(
    mix: Option[MixOfMixInMixDto.MixDto.Impl]

)
given codec: JsonValueCodec[MixOfMixInMixDto.Impl] = JsonCodecMaker.make
    trait MixDto extends Parent1Dto {
xx
        trait MixInMixDto extends Parent2Dto {
override val int: Option[Long]

    }
object MixInMixDto {
final case class Impl(
)
given codec: JsonValueCodec[MixInMixDto.Impl] = JsonCodecMaker.make
}

val mixInMix: MixDto.MixInMixDto
override val dec: Option[Double]

    }
object MixDto {
final case class Impl(
    mixInMix: MixDto.MixInMixDto.Impl

)
given codec: JsonValueCodec[MixDto.Impl] = JsonCodecMaker.make
    trait MixInMixDto extends Parent2Dto {
override val int: Option[Long]

    }
object MixInMixDto {
final case class Impl(
)
given codec: JsonValueCodec[MixInMixDto.Impl] = JsonCodecMaker.make
}

    
}

    
}

