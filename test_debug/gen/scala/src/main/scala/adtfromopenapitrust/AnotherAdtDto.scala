package adt-from-open-api-trust

import com.github.plokhotnyuk.jsoniter_scala.core._
import com.github.plokhotnyuk.jsoniter_scala.macros._

trait AnotherAdtDto {

}
object AnotherAdtDto {
final case class Impl(
)
given codec: JsonValueCodec[AnotherAdtDto.Impl] = JsonCodecMaker.make
trait TypeToken0Dto extends AnotherAdtDto{

}
trait TypeToken1Dto extends AnotherAdtDto{

}
}

