package types-trust

import com.github.plokhotnyuk.jsoniter_scala.core._
import com.github.plokhotnyuk.jsoniter_scala.macros._

trait ObjDto {
val `var`: String

}
object ObjDto {
final case class Impl(
    `var`: String

)
given codec: JsonValueCodec[ObjDto.Impl] = JsonCodecMaker.make
}



