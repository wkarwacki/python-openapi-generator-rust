package vars-trust

import com.github.plokhotnyuk.jsoniter_scala.core._
import com.github.plokhotnyuk.jsoniter_scala.macros._

trait ObjectWithLongDto {
val long: Long

}
object ObjectWithLongDto {
final case class Impl(
    long: Long

)
given codec: JsonValueCodec[ObjectWithLongDto.Impl] = JsonCodecMaker.make
}



