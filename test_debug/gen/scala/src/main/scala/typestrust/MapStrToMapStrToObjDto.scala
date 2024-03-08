package types-trust

import com.github.plokhotnyuk.jsoniter_scala.core._
import com.github.plokhotnyuk.jsoniter_scala.macros._

opaque type MapStrToMapStrToObjDto = Map[String, Option[Map[String, Option[MapStrToMapStrToObjDto.Value]]]]


object MapStrToMapStrToObjDto {
trait Value {
val int: Long
 }
object Value {
final case class Impl(
    int: Long

)
given codec: JsonValueCodec[Value.Impl] = JsonCodecMaker.make
}

}

