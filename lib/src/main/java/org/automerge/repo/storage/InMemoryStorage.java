package org.automerge.repo.storage;

import java.util.HashMap;
import java.util.Map;
import java.util.Optional;
import java.util.concurrent.CompletableFuture;
import java.util.concurrent.ConcurrentHashMap;
import org.automerge.repo.Storage;
import org.automerge.repo.StorageKey;

/**
 * In-memory storage implementation using ConcurrentHashMap.
 *
 * This implementation is thread-safe and suitable for testing or applications
 * that don't require persistence.
 *
 * All operations complete immediately (synchronously) but return
 * CompletableFuture for API consistency.
 */
public class InMemoryStorage implements Storage {

    private final ConcurrentHashMap<StorageKey, byte[]> data;

    public InMemoryStorage() {
        this.data = new ConcurrentHashMap<>();
    }

    @Override
    public CompletableFuture<Optional<byte[]>> load(StorageKey key) {
        byte[] value = data.get(key);
        if (value == null) {
            return CompletableFuture.completedFuture(Optional.empty());
        }
        // Return a defensive copy to prevent external modification
        return CompletableFuture.completedFuture(Optional.of(value.clone()));
    }

    @Override
    public CompletableFuture<Map<StorageKey, byte[]>> loadRange(StorageKey prefix) {
        Map<StorageKey, byte[]> result = new HashMap<>();

        for (Map.Entry<StorageKey, byte[]> entry : data.entrySet()) {
            if (prefix.isPrefixOf(entry.getKey())) {
                // Defensive copy of values
                result.put(entry.getKey(), entry.getValue().clone());
            }
        }

        return CompletableFuture.completedFuture(result);
    }

    @Override
    public CompletableFuture<Void> put(StorageKey key, byte[] value) {
        // Store a defensive copy to prevent external modification
        data.put(key, value.clone());
        return CompletableFuture.completedFuture(null);
    }

    @Override
    public CompletableFuture<Void> delete(StorageKey key) {
        data.remove(key);
        return CompletableFuture.completedFuture(null);
    }

    /**
     * @return the number of stored key-value pairs
     */
    public int size() {
        return data.size();
    }

    /**
     * Clears all stored data. Useful for testing.
     */
    public void clear() {
        data.clear();
    }
}
