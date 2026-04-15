package org.automerge.repo.integration;

import static org.automerge.repo.integration.helpers.TestHelpers.eventually;
import static org.automerge.repo.integration.helpers.TestHelpers.logf;
import static org.automerge.repo.integration.helpers.TestHelpers.sleep;
import static org.automerge.repo.integration.helpers.TestHelpers.waitFor;
import static org.junit.jupiter.api.Assertions.assertEquals;
import static org.junit.jupiter.api.Assertions.assertNotNull;
import static org.junit.jupiter.api.Assertions.assertNull;
import static org.junit.jupiter.api.Assertions.assertTrue;

import java.time.Duration;
import java.util.concurrent.CompletableFuture;
import org.automerge.AmValue;
import org.automerge.ObjectId;
import org.automerge.Transaction;
import org.automerge.repo.AcceptorHandle;
import org.automerge.repo.AnnounceAll;
import org.automerge.repo.AnnounceNone;
import org.automerge.repo.DialerHandle;
import org.automerge.repo.DocHandle;
import org.automerge.repo.DocumentId;
import org.automerge.repo.PeerId;
import org.automerge.repo.Repo;
import org.automerge.repo.RepoConfig;
import org.automerge.repo.integration.helpers.ChannelDialer;
import org.junit.jupiter.api.Test;

/**
 * Integration tests for document synchronization between repos.
 *
 * These tests validate: - Document creation and replication across connected
 * repos - Document modification sync - CRDT merge behavior with concurrent
 * edits - Sync resumption after disconnect/reconnect - Multiple document sync -
 * Announce policy behavior
 *
 * Tests use ChannelDialer/ChannelAdapter for in-memory communication without
 * actual networking.
 */
class DocumentSyncTest {

    /**
     * Helper to create two connected repos.
     */
    private static class RepoConnectionPair implements AutoCloseable {

        final Repo alice;
        final Repo bob;
        DialerHandle dialerHandle;
        AcceptorHandle acceptorHandle;

        RepoConnectionPair(
                Repo alice,
                Repo bob,
                DialerHandle dialerHandle,
                AcceptorHandle acceptorHandle) {
            this.alice = alice;
            this.bob = bob;
            this.dialerHandle = dialerHandle;
            this.acceptorHandle = acceptorHandle;
        }

        @Override
        public void close() {
            // Close dialer and acceptor first
            if (dialerHandle != null) {
                dialerHandle.close();
            }
            if (acceptorHandle != null) {
                acceptorHandle.close();
            }

            // Then close repos
            if (alice != null) {
                alice.close();
            }
            if (bob != null) {
                bob.close();
            }
        }

        void disconnect() {
            if (dialerHandle != null) {
                dialerHandle.close();
                dialerHandle = null;
            }
            if (acceptorHandle != null) {
                acceptorHandle.close();
                acceptorHandle = null;
            }
        }

        CompletableFuture<Void> connect() {
            AcceptorHandle newAcceptor = bob.makeAcceptor("channel://bob");
            DialerHandle newDialer = alice.dial(
                    new ChannelDialer(newAcceptor));

            this.acceptorHandle = newAcceptor;
            this.dialerHandle = newDialer;

            return newDialer.onEstablished().thenApply(peerId -> {
                return null;
            });
        }
    }

    /**
     * Creates two repos and establishes a connection between them.
     *
     * @return A pair of connected repos
     */
    private static RepoConnectionPair createConnectedRepos() throws Exception {
        // Create two repos with in-memory storage
        RepoConfig aliceConfig = RepoConfig.builder()
                .peerId(PeerId.fromString("alice"))
                .build();
        RepoConfig bobConfig = RepoConfig.builder()
                .peerId(PeerId.fromString("bob"))
                .build();

        Repo alice = Repo.load(aliceConfig);
        Repo bob = Repo.load(bobConfig);

        AcceptorHandle acceptorB = bob.makeAcceptor("channel://bob");
        DialerHandle dialerA = alice.dial(
                new ChannelDialer(acceptorB));

        RepoConnectionPair pair = new RepoConnectionPair(alice, bob, dialerA, acceptorB);

        waitFor(dialerA.onEstablished(), "connection established");
        return pair;
    }

