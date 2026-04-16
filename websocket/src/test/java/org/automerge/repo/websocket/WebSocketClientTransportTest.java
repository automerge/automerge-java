package org.automerge.repo.websocket;

import static org.junit.jupiter.api.Assertions.*;

import java.net.InetSocketAddress;
import java.net.URI;
import java.nio.ByteBuffer;
import java.util.concurrent.CompletableFuture;
import java.util.concurrent.CopyOnWriteArrayList;
import java.util.concurrent.CountDownLatch;
import java.util.concurrent.TimeUnit;
import java.util.concurrent.atomic.AtomicInteger;
import org.automerge.repo.Transport;
import org.java_websocket.WebSocket;
import org.java_websocket.client.WebSocketClient;
import org.java_websocket.handshake.ClientHandshake;
import org.java_websocket.handshake.ServerHandshake;
import org.java_websocket.server.WebSocketServer;
import org.junit.jupiter.api.AfterEach;
import org.junit.jupiter.api.BeforeEach;
import org.junit.jupiter.api.Test;

class WebSocketClientTransportTest {

    private TestServer server;
    private int port;

    @BeforeEach
    void setUp() throws Exception {
        server = new TestServer(new InetSocketAddress(0));
        server.start();
        // Wait for server to start
        assertTrue(server.startLatch.await(5, TimeUnit.SECONDS), "Server should start");
        port = server.getPort();
    }

    @AfterEach
    void tearDown() throws Exception {
        if (server != null) {
            server.stop();
        }
    }

    /**
     * Creates a connected WebSocket client + Transport pair. This mirrors what
     * {@link WebSocketDialer#connect()} does internally, but returns the
     * transport for direct testing.
     */
    private Transport connectTransport() throws InterruptedException {
        final WebSocketClient[] clientHolder = new WebSocketClient[1];

        Transport transport = new Transport(
                data -> clientHolder[0].send(data),
                () -> clientHolder[0].close());

        WebSocketClient wsClient = new WebSocketClient(
                URI.create("ws://localhost:" + port)) {
            @Override
            public void onOpen(ServerHandshake handshake) {}

            @Override
            public void onMessage(String message) {}

            @Override
            public void onMessage(ByteBuffer bytes) {
                byte[] data = new byte[bytes.remaining()];
                bytes.get(data);
                transport.onMessage(data);
            }

            @Override
            public void onClose(int code, String reason, boolean remote) {
                transport.onClose();
            }

            @Override
            public void onError(Exception ex) {}
        };
        clientHolder[0] = wsClient;
        wsClient.setConnectionLostTimeout(0);

        assertTrue(wsClient.connectBlocking(5, TimeUnit.SECONDS), "Should connect");
        return transport;
    }

    @Test
    void testConnectAndReceiveBinaryMessage() throws Exception {
        CountDownLatch messageLatch = new CountDownLatch(1);
        CopyOnWriteArrayList<byte[]> received = new CopyOnWriteArrayList<>();

        Transport transport = connectTransport();
        transport.connect(msg -> {
            received.add(msg);
            messageLatch.countDown();
        }, () -> {});

        try {
            WebSocket conn = server.lastConnection;
            assertNotNull(conn);
            conn.send(new byte[]{1, 2, 3});

            assertTrue(messageLatch.await(5, TimeUnit.SECONDS), "Should receive message");
            assertEquals(1, received.size());
            assertArrayEquals(new byte[]{1, 2, 3}, received.get(0));
        } finally {
            transport.close();
        }
    }

    @Test
    void testSendAfterConnection() throws Exception {
        CountDownLatch serverReceiveLatch = new CountDownLatch(1);
        CopyOnWriteArrayList<byte[]> serverReceived = new CopyOnWriteArrayList<>();
        server.onMessageCallback = (conn, data) -> {
            serverReceived.add(data);
            serverReceiveLatch.countDown();
        };

        Transport transport = connectTransport();
        transport.connect(msg -> {}, () -> {});

        try {
            CompletableFuture<Void> sendFuture = transport.send(new byte[]{40, 50, 60});
            sendFuture.get(5, TimeUnit.SECONDS);

            assertTrue(serverReceiveLatch.await(5, TimeUnit.SECONDS), "Server should receive message");
            assertArrayEquals(new byte[]{40, 50, 60}, serverReceived.get(0));
        } finally {
            transport.close();
        }
    }

