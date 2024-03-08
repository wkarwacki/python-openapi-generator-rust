package vars-trust

import com.github.plokhotnyuk.jsoniter_scala.core._
import com.github.plokhotnyuk.jsoniter_scala.macros._

trait ObjectWithOptStringDto {
val string: Option[String]

}
object ObjectWithOptStringDto {
final case class Impl(
    string: Option[String]

)
given codec: JsonValueCodec[ObjectWithOptStringDto.Impl] = JsonCodecMaker.make
}



