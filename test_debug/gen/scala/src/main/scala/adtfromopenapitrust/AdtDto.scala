package adt-from-open-api-trust

import com.github.plokhotnyuk.jsoniter_scala.core._
import com.github.plokhotnyuk.jsoniter_scala.macros._

trait AdtDto {

}
object AdtDto {
final case class Impl(
)
given codec: JsonValueCodec[AdtDto.Impl] = JsonCodecMaker.make
trait TypeToken0Dto extends AdtDto{
val bool: Boolean
val dec: Option[Double]

}
trait TypeToken1Dto extends AdtDto{
val int: Long
val str: Option[String]

}
}

