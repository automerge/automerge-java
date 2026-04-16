package org.automerge.repo.storage;

import static org.junit.jupiter.api.Assertions.*;

import java.io.IOException;
import java.nio.file.Files;
import java.nio.file.Path;
import java.util.Map;
import java.util.Optional;
import org.automerge.repo.StorageKey;
import org.junit.jupiter.api.BeforeEach;
import org.junit.jupiter.api.Test;
import org.junit.jupiter.api.io.TempDir;

class FileSystemStorageTest {

    @TempDir
    Path tempDir;

    FileSystemStorage storage;

    @BeforeEach
    void setUp() {
        storage = new FileSystemStorage(tempDir.resolve("data"));
    }

    // --- Key-to-path splay algorithm ---

    @Test
    void keyToPathSplaysFirstComponent() {
        StorageKey key = new StorageKey("abc123");
        Path path = storage.keyToPath(key);
        assertEquals(tempDir.resolve("data/ab/c123"), path);
    }

    @Test
    void keyToPathMultipleComponents() {
        StorageKey key = new StorageKey("abc123", "incremental", "hash456");
        Path path = storage.keyToPath(key);
        assertEquals(tempDir.resolve("data/ab/c123/incremental/hash456"), path);
    }

    @Test
    void keyToPathShortFirstComponent() {
        // First component with exactly 2 chars: no second segment
        StorageKey key = new StorageKey("ab");
        Path path = storage.keyToPath(key);
        assertEquals(tempDir.resolve("data/ab"), path);
    }

    @Test
    void keyToPathSingleCharFirstComponent() {
        StorageKey key = new StorageKey("a");
        Path path = storage.keyToPath(key);
        assertEquals(tempDir.resolve("data/a"), path);
    }

    // --- put + load round-trip ---

    @Test
    void putThenLoad() {
        StorageKey key = new StorageKey("doc1", "snapshot", "v1");
        byte[] data = "hello world".getBytes();

        storage.put(key, data).join();

        Optional<byte[]> loaded = storage.load(key).join();
        assertTrue(loaded.isPresent());
        assertArrayEquals(data, loaded.get());
    }

    @Test
    void putOverwritesExistingValue() {
        StorageKey key = new StorageKey("doc1", "snapshot", "v1");

        storage.put(key, "first".getBytes()).join();
        storage.put(key, "second".getBytes()).join();

        Optional<byte[]> loaded = storage.load(key).join();
        assertTrue(loaded.isPresent());
        assertArrayEquals("second".getBytes(), loaded.get());
    }

    @Test
    void putCreatesParentDirectories() {
        StorageKey key = new StorageKey("abcdef", "deep", "nested", "path");

        storage.put(key, "data".getBytes()).join();

        Optional<byte[]> loaded = storage.load(key).join();
        assertTrue(loaded.isPresent());
    }

    @Test
    void putWritesAtomically() throws IOException {
        StorageKey key = new StorageKey("doc1", "snapshot", "v1");
        byte[] data = new byte[1024 * 1024]; // 1MB
        java.util.Arrays.fill(data, (byte) 0x42);

        storage.put(key, data).join();

        // Verify no temp files remain in the .temp directory
        Path tempPath = tempDir.resolve("data/.temp");
        try (java.util.stream.Stream<Path> stream = Files.list(tempPath)) {
            long tempFiles = stream
                    .filter(p -> p.getFileName().toString().startsWith(".automerge-"))
                    .count();
            assertEquals(0, tempFiles, "No temp files should remain after put");
        }
    }

    // --- load ---

    @Test
    void loadMissingKeyReturnsEmpty() {
        StorageKey key = new StorageKey("nonexistent", "key");
        Optional<byte[]> loaded = storage.load(key).join();
        assertFalse(loaded.isPresent());
    }

    @Test
    void loadDirectoryReturnsEmpty() throws IOException {
        // Create a directory where a file would be expected
        Path dirPath = storage.keyToPath(new StorageKey("doc1", "snapshot"));
        Files.createDirectories(dirPath);

        StorageKey key = new StorageKey("doc1", "snapshot");
        Optional<byte[]> loaded = storage.load(key).join();
        assertFalse(loaded.isPresent());
    }

    // --- loadRange ---

