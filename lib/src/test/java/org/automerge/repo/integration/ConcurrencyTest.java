package org.automerge.repo.integration;

import static org.automerge.repo.integration.helpers.TestHelpers.*;
import static org.junit.jupiter.api.Assertions.*;

import java.time.Duration;
import java.util.ArrayList;
import java.util.Collections;
import java.util.List;
import java.util.concurrent.CompletableFuture;
import java.util.concurrent.CountDownLatch;
import java.util.concurrent.ExecutorService;
import java.util.concurrent.Executors;
import java.util.concurrent.TimeUnit;
import java.util.concurrent.atomic.AtomicInteger;
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
import org.automerge.repo.integration.helpers.ChannelDialer;
import org.automerge.repo.storage.InMemoryStorage;
import org.junit.jupiter.api.Test;

/**
 * Integration tests for concurrent operations on repos.
 */
class ConcurrencyTest {

    @Test
    void testConcurrentDocumentCreation() throws Exception {
        RepoConfig config = RepoConfig.builder()
                .peerId(PeerId.fromString("test"))
                .build();

        try (Repo repo = Repo.load(config)) {
            int threadCount = 10;
            ExecutorService executor = Executors.newFixedThreadPool(threadCount);
            List<CompletableFuture<DocHandle>> futures = new ArrayList<>();

            try {
                // Create documents concurrently from multiple threads
                for (int i = 0; i < threadCount; i++) {
                    CompletableFuture<DocHandle> future = CompletableFuture.supplyAsync(
                            () -> {
                                try {
                                    return waitFor(repo.create(), "create document");
                                } catch (Exception e) {
                                    throw new RuntimeException(e);
                                }
                            },
                            executor);
                    futures.add(future);
                }

                // Wait for all to complete
                CompletableFuture.allOf(futures.toArray(new CompletableFuture[0]))
                        .get(10, TimeUnit.SECONDS);

                // Verify all documents were created successfully
                List<DocHandle> handles = new ArrayList<>();
                for (CompletableFuture<DocHandle> future : futures) {
                    DocHandle handle = future.get();
                    assertNotNull(handle, "Document should be created");
                    handles.add(handle);
                }

                assertEquals(threadCount, handles.size(), "All documents should be created");

                // Verify all documents have unique IDs
                List<DocumentId> ids = new ArrayList<>();
                for (DocHandle handle : handles) {
                    ids.add(handle.getDocumentId());
                }

                assertEquals(
                        threadCount,
                        ids.stream().distinct().count(),
                        "All document IDs should be unique");
            } finally {
                executor.shutdown();
                executor.awaitTermination(5, TimeUnit.SECONDS);
            }
        }
    }

    @Test
    void testConcurrentModificationsToDifferentDocuments() throws Exception {
        RepoConfig config = RepoConfig.builder()
                .peerId(PeerId.fromString("test"))
                .build();

        try (Repo repo = Repo.load(config)) {
            int docCount = 10;
            List<DocHandle> handles = new ArrayList<>();

            // Create documents first
            for (int i = 0; i < docCount; i++) {
                handles.add(waitFor(repo.create(), "create document " + i));
            }

            ExecutorService executor = Executors.newFixedThreadPool(docCount);
            List<CompletableFuture<Void>> futures = new ArrayList<>();

            try {
                // Modify each document concurrently
                for (int i = 0; i < docCount; i++) {
                    final int index = i;
                    final DocHandle handle = handles.get(i);

                    CompletableFuture<Void> future = CompletableFuture.runAsync(
                            () -> {
                                try {
                                    waitFor(
                                            handle.withDocument(doc -> {
                                                try (Transaction tx = doc.startTransaction()) {
                                                    tx.set(ObjectId.ROOT, "value", index);
                                                    tx.commit();
                                                }
                                                return null;
                                            }),
                                            "modify document " + index);
                                } catch (Exception e) {
                                    throw new RuntimeException(e);
                                }
                            },
                            executor);
                    futures.add(future);
                }

                // Wait for all modifications to complete
                CompletableFuture.allOf(futures.toArray(new CompletableFuture[0]))
                        .get(10, TimeUnit.SECONDS);

                // Verify all documents have correct values
                for (int i = 0; i < docCount; i++) {
                    final int expected = i;
                    Long value = waitFor(
                            handles.get(i).withDocument(doc -> {
                                return doc
                                        .get(ObjectId.ROOT, "value")
                                        .map(v -> v instanceof AmValue.Int
                                                ? ((AmValue.Int) v).getValue()
                                                : null)
                                        .orElse(null);
                            }),
                            "read document " + i);

                    assertEquals((long) expected, value, "Document " + i + " should have correct value");
                }
            } finally {
                executor.shutdown();
                executor.awaitTermination(5, TimeUnit.SECONDS);
            }
        }
    }

