package types-trust

import com.github.plokhotnyuk.jsoniter_scala.core._
import com.github.plokhotnyuk.jsoniter_scala.macros._

trait ObjWithSeqDto {
val `var`: Seq[Boolean]

}
object ObjWithSeqDto {
final case class Impl(
    `var`: Seq[Boolean]

)
given codec: JsonValueCodec[ObjWithSeqDto.Impl] = JsonCodecMaker.make
}



