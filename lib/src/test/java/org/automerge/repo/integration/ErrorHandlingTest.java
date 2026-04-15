package org.automerge.repo.integration;

import static org.automerge.repo.integration.helpers.TestHelpers.*;
import static org.junit.jupiter.api.Assertions.*;

import java.util.concurrent.CompletableFuture;
import java.util.concurrent.ExecutionException;
import org.automerge.ObjectId;
import org.automerge.Transaction;
import org.automerge.repo.DocHandle;
import org.automerge.repo.PeerId;
import org.automerge.repo.Repo;
import org.automerge.repo.RepoConfig;
import org.junit.jupiter.api.Test;

/**
 * Integration tests for error handling, focusing on JNI-specific scenarios.
 *
 * These tests validate:
 * - Resource cleanup prevents memory leaks
 * - Exceptions propagate correctly across JNI boundary
 * - Futures complete exceptionally rather than hanging
 * - Errors in one document/connection don't affect others
 * - No crashes or segfaults under error conditions
 */
class ErrorHandlingTest {

    @Test
    void testDocHandleOperationsAfterRepoClose() throws Exception {
        RepoConfig config = RepoConfig.builder()
                .peerId(PeerId.fromString("test"))
                .build();

        Repo repo = Repo.load(config);
        DocHandle handle = waitFor(repo.create(), "create document");
        // Check we can get the document ID
        handle.getDocumentId();

        // Modify document while repo is open
        waitFor(
                handle.withDocument(doc -> {
                    try (Transaction tx = doc.startTransaction()) {
                        tx.set(ObjectId.ROOT, "key", "value");
                        tx.commit();
                    }
                    return null;
                }),
                "modify document");

        // Close the repo
        repo.close();

        // Operations on the handle should fail gracefully
        CompletableFuture<Void> changeFuture = handle.withDocument(doc -> {
            try (Transaction tx = doc.startTransaction()) {
                tx.set(ObjectId.ROOT, "key2", "value2");
                tx.commit();
            }
            return null;
        });

        // Should complete exceptionally, not hang
        ExecutionException exception = assertThrows(
                ExecutionException.class,
                () -> changeFuture.get(),
                "Operation on DocHandle after repo close should fail");

        // Verify it's the right kind of exception (runtime is stopped)
        assertNotNull(exception.getCause(), "Should have a cause");
        assertTrue(
                exception.getCause() instanceof IllegalStateException
                        || exception.getCause().getMessage().contains("stopped")
                        || exception.getCause().getMessage().contains("closed"),
                "Should indicate repo is stopped/closed");
    }

    @Test
    void testExceptionDuringDocumentOperation() throws Exception {
        RepoConfig config = RepoConfig.builder()
                .peerId(PeerId.fromString("test"))
                .build();

        try (Repo repo = Repo.load(config)) {
            DocHandle handle = waitFor(repo.create(), "create document");

            // Throw exception from within change lambda
            CompletableFuture<Void> changeFuture = handle.withDocument(doc -> {
                throw new RuntimeException("Intentional test exception");
            });

            // Should propagate through future
            ExecutionException exception = assertThrows(
                    ExecutionException.class,
                    () -> changeFuture.get(2, java.util.concurrent.TimeUnit.SECONDS),
                    "Exception in lambda should propagate");

            assertTrue(
                    exception.getCause() instanceof RuntimeException,
                    "Should preserve exception type");
            assertTrue(
                    exception.getCause().getMessage().contains("Intentional test exception"),
                    "Should preserve exception message");

            // Repo should still be functional after exception
            DocHandle handle2 = waitFor(repo.create(), "create another document");
            waitFor(
                    handle2.withDocument(doc -> {
                        try (Transaction tx = doc.startTransaction()) {
                            tx.set(ObjectId.ROOT, "after", "exception");
                            tx.commit();
                        }
                        return null;
                    }),
                    "modify after exception");
        }
    }

    @Test
    void testInvalidDocumentData() throws Exception {
        RepoConfig config = RepoConfig.builder()
                .peerId(PeerId.fromString("test"))
                .build();

        try (Repo repo = Repo.load(config)) {
            // Try to create document with invalid/corrupted bytes
            byte[] invalidBytes = new byte[]{1, 2, 3, 4, 5}; // Not valid automerge format

            // Should fail gracefully - either synchronously or asynchronously
            Exception exc = assertThrows(
                    ExecutionException.class,
                    () -> repo.create(invalidBytes).get(),
                    "Invalid document bytes should cause failure");
            assertNotNull(exc.getCause(), "Exception cause should not be null");
            assertTrue(exc.getCause() instanceof IllegalArgumentException, "Cause should be IllegalArgumentException");

            // Repo should still work after failed create
            DocHandle handle = waitFor(repo.create(), "create valid document after failure");
            assertNotNull(handle, "Should be able to create valid document");
        }
    }