    @Test
    void testConcurrentModificationsToSameDocument() throws Exception {
        RepoConfig config = RepoConfig.builder()
                .peerId(PeerId.fromString("test"))
                .build();

        try (Repo repo = Repo.load(config)) {
            DocHandle handle = waitFor(repo.create(), "create document");

            int threadCount = 10;
            ExecutorService executor = Executors.newFixedThreadPool(threadCount);
            CountDownLatch startLatch = new CountDownLatch(1);
            List<CompletableFuture<Void>> futures = new ArrayList<>();

            try {
                // All threads modify the same document concurrently
                for (int i = 0; i < threadCount; i++) {
                    final int index = i;

                    CompletableFuture<Void> future = CompletableFuture.runAsync(
                            () -> {
                                try {
                                    // Wait for all threads to be ready
                                    startLatch.await();

                                    waitFor(
                                            handle.withDocument(doc -> {
                                                try (Transaction tx = doc.startTransaction()) {
                                                    tx.set(ObjectId.ROOT, "key" + index, "value" + index);
                                                    tx.commit();
                                                }
                                                return null;
                                            }),
                                            "modify document by thread " + index);
                                } catch (Exception e) {
                                    throw new RuntimeException(e);
                                }
                            },
                            executor);
                    futures.add(future);
                }

                // Start all threads at once
                startLatch.countDown();

                // Wait for all modifications to complete
                CompletableFuture.allOf(futures.toArray(new CompletableFuture[0]))
                        .get(10, TimeUnit.SECONDS);

                // Verify all keys are present (merge should preserve all changes)
                for (int i = 0; i < threadCount; i++) {
                    final int index = i;
                    String value = waitFor(
                            handle.withDocument(doc -> {
                                return doc
                                        .get(ObjectId.ROOT, "key" + index)
                                        .map(v -> v instanceof AmValue.Str
                                                ? ((AmValue.Str) v).getValue()
                                                : null)
                                        .orElse(null);
                            }),
                            "read key" + index);

                    assertEquals("value" + index, value, "All concurrent changes should be preserved");
                }
            } finally {
                executor.shutdown();
                executor.awaitTermination(5, TimeUnit.SECONDS);
            }
        }
    }

