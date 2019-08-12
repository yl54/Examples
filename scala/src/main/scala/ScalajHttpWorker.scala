package scala

import org.json4s.JsonAST.JNothing
import scalaj.http._
import scalaj.http

import org.json4s.{ DefaultFormats, JValue, MappingException }
import org.json4s.jackson.JsonMethods.parse

object ScalajHttpWorker {
    implicit class JValueExtended(value: JValue) {
    def has(childString: String): Boolean =
          if ((value \ childString) != JNothing) {
            true
          } else {
            false
          }
      }

      private implicit val formats = DefaultFormats

    def sendRequestExample(): String = {
        // Create url
        val url = "http://localhost:20000/status"

        // Create an http request
        val request: http.HttpRequest = Http(url)
          .header("Content-Type", "application/json")

        // Send the request and get the response
        val response = request.asString
        if (response.isError) {
          throw new Exception("Response code: %s\nResponse body: %s\n".format(response.code, response.body))
        }

        // Get the body (this should be something known)
        val content = parse(response.body)

        // Throws a MappingException for parsing exceptions
        val nested = content.extract[NestedResponse].nest

        content.extract[NestedResponse].nest.status
    }
}

// Case class for parsing body into object
final case class StatusResponse(
    status: String
)

final case class NestedResponse(
    status: String,
    nest: StatusResponse
)
