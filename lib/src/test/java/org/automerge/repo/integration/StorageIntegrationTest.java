package org.automerge.repo.integration;

import static org.automerge.repo.integration.helpers.TestHelpers.*;
import static org.junit.jupiter.api.Assertions.*;

import java.util.Map;
import java.util.Optional;
import java.util.concurrent.CompletableFuture;
import java.util.concurrent.ConcurrentHashMap;
import org.automerge.AmValue;
import org.automerge.ObjectId;
import org.automerge.Transaction;
import org.automerge.repo.AutomergeUrl;
import org.automerge.repo.DocHandle;
import org.automerge.repo.Repo;
import org.automerge.repo.RepoConfig;
import org.automerge.repo.Storage;
import org.automerge.repo.StorageKey;
import org.automerge.repo.storage.InMemoryStorage;
import org.junit.jupiter.api.Test;

/**
 * Integration tests for Storage persistence.
 *
 * These tests validate:
 * - Document persistence across repo reload
 * - Content verification after reload
 * - Multiple document persistence
 * - Custom Storage implementations
 * - Error handling in storage operations
 *
 * Tests use only the public API and don't access internal implementation
 * details.
 */
class StorageIntegrationTest {

    @Test
    void testCreateDocumentCloseReloadFind() {
        InMemoryStorage storage = new InMemoryStorage();
        AutomergeUrl url;

        // Create a repo, create a document, then close
        RepoConfig config = RepoConfig.builder().storage(storage).build();
        try (Repo repo = Repo.load(config)) {
            DocHandle handle = waitFor(repo.create(), "document creation");
            url = handle.getUrl();

            waitFor(handle.withDocument(doc -> {
                try (Transaction tx = doc.startTransaction()) {
                    tx.set(ObjectId.ROOT, "name", "Test Document");
                    tx.commit();
                }
                return null;
            }), "document change");
        }

        // Reload repo with same storage
        RepoConfig config2 = RepoConfig.builder().storage(storage).build();
        try (Repo repo = Repo.load(config2)) {
            DocHandle handle = waitFor(repo.find(url), "document find after reload").get();

            assertNotNull(handle, "Document should be found after reload");
            assertEquals(url, handle.getUrl(), "Document URL should match");
        }
    }

    @Test
    void testDocumentContentPersistsAcrossReload() {
        InMemoryStorage storage = new InMemoryStorage();
        AutomergeUrl url;

        // Create document with content
        RepoConfig config = RepoConfig.builder().storage(storage).build();
        try (Repo repo = Repo.load(config)) {
            DocHandle handle = waitFor(repo.create(), "document creation");
            url = handle.getUrl();

            waitFor(handle.withDocument(doc -> {
                try (Transaction tx = doc.startTransaction()) {
                    tx.set(ObjectId.ROOT, "title", "My Document");
                    tx.set(ObjectId.ROOT, "count", 42);
                    tx.commit();
                }
                return null;
            }), "document change");
        }

        // Reload and verify content
        RepoConfig config2 = RepoConfig.builder().storage(storage).build();
        try (Repo repo = Repo.load(config2)) {
            DocHandle handle = waitFor(repo.find(url), "document find").get();

            String title = waitFor(handle.withDocument(doc -> {
                return doc.get(ObjectId.ROOT, "title")
                        .map(v -> v instanceof AmValue.Str ? ((AmValue.Str) v).getValue() : null).orElse(null);
            }), "view title");

            Long count = waitFor(handle.withDocument(doc -> {
                return doc.get(ObjectId.ROOT, "count")
                        .map(v -> v instanceof AmValue.Int ? ((AmValue.Int) v).getValue() : null).orElse(null);
            }), "view count");

            assertEquals("My Document", title, "Title should persist");
            assertEquals(42L, count, "Count should persist");
        }
    }

