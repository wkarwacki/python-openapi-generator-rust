package mix-of-mix-trust

import com.github.plokhotnyuk.jsoniter_scala.core._
import com.github.plokhotnyuk.jsoniter_scala.macros._

trait MixOfMixDto extends Parent0Dto, Parent1Dto, Parent2Dto {
override val str: String
override val dec: Option[Double]
override val int: Option[Long]

}
object MixOfMixDto {
final case class Impl(
)
given codec: JsonValueCodec[MixOfMixDto.Impl] = JsonCodecMaker.make
}

