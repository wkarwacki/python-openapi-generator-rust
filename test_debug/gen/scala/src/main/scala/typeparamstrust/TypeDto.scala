package type-params-trust

import com.github.plokhotnyuk.jsoniter_scala.core._
import com.github.plokhotnyuk.jsoniter_scala.macros._

trait TypeDto {
val dec: Double
val str: Option[String]

}
object TypeDto {
final case class Impl(
    dec: Double
, 
    str: Option[String]

)
given codec: JsonValueCodec[TypeDto.Impl] = JsonCodecMaker.make
}


}