    @Test
    void testConcurrentConnectionEstablishment() throws Exception {
        RepoConfig config1 = RepoConfig.builder()
                .peerId(PeerId.fromString("repo1"))
                .build();
        RepoConfig config2 = RepoConfig.builder()
                .peerId(PeerId.fromString("repo2"))
                .build();

        try (Repo repo1 = Repo.load(config1); Repo repo2 = Repo.load(config2)) {

            int connectionCount = 5;
            ExecutorService executor = Executors.newFixedThreadPool(connectionCount);
            List<DialerHandle> dialers = Collections.synchronizedList(new ArrayList<>());
            List<AcceptorHandle> acceptors = Collections.synchronizedList(new ArrayList<>());

            try {
                // Establish multiple connections concurrently
                List<CompletableFuture<Void>> futures = new ArrayList<>();
                for (int i = 0; i < connectionCount; i++) {
                    final int index = i;
                    CompletableFuture<Void> f = new CompletableFuture<>();
                    futures.add(f);
                    executor.submit(() -> {
                        try {
                            AcceptorHandle acceptor = repo2.makeAcceptor("channel://repo2-" + index);
                            DialerHandle dialer = repo1.dial(
                                    new ChannelDialer(acceptor));

                            waitFor(dialer.onEstablished(), Duration.ofSeconds(10),
                                    "connection " + index + " established");

                            acceptors.add(acceptor);
                            dialers.add(dialer);
                            f.complete(null);
                        } catch (Throwable t) {
                            f.completeExceptionally(t);
                        }
                    });
                }

                // Wait for all connections to establish
                waitFor(CompletableFuture.allOf(futures.toArray(new CompletableFuture[0])),
                        Duration.ofSeconds(10),
                        "all connections established");

                // Verify all connections are established
                assertEquals(
                        connectionCount,
                        dialers.size(),
                        "All dialers should be established");

                for (DialerHandle dialer : dialers) {
                    assertTrue(dialer.isConnected(), "Dialer should be connected");
                }

                // Close all connections
                for (DialerHandle dialer : dialers) {
                    dialer.close();
                }
                for (AcceptorHandle acceptor : acceptors) {
                    acceptor.close();
                }
            } finally {
                executor.shutdown();
                executor.awaitTermination(5, TimeUnit.SECONDS);
            }
        }
    }

    @Test
    void testDocumentOperationsDuringConnectionEstablishment() throws Exception {
        RepoConfig aliceConfig = RepoConfig.builder()
                .peerId(PeerId.fromString("alice"))
                .build();
        RepoConfig bobConfig = RepoConfig.builder()
                .peerId(PeerId.fromString("bob"))
                .build();

        try (Repo alice = Repo.load(aliceConfig); Repo bob = Repo.load(bobConfig)) {

            ExecutorService executor = Executors.newFixedThreadPool(3);

            try {
                // Create document in Alice
                DocHandle aliceHandle = waitFor(alice.create(), "create document in Alice");

                // Start connection establishment
                AcceptorHandle acceptorB = bob.makeAcceptor("channel://bob");

                CompletableFuture<DialerHandle> dialerFuture = CompletableFuture.supplyAsync(
                        () -> alice.dial(
                                new ChannelDialer(acceptorB)),
                        executor);

                // Modify document while connection is establishing
                CompletableFuture<Void> modifyFuture = CompletableFuture.runAsync(
                        () -> {
                            try {
                                for (int i = 0; i < 5; i++) {
                                    final int index = i;
                                    waitFor(
                                            aliceHandle.withDocument(doc -> {
                                                try (Transaction tx = doc.startTransaction()) {
                                                    tx.set(ObjectId.ROOT, "counter", index);
                                                    tx.commit();
                                                }
                                                return null;
                                            }),
                                            "modify during connection " + index);
                                    Thread.sleep(10);
                                }
                            } catch (Exception e) {
                                throw new RuntimeException(e);
                            }
                        },
                        executor);

                // Wait for everything to complete
                DialerHandle dialerA = dialerFuture.get(5, TimeUnit.SECONDS);
                modifyFuture.get(5, TimeUnit.SECONDS);

                waitFor(dialerA.onEstablished(), "Alice connection established");

                // Verify final state
                Long value = waitFor(
                        aliceHandle.withDocument(doc -> {
                            return doc
                                    .get(ObjectId.ROOT, "counter")
                                    .map(v -> v instanceof AmValue.Int
                                            ? ((AmValue.Int) v).getValue()
                                            : null)
                                    .orElse(null);
                        }),
                        "read final value");

                assertEquals(4L, value, "All modifications should succeed");

                dialerA.close();
                acceptorB.close();
            } finally {
                executor.shutdown();
                executor.awaitTermination(5, TimeUnit.SECONDS);
            }
        }
    }