    @Test
    void testModifyDocumentReloadVerifyChanges() {
        InMemoryStorage storage = new InMemoryStorage();
        AutomergeUrl url;

        // Create document
        RepoConfig config = RepoConfig.builder().storage(storage).build();
        try (Repo repo = Repo.load(config)) {
            DocHandle handle = waitFor(repo.create(), "document creation");
            url = handle.getUrl();

            waitFor(handle.withDocument(doc -> {
                try (Transaction tx = doc.startTransaction()) {
                    tx.set(ObjectId.ROOT, "version", 1);
                    tx.commit();
                }
                return null;
            }), "initial change");
        }

        // Reload and modify
        RepoConfig config2 = RepoConfig.builder().storage(storage).build();
        try (Repo repo = Repo.load(config2)) {
            DocHandle handle = waitFor(repo.find(url), "document find").get();

            waitFor(handle.withDocument(doc -> {
                try (Transaction tx = doc.startTransaction()) {
                    tx.set(ObjectId.ROOT, "version", 2);
                    tx.set(ObjectId.ROOT, "updated", true);
                    tx.commit();
                }
                return null;
            }), "second change");
        }

        // Reload again and verify
        RepoConfig config3 = RepoConfig.builder().storage(storage).build();
        try (Repo repo = Repo.load(config3)) {
            DocHandle handle = waitFor(repo.find(url), "document find").get();

            Long version = waitFor(handle.withDocument(doc -> {
                return doc.get(ObjectId.ROOT, "version")
                        .map(v -> v instanceof AmValue.Int ? ((AmValue.Int) v).getValue() : null).orElse(null);
            }), "view version");

            Boolean updated = waitFor(handle.withDocument(doc -> {
                return doc.get(ObjectId.ROOT, "updated")
                        .map(v -> v instanceof AmValue.Bool ? ((AmValue.Bool) v).getValue() : null).orElse(null);
            }), "view updated");

            assertEquals(2L, version, "Version should be updated");
            assertTrue(updated, "Updated flag should be set");
        }
    }

    @Test
    void testMultipleDocumentsPersist() {
        InMemoryStorage storage = new InMemoryStorage();
        AutomergeUrl url1;
        AutomergeUrl url2;
        AutomergeUrl url3;

        // Create multiple documents
        RepoConfig config = RepoConfig.builder().storage(storage).build();
        try (Repo repo = Repo.load(config)) {
            DocHandle handle1 = waitFor(repo.create(), "doc1 creation");
            url1 = handle1.getUrl();
            waitFor(handle1.withDocument(doc -> {
                try (Transaction tx = doc.startTransaction()) {
                    tx.set(ObjectId.ROOT, "name", "Document 1");
                    tx.commit();
                }
                return null;
            }), "doc1 change");

            DocHandle handle2 = waitFor(repo.create(), "doc2 creation");
            url2 = handle2.getUrl();
            waitFor(handle2.withDocument(doc -> {
                try (Transaction tx = doc.startTransaction()) {
                    tx.set(ObjectId.ROOT, "name", "Document 2");
                    tx.commit();
                }
                return null;
            }), "doc2 change");

            DocHandle handle3 = waitFor(repo.create(), "doc3 creation");
            url3 = handle3.getUrl();
            waitFor(handle3.withDocument(doc -> {
                try (Transaction tx = doc.startTransaction()) {
                    tx.set(ObjectId.ROOT, "name", "Document 3");
                    tx.commit();
                }
                return null;
            }), "doc3 change");
        }

        // Reload and verify all documents
        RepoConfig config2 = RepoConfig.builder().storage(storage).build();
        try (Repo repo = Repo.load(config2)) {
            DocHandle handle1 = waitFor(repo.find(url1), "find doc1").get();
            String name1 = waitFor(
                    handle1.withDocument(doc -> doc.get(ObjectId.ROOT, "name")
                            .map(v -> v instanceof AmValue.Str ? ((AmValue.Str) v).getValue() : null).orElse(null)),
                    "view doc1");
            assertEquals("Document 1", name1);

            DocHandle handle2 = waitFor(repo.find(url2), "find doc2").get();
            String name2 = waitFor(
                    handle2.withDocument(doc -> doc.get(ObjectId.ROOT, "name")
                            .map(v -> v instanceof AmValue.Str ? ((AmValue.Str) v).getValue() : null).orElse(null)),
                    "view doc2");
            assertEquals("Document 2", name2);

            DocHandle handle3 = waitFor(repo.find(url3), "find doc3").get();
            String name3 = waitFor(
                    handle3.withDocument(doc -> doc.get(ObjectId.ROOT, "name")
                            .map(v -> v instanceof AmValue.Str ? ((AmValue.Str) v).getValue() : null).orElse(null)),
                    "view doc3");
            assertEquals("Document 3", name3);
        }
    }

