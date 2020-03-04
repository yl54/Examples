package com.example.me;

import org.junit.Test;
import org.junit.runner.RunWith;
import org.junit.runners.JUnit4;

import static org.junit.Assert.assertEquals;

import io.grpc.ManagedChannel;
import io.grpc.ManagedChannelBuilder;

@RunWith(JUnit4.class)
public class GrpcClientTest {
    // private final GreeterGrpc.GreeterBlockingStub blockingStub;

    @Test
    public void testSanityCheck() {
        assertEquals(2, 2);
    }

    // Test to do a simple grpc client thing
    @Test
    public void testGrpcCallGranular() {
        // Define the host and port
        String user = "world";
        // Access a service running on the local machine on port 50051
        String target = "localhost:50051";

        // Create a communication channel to the server, known as a Channel. Channels are thread-safe
        // and reusable. It is common to create channels at the beginning of your application and reuse
        // them until the application shuts down.
        ManagedChannel channel = ManagedChannelBuilder.forTarget(target)
                // Channels are secure by default (via SSL/TLS). For the example we disable TLS to avoid
                // needing certificates.
                .usePlaintext()
                .build();

        // Create a client
        GrpcClient client = new GrpcClient(channel);

        // Make a call to the service
        GrpcClient.CallClient(channel, user);

        // Close the connection
        try {
            GrpcClient.CloseClient(channel);
        } catch (Exception e) {
            // move on with it
        }
    }
}
