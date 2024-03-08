package adt-from-open-api-trust

import com.github.plokhotnyuk.jsoniter_scala.core._
import com.github.plokhotnyuk.jsoniter_scala.macros._

trait AdtTypeToken1Dto extends AdtDto {
val int: Long
val str: Option[String]

}
object AdtTypeToken1Dto {
final case class Impl(
    int: Long
, 
    str: Option[String]

)
given codec: JsonValueCodec[AdtTypeToken1Dto.Impl] = JsonCodecMaker.make
}


}

