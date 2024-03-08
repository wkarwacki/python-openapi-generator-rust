package vars-trust

import com.github.plokhotnyuk.jsoniter_scala.core._
import com.github.plokhotnyuk.jsoniter_scala.macros._

trait ObjectWithOptObjectInlinedDto {
val inlinedObj: Option[ObjectWithOptObjectInlinedDto.InlinedObjDto]

}
object ObjectWithOptObjectInlinedDto {
final case class Impl(
    inlinedObj: Option[ObjectWithOptObjectInlinedDto.InlinedObjDto.Impl]

)
given codec: JsonValueCodec[ObjectWithOptObjectInlinedDto.Impl] = JsonCodecMaker.make
    trait InlinedObjDto {
val str: String

    }
object InlinedObjDto {
final case class Impl(
    str: String

)
given codec: JsonValueCodec[InlinedObjDto.Impl] = JsonCodecMaker.make
}

    
}

= JsonCodecMaker.make
}

    
}

 String

    }
object InlinedObjDto {
final case class Impl(
    str: String

)
given codec: JsonValueCodec[InlinedObjDto.Impl] = JsonCodecMaker.make
}

    
}