    @Test
    void testCustomStorageImplementation() {
        // Create a simple custom storage that tracks method calls
        class TrackingStorage implements Storage {

            private final ConcurrentHashMap<StorageKey, byte[]> data = new ConcurrentHashMap<>();
            public int loadCount = 0;
            public int loadRangeCount = 0;
            public int putCount = 0;
            public int deleteCount = 0;

            @Override
            public CompletableFuture<Optional<byte[]>> load(StorageKey key) {
                loadCount++;
                byte[] value = data.get(key);
                return CompletableFuture.completedFuture(value == null ? Optional.empty() : Optional.of(value));
            }

            @Override
            public CompletableFuture<Map<StorageKey, byte[]>> loadRange(StorageKey prefix) {
                loadRangeCount++;
                Map<StorageKey, byte[]> result = new ConcurrentHashMap<>();
                for (Map.Entry<StorageKey, byte[]> entry : data.entrySet()) {
                    if (prefix.isPrefixOf(entry.getKey())) {
                        result.put(entry.getKey(), entry.getValue());
                    }
                }
                return CompletableFuture.completedFuture(result);
            }

            @Override
            public CompletableFuture<Void> put(StorageKey key, byte[] value) {
                putCount++;
                data.put(key, value);
                return CompletableFuture.completedFuture(null);
            }

            @Override
            public CompletableFuture<Void> delete(StorageKey key) {
                deleteCount++;
                data.remove(key);
                return CompletableFuture.completedFuture(null);
            }
        }

        TrackingStorage storage = new TrackingStorage();
        AutomergeUrl url;

        // Create document - should trigger storage operations
        RepoConfig config = RepoConfig.builder().storage(storage).build();
        try (Repo repo = Repo.load(config)) {
            DocHandle handle = waitFor(repo.create(), "document creation");
            url = handle.getUrl();

            waitFor(handle.withDocument(doc -> {
                try (Transaction tx = doc.startTransaction()) {
                    tx.set(ObjectId.ROOT, "test", "value");
                    tx.commit();
                }
                return null;
            }), "document change");
        }

        // Verify storage was actually used
        assertTrue(storage.loadRangeCount > 0 || storage.loadCount > 0, "Storage load should be called");
        assertTrue(storage.putCount > 0, "Storage put should be called");

        // Reload should call load operations
        int loadBeforeReload = storage.loadCount + storage.loadRangeCount;
        RepoConfig config2 = RepoConfig.builder().storage(storage).build();
        try (Repo repo = Repo.load(config2)) {
            DocHandle handle = waitFor(repo.find(url), "document find").get();
            assertNotNull(handle);
        }

        int loadAfterReload = storage.loadCount + storage.loadRangeCount;
        assertTrue(loadAfterReload > loadBeforeReload, "Storage should be loaded during repo reload");
    }

    @Test
    void testEmptyStorageCreatesNewRepo() {
        InMemoryStorage storage = new InMemoryStorage();

        // Create repo with empty storage
        RepoConfig config = RepoConfig.builder().storage(storage).build();
        try (Repo repo = Repo.load(config)) {
            assertNotNull(repo, "Repo should be created with empty storage");

            // Should be able to create documents
            DocHandle handle = waitFor(repo.create(), "document creation");
            assertNotNull(handle);
        }
    }

