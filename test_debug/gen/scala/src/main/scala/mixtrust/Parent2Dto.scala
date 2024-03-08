package mix-trust

import com.github.plokhotnyuk.jsoniter_scala.core._
import com.github.plokhotnyuk.jsoniter_scala.macros._

trait Parent2Dto {
val int: Option[Long]

}
object Parent2Dto {
final case class Impl(
    int: Option[Long]

)
given codec: JsonValueCodec[Parent2Dto.Impl] = JsonCodecMaker.make
}



