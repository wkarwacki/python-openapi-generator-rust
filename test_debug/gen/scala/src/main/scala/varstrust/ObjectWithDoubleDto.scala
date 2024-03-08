package vars-trust

import com.github.plokhotnyuk.jsoniter_scala.core._
import com.github.plokhotnyuk.jsoniter_scala.macros._

trait ObjectWithDoubleDto {
val double: Double

}
object ObjectWithDoubleDto {
final case class Impl(
    double: Double

)
given codec: JsonValueCodec[ObjectWithDoubleDto.Impl] = JsonCodecMaker.make
}



