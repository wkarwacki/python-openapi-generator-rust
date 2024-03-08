package vars-trust

import com.github.plokhotnyuk.jsoniter_scala.core._
import com.github.plokhotnyuk.jsoniter_scala.macros._

trait ObjectWithObjectInlinedWithAnotherObjectInlinedDto {
val inlinedObj: ObjectWithObjectInlinedWithAnotherObjectInlinedDto.InlinedObjDto

}
object ObjectWithObjectInlinedWithAnotherObjectInlinedDto {
final case class Impl(
    inlinedObj: ObjectWithObjectInlinedWithAnotherObjectInlinedDto.InlinedObjDto.Impl

)
given codec: JsonValueCodec[ObjectWithObjectInlinedWithAnotherObjectInlinedDto.Impl] = JsonCodecMaker.make
    trait InlinedObjDto {
val anotherInlinedObj: InlinedObjDto.AnotherInlinedObjDto

    }
object InlinedObjDto {
final case class Impl(
    anotherInlinedObj: InlinedObjDto.AnotherInlinedObjDto.Impl

)
given codec: JsonValueCodec[InlinedObjDto.Impl] = JsonCodecMaker.make
    trait AnotherInlinedObjDto {
val str: String

    }
object AnotherInlinedObjDto {
final case class Impl(
    str: String

)
given codec: JsonValueCodec[AnotherInlinedObjDto.Impl] = JsonCodecMaker.make
}

    
}

    
}

nValueCodec[AnotherInlinedObjDto.Impl] = JsonCodecMaker.make
}

    
}

    
}

dDto.InlinedObjDto.Impl

)
given codec: JsonValueCodec[ObjectWithObjectInlinedWithAnotherObjectInlinedDto.Impl] = JsonCodecMaker.make
    trait InlinedObjDto {
xx
        trait AnotherInlinedObjDto {
xxval str: String

    }
object AnotherInlinedObjDto {
final case class Impl(
    str: String

)
given codec: JsonValueCodec[AnotherInlinedObjDto.Impl] = JsonCodecMaker.make
}

val anotherInlinedObj: InlinedObjDto.AnotherInlinedObjDto

    }
object InlinedObjDto {
final case class Impl(
    anotherInlinedObj: InlinedObjDto.AnotherInlinedObjDto.Impl

)
given codec: JsonValueCodec[InlinedObjDto.Impl] = JsonCodecMaker.make
    trait AnotherInlinedObjDto {
xxval str: String

    }
object AnotherInlinedObjDto {
final case class Impl(
    str: String

)
given codec: JsonValueCodec[AnotherInlinedObjDto.Impl] = JsonCodecMaker.make
}

    
}

    
}

