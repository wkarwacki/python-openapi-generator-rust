organization := "io.trust"
name := "sbt-trust"
version := "0.1.0-SNAPSHOT"

sbtPlugin := true

enablePlugins(SbtPlugin)

libraryDependencies += "com.lihaoyi" %% "os-lib" % "0.10.1"
