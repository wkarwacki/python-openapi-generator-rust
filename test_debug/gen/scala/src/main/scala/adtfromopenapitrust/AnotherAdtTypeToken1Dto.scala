package adt-from-open-api-trust

import com.github.plokhotnyuk.jsoniter_scala.core._
import com.github.plokhotnyuk.jsoniter_scala.macros._

trait AnotherAdtTypeToken1Dto extends AdtDto {

}
object AnotherAdtTypeToken1Dto {
final case class Impl(
)
given codec: JsonValueCodec[AnotherAdtTypeToken1Dto.Impl] = JsonCodecMaker.make
}