    @Test
    void testCanCreateConnectedRepos() throws Exception {
        // Just test that we can create and connect two repos
        try (RepoConnectionPair pair = createConnectedRepos()) {
            assertNotNull(pair.alice, "Alice should not be null");
            assertNotNull(pair.bob, "Bob should not be null");
            assertTrue(
                    pair.dialerHandle.isConnected(),
                    "Dialer should be connected");
        }
    }

    @Test
    void testCanCreateDocumentAfterConnection() throws Exception {
        try (RepoConnectionPair pair = createConnectedRepos()) {
            logf("Creating document in repo A");
            DocHandle aliceHandle = waitFor(
                    pair.alice.create(),
                    "create document in Alice");
            assertNotNull(aliceHandle, "Document handle should not be null");

            DocumentId docId = aliceHandle.getDocumentId();
            assertNotNull(docId, "Document ID should not be null");
            logf("Created document: %s", docId);

            // Modify it
            logf("Modifying document");
            waitFor(
                    aliceHandle.withDocument(doc -> {
                        try (Transaction tx = doc.startTransaction()) {
                            tx.set(ObjectId.ROOT, "test", "value");
                            tx.commit();
                        }
                        return null;
                    }),
                    "modify document in Alice");
            logf("Document modified successfully");

            // Try to find it in repo B
            logf("Looking for document in Bob");
            DocHandle bobHandle = waitFor(
                    pair.bob.find(docId),
                    Duration.ofSeconds(2),
                    "find in Bob").get();
            logf("Find completed, result: %s", bobHandle);
        }
    }

    @Test
    void testDocumentCreatedInAAppearsInB() throws Exception {
        try (RepoConnectionPair pair = createConnectedRepos()) {
            logf("Repos connected");

            // Create a document in repo A
            DocHandle aliceHandle = waitFor(
                    pair.alice.create(),
                    "create document in Alice");
            DocumentId docId = aliceHandle.getDocumentId();
            logf("Created document in Alice: %s", docId);

            // Modify it so there's content to sync
            waitFor(
                    aliceHandle.withDocument(doc -> {
                        try (Transaction tx = doc.startTransaction()) {
                            tx.set(ObjectId.ROOT, "key", "value");
                            tx.commit();
                        }
                        return null;
                    }),
                    "modify document in Alice");
            logf("Modified document in Alice");

            // Wait for document to appear in repo B
            logf("Waiting for document to appear in Bob...");
            eventually(() -> {
                try {
                    DocHandle bobHandle = pair.bob.find(docId).get().get();
                    boolean found = bobHandle != null;
                    if (found) {
                        logf("Document found in Bob!");
                    }
                    return found;
                } catch (Exception e) {
                    logf(
                            "Error checking for document in Bob: %s",
                            e.getMessage());
                    return false;
                }
            })
                    .timeout(Duration.ofSeconds(10))
                    .pollInterval(Duration.ofMillis(100))
                    .succeeds("document appears in Bob");

            // Verify the content
            DocHandle bobHandle = waitFor(
                    pair.bob.find(docId),
                    "find document in Bob").get();
            String value = waitFor(
                    bobHandle.withDocument(doc -> {
                        return doc
                                .get(ObjectId.ROOT, "key")
                                .map(v -> v instanceof AmValue.Str
                                        ? ((AmValue.Str) v).getValue()
                                        : null)
                                .orElse(null);
                    }),
                    "read value from Bob");

            assertEquals(
                    "value",
                    value,
                    "Document content should sync from Alice to Bob");
        }
    }