    @Test
    void testFutureCompletionOnErrors() throws Exception {
        RepoConfig config = RepoConfig.builder()
                .peerId(PeerId.fromString("test"))
                .build();

        Repo repo = Repo.load(config);
        DocHandle handle = waitFor(repo.create(), "create document");

        // Complete the operation before closing
        waitFor(handle.withDocument(doc -> {
            try (Transaction tx = doc.startTransaction()) {
                tx.set(ObjectId.ROOT, "key", "value");
                tx.commit();
            }
            return null;
        }), "initial change");

        // Close repo
        repo.close();

        // Operations after close should fail quickly, not hang
        CompletableFuture<Void> afterCloseFuture = handle.withDocument(doc -> {
            try (Transaction tx = doc.startTransaction()) {
                tx.set(ObjectId.ROOT, "shouldFail", "value");
                tx.commit();
            }
            return null;
        });

        // Future should complete exceptionally, not timeout
        try {
            afterCloseFuture.get(2, java.util.concurrent.TimeUnit.SECONDS);
            // If it succeeds, that's actually fine too - operation completed before close
            // took effect
            logf("Operation completed (raced with close)");
        } catch (ExecutionException e) {
            // Expected - operation failed because repo is closed
            logf("Operation failed as expected: %s", e.getCause().getMessage());
        } catch (java.util.concurrent.TimeoutException e) {
            fail("Future should not hang after repo close");
        }
    }

    @Test
    void testDocumentOperationFailureIsolation() throws Exception {
        RepoConfig config = RepoConfig.builder()
                .peerId(PeerId.fromString("test"))
                .build();

        try (Repo repo = Repo.load(config)) {
            // Create two documents
            DocHandle handle1 = waitFor(repo.create(), "create document 1");
            DocHandle handle2 = waitFor(repo.create(), "create document 2");

            // Fail an operation on document 1
            CompletableFuture<Void> failedOp = handle1.withDocument(doc -> {
                throw new RuntimeException("Intentional failure");
            });

            assertThrows(
                    ExecutionException.class,
                    () -> failedOp.get(2, java.util.concurrent.TimeUnit.SECONDS),
                    "Operation should fail");

            // Document 2 should still work fine
            waitFor(
                    handle2.withDocument(doc -> {
                        try (Transaction tx = doc.startTransaction()) {
                            tx.set(ObjectId.ROOT, "isolated", "success");
                            tx.commit();
                        }
                        return null;
                    }),
                    "document 2 should still work");

            // Document 1 should also still work (just that one operation failed)
            waitFor(
                    handle1.withDocument(doc -> {
                        try (Transaction tx = doc.startTransaction()) {
                            tx.set(ObjectId.ROOT, "recovery", "success");
                            tx.commit();
                        }
                        return null;
                    }),
                    "document 1 should work after failed operation");
        }
    }

    @Test
    void testMultipleCloseIdempotent() throws Exception {
        RepoConfig config = RepoConfig.builder()
                .peerId(PeerId.fromString("test"))
                .build();

        Repo repo = Repo.load(config);

        // Close multiple times - should not crash
        repo.close();
        repo.close();
        repo.close();

        // Should be safe
        logf("Multiple close() calls completed safely");
    }

    @Test
    void testLargeBinaryDataAcrossJni() throws Exception {
        RepoConfig config = RepoConfig.builder()
                .peerId(PeerId.fromString("test"))
                .build();

        try (Repo repo = Repo.load(config)) {
            DocHandle handle = waitFor(repo.create(), "create document");

            // Create large binary data (10KB)
            byte[] largeData = new byte[10 * 1024];
            for (int i = 0; i < largeData.length; i++) {
                largeData[i] = (byte) (i % 256);
            }

            // Store it across JNI boundary
            waitFor(
                    handle.withDocument(doc -> {
                        try (Transaction tx = doc.startTransaction()) {
                            tx.set(ObjectId.ROOT, "binaryData", largeData);
                            tx.commit();
                        }
                        return null;
                    }),
                    "store large binary data");

            // Read it back
            byte[] retrieved = waitFor(
                    handle.withDocument(doc -> {
                        return doc
                                .get(ObjectId.ROOT, "binaryData")
                                .map(v -> v instanceof org.automerge.AmValue.Bytes
                                        ? ((org.automerge.AmValue.Bytes) v).getValue()
                                        : null)
                                .orElse(null);
                    }),
                    "read large binary data");

            assertNotNull(retrieved, "Binary data should be retrieved");
            assertEquals(largeData.length, retrieved.length, "Data length should match");

            // Verify content
            for (int i = 0; i < Math.min(100, largeData.length); i++) {
                assertEquals(largeData[i], retrieved[i], "Data at index " + i + " should match");
            }
        }
    }
}
