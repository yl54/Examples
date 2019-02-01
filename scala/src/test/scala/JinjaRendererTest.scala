package scala

import java.util.{HashMap}

import org.scalatest.{ FunSuite }

class JinjaRendererTest extends FunSuite {
    test("check renderTemplateFile returns expected output") {
        // Get the file contents to render into
        val template = "{{name}}: {{dest}}"

        // Set the variables necessary to render into the file
        var context = new HashMap[String, String]()
        context.put("name", "Jared")
        context.put("dest", "Alaska")

        val expected: String = "Jared: Alaska"
        val actual = JinjaRenderer.renderTemplateFile(template, context)
        assert(expected == actual)
    }

    test("check renderTemplateFile works for keyword or") {
        // Get the template
        val template = "{{name or ''}}: {{dest or '\"\"'}}: game: {{game or 'sm'}}: thing: {{thing or ''}}" 

        // Set the necessary variables
        var context = new HashMap[String, String]()
        context.put("name", "Jared")

        val expected: String = "Jared: \"\": game: sm: thing: "
        val actual = JinjaRenderer.renderTemplateFile(template, context)

        println(expected)
        println(actual)
        assert(expected == actual)
    }
}