    @Test
    void testDocumentModificationSyncsFromAToB() throws Exception {
        try (RepoConnectionPair pair = createConnectedRepos()) {
            // Create document in A
            DocHandle aliceHandle = waitFor(
                    pair.alice.create(),
                    "create document in Alice");
            DocumentId docId = aliceHandle.getDocumentId();

            // Set initial value
            waitFor(
                    aliceHandle.withDocument(doc -> {
                        try (Transaction tx = doc.startTransaction()) {
                            tx.set(ObjectId.ROOT, "counter", 1);
                            tx.commit();
                        }
                        return null;
                    }),
                    "set initial value in Alice");

            // Wait for document to appear in Bob
            eventually(() -> {
                try {
                    return pair.bob.find(docId).get() != null;
                } catch (Exception e) {
                    return false;
                }
            }).succeeds("document appears in Bob");

            DocHandle bobHandle = waitFor(
                    pair.bob.find(docId),
                    "find document in Bob").get();

            // Modify in Alice
            waitFor(
                    aliceHandle.withDocument(doc -> {
                        try (Transaction tx = doc.startTransaction()) {
                            tx.set(ObjectId.ROOT, "counter", 2);
                            tx.commit();
                        }
                        return null;
                    }),
                    "update counter in Alice");

            // Wait for change to sync to Bob
            eventually(() -> {
                try {
                    Long value = bobHandle
                            .withDocument(doc -> {
                                return doc
                                        .get(ObjectId.ROOT, "counter")
                                        .map(v -> v instanceof AmValue.Int
                                                ? ((AmValue.Int) v).getValue()
                                                : null)
                                        .orElse(null);
                            })
                            .get();
                    return value != null && value == 2L;
                } catch (Exception e) {
                    return false;
                }
            }).succeeds("counter update syncs to Bob");
        }
    }

    @Test
    void testDocumentCreatedInBAfterConnection() throws Exception {
        try (RepoConnectionPair pair = createConnectedRepos()) {
            logf("Repos connected");

            // Create a document in repo B
            DocHandle bobHandle = waitFor(
                    pair.bob.create(),
                    "create document in Bob");
            DocumentId docId = bobHandle.getDocumentId();
            logf("Created document in Bob: %s", docId);

            // Modify it so there's content to sync
            waitFor(
                    bobHandle.withDocument(doc -> {
                        try (Transaction tx = doc.startTransaction()) {
                            tx.set(ObjectId.ROOT, "source", "bob");
                            tx.commit();
                        }
                        return null;
                    }),
                    "modify document in Bob");
            logf("Modified document in Bob");

            // Wait for document to appear in repo A
            logf("Waiting for document to appear in Alice...");
            eventually(() -> {
                try {
                    DocHandle aliceHandle = pair.alice.find(docId).get().get();
                    boolean found = aliceHandle != null;
                    if (found) {
                        logf("Document found in Alice!");
                    }
                    return found;
                } catch (Exception e) {
                    logf(
                            "Error checking for document in Alice: %s",
                            e.getMessage());
                    return false;
                }
            })
                    .timeout(Duration.ofSeconds(10))
                    .pollInterval(Duration.ofMillis(100))
                    .succeeds("document appears in Alice");

            // Verify the content
            DocHandle aliceHandle = waitFor(
                    pair.alice.find(docId),
                    "find document in Alice").get();
            String value = waitFor(
                    aliceHandle.withDocument(doc -> {
                        return doc
                                .get(ObjectId.ROOT, "source")
                                .map(v -> v instanceof AmValue.Str
                                        ? ((AmValue.Str) v).getValue()
                                        : null)
                                .orElse(null);
                    }),
                    "read value from Alice");

            assertEquals(
                    "bob",
                    value,
                    "Document content should sync from Bob to Alice");
        }
    }

