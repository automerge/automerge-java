package org.automerge.repo.websocket;

import static org.junit.jupiter.api.Assertions.*;

import java.net.InetSocketAddress;
import java.net.URI;
import java.time.Duration;
import java.util.concurrent.CompletableFuture;
import java.util.concurrent.ExecutionException;
import java.util.concurrent.TimeUnit;
import java.util.concurrent.TimeoutException;
import java.util.function.Supplier;
import org.automerge.AmValue;
import org.automerge.ObjectId;
import org.automerge.Transaction;
import org.automerge.repo.AcceptorHandle;
import org.automerge.repo.DialerHandle;
import org.automerge.repo.DocHandle;
import org.automerge.repo.DocumentId;
import org.automerge.repo.PeerId;
import org.automerge.repo.Repo;
import org.automerge.repo.RepoConfig;
import org.automerge.repo.storage.InMemoryStorage;
import org.junit.jupiter.api.Test;

/**
 * Integration tests using both WebSocket client and server transports together
 * with real Repo instances.
 *
 * <p>
 * These tests require a working native library (automerge-jni). They will fail
 * in environments where the JNI hub event loop cannot function properly (e.g.,
 * sandboxed environments with restricted threading).
 */
class WebSocketIntegrationTest {

    private static final Duration DEFAULT_TIMEOUT = Duration.ofSeconds(10);
    private static final Duration POLL_INTERVAL = Duration.ofMillis(50);

    @Test
    void testTwoReposSyncOverWebSocket() throws Exception {
        Repo serverRepo = Repo.load(RepoConfig.builder().peerId(PeerId.fromString("server")).build());

        AcceptorHandle acceptor = serverRepo.makeAcceptor("ws://0.0.0.0:0");
        WebSocketServerTransport wsServer = new WebSocketServerTransport(new InetSocketAddress(0), acceptor);
        wsServer.start();
        Thread.sleep(200);
        int port = wsServer.getPort();

        Repo clientRepo = Repo.load(RepoConfig.builder().storage(new InMemoryStorage()).peerId(PeerId.fromString("client")).build());

        try {
            // Connect client to server via Dialer
            DialerHandle dialerHandle = clientRepo.dial(
                    new WebSocketDialer(URI.create("ws://localhost:" + port)));
            waitFor(dialerHandle.onEstablished(), "client connection established");

            // Create document on client
            DocHandle clientHandle = waitFor(clientRepo.create(), "create document on client");
            DocumentId docId = clientHandle.getDocumentId();

            waitFor(clientHandle.withDocument(doc -> {
                try (Transaction tx = doc.startTransaction()) {
                    tx.set(ObjectId.ROOT, "source", "client");
                    tx.commit();
                }
                return null;
            }), "set content on client");

            // Wait for document to appear on server
            eventually(() -> {
                try {
                    return serverRepo.find(docId).get() != null;
                } catch (Exception e) {
                    return false;
                }
            }).succeeds("document appears on server");

            // Verify content on server
            DocHandle serverHandle = waitFor(serverRepo.find(docId), "find document on server").get();
            String value = waitFor(serverHandle.withDocument(doc -> {
                return doc.get(ObjectId.ROOT, "source").map(v -> v instanceof AmValue.Str ? ((AmValue.Str) v).getValue() : null).orElse(null);
            }), "read value from server");

            assertEquals("client", value, "Document should sync from client to server");

            dialerHandle.close();
        } finally {
            acceptor.close();
            clientRepo.close();
            serverRepo.close();
            wsServer.stop();
        }
    }

    @Test
    void testBidirectionalSync() throws Exception {
        Repo serverRepo = Repo.load(RepoConfig.builder().storage(new InMemoryStorage()).peerId(PeerId.fromString("server")).build());

        AcceptorHandle acceptor = serverRepo.makeAcceptor("ws://0.0.0.0:0");
        WebSocketServerTransport wsServer = new WebSocketServerTransport(new InetSocketAddress(0), acceptor);
        wsServer.start();
        Thread.sleep(200);
        int port = wsServer.getPort();

        Repo clientRepo = Repo.load(RepoConfig.builder().storage(new InMemoryStorage()).peerId(PeerId.fromString("client")).build());

        try {
            DialerHandle dialerHandle = clientRepo.dial(
                    new WebSocketDialer(URI.create("ws://localhost:" + port)));
            waitFor(dialerHandle.onEstablished(), "connection established");

            // Create document on server
            DocHandle serverHandle = waitFor(serverRepo.create(), "create document on server");
            DocumentId docId = serverHandle.getDocumentId();

            waitFor(serverHandle.withDocument(doc -> {
                try (Transaction tx = doc.startTransaction()) {
                    tx.set(ObjectId.ROOT, "server-key", "server-value");
                    tx.commit();
                }
                return null;
            }), "set content on server");

            // Wait for document to appear on client
            eventually(() -> {
                try {
                    return clientRepo.find(docId).get() != null;
                } catch (Exception e) {
                    return false;
                }
            }).succeeds("document appears on client");

            DocHandle clientHandle = waitFor(clientRepo.find(docId), "find document on client").get();

            // Verify server content arrived
            String serverValue = waitFor(clientHandle.withDocument(doc -> {
                return doc.get(ObjectId.ROOT, "server-key").map(v -> v instanceof AmValue.Str ? ((AmValue.Str) v).getValue() : null).orElse(null);
            }), "read server value from client");
            assertEquals("server-value", serverValue);

            // Now modify on client side
            waitFor(clientHandle.withDocument(doc -> {
                try (Transaction tx = doc.startTransaction()) {
                    tx.set(ObjectId.ROOT, "client-key", "client-value");
                    tx.commit();
                }
                return null;
            }), "set content on client");

            // Wait for client change to sync to server
            eventually(() -> {
                try {
                    String val = serverHandle.withDocument(doc -> {
                        return doc.get(ObjectId.ROOT, "client-key").map(v -> v instanceof AmValue.Str ? ((AmValue.Str) v).getValue() : null).orElse(null);
                    }).get();
                    return "client-value".equals(val);
                } catch (Exception e) {
                    return false;
                }
            }).succeeds("client change syncs to server");

            dialerHandle.close();
        } finally {
            acceptor.close();
            clientRepo.close();
            serverRepo.close();
            wsServer.stop();
        }
    }

