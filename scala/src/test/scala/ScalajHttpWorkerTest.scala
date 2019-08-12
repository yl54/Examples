package scala

import org.scalatest.{ FunSuite }

class ScalajHttpWorkerTest extends FunSuite {
    // Ignore unless there is a webserver with an status endpoint
    ignore("Check sendRequestExample works") {
        val expected = "ok"
        val actual = ScalajHttpWorker.sendRequestExample()
        assert(expected == actual)
    }
}