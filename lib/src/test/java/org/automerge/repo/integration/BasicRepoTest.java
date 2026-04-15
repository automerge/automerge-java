package org.automerge.repo.integration;

import static org.automerge.repo.integration.helpers.TestHelpers.*;
import static org.junit.jupiter.api.Assertions.*;

import java.util.concurrent.CompletableFuture;
import java.util.concurrent.ExecutionException;
import java.util.concurrent.TimeUnit;
import org.automerge.AmValue;
import org.automerge.ObjectId;
import org.automerge.Transaction;
import org.automerge.repo.DocHandle;
import org.automerge.repo.Repo;
import org.automerge.repo.RepoConfig;
import org.automerge.repo.storage.InMemoryStorage;
import org.junit.jupiter.api.Test;

/**
 * Integration tests for basic Repo operations without networking.
 *
 * These tests validate: - Repo initialization and configuration - Document
 * creation and management - DocHandle.change - Document
 * finding by ID - Resource cleanup
 *
 * Tests use only the public API and don't access internal implementation
 * details.
 */
class BasicRepoTest {

    @Test
    void testCreateRepoWithDefaultConfig() {
        RepoConfig config = RepoConfig.builder().storage(new InMemoryStorage()).build();

        try (Repo repo = Repo.load(config)) {
            assertNotNull(repo, "Repo should be created");
        }
    }

    @Test
    void testCreateDocument() {
        RepoConfig config = RepoConfig.builder().storage(new InMemoryStorage()).build();

        try (Repo repo = Repo.load(config)) {
            DocHandle handle = waitFor(repo.create(), "document creation");

            assertNotNull(handle, "DocHandle should not be null");
            assertNotNull(handle.getDocumentId(), "DocumentId should not be null");
            assertNotNull(handle.getUrl(), "URL should not be null");
        }
    }

    @Test
    void testModifyDocumentViaWithDocument() {
        RepoConfig config = RepoConfig.builder().storage(new InMemoryStorage()).build();

        try (Repo repo = Repo.load(config)) {
            DocHandle handle = waitFor(repo.create(), "document creation");

            // Modify the document
            String result = waitFor(handle.withDocument(doc -> {
                try (Transaction tx = doc.startTransaction()) {
                    tx.set(ObjectId.ROOT, "title", "Hello World");
                    tx.commit();
                }
                return "modified";
            }), "document change");

            assertEquals("modified", result, "Change should return the function result");
        }
    }

    @Test
    void testQueryDocument() {
        RepoConfig config = RepoConfig.builder().storage(new InMemoryStorage()).build();

        try (Repo repo = Repo.load(config)) {
            DocHandle handle = waitFor(repo.create(), "document creation");

            // Set some content
            waitFor(handle.withDocument(doc -> {
                try (Transaction tx = doc.startTransaction()) {
                    tx.set(ObjectId.ROOT, "name", "Alice");
                    tx.set(ObjectId.ROOT, "age", "30");
                    tx.commit();
                }
                return null;
            }), "setting content");

            // Query the content
            String name = waitFor(
                    handle.withDocument(doc -> doc.get(ObjectId.ROOT, "name")
                            .map(v -> v instanceof AmValue.Str ? ((AmValue.Str) v).getValue() : null).orElse(null)),
                    "reading name");

            String age = waitFor(
                    handle.withDocument(doc -> doc.get(ObjectId.ROOT, "age")
                            .map(v -> v instanceof AmValue.Str ? ((AmValue.Str) v).getValue() : null).orElse(null)),
                    "reading age");

            assertEquals("Alice", name);
            assertEquals("30", age);
        }
    }

