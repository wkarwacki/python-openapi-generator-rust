package vars-trust

import com.github.plokhotnyuk.jsoniter_scala.core._
import com.github.plokhotnyuk.jsoniter_scala.macros._

trait ObjectWithOptLongDto {
val long: Option[Long]

}
object ObjectWithOptLongDto {
final case class Impl(
    long: Option[Long]

)
given codec: JsonValueCodec[ObjectWithOptLongDto.Impl] = JsonCodecMaker.make
}



