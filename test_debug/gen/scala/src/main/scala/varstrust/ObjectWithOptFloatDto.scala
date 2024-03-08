package vars-trust

import com.github.plokhotnyuk.jsoniter_scala.core._
import com.github.plokhotnyuk.jsoniter_scala.macros._

trait ObjectWithOptFloatDto {
val float: Option[Double]

}
object ObjectWithOptFloatDto {
final case class Impl(
    float: Option[Double]

)
given codec: JsonValueCodec[ObjectWithOptFloatDto.Impl] = JsonCodecMaker.make
}



