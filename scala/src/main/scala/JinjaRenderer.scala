package scala

import java.util.{Map}

import com.hubspot.jinjava.Jinjava
import com.hubspot.jinjava.interpret.JinjavaInterpreter
import com.hubspot.jinjava.loader.FileLocator
import com.hubspot.jinjava.loader.ResourceLocator
import com.hubspot.jinjava.tree.Node

// object for renderer runner
object JinjaRenderer {
    def renderTemplateFile(template: String, context: Map[String, String]): String = {
        // Create a renderer object
        val jinjava = new Jinjava()

        // Render the variables into the file
        return jinjava.render(template, context)
    }
}