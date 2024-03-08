package adt-to-open-api-trust

import com.github.plokhotnyuk.jsoniter_scala.core._
import com.github.plokhotnyuk.jsoniter_scala.macros._

trait AdtDto {
val bool: Boolean
val dec: Option[Double]

}
object AdtDto {
final case class Impl(
    bool: Boolean
, 
    dec: Option[Double]

)
given codec: JsonValueCodec[AdtDto.Impl] = JsonCodecMaker.make
trait TypeToken0Dto extends AdtDto{
val str: String

}
trait TypeToken1Dto extends AdtDto{
val int: Long
val str: Option[String]

}
trait TypeToken2Dto extends AdtDto{

}
}


}