    @Test
    void testCloseRepoWhileOperationsPending() throws Exception {
        RepoConfig config = RepoConfig.builder()
                .storage(new InMemoryStorage())
                .peerId(PeerId.fromString("test"))
                .build();

        Repo repo = Repo.load(config);
        DocHandle handle = waitFor(repo.create(), "create document");

        ExecutorService executor = Executors.newFixedThreadPool(5);
        AtomicInteger successCount = new AtomicInteger(0);
        AtomicInteger failureCount = new AtomicInteger(0);

        try {
            // Start many operations
            for (int i = 0; i < 20; i++) {
                final int index = i;
                executor.submit(() -> {
                    try {
                        handle.withDocument(doc -> {
                            try (Transaction tx = doc.startTransaction()) {
                                tx.set(ObjectId.ROOT, "key" + index, "value" + index);
                                tx.commit();
                            }
                            return null;
                        })
                                .get(5, TimeUnit.SECONDS);
                        successCount.incrementAndGet();
                    } catch (Exception e) {
                        // Expected - some operations will fail when repo closes
                        failureCount.incrementAndGet();
                    }
                });
            }

            // Close repo while operations are in flight
            Thread.sleep(50);
            repo.close();

            // Wait for executor to finish
            executor.shutdown();
            executor.awaitTermination(5, TimeUnit.SECONDS);

            // Some operations should succeed, some should fail
            logf("Success: %d, Failure: %d", successCount.get(), failureCount.get());
            assertTrue(
                    successCount.get() + failureCount.get() == 20,
                    "All operations should complete (success or failure)");
        } finally {
            if (!executor.isShutdown()) {
                executor.shutdown();
                executor.awaitTermination(5, TimeUnit.SECONDS);
            }
        }
    }

    @Test
    void testMultipleThreadsCallingSameDocHandle() throws Exception {
        RepoConfig config = RepoConfig.builder()
                .storage(new InMemoryStorage())
                .peerId(PeerId.fromString("test"))
                .build();

        try (Repo repo = Repo.load(config)) {
            DocHandle handle = waitFor(repo.create(), "create document");

            int threadCount = 10;
            int operationsPerThread = 10;
            ExecutorService executor = Executors.newFixedThreadPool(threadCount);
            CountDownLatch startLatch = new CountDownLatch(1);
            List<CompletableFuture<Void>> futures = new ArrayList<>();

            try {
                // Each thread performs multiple operations on the same handle
                for (int t = 0; t < threadCount; t++) {
                    final int threadId = t;

                    CompletableFuture<Void> future = CompletableFuture.runAsync(
                            () -> {
                                try {
                                    startLatch.await();

                                    for (int op = 0; op < operationsPerThread; op++) {
                                        final int opId = op;

                                        // Mix of change and view operations
                                        if (op % 2 == 0) {
                                            waitFor(
                                                    handle.withDocument(doc -> {
                                                        try (Transaction tx = doc.startTransaction()) {
                                                            tx.set(
                                                                    ObjectId.ROOT,
                                                                    "t" + threadId + "_op" + opId,
                                                                    threadId * 100 + opId);
                                                            tx.commit();
                                                        }
                                                        return null;
                                                    }),
                                                    "change by thread " + threadId);
                                        } else {
                                            waitFor(
                                                    handle.withDocument(doc -> {
                                                        // Just read - verifies concurrent views work
                                                        doc.get(ObjectId.ROOT, "t" + threadId + "_op" + (opId - 1));
                                                        return null;
                                                    }),
                                                    "view by thread " + threadId);
                                        }
                                    }
                                } catch (Exception e) {
                                    throw new RuntimeException(e);
                                }
                            },
                            executor);
                    futures.add(future);
                }

                // Start all threads
                startLatch.countDown();

                // Wait for all operations to complete
                CompletableFuture.allOf(futures.toArray(new CompletableFuture[0]))
                        .get(30, TimeUnit.SECONDS);

                // Verify all changes are present
                for (int t = 0; t < threadCount; t++) {
                    for (int op = 0; op < operationsPerThread; op += 2) {
                        final int threadId = t;
                        final int opId = op;

                        Long value = waitFor(
                                handle.withDocument(doc -> {
                                    return doc
                                            .get(ObjectId.ROOT, "t" + threadId + "_op" + opId)
                                            .map(v -> v instanceof AmValue.Int
                                                    ? ((AmValue.Int) v).getValue()
                                                    : null)
                                            .orElse(null);
                                }),
                                "verify t" + threadId + "_op" + opId);

                        assertEquals(
                                (long) (threadId * 100 + opId),
                                value,
                                "Value should match for thread " + threadId + " operation " + opId);
                    }
                }
            } finally {
                executor.shutdown();
                executor.awaitTermination(5, TimeUnit.SECONDS);
            }
        }
    }

