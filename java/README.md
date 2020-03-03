# Java Project

## Summary
This is an example Java project. It gets libraries from Maven and leverages the Gradle build system. It also includes a protobuf library.

## Useful Commands

* `gradle jar`: Creates a jar. Good check to see if your code compiles properly.
* `gradle test`: Runs the unit tests for your code. Easier to setup than running a specific main from a jar.
* `./gradlew generateProto`: Generate the java protobuf code files.
* `./gradlew build`: Build the project. It will handle generating the proto files along the way.
* `./gradlew test`: Build and test the project.
