package org.automerge.repo;

import java.util.Map;
import java.util.Optional;
import java.util.concurrent.CompletableFuture;

/**
 * Storage interface for persisting Automerge documents and metadata.
 *
 * All methods return CompletableFuture to support asynchronous storage
 * implementations (e.g., disk I/O, network storage).
 *
 * Implementations must handle concurrent access safely.
 */
public interface Storage {
    /**
     * Loads a single value by key.
     *
     * @param key
     *            The storage key
     * @return A future containing Optional with the value, or empty if key not
     *         found
     */
    CompletableFuture<Optional<byte[]>> load(StorageKey key);

    /**
     * Loads all key-value pairs with keys matching the given prefix.
     *
     * @param prefix
     *            The key prefix to match
     * @return A future containing a map of matching key-value pairs (may be empty)
     */
    CompletableFuture<Map<StorageKey, byte[]>> loadRange(StorageKey prefix);

    /**
     * Stores a key-value pair.
     *
     * @param key
     *            The storage key
     * @param value
     *            The value to store
     * @return A future that completes when the operation is done
     */
    CompletableFuture<Void> put(StorageKey key, byte[] value);

    /**
     * Deletes a key-value pair.
     *
     * @param key
     *            The storage key to delete
     * @return A future that completes when the operation is done
     */
    CompletableFuture<Void> delete(StorageKey key);
}