    @Test
    void testDisconnectAndReconnect() throws Exception {
        Repo serverRepo = Repo.load(RepoConfig.builder().storage(new InMemoryStorage()).peerId(PeerId.fromString("server")).build());

        AcceptorHandle acceptor = serverRepo.makeAcceptor("ws://0.0.0.0:0");
        WebSocketServerTransport wsServer = new WebSocketServerTransport(new InetSocketAddress(0), acceptor);
        wsServer.start();
        Thread.sleep(200);
        int port = wsServer.getPort();

        Repo clientRepo = Repo.load(RepoConfig.builder().storage(new InMemoryStorage()).peerId(PeerId.fromString("client")).build());

        try {
            // First connection: create and sync a document
            DialerHandle dialer1 = clientRepo.dial(
                    new WebSocketDialer(URI.create("ws://localhost:" + port)));
            waitFor(dialer1.onEstablished(), "first connection established");

            DocHandle clientHandle = waitFor(clientRepo.create(), "create document");
            DocumentId docId = clientHandle.getDocumentId();

            waitFor(clientHandle.withDocument(doc -> {
                try (Transaction tx = doc.startTransaction()) {
                    tx.set(ObjectId.ROOT, "phase", "initial");
                    tx.commit();
                }
                return null;
            }), "set initial content");

            eventually(() -> {
                try {
                    return serverRepo.find(docId).get() != null;
                } catch (Exception e) {
                    return false;
                }
            }).succeeds("document synced to server");

            // Disconnect
            dialer1.close();

            // Modify while disconnected
            waitFor(clientHandle.withDocument(doc -> {
                try (Transaction tx = doc.startTransaction()) {
                    tx.set(ObjectId.ROOT, "phase", "after-disconnect");
                    tx.commit();
                }
                return null;
            }), "modify while disconnected");

            // Reconnect with a new dialer
            DialerHandle dialer2 = clientRepo.dial(
                    new WebSocketDialer(URI.create("ws://localhost:" + port)));
            waitFor(dialer2.onEstablished(), "second connection established");

            // Wait for disconnected change to sync
            DocHandle serverHandle = waitFor(serverRepo.find(docId), "find document on server").get();
            eventually(() -> {
                try {
                    String val = serverHandle.withDocument(doc -> {
                        return doc.get(ObjectId.ROOT, "phase").map(v -> v instanceof AmValue.Str ? ((AmValue.Str) v).getValue() : null).orElse(null);
                    }).get();
                    return "after-disconnect".equals(val);
                } catch (Exception e) {
                    return false;
                }
            }).succeeds("disconnected change syncs after reconnect");

            dialer2.close();
        } finally {
            acceptor.close();
            clientRepo.close();
            serverRepo.close();
            wsServer.stop();
        }
    }

    // -- Helpers --

    private static <T> T waitFor(CompletableFuture<T> future, String description) {
        try {
            return future.get(DEFAULT_TIMEOUT.toMillis(), TimeUnit.MILLISECONDS);
        } catch (TimeoutException e) {
            throw new AssertionError(String.format("Timeout waiting for: %s (waited %dms)", description, DEFAULT_TIMEOUT.toMillis()), e);
        } catch (ExecutionException e) {
            throw new AssertionError(String.format("Future completed exceptionally while waiting for: %s", description), e.getCause());
        } catch (InterruptedException e) {
            Thread.currentThread().interrupt();
            throw new AssertionError(String.format("Interrupted while waiting for: %s", description), e);
        }
    }

    private static EventuallyAssertion eventually(Supplier<Boolean> condition) {
        return new EventuallyAssertion(condition);
    }

    static class EventuallyAssertion {

        private final Supplier<Boolean> condition;
        private Duration timeout = DEFAULT_TIMEOUT;

        EventuallyAssertion(Supplier<Boolean> condition) {
            this.condition = condition;
        }

        EventuallyAssertion timeout(Duration timeout) {
            this.timeout = timeout;
            return this;
        }

        void succeeds(String description) {
            long startTime = System.currentTimeMillis();
            long timeoutMs = timeout.toMillis();
            while (System.currentTimeMillis() - startTime < timeoutMs) {
                if (condition.get()) {
                    return;
                }
                try {
                    Thread.sleep(POLL_INTERVAL.toMillis());
                } catch (InterruptedException e) {
                    Thread.currentThread().interrupt();
                    throw new AssertionError("Interrupted while waiting for: " + description, e);
                }
            }
            throw new AssertionError(String.format("Condition not met within timeout: %s (waited %dms)", description, timeoutMs));
        }
    }
}
