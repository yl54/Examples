name := "scala"

version := "0.1"

scalaVersion := "2.11.8"

libraryDependencies ++= Seq(
    "com.hubspot.jinjava" % "jinjava" % "2.4.14",
    "org.scalatest" %% "scalatest" % "3.0.1" % "test",
)

lazy val scalafmtSettings =
  Seq(
    scalafmtOnCompile := true,
    scalafmtTestOnCompile := true,
    scalafmtVersion := "1.2.0"
  )

parallelExecution in ThisBuild := false
parallelExecution in Test := false