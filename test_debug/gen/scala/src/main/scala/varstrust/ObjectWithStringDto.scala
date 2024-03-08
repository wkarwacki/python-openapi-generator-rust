package vars-trust

import com.github.plokhotnyuk.jsoniter_scala.core._
import com.github.plokhotnyuk.jsoniter_scala.macros._

trait ObjectWithStringDto {
val string: String

}
object ObjectWithStringDto {
final case class Impl(
    string: String

)
given codec: JsonValueCodec[ObjectWithStringDto.Impl] = JsonCodecMaker.make
}



