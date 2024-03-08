package mix-of-mix-trust

import com.github.plokhotnyuk.jsoniter_scala.core._
import com.github.plokhotnyuk.jsoniter_scala.macros._

trait Parent0Dto {
val str: String

}
object Parent0Dto {
final case class Impl(
    str: String

)
given codec: JsonValueCodec[Parent0Dto.Impl] = JsonCodecMaker.make
}