    @Test
    void testDisconnectModifyReconnectSync() throws Exception {

        try (RepoConnectionPair pair = createConnectedRepos()) {

            // Create a document and let it sync
            DocHandle aliceHandle = waitFor(
                    pair.alice.create(),
                    "create document in Alice");
            DocumentId docId = aliceHandle.getDocumentId();

            waitFor(
                    aliceHandle.withDocument(doc -> {
                        try (Transaction tx = doc.startTransaction()) {
                            tx.set(ObjectId.ROOT, "phase", "initial");
                            tx.commit();
                        }
                        return null;
                    }),
                    "set initial value in Alice");

            // Wait for document to sync
            eventually(() -> {
                try {
                    return pair.bob.find(docId).get() != null;
                } catch (Exception e) {
                    return false;
                }
            }).succeeds("initial document appears in Bob");

            DocHandle bobHandle = waitFor(
                    pair.bob.find(docId),
                    "find document in Bob").get();

            // Disconnect
            logf("Closing dialer and acceptor");
            pair.disconnect();
            logf("Dialer and acceptor closed");

            // Modify in Alice while disconnected
            waitFor(
                    aliceHandle.withDocument(doc -> {
                        try (Transaction tx = doc.startTransaction()) {
                            tx.set(ObjectId.ROOT, "phase", "disconnected-alice");
                            tx.commit();
                        }
                        return null;
                    }),
                    "modify in Alice while disconnected");

            // Modify in Bob while disconnected
            waitFor(
                    bobHandle.withDocument(doc -> {
                        try (Transaction tx = doc.startTransaction()) {
                            tx.set(ObjectId.ROOT, "bob-field", "disconnected-bob");
                            tx.commit();
                        }
                        return null;
                    }),
                    "modify in Bob while disconnected");

            // Reconnect with new dialer/acceptor
            logf("Reconnecting");
            waitFor(pair.connect(), "reconnection established");
            logf("Reconnected");

            // Wait for Alice's change to sync to Bob
            eventually(() -> {
                try {
                    String value = bobHandle
                            .withDocument(doc -> {
                                return doc
                                        .get(ObjectId.ROOT, "phase")
                                        .map(v -> v instanceof AmValue.Str
                                                ? ((AmValue.Str) v).getValue()
                                                : null)
                                        .orElse(null);
                            })
                            .get();
                    return "disconnected-alice".equals(value);
                } catch (Exception e) {
                    return false;
                }
            })
                    .timeout(Duration.ofSeconds(10))
                    .succeeds("Alice's disconnected change syncs to Bob");

            // Wait for Bob's change to sync to Alice
            eventually(() -> {
                try {
                    String value = aliceHandle
                            .withDocument(doc -> {
                                return doc
                                        .get(ObjectId.ROOT, "bob-field")
                                        .map(v -> v instanceof AmValue.Str
                                                ? ((AmValue.Str) v).getValue()
                                                : null)
                                        .orElse(null);
                            })
                            .get();
                    return "disconnected-bob".equals(value);
                } catch (Exception e) {
                    return false;
                }
            })
                    .timeout(Duration.ofSeconds(10))
                    .succeeds("Bob's disconnected change syncs to Alice");

            // Clean up new connections
            pair.disconnect();
        }
    }

