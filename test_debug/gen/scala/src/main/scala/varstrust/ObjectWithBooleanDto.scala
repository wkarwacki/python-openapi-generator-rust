package vars-trust

import com.github.plokhotnyuk.jsoniter_scala.core._
import com.github.plokhotnyuk.jsoniter_scala.macros._

trait ObjectWithBooleanDto {
val boolean: Boolean

}
object ObjectWithBooleanDto {
final case class Impl(
    boolean: Boolean

)
given codec: JsonValueCodec[ObjectWithBooleanDto.Impl] = JsonCodecMaker.make
}



