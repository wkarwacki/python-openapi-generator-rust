package vars-trust

import com.github.plokhotnyuk.jsoniter_scala.core._
import com.github.plokhotnyuk.jsoniter_scala.macros._

trait ObjectWithOptIntegerDto {
val integer: Option[Long]

}
object ObjectWithOptIntegerDto {
final case class Impl(
    integer: Option[Long]

)
given codec: JsonValueCodec[ObjectWithOptIntegerDto.Impl] = JsonCodecMaker.make
}