    @Test
    void testMultipleDocumentsSync() throws Exception {
        try (RepoConnectionPair pair = createConnectedRepos()) {
            logf("Creating multiple documents in Alice");

            // Create 3 documents in Alice
            DocHandle doc1 = waitFor(
                    pair.alice.create(),
                    "create doc1 in Alice");
            DocHandle doc2 = waitFor(
                    pair.alice.create(),
                    "create doc2 in Alice");
            DocHandle doc3 = waitFor(
                    pair.alice.create(),
                    "create doc3 in Alice");

            // Add content to each
            waitFor(
                    doc1.withDocument(doc -> {
                        try (Transaction tx = doc.startTransaction()) {
                            tx.set(ObjectId.ROOT, "name", "doc1");
                            tx.commit();
                        }
                        return null;
                    }),
                    "set content in doc1");

            waitFor(
                    doc2.withDocument(doc -> {
                        try (Transaction tx = doc.startTransaction()) {
                            tx.set(ObjectId.ROOT, "name", "doc2");
                            tx.commit();
                        }
                        return null;
                    }),
                    "set content in doc2");

            waitFor(
                    doc3.withDocument(doc -> {
                        try (Transaction tx = doc.startTransaction()) {
                            tx.set(ObjectId.ROOT, "name", "doc3");
                            tx.commit();
                        }
                        return null;
                    }),
                    "set content in doc3");

            logf("Documents created and modified in Alice");

            // Wait for all three to appear in Bob
            DocumentId id1 = doc1.getDocumentId();
            DocumentId id2 = doc2.getDocumentId();
            DocumentId id3 = doc3.getDocumentId();

            eventually(() -> {
                try {
                    return pair.bob.find(id1).get() != null &&
                            pair.bob.find(id2).get() != null &&
                            pair.bob.find(id3).get() != null;
                } catch (Exception e) {
                    return false;
                }
            })
                    .timeout(Duration.ofSeconds(10))
                    .succeeds("all three documents appear in Bob");

            // Verify content of all three
            DocHandle bobDoc1 = waitFor(
                    pair.bob.find(id1),
                    "find doc1 in Bob").get();
            DocHandle bobDoc2 = waitFor(
                    pair.bob.find(id2),
                    "find doc2 in Bob").get();
            DocHandle bobDoc3 = waitFor(
                    pair.bob.find(id3),
                    "find doc3 in Bob").get();

            String name1 = waitFor(
                    bobDoc1.withDocument(doc -> {
                        return doc
                                .get(ObjectId.ROOT, "name")
                                .map(v -> v instanceof AmValue.Str
                                        ? ((AmValue.Str) v).getValue()
                                        : null)
                                .orElse(null);
                    }),
                    "read doc1 name from Bob");

            String name2 = waitFor(
                    bobDoc2.withDocument(doc -> {
                        return doc
                                .get(ObjectId.ROOT, "name")
                                .map(v -> v instanceof AmValue.Str
                                        ? ((AmValue.Str) v).getValue()
                                        : null)
                                .orElse(null);
                    }),
                    "read doc2 name from Bob");

            String name3 = waitFor(
                    bobDoc3.withDocument(doc -> {
                        return doc
                                .get(ObjectId.ROOT, "name")
                                .map(v -> v instanceof AmValue.Str
                                        ? ((AmValue.Str) v).getValue()
                                        : null)
                                .orElse(null);
                    }),
                    "read doc3 name from Bob");

            assertEquals("doc1", name1, "doc1 content should sync");
            assertEquals("doc2", name2, "doc2 content should sync");
            assertEquals("doc3", name3, "doc3 content should sync");
        }
    }

    @Test
    void testThreeRepoTransitiveSync() throws Exception {
        // Create three repos
        RepoConfig aliceConfig = RepoConfig.builder()
                .peerId(PeerId.fromString("alice"))
                .build();
        RepoConfig bobConfig = RepoConfig.builder()
                .peerId(PeerId.fromString("bob"))
                .build();
        RepoConfig charlieConfig = RepoConfig.builder()
                .peerId(PeerId.fromString("charlie"))
                .build();

        Repo alice = Repo.load(aliceConfig);
        Repo bob = Repo.load(bobConfig);
        Repo charlie = Repo.load(charlieConfig);

        try {
            // Connect Alice -> Bob
            AcceptorHandle acceptorBob = bob.makeAcceptor("channel://bob");
            DialerHandle aliceToBob = alice.dial(
                    new ChannelDialer(acceptorBob));

            waitFor(aliceToBob.onEstablished(), "Alice-Bob connection established");

            // Connect Bob -> Charlie
            AcceptorHandle acceptorCharlie = charlie.makeAcceptor("channel://charlie");
            DialerHandle bobToCharlie = bob.dial(
                    new ChannelDialer(acceptorCharlie));

            waitFor(bobToCharlie.onEstablished(), "Bob-Charlie connection established");

            logf("Three repos connected in a chain: Alice <-> Bob <-> Charlie");

            // Create document in Alice
            DocHandle aliceHandle = waitFor(alice.create(), "create document in Alice");
            DocumentId docId = aliceHandle.getDocumentId();

            waitFor(
                    aliceHandle.withDocument(doc -> {
                        try (Transaction tx = doc.startTransaction()) {
                            tx.set(ObjectId.ROOT, "origin", "alice");
                            tx.commit();
                        }
                        return null;
                    }),
                    "set content in Alice");

            logf("Document created in Alice: %s", docId);

            // Wait for it to appear in Bob
            eventually(() -> {
                try {
                    return bob.find(docId).get() != null;
                } catch (Exception e) {
                    return false;
                }
            })
                    .timeout(Duration.ofSeconds(10))
                    .succeeds("document appears in Bob");

            logf("Document appeared in Bob");

            // Wait for it to appear in Charlie (transitive sync through Bob)
            eventually(() -> {
                try {
                    return charlie.find(docId).get() != null;
                } catch (Exception e) {
                    return false;
                }
            })
                    .timeout(Duration.ofSeconds(10))
                    .succeeds("document appears in Charlie");

            logf("Document appeared in Charlie");

            // Verify content in Charlie
            DocHandle charlieHandle = waitFor(
                    charlie.find(docId),
                    "find document in Charlie").get();
            String value = waitFor(
                    charlieHandle.withDocument(doc -> {
                        return doc
                                .get(ObjectId.ROOT, "origin")
                                .map(v -> v instanceof AmValue.Str
                                        ? ((AmValue.Str) v).getValue()
                                        : null)
                                .orElse(null);
                    }),
                    "read value from Charlie");

            assertEquals(
                    "alice",
                    value,
                    "Document should sync transitively from Alice to Charlie through Bob");

            // Clean up
            aliceToBob.close();
            acceptorBob.close();
            bobToCharlie.close();
            acceptorCharlie.close();
        } finally {
            alice.close();
            bob.close();
            charlie.close();
        }
    }