    @Test
    void testCloseCallsOnCloseExactlyOnce() throws Exception {
        AtomicInteger closeCount = new AtomicInteger(0);
        CountDownLatch closeLatch = new CountDownLatch(1);

        Transport transport = connectTransport();
        transport.connect(msg -> {}, () -> {
            closeCount.incrementAndGet();
            closeLatch.countDown();
        });

        transport.close();

        assertTrue(closeLatch.await(5, TimeUnit.SECONDS), "onClose should be called");
        Thread.sleep(200); // Wait to see if called again
        assertEquals(1, closeCount.get(), "onClose should be called exactly once");
    }

    @Test
    void testCloseIsIdempotent() throws Exception {
        AtomicInteger closeCount = new AtomicInteger(0);

        Transport transport = connectTransport();
        transport.connect(msg -> {}, () -> {
            closeCount.incrementAndGet();
        });

        transport.close();
        transport.close();
        transport.close();

        Thread.sleep(200);
        assertEquals(1, closeCount.get(), "onClose should be called exactly once despite multiple close() calls");
    }

    @Test
    void testSendAfterCloseFails() throws Exception {
        Transport transport = connectTransport();
        transport.connect(msg -> {}, () -> {});

        transport.close();

        CompletableFuture<Void> sendFuture = transport.send(new byte[]{1});
        assertTrue(sendFuture.isCompletedExceptionally(), "Send after close should fail");
    }

    @Test
    void testDialerConnectFailsForBadPort() throws Exception {
        WebSocketDialer dialer = new WebSocketDialer(
                WebSocketClientTransportConfig.builder(URI.create("ws://localhost:1")).build());
        CompletableFuture<?> result = dialer.connect();
        try {
            result.get(5, TimeUnit.SECONDS);
            fail("Connection to bad port should fail");
        } catch (java.util.concurrent.ExecutionException e) {
            // expected
        }
    }

    @Test
    void testMultipleMessagesArriveInOrder() throws Exception {
        int messageCount = 10;
        CountDownLatch messageLatch = new CountDownLatch(messageCount);
        CopyOnWriteArrayList<byte[]> received = new CopyOnWriteArrayList<>();

        Transport transport = connectTransport();
        transport.connect(msg -> {
            received.add(msg);
            messageLatch.countDown();
        }, () -> {});

        try {
            WebSocket conn = server.lastConnection;
            for (int i = 0; i < messageCount; i++) {
                conn.send(new byte[]{(byte) i});
            }

            assertTrue(messageLatch.await(5, TimeUnit.SECONDS), "Should receive all messages");
            assertEquals(messageCount, received.size());
            for (int i = 0; i < messageCount; i++) {
                assertArrayEquals(new byte[]{(byte) i}, received.get(i), "Message " + i + " should be in order");
            }
        } finally {
            transport.close();
        }
    }

    @Test
    void testRemoteCloseTriggersOnClose() throws Exception {
        CountDownLatch closeLatch = new CountDownLatch(1);

        Transport transport = connectTransport();
        transport.connect(msg -> {}, () -> {
            closeLatch.countDown();
        });

        try {
            // Server closes the connection
            server.lastConnection.close();

            assertTrue(closeLatch.await(5, TimeUnit.SECONDS), "onClose should be called on remote close");
        } finally {
            transport.close();
        }
    }

    /**
     * Minimal WebSocket server for testing.
     */
    static class TestServer extends WebSocketServer {

        final CountDownLatch startLatch = new CountDownLatch(1);
        final CountDownLatch connectionLatch = new CountDownLatch(1);
        volatile WebSocket lastConnection;
        volatile MessageCallback onMessageCallback;

        interface MessageCallback {
            void onMessage(WebSocket conn, byte[] data);
        }

        TestServer(InetSocketAddress address) {
            super(address);
            setReuseAddr(true);
        }

        @Override
        public void onOpen(WebSocket conn, ClientHandshake handshake) {
            lastConnection = conn;
            connectionLatch.countDown();
        }

        @Override
        public void onClose(WebSocket conn, int code, String reason, boolean remote) {}

        @Override
        public void onMessage(WebSocket conn, String message) {}

        @Override
        public void onMessage(WebSocket conn, ByteBuffer message) {
            if (onMessageCallback != null) {
                byte[] data = new byte[message.remaining()];
                message.get(data);
                onMessageCallback.onMessage(conn, data);
            }
        }

        @Override
        public void onError(WebSocket conn, Exception ex) {}

        @Override
        public void onStart() {
            startLatch.countDown();
        }
    }
}
