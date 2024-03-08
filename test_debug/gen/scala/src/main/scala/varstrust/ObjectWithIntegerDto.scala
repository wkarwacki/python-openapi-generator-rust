package vars-trust

import com.github.plokhotnyuk.jsoniter_scala.core._
import com.github.plokhotnyuk.jsoniter_scala.macros._

trait ObjectWithIntegerDto {
val integer: Long

}
object ObjectWithIntegerDto {
final case class Impl(
    integer: Long

)
given codec: JsonValueCodec[ObjectWithIntegerDto.Impl] = JsonCodecMaker.make
}



