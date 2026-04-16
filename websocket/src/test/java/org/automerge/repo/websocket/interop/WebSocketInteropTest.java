package org.automerge.repo.websocket.interop;

import static org.junit.jupiter.api.Assertions.*;

import java.io.File;
import java.net.InetSocketAddress;
import java.net.URI;
import java.time.Duration;
import java.util.List;
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
import org.automerge.repo.websocket.WebSocketDialer;
import org.automerge.repo.websocket.WebSocketServerTransport;
import org.junit.jupiter.api.BeforeAll;
import org.junit.jupiter.api.Test;

/**
 * Interop tests that spin up the JavaScript automerge-repo sync server and
 * verify that Java repos can sync documents with it over WebSocket.
 *
 * <p>
 * Requires {@code node} to be on the PATH. The interop-test-server sources live
 * in {@code websocket/interop-test-server/} and are built automatically on
 * first run via {@code npm install} + {@code npm run build}.
 *
 * <p>
 * Also requires the native automerge JNI library to be loadable.
 */
class WebSocketInteropTest {

    private static final Duration DEFAULT_TIMEOUT = Duration.ofSeconds(15);
    private static final Duration POLL_INTERVAL = Duration.ofMillis(100);

    private static File interopServerDir;

    @BeforeAll
    static void findInteropServer() {
        interopServerDir = JsServerWrapper.findServerDir();
        assertNotNull(interopServerDir,
                "interop-test-server directory not found. Expected websocket/interop-test-server/ relative to working directory.");
    }

    // -- Tests --

    /**
     * Two Java repos each connect to the JS server and sync a document through
     * it. Mirrors the {@code sync_rust_clients_via_js_server} test in samod.
     */
    @Test
    void twoJavaReposSyncViaJsServer() throws Exception {
        try (JsServerWrapper js = JsServerWrapper.start(interopServerDir)) {
            int port = js.getPort();

            Repo repo1 = Repo.load(RepoConfig.builder()
                    .storage(new InMemoryStorage())
                    .peerId(PeerId.fromString("java-repo1"))
                    .build());

            Repo repo2 = Repo.load(RepoConfig.builder()
                    .storage(new InMemoryStorage())
                    .peerId(PeerId.fromString("java-repo2"))
                    .build());

            try {
                // Both repos connect to the JS server
                DialerHandle dialer1 = connectToJsServer(repo1, port);
                DialerHandle dialer2 = connectToJsServer(repo2, port);

                waitFor(dialer1.onEstablished(), "repo1 connection established");
                waitFor(dialer2.onEstablished(), "repo2 connection established");

                // Create document on repo1
                DocHandle docHandle1 = waitFor(repo1.create(), "create document on repo1");
                DocumentId docId = docHandle1.getDocumentId();

                waitFor(docHandle1.withDocument(doc -> {
                    try (Transaction tx = doc.startTransaction()) {
                        tx.set(ObjectId.ROOT, "source", "java-repo1");
                        tx.commit();
                    }
                    return null;
                }), "set content on repo1");

                // Wait for document to appear on repo2 (via JS server relay)
                eventually(() -> {
                    try {
                        return repo2.find(docId).get() != null;
                    } catch (Exception e) {
                        return false;
                    }
                }).succeeds("document synced to repo2 via JS server");

                // Verify content arrived correctly
                DocHandle docHandle2 = waitFor(repo2.find(docId), "find document on repo2").get();
                String value = waitFor(docHandle2.withDocument(doc -> {
                    return doc.get(ObjectId.ROOT, "source")
                            .map(v -> v instanceof AmValue.Str ? ((AmValue.Str) v).getValue() : null)
                            .orElse(null);
                }), "read value from repo2");

                assertEquals("java-repo1", value, "Document content should have synced from repo1 to repo2 via JS server");

                dialer1.close();
                dialer2.close();
            } finally {
                repo1.close();
                repo2.close();
            }
        }
    }