    @Test
    void testAnnouncePolicyAllowsSync() throws Exception {
        // Create repos with AnnounceAll policy (default behavior)
        RepoConfig aliceConfig = RepoConfig.builder()
                .peerId(PeerId.fromString("alice"))
                .announcePolicy(new AnnounceAll())
                .build();
        RepoConfig bobConfig = RepoConfig.builder()
                .peerId(PeerId.fromString("bob"))
                .announcePolicy(new AnnounceAll())
                .build();

        Repo alice = Repo.load(aliceConfig);
        Repo bob = Repo.load(bobConfig);

        try {
            // Connect repos
            AcceptorHandle acceptorB = bob.makeAcceptor("channel://bob");
            DialerHandle dialerA = alice.dial(
                    new ChannelDialer(acceptorB));

            waitFor(dialerA.onEstablished(), "Alice connection established");

            logf("Repos connected with AnnounceAll policy");

            // Create document in Alice AFTER connection
            DocHandle aliceHandle = waitFor(alice.create(), "create document in Alice");
            DocumentId docId = aliceHandle.getDocumentId();

            waitFor(
                    aliceHandle.withDocument(doc -> {
                        try (Transaction tx = doc.startTransaction()) {
                            tx.set(ObjectId.ROOT, "test", "announce-all");
                            tx.commit();
                        }
                        return null;
                    }),
                    "set content in Alice");

            logf("Document created in Alice: %s", docId);

            // Wait a moment for announcement to happen
            sleep(Duration.ofMillis(500));

            // Now disconnect BEFORE calling find()
            dialerA.close();
            acceptorB.close();

            logf("Repos disconnected");

            // Because AnnounceAll was used, the document should have been announced
            // and synced to Bob while connected, so find() should succeed after disconnect
            DocHandle bobHandle = waitFor(
                    bob.find(docId),
                    Duration.ofSeconds(1),
                    "find announced document in Bob after disconnect").get();

            assertNotNull(bobHandle, "Document should be cached locally due to AnnounceAll");

            // Verify content
            String value = waitFor(
                    bobHandle.withDocument(doc -> {
                        return doc
                                .get(ObjectId.ROOT, "test")
                                .map(v -> v instanceof AmValue.Str
                                        ? ((AmValue.Str) v).getValue()
                                        : null)
                                .orElse(null);
                    }),
                    "read value from Bob");

            assertEquals("announce-all", value, "Cached document content should be accessible");
        } finally {
            alice.close();
            bob.close();
        }
    }

