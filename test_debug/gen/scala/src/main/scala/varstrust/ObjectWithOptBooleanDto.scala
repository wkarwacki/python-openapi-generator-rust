package vars-trust

import com.github.plokhotnyuk.jsoniter_scala.core._
import com.github.plokhotnyuk.jsoniter_scala.macros._

trait ObjectWithOptBooleanDto {
val boolean: Option[Boolean]

}
object ObjectWithOptBooleanDto {
final case class Impl(
    boolean: Option[Boolean]

)
given codec: JsonValueCodec[ObjectWithOptBooleanDto.Impl] = JsonCodecMaker.make
}



