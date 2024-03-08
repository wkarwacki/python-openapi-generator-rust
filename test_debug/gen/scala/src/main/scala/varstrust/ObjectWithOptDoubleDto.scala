package vars-trust

import com.github.plokhotnyuk.jsoniter_scala.core._
import com.github.plokhotnyuk.jsoniter_scala.macros._

trait ObjectWithOptDoubleDto {
val double: Option[Double]

}
object ObjectWithOptDoubleDto {
final case class Impl(
    double: Option[Double]

)
given codec: JsonValueCodec[ObjectWithOptDoubleDto.Impl] = JsonCodecMaker.make
}