    @Test
    void testCreateMultipleDocuments() {
        RepoConfig config = RepoConfig.builder().storage(new InMemoryStorage()).build();

        try (Repo repo = Repo.load(config)) {
            DocHandle doc1 = waitFor(repo.create(), "creating doc1");
            DocHandle doc2 = waitFor(repo.create(), "creating doc2");
            DocHandle doc3 = waitFor(repo.create(), "creating doc3");

            assertNotNull(doc1);
            assertNotNull(doc2);
            assertNotNull(doc3);

            // Each document should have a unique ID
            assertNotEquals(doc1.getDocumentId(), doc2.getDocumentId());
            assertNotEquals(doc2.getDocumentId(), doc3.getDocumentId());
            assertNotEquals(doc1.getDocumentId(), doc3.getDocumentId());

            // Modify each with different content
            waitFor(doc1.withDocument(d -> {
                try (Transaction tx = d.startTransaction()) {
                    tx.set(ObjectId.ROOT, "doc", "one");
                    tx.commit();
                }
                return null;
            }), "setting doc1 content");
            waitFor(doc2.withDocument(d -> {
                try (Transaction tx = d.startTransaction()) {
                    tx.set(ObjectId.ROOT, "doc", "two");
                    tx.commit();
                }
                return null;
            }), "setting doc2 content");
            waitFor(doc3.withDocument(d -> {
                try (Transaction tx = d.startTransaction()) {
                    tx.set(ObjectId.ROOT, "doc", "three");
                    tx.commit();
                }
                return null;
            }), "setting doc3 content");

            // Verify each has correct content
            assertEquals("one",
                    waitFor(doc1.withDocument(d -> d.get(ObjectId.ROOT, "doc")
                            .map(v -> v instanceof AmValue.Str ? ((AmValue.Str) v).getValue() : null).orElse(null)),
                            "reading doc1"));
            assertEquals("two",
                    waitFor(doc2.withDocument(d -> d.get(ObjectId.ROOT, "doc")
                            .map(v -> v instanceof AmValue.Str ? ((AmValue.Str) v).getValue() : null).orElse(null)),
                            "reading doc2"));
            assertEquals("three",
                    waitFor(doc3.withDocument(d -> d.get(ObjectId.ROOT, "doc")
                            .map(v -> v instanceof AmValue.Str ? ((AmValue.Str) v).getValue() : null).orElse(null)),
                            "reading doc3"));
        }
    }

    @Test
    void testCreateDocumentWithInitialContent() {
        RepoConfig config = RepoConfig.builder().storage(new InMemoryStorage()).build();

        try (Repo repo = Repo.load(config)) {
            // Create a document with initial content
            byte[] initialContent = createSimpleDocument("greeting", "Hello");

            DocHandle handle = waitFor(repo.create(initialContent), "creating document with initial content");

            // Verify the content is present
            String greeting = waitFor(
                    handle.withDocument(doc -> doc.get(ObjectId.ROOT, "greeting")
                            .map(v -> v instanceof AmValue.Str ? ((AmValue.Str) v).getValue() : null).orElse(null)),
                    "reading greeting");

            assertEquals("Hello", greeting);
        }
    }

    @Test
    void testRepoWithTryWithResources() {
        RepoConfig config = RepoConfig.builder().storage(new InMemoryStorage()).build();

        DocHandle[] handleHolder = new DocHandle[1];

        // Use try-with-resources
        try (Repo repo = Repo.load(config)) {
            handleHolder[0] = waitFor(repo.create(), "creating document");
            assertNotNull(handleHolder[0]);
        } // Repo should be closed here

        // After closing, operations should fail
        DocHandle handle = handleHolder[0];
        CompletableFuture<?> future = handle.withDocument(doc -> {
            try (Transaction tx = doc.startTransaction()) {
                tx.set(ObjectId.ROOT, "test", "fail");
                tx.commit();
            }
            return null;
        });

        // The future should complete exceptionally
        assertThrows(ExecutionException.class, () -> future.get(5, TimeUnit.SECONDS),
                "Operations on closed repo should fail");
    }

    @Test
    void testOperationsOnClosedRepoThrowException() {
        RepoConfig config = RepoConfig.builder().storage(new InMemoryStorage()).build();

        Repo repo = Repo.load(config);
        repo.close();

        // Try to create a document after closing
        // Should throw IllegalStateException directly from submitHubEvent
        assertThrows(IllegalStateException.class, () -> {
            repo.create();
        }, "Creating document on closed repo should fail");
    }

    @Test
    void testWithDocumentReturnsCustomValue() {
        RepoConfig config = RepoConfig.builder().storage(new InMemoryStorage()).build();

        try (Repo repo = Repo.load(config)) {
            DocHandle handle = waitFor(repo.create(), "creating document");

            // Change and return a custom value
            Integer result = waitFor(handle.withDocument(doc -> {
                try (Transaction tx = doc.startTransaction()) {
                    tx.set(ObjectId.ROOT, "item", "apple");
                    tx.commit();
                }
                return 42;
            }), "change with return value");

            assertEquals(42, result, "Should return the custom value");
        }
    }
}
