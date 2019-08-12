package scala

import io.nats.client.Nats
import java.nio.charset.StandardCharsets

// object for NATS stuff
object NatsWorker {
    // function to try sending a thing to nats
    def publishToNats(subject: String, input: String): Unit = {
        // create a nats connection
        val nc = Nats.connect();

        // send a message to nats
        nc.publish(subject, input.getBytes(StandardCharsets.UTF_8))
    } 
}