    @Test
    void testStorageLoadErrorDuringRepoLoad() {
        // Storage that fails on loadRange
        Storage failingStorage = new Storage() {
            @Override
            public CompletableFuture<Optional<byte[]>> load(StorageKey key) {
                return CompletableFuture.completedFuture(Optional.empty());
            }

            @Override
            public CompletableFuture<Map<StorageKey, byte[]>> loadRange(StorageKey prefix) {
                CompletableFuture<Map<StorageKey, byte[]>> future = new CompletableFuture<>();
                future.completeExceptionally(new RuntimeException("Storage load failed"));
                return future;
            }

            @Override
            public CompletableFuture<Void> put(StorageKey key, byte[] value) {
                return CompletableFuture.completedFuture(null);
            }

            @Override
            public CompletableFuture<Void> delete(StorageKey key) {
                return CompletableFuture.completedFuture(null);
            }
        };

        RepoConfig config = RepoConfig.builder().storage(failingStorage).build();

        // Repo should handle storage load failure gracefully
        // It may start with empty state instead of failing
        try (Repo repo = Repo.load(config)) {
            assertNotNull(repo, "Repo should be created even with failing storage load");
            // Should be able to create new documents
            DocHandle handle = waitFor(repo.create(), "document creation");
            assertNotNull(handle);
        }
    }

    @Test
    void testStoragePutErrorDuringDocumentModification() {
        // Storage that can enable failures after creation
        class FailableStorage implements Storage {

            private final ConcurrentHashMap<StorageKey, byte[]> data = new ConcurrentHashMap<>();
            private boolean failPut = false;

            @Override
            public CompletableFuture<Optional<byte[]>> load(StorageKey key) {
                byte[] value = data.get(key);
                return CompletableFuture.completedFuture(value == null ? Optional.empty() : Optional.of(value));
            }

            @Override
            public CompletableFuture<Map<StorageKey, byte[]>> loadRange(StorageKey prefix) {
                Map<StorageKey, byte[]> result = new ConcurrentHashMap<>();
                for (Map.Entry<StorageKey, byte[]> entry : data.entrySet()) {
                    if (prefix.isPrefixOf(entry.getKey())) {
                        result.put(entry.getKey(), entry.getValue());
                    }
                }
                return CompletableFuture.completedFuture(result);
            }

            @Override
            public CompletableFuture<Void> put(StorageKey key, byte[] value) {
                if (failPut) {
                    CompletableFuture<Void> future = new CompletableFuture<>();
                    future.completeExceptionally(new RuntimeException("Storage put failed"));
                    return future;
                }
                data.put(key, value);
                return CompletableFuture.completedFuture(null);
            }

            @Override
            public CompletableFuture<Void> delete(StorageKey key) {
                return CompletableFuture.completedFuture(null);
            }

            public void enableFailure() {
                failPut = true;
            }
        }

        FailableStorage storage = new FailableStorage();
        RepoConfig config = RepoConfig.builder().storage(storage).build();

        try (Repo repo = Repo.load(config)) {
            DocHandle handle = waitFor(repo.create(), "document creation");

            // Enable storage failure
            storage.enableFailure();

            // Document modification may complete even if storage fails
            // The repo may cache the change or handle storage errors gracefully
            CompletableFuture<?> future = handle.withDocument(doc -> {
                try (Transaction tx = doc.startTransaction()) {
                    tx.set(ObjectId.ROOT, "test", "fail");
                    tx.commit();
                }
                return null;
            });

            // The operation completes even if storage fails - this is acceptable
            // as the change is still in memory
            waitFor(future, "document change");

            // Verify the change is in memory even if storage failed
            String value = waitFor(
                    handle.withDocument(doc -> doc.get(ObjectId.ROOT, "test")
                            .map(v -> v instanceof AmValue.Str ? ((AmValue.Str) v).getValue() : null).orElse(null)),
                    "view after storage failure");
            assertEquals("fail", value, "Change should be in memory even if storage fails");
        }
    }
}
