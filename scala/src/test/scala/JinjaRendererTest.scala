package scala

import org.scalatest.{ FunSuite }

class JinjaRendererTest extends FunSuite {
    test("check renderTemplateFile returns expected output") {
        val expected: String = "Jared: Alaska"
        val actual = JinjaRenderer.renderTemplateFile("file")
        assert(expected == actual)
    }
}