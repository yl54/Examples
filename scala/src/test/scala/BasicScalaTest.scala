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

    // test to check if the date formatter returns the right list
    test("check getPriorDaysList works") {
        // define an amount of days
        val days = 4

        // define a process day
        val processDate = "2019-02-03"

        // expected
        val expected = List("2019-02-03", "2019-02-02", "2019-02-01", "2019-01-31", "2019-01-30")

        // actual
        val actual = BasicScala.getPriorDaysList(days, processDate)

        // assert
        assert(expected === actual)
    }
}