    /**
     * JS clients create and fetch a document through the Java WebSocket server.
     * Mirrors the {@code two_js_clients_can_sync_through_rust_server} test in samod.
     */
    @Test
    void jsTwoClientsSyncThroughJavaServer() throws Exception {
        Repo serverRepo = Repo.load(RepoConfig.builder()
                .storage(new InMemoryStorage())
                .peerId(PeerId.fromString("java-server"))
                .build());

        AcceptorHandle acceptor = serverRepo.makeAcceptor("ws://0.0.0.0:0");
        WebSocketServerTransport wsServer = new WebSocketServerTransport(
                new InetSocketAddress(0),
                acceptor);
        wsServer.start();
        Thread.sleep(200);
        int port = wsServer.getPort();

        try {
            // JS client creates a document and syncs it to the Java server
            List<String> createOutput = waitForClient(interopServerDir, "create", String.valueOf(port));
            // Output: line 0 = automerge URL, line 1 = comma-separated heads
            assertTrue(createOutput.size() >= 2,
                    "Expected at least 2 output lines from 'create', got: " + createOutput);
            String docUrl = createOutput.get(0).trim();
            String originalHeads = createOutput.get(1).trim();

            assertTrue(docUrl.startsWith("automerge:"),
                    "Expected automerge URL, got: " + docUrl);
            assertFalse(originalHeads.isEmpty(), "Heads should not be empty");

            // Another JS client fetches the same document from the Java server
            List<String> fetchOutput = waitForClient(interopServerDir, "fetch", String.valueOf(port), docUrl);
            assertTrue(fetchOutput.size() >= 1,
                    "Expected at least 1 output line from 'fetch', got: " + fetchOutput);
            String fetchedHeads = fetchOutput.get(0).trim();

            assertEquals(originalHeads, fetchedHeads,
                    "Fetched heads should match original heads - document should have synced through Java server");
        } finally {
            acceptor.close();
            serverRepo.close();
            wsServer.stop();
        }
    }

    /**
     * Verifies that the JS server saves sync state for a non-ephemeral Java repo.
     * Mirrors {@code js_server_saves_sync_state_for_non_ephemeral_samod_peer} in samod.
     */
    @Test
    void jsSavedSyncStateForJavaRepo() throws Exception {
        try (JsServerWrapper js = JsServerWrapper.start(interopServerDir)) {
            int port = js.getPort();

            Repo repo = Repo.load(RepoConfig.builder()
                    .storage(new InMemoryStorage())
                    .peerId(PeerId.fromString("java-storage-repo"))
                    .build());

            try {
                DialerHandle dialerHandle = connectToJsServer(repo, port);
                waitFor(dialerHandle.onEstablished(), "connection established");

                DocHandle docHandle = waitFor(repo.create(), "create document");
                waitFor(docHandle.withDocument(doc -> {
                    try (Transaction tx = doc.startTransaction()) {
                        tx.set(ObjectId.ROOT, "key", "value");
                        tx.commit();
                    }
                    return null;
                }), "set content");

                // Wait for sync and storage to complete
                Thread.sleep(2000);

                List<List<String>> keys = js.storageKeys();
                System.out.println("JS server storage keys: " + keys);

                // The JS server should have saved sync state for the Java peer.
                // Sync state keys have the form [documentId, "sync-state", storageId].
                boolean hasSyncState = keys.stream()
                        .anyMatch(key -> key.size() >= 2 && "sync-state".equals(key.get(1)));

                assertTrue(hasSyncState,
                        "JS server should have saved sync state for the Java peer, but storage keys were: " + keys);

                dialerHandle.close();
            } finally {
                repo.close();
            }
        }
    }

    // -- Helpers --

    private static DialerHandle connectToJsServer(Repo repo, int port) {
        return repo.dial(
                new WebSocketDialer(URI.create("ws://localhost:" + port)));
    }

    /**
     * Runs the JS client script and returns its stdout lines. Wraps the
     * InterruptedException/IOException as RuntimeException for test convenience.
     */
    private static List<String> waitForClient(File serverDir, String... args) {
        try {
            JsServerWrapper clientRunner = new JsServerWrapper(serverDir);
            return clientRunner.runClient(args);
        } catch (Exception e) {
            throw new AssertionError("JS client failed: " + e.getMessage(), e);
        }
    }

    private static <T> T waitFor(CompletableFuture<T> future, String description) {
        try {
            return future.get(DEFAULT_TIMEOUT.toMillis(), TimeUnit.MILLISECONDS);
        } catch (TimeoutException e) {
            throw new AssertionError(
                    String.format("Timeout waiting for: %s (waited %dms)", description, DEFAULT_TIMEOUT.toMillis()), e);
        } catch (ExecutionException e) {
            throw new AssertionError(
                    String.format("Future completed exceptionally while waiting for: %s", description), e.getCause());
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

        EventuallyAssertion timeout(Duration t) {
            this.timeout = t;
            return this;
        }

        void succeeds(String description) {
            long startTime = System.currentTimeMillis();
            long timeoutMs = timeout.toMillis();
            while (System.currentTimeMillis() - startTime < timeoutMs) {
                if (condition.get())
                    return;
                try {
                    Thread.sleep(POLL_INTERVAL.toMillis());
                } catch (InterruptedException e) {
                    Thread.currentThread().interrupt();
                    throw new AssertionError("Interrupted while waiting for: " + description, e);
                }
            }
            throw new AssertionError(String.format("Condition not met within %dms: %s", timeoutMs, description));
        }
    }
}
