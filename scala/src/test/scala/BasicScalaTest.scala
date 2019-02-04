package scala

import org.scalatest.{ FunSuite }

class BasicScalaTest extends FunSuite {
    test("Check SeqToString works") {
        val slist = Seq("\"thing\"", "\"other\"", "\"word\"")
        val delimiter = ", "

        // Make the expected string
        val expected = "[ \"thing\", \"other\", \"word\" ]"

        // actual 
        val actual = "[ " + BasicScala.SeqToString(slist, delimiter) + " ]"

        // expected == actual
        assert(expected == actual)
    }
}