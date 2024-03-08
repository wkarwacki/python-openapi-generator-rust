package type-params-trust

import com.github.plokhotnyuk.jsoniter_scala.core._
import com.github.plokhotnyuk.jsoniter_scala.macros._

trait SubtypeDto extends ParamTypeDto[Boolean, TypeDto] {

}
object SubtypeDto {
final case class Impl(
)
given codec: JsonValueCodec[SubtypeDto.Impl] = JsonCodecMaker.make
}