    @Test
    void loadRangeFindsAllMatchingKeys() {
        StorageKey k1 = new StorageKey("doc1", "incremental", "hash1");
        StorageKey k2 = new StorageKey("doc1", "incremental", "hash2");
        StorageKey k3 = new StorageKey("doc1", "snapshot", "snap1");

        storage.put(k1, "data1".getBytes()).join();
        storage.put(k2, "data2".getBytes()).join();
        storage.put(k3, "data3".getBytes()).join();

        // Load all under doc1
        StorageKey prefix = new StorageKey("doc1");
        Map<StorageKey, byte[]> result = storage.loadRange(prefix).join();

        assertEquals(3, result.size());
        assertArrayEquals("data1".getBytes(), result.get(k1));
        assertArrayEquals("data2".getBytes(), result.get(k2));
        assertArrayEquals("data3".getBytes(), result.get(k3));
    }

    @Test
    void loadRangeNarrowPrefix() {
        StorageKey k1 = new StorageKey("doc1", "incremental", "hash1");
        StorageKey k2 = new StorageKey("doc1", "incremental", "hash2");
        StorageKey k3 = new StorageKey("doc1", "snapshot", "snap1");

        storage.put(k1, "data1".getBytes()).join();
        storage.put(k2, "data2".getBytes()).join();
        storage.put(k3, "data3".getBytes()).join();

        // Load only incrementals
        StorageKey prefix = new StorageKey("doc1", "incremental");
        Map<StorageKey, byte[]> result = storage.loadRange(prefix).join();

        assertEquals(2, result.size());
        assertTrue(result.containsKey(k1));
        assertTrue(result.containsKey(k2));
        assertFalse(result.containsKey(k3));
    }

    @Test
    void loadRangeNonExistentPrefixReturnsEmpty() {
        Map<StorageKey, byte[]> result = storage.loadRange(new StorageKey("nonexistent")).join();
        assertTrue(result.isEmpty());
    }

    @Test
    void loadRangeDoesNotCrossSplayBoundaries() {
        // Two docs with different first components should not cross
        StorageKey k1 = new StorageKey("aaa111", "data");
        StorageKey k2 = new StorageKey("bbb222", "data");

        storage.put(k1, "a".getBytes()).join();
        storage.put(k2, "b".getBytes()).join();

        Map<StorageKey, byte[]> result = storage.loadRange(new StorageKey("aaa111")).join();
        assertEquals(1, result.size());
        assertTrue(result.containsKey(k1));
    }

    // --- delete ---

    @Test
    void deleteRemovesKey() {
        StorageKey key = new StorageKey("doc1", "snapshot", "v1");
        storage.put(key, "data".getBytes()).join();

        storage.delete(key).join();

        Optional<byte[]> loaded = storage.load(key).join();
        assertFalse(loaded.isPresent());
    }

    @Test
    void deleteNonExistentKeySucceeds() {
        StorageKey key = new StorageKey("nonexistent", "key");
        // Should not throw
        storage.delete(key).join();
    }

    // --- Multiple documents ---

    @Test
    void multipleDocumentsIsolated() {
        StorageKey k1 = new StorageKey("doc1", "snapshot", "v1");
        StorageKey k2 = new StorageKey("doc2", "snapshot", "v1");

        storage.put(k1, "doc1-data".getBytes()).join();
        storage.put(k2, "doc2-data".getBytes()).join();

        Optional<byte[]> loaded1 = storage.load(k1).join();
        Optional<byte[]> loaded2 = storage.load(k2).join();

        assertArrayEquals("doc1-data".getBytes(), loaded1.get());
        assertArrayEquals("doc2-data".getBytes(), loaded2.get());
    }

    // --- Edge cases ---

    @Test
    void emptyValueRoundTrips() {
        StorageKey key = new StorageKey("doc1", "empty");
        storage.put(key, new byte[0]).join();

        Optional<byte[]> loaded = storage.load(key).join();
        assertTrue(loaded.isPresent());
        assertEquals(0, loaded.get().length);
    }

    @Test
    void largeValueRoundTrips() {
        StorageKey key = new StorageKey("doc1", "large");
        byte[] data = new byte[1024 * 1024]; // 1MB
        java.util.Arrays.fill(data, (byte) 0xAB);

        storage.put(key, data).join();

        Optional<byte[]> loaded = storage.load(key).join();
        assertTrue(loaded.isPresent());
        assertArrayEquals(data, loaded.get());
    }

    @Test
    void storageAdapterIdKey() {
        // This is a real key pattern used by samod-core
        StorageKey key = new StorageKey("storage-adapter-id");
        storage.put(key, "some-uuid".getBytes()).join();

        Optional<byte[]> loaded = storage.load(key).join();
        assertTrue(loaded.isPresent());
        assertArrayEquals("some-uuid".getBytes(), loaded.get());
    }
}
