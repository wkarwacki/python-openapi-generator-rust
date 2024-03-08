package io.trust

import sbt.*
import sbt.Keys.*

import scala.sys.process.Process
import scala.util.{ Failure, Success, Try }


// TODO: keep container paths somewhere globally and reuse
// TODO: templates dir should override templates optionally
object TrustSbtPlugin extends AutoPlugin {
  val trustGen = TaskKey[Seq[File]]("trustGen")

  val input = SettingKey[File]("input")
  val config = SettingKey[Option[File]]("config")
  val templates = SettingKey[Option[File]]("templates")

  // TODO: refactor
  override lazy val projectSettings: Seq[Def.Setting[_]] = Seq[sbt.Setting[_]](
    config := None,
    templates := None,
    trustGen := {
      // TODO: refactor processes, use os lib perhaps?
      // TODO: log execution to user
      val configVolume = config.value.map(c => c.getAbsoluteFile).map(c => s"-v $c:/run/trust/cfg.yml").getOrElse("")
      val templatesVolume = templates.value.map(t => t.getAbsoluteFile).map(t => s"-v $t:/usr/src/trust/src/gen/scala").getOrElse("")
      Process(s"docker run --name trust -v ${input.value.getAbsoluteFile.getParent}:/run/trust/api $configVolume $templatesVolume trust generate scala server /run/trust/api/${input.value.getName} /run/trust/out /run/trust/cfg.yml").!
      file(s"${sourceManaged.value}/trust").delete()
      file(s"${sourceManaged.value}/trust/scala").mkdirs()
      Process(s"docker cp trust:/run/trust/out/. ${sourceManaged.value}/trust/scala").!
      Process(s"docker rm -f trust").! // FIXME: Unique container name
      val files = os.walk(os.Path(s"${sourceManaged.value}/trust/scala")).map(_.toIO).filter(_.isFile)
      files
    }
  )
}
