name := "trust"
version := "0.1.0"
scalaVersion := "3.4.2"

val jsoniterScalaVersion = "2.28.5"

libraryDependencies ++= Seq(
  "com.lihaoyi" %% "cask" % "0.9.2",
  "com.github.plokhotnyuk.jsoniter-scala" %% "jsoniter-scala-circe" % jsoniterScalaVersion,
  "com.github.plokhotnyuk.jsoniter-scala" %% "jsoniter-scala-core" % jsoniterScalaVersion,
  "com.github.plokhotnyuk.jsoniter-scala" %% "jsoniter-scala-macros" % jsoniterScalaVersion % "compile-internal",
  "io.circe" %% "circe-core" % "0.15.0-M1"
)

scalafmtOnCompile := true
