package mix-trust

import com.github.plokhotnyuk.jsoniter_scala.core._
import com.github.plokhotnyuk.jsoniter_scala.macros._

trait MixWithVarsDto extends Parent0Dto {
val bool: Boolean
val dec: Option[Double]
override val str: String

}
object MixWithVarsDto {
final case class Impl(
    bool: Boolean
, 
    dec: Option[Double]

)
given codec: JsonValueCodec[MixWithVarsDto.Impl] = JsonCodecMaker.make
}


}

