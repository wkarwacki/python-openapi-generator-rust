package mix-trust

import com.github.plokhotnyuk.jsoniter_scala.core._
import com.github.plokhotnyuk.jsoniter_scala.macros._

trait Parent1Dto {
val dec: Option[Double]

}
object Parent1Dto {
final case class Impl(
    dec: Option[Double]

)
given codec: JsonValueCodec[Parent1Dto.Impl] = JsonCodecMaker.make
}



