package vars-trust

import com.github.plokhotnyuk.jsoniter_scala.core._
import com.github.plokhotnyuk.jsoniter_scala.macros._

trait ObjectWithFloatDto {
val float: Double

}
object ObjectWithFloatDto {
final case class Impl(
    float: Double

)
given codec: JsonValueCodec[ObjectWithFloatDto.Impl] = JsonCodecMaker.make
}



