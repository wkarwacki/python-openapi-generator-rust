package adt-from-open-api-trust

import com.github.plokhotnyuk.jsoniter_scala.core._
import com.github.plokhotnyuk.jsoniter_scala.macros._

trait AdtTypeToken0Dto extends AdtDto {
val bool: Boolean
val dec: Option[Double]

}
object AdtTypeToken0Dto {
final case class Impl(
    bool: Boolean
, 
    dec: Option[Double]

)
given codec: JsonValueCodec[AdtTypeToken0Dto.Impl] = JsonCodecMaker.make
}


}

