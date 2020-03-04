package com.example.me;

// Imports
import io.grpc.Channel;
import io.grpc.ManagedChannel;
import io.grpc.ManagedChannelBuilder;
import io.grpc.StatusRuntimeException;
import java.util.concurrent.TimeUnit;
import java.util.logging.Level;
import java.util.logging.Logger;



public class GrpcClient {
    private final GreeterGrpc.GreeterBlockingStub blockingStub;

    // Function to create a new gRPC client
    /** Construct client for accessing HelloWorld server using the existing channel. */
    public GrpcClient(ManagedChannel channel) {
        // 'channel' here is a Channel, not a ManagedChannel, so it is not this code's responsibility to
        // shut it down.

        // Passing Channels to code makes code easier to test and makes it easier to reuse Channels.
        blockingStub = GreeterGrpc.newBlockingStub(channel);
    }

    // Function to make a quick protobuf call
    public static void CallClient(ManagedChannel channel, String user) {
        try {
            GrpcClient client = new GrpcClient(channel);
            client.greet(user);
        } catch(Exception e) {
            // move on with it
        }
    }

    /** Say hello to server. */
    public void greet(String name) {
        HelloRequest request = HelloRequest.newBuilder().setName(name).build();
        HelloReply response;
        try {
            response = blockingStub.sayHello(request);
        } catch (StatusRuntimeException e) {
            return;
        }
    }

    // Function to close the client
    public static void CloseClient(ManagedChannel channel) throws InterruptedException {
        channel.shutdown().awaitTermination(5, TimeUnit.SECONDS);
    }

    // Function to check for expected response of protobuf call
}
