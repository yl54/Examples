package scala

import java.util.{Map}

import com.hubspot.jinjava.Jinjava

// object for renderer runner
object JinjaRenderer {
    def renderTemplateFile(template: String, context: Map[String, String]): String = {
        // Create a renderer object
        val jinjava = new Jinjava()

        // Render the variables into the file
        return jinjava.render(template, context)
    }
}