    @Test
    void testAnnounceAllSyncsDocumentToCache() throws Exception {
        // Create repos with AnnounceAll policy (default behavior)
        RepoConfig aliceConfig = RepoConfig.builder()
                .peerId(PeerId.fromString("alice"))
                .announcePolicy(new AnnounceAll())
                .build();
        RepoConfig bobConfig = RepoConfig.builder()
                .peerId(PeerId.fromString("bob"))
                .announcePolicy(new AnnounceAll())
                .build();

        Repo alice = Repo.load(aliceConfig);
        Repo bob = Repo.load(bobConfig);

        try {
            // Connect repos
            AcceptorHandle acceptorB = bob.makeAcceptor("channel://bob");
            DialerHandle dialerA = alice.dial(
                    new ChannelDialer(acceptorB));

            waitFor(dialerA.onEstablished(), "Alice connection established");

            logf("Repos connected with AnnounceAll policy");

            // Create document in Alice AFTER connection
            DocHandle aliceHandle = waitFor(alice.create(), "create document in Alice");
            DocumentId docId = aliceHandle.getDocumentId();

            waitFor(
                    aliceHandle.withDocument(doc -> {
                        try (Transaction tx = doc.startTransaction()) {
                            tx.set(ObjectId.ROOT, "test", "cached-by-announce");
                            tx.commit();
                        }
                        return null;
                    }),
                    "set content in Alice");

            logf("Document created in Alice: %s", docId);

            // Wait a moment for announcement and sync to happen
            sleep(Duration.ofMillis(500));

            logf("Waited for announcement to complete");

            // Now disconnect BEFORE calling find()
            dialerA.close();
            acceptorB.close();

            logf("Repos disconnected");

            // Bob should be able to find the document because it was
            // announced and cached locally during the connection
            DocHandle bobHandle = waitFor(
                    bob.find(docId),
                    Duration.ofSeconds(1),
                    "find document in Bob after disconnect").get();

            assertNotNull(
                    bobHandle,
                    "Document should be cached in Bob after AnnounceAll announcement");

            // Verify content is accessible
            String value = waitFor(
                    bobHandle.withDocument(doc -> {
                        return doc
                                .get(ObjectId.ROOT, "test")
                                .map(v -> v instanceof AmValue.Str
                                        ? ((AmValue.Str) v).getValue()
                                        : null)
                                .orElse(null);
                    }),
                    "read value from Bob");

            assertEquals(
                    "cached-by-announce",
                    value,
                    "Cached document content should be accessible");
        } finally {
            alice.close();
            bob.close();
        }
    }

    @Test
    void testAnnounceNoneDoesNotSyncToCache() throws Exception {
        // Create repos where Alice has AnnounceNone policy
        RepoConfig aliceConfig = RepoConfig.builder()
                .peerId(PeerId.fromString("alice"))
                .announcePolicy(new AnnounceNone())
                .build();
        RepoConfig bobConfig = RepoConfig.builder()
                .peerId(PeerId.fromString("bob"))
                .announcePolicy(new AnnounceAll())
                .build();

        Repo alice = Repo.load(aliceConfig);
        Repo bob = Repo.load(bobConfig);

        try {
            // Connect repos
            AcceptorHandle acceptorB = bob.makeAcceptor("channel://bob");
            DialerHandle dialerA = alice.dial(
                    new ChannelDialer(acceptorB));

            waitFor(dialerA.onEstablished(), "Alice connection established");

            logf("Repos connected with AnnounceNone policy on Alice");

            // Create document in Alice
            DocHandle aliceHandle = waitFor(alice.create(), "create document in Alice");
            DocumentId docId = aliceHandle.getDocumentId();

            waitFor(
                    aliceHandle.withDocument(doc -> {
                        try (Transaction tx = doc.startTransaction()) {
                            tx.set(ObjectId.ROOT, "test", "not-announced");
                            tx.commit();
                        }
                        return null;
                    }),
                    "set content in Alice");

            logf("Document created in Alice: %s", docId);

            // Wait a bit to ensure no announcement happens
            sleep(Duration.ofSeconds(2));

            logf("Waited for potential announcement");

            // Now disconnect
            dialerA.close();
            acceptorB.close();

            logf("Repos disconnected");

            DocHandle bobHandle = null;
            try {
                bobHandle = bob.find(docId)
                        .get().get();
            } catch (Exception e) {
                // Other exceptions are okay too - document not available
                logf("find() threw exception as expected: %s", e.getClass().getSimpleName());
            }

            assertNull(
                    bobHandle,
                    "Document should NOT be cached in Bob when AnnounceNone prevents announcement");
        } finally {
            alice.close();
            bob.close();
        }
    }
}
