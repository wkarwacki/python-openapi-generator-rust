package mix-trust

import com.github.plokhotnyuk.jsoniter_scala.core._
import com.github.plokhotnyuk.jsoniter_scala.macros._

trait MixDto extends Parent0Dto, Parent1Dto {
override val str: String
override val dec: Option[Double]

}
object MixDto {
final case class Impl(
)
given codec: JsonValueCodec[MixDto.Impl] = JsonCodecMaker.make
}

