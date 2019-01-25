import sbt.util
logLevel := util.Level.Error

addSbtPlugin("io.get-coursier" % "sbt-coursier" % "1.0.2")
addSbtPlugin("com.eed3si9n" % "sbt-assembly" % "0.14.5")
addSbtPlugin("org.scalastyle" %% "scalastyle-sbt-plugin" % "1.0.0")
addSbtPlugin("com.lucidchart" % "sbt-scalafmt-coursier" % "1.12")