    @Test
    void testStressManyOperationsInParallel() throws Exception {
        RepoConfig config = RepoConfig.builder()
                .storage(new InMemoryStorage())
                .peerId(PeerId.fromString("test"))
                .build();

        try (Repo repo = Repo.load(config)) {
            int docCount = 5;
            int threadsPerDoc = 5;
            int opsPerThread = 10;

            List<DocHandle> handles = new ArrayList<>();
            for (int i = 0; i < docCount; i++) {
                handles.add(waitFor(repo.create(), "create document " + i));
            }

            ExecutorService executor = Executors.newFixedThreadPool(docCount * threadsPerDoc);
            List<CompletableFuture<Void>> futures = new ArrayList<>();
            CountDownLatch startLatch = new CountDownLatch(1);

            try {
                // Many threads operating on many documents
                for (int d = 0; d < docCount; d++) {
                    final int docId = d;
                    final DocHandle handle = handles.get(d);

                    for (int t = 0; t < threadsPerDoc; t++) {
                        final int threadId = t;

                        CompletableFuture<Void> future = CompletableFuture.runAsync(
                                () -> {
                                    try {
                                        startLatch.await();

                                        for (int op = 0; op < opsPerThread; op++) {
                                            final int opId = op;

                                            waitFor(
                                                    handle.withDocument(doc -> {
                                                        try (Transaction tx = doc.startTransaction()) {
                                                            tx.set(
                                                                    ObjectId.ROOT,
                                                                    "d" + docId + "_t" + threadId + "_op" + opId,
                                                                    docId * 10000 + threadId * 100 + opId);
                                                            tx.commit();
                                                        }
                                                        return null;
                                                    }),
                                                    "stress operation");
                                        }
                                    } catch (Exception e) {
                                        throw new RuntimeException(e);
                                    }
                                },
                                executor);
                        futures.add(future);
                    }
                }

                logf("Starting stress test: %d docs x %d threads x %d ops = %d total operations",
                        docCount, threadsPerDoc, opsPerThread, docCount * threadsPerDoc * opsPerThread);

                // Start all threads at once
                startLatch.countDown();

                // Wait for all operations to complete
                CompletableFuture.allOf(futures.toArray(new CompletableFuture[0]))
                        .get(60, TimeUnit.SECONDS);

                logf("Stress test completed successfully");

                // Spot check some values
                for (int d = 0; d < docCount; d++) {
                    final int docId = d;
                    final DocHandle handle = handles.get(d);

                    Long value = waitFor(
                            handle.withDocument(doc -> {
                                return doc
                                        .get(ObjectId.ROOT, "d" + docId + "_t0_op0")
                                        .map(v -> v instanceof AmValue.Int
                                                ? ((AmValue.Int) v).getValue()
                                                : null)
                                        .orElse(null);
                            }),
                            "verify doc " + docId);

                    assertEquals((long) (docId * 10000), value, "Spot check value for doc " + docId);
                }
            } finally {
                executor.shutdown();
                executor.awaitTermination(10, TimeUnit.SECONDS);
            }
        }
    }
}
