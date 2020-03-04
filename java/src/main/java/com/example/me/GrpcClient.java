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
    public GrpcClient(Channel channel) {
        // 'channel' here is a Channel, not a ManagedChannel, so it is not this code's responsibility to
        // shut it down.

        // Passing Channels to code makes code easier to test and makes it easier to reuse Channels.
        blockingStub = GreeterGrpc.newBlockingStub(channel);
    }

    // Function to make a quick protobuf call

    // Function to check for expected response of protobuf call
}
