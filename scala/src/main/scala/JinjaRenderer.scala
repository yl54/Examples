package scala

import java.util.{HashMap, Map}


import com.hubspot.jinjava.Jinjava
import com.hubspot.jinjava.interpret.JinjavaInterpreter
import com.hubspot.jinjava.loader.FileLocator
import com.hubspot.jinjava.loader.ResourceLocator
import com.hubspot.jinjava.tree.Node

// object for renderer runner
object JinjaRenderer {
    def renderTemplateFile(file: String): String = {
        // Create a renderer object

        // Set the variables necessary to render into the file

        // Render the variables into the file

        // Get the output of that rendered stuff

        val jinjava = new Jinjava()
        var context = new HashMap[String, String]()
        context.put("name", "Jared")
        context.put("dest", "Alaska")

        val template = "{{ name }}: {{ dest }}"

        val renderedTemplate = jinjava.render(template, context)

        return renderedTemplate
    }
}