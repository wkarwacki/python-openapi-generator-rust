package types-trust

import com.github.plokhotnyuk.jsoniter_scala.core._
import com.github.plokhotnyuk.jsoniter_scala.macros._

opaque type SeqObjDto = Seq[SeqObjDto.Item]

object SeqObjDto {
trait Item {
val `var`: String
 }
object Item {
final case class Impl(
    `var`: String

)
given codec: JsonValueCodec[Item.Impl] = JsonCodecMaker.make
}

}
