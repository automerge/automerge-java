package org.automerge.repo.storage;

import java.io.IOException;
import java.io.UncheckedIOException;
import java.nio.file.DirectoryStream;
import java.nio.file.Files;
import java.nio.file.Path;
import java.nio.file.StandardCopyOption;
import java.util.ArrayDeque;
import java.util.Collections;
import java.util.Deque;
import java.util.HashMap;
import java.util.Map;
import java.util.Optional;
import java.util.concurrent.CompletableFuture;
import org.automerge.repo.Storage;
import org.automerge.repo.StorageKey;

/**
 * Filesystem-backed storage implementation.
 *
 * Persists Automerge document data to the local filesystem. The first component
 * of each {@link StorageKey} is "splayed" into two directory levels by splitting
 * at the second character, distributing files across subdirectories to avoid
 * filesystem performance issues with large flat directories.
 *
 * <p>For example, the key {@code ["abc123", "incremental", "hash"]} maps to the
 * path {@code <baseDir>/ab/c123/incremental/hash}.
 *
 * <p>Writes are atomic: data is written to a temporary file in a {@code .temp}
 * directory under the base directory, then renamed into place. This keeps
 * temporary files out of the data tree so that range scans by other
 * implementations sharing the same directory never see in-flight writes.
 *
 * <p>All operations complete synchronously but return {@link CompletableFuture}
 * for API consistency with the {@link Storage} interface.
 *
 * <p>This implementation is thread-safe for distinct keys. Concurrent writes to
 * the same key rely on the filesystem's rename atomicity guarantees.
 */
public class FileSystemStorage implements Storage {

    private final Path baseDir;
    private final Path tempDir;

    /**
     * Creates a new FileSystemStorage rooted at the given directory.
     *
     * The directory will be created if it does not exist.
     *
     * @param baseDir
     *            The root directory for storage
     * @throws UncheckedIOException
     *             if the directory cannot be created
     */
    public FileSystemStorage(Path baseDir) {
        this.baseDir = baseDir;
        this.tempDir = baseDir.resolve(".temp");
        try {
            Files.createDirectories(baseDir);
            Files.createDirectories(tempDir);
        } catch (IOException e) {
            throw new UncheckedIOException("Failed to create storage directory: " + baseDir, e);
        }
    }

    @Override
    public CompletableFuture<Optional<byte[]>> load(StorageKey key) {
        Path path = keyToPath(key);
        if (!Files.isRegularFile(path)) {
            return CompletableFuture.completedFuture(Optional.empty());
        }
        try {
            byte[] data = Files.readAllBytes(path);
            return CompletableFuture.completedFuture(Optional.of(data));
        } catch (IOException e) {
            return failedFuture(new UncheckedIOException("Failed to load key: " + key, e));
        }
    }

    @Override
    public CompletableFuture<Map<StorageKey, byte[]>> loadRange(StorageKey prefix) {
        Path prefixPath = keyToPath(prefix);
        if (!Files.isDirectory(prefixPath)) {
            return CompletableFuture.completedFuture(Collections.emptyMap());
        }
        try {
            Map<StorageKey, byte[]> result = new HashMap<>();
            Deque<PrefixEntry> toVisit = new ArrayDeque<>();
            toVisit.push(new PrefixEntry(prefixPath, prefix));

            while (!toVisit.isEmpty()) {
                PrefixEntry current = toVisit.pop();
                try (DirectoryStream<Path> stream = Files.newDirectoryStream(current.path)) {
                    for (Path entry : stream) {
                        String filename = entry.getFileName().toString();
                        String[] parentParts = current.key.getParts();
                        String[] childParts = new String[parentParts.length + 1];
                        System.arraycopy(parentParts, 0, childParts, 0, parentParts.length);
                        childParts[parentParts.length] = filename;
                        StorageKey childKey = new StorageKey(childParts);

                        if (Files.isDirectory(entry)) {
                            toVisit.push(new PrefixEntry(entry, childKey));
                        } else if (Files.isRegularFile(entry)) {
                            byte[] data = Files.readAllBytes(entry);
                            result.put(childKey, data);
                        }
                    }
                }
            }
            return CompletableFuture.completedFuture(result);
        } catch (IOException e) {
            return failedFuture(new UncheckedIOException("Failed to load range for prefix: " + prefix, e));
        }
    }

    @Override
    public CompletableFuture<Void> put(StorageKey key, byte[] value) {
        Path path = keyToPath(key);
        try {
            Files.createDirectories(path.getParent());
            // Atomic write: write to temp file in .temp dir, then rename.
            // Temp files are kept out of the data tree so that loadRange
            // never picks them up as real entries.
            Path tmp = Files.createTempFile(tempDir, ".automerge-", ".tmp");
            try {
                Files.write(tmp, value);
                Files.move(tmp, path,
                        StandardCopyOption.REPLACE_EXISTING,
                        StandardCopyOption.ATOMIC_MOVE);
            } catch (IOException e) {
                // Clean up temp file on failure
                try {
                    Files.deleteIfExists(tmp);
                } catch (IOException suppressed) {
                    e.addSuppressed(suppressed);
                }
                throw e;
            }
            return CompletableFuture.completedFuture(null);
        } catch (IOException e) {
            return failedFuture(new UncheckedIOException("Failed to put key: " + key, e));
        }
    }

    @Override
    public CompletableFuture<Void> delete(StorageKey key) {
        Path path = keyToPath(key);
        try {
            Files.deleteIfExists(path);
            return CompletableFuture.completedFuture(null);
        } catch (IOException e) {
            return failedFuture(new UncheckedIOException("Failed to delete key: " + key, e));
        }
    }

    @SuppressWarnings("unchecked")
    private static <T> CompletableFuture<T> failedFuture(Throwable ex) {
        CompletableFuture<T> f = new CompletableFuture<>();
        f.completeExceptionally(ex);
        return f;
    }

    /**
     * Converts a {@link StorageKey} to a filesystem path.
     *
     * The first key component is splayed: its first two characters become one
     * directory level, and the remainder becomes the next. Subsequent components
     * map directly to path segments.
     *
     * @param key
     *            The storage key
     * @return The resolved path under the base directory
     */
    Path keyToPath(StorageKey key) {
        String[] parts = key.getParts();
        Path path = baseDir;
        for (int i = 0; i < parts.length; i++) {
            if (i == 0) {
                // Splay first component: first 2 chars → dir, rest → subdir
                String component = parts[i];
                int splitAt = Math.min(2, component.length());
                path = path.resolve(component.substring(0, splitAt));
                if (splitAt < component.length()) {
                    path = path.resolve(component.substring(splitAt));
                }
            } else {
                path = path.resolve(parts[i]);
            }
        }
        return path;
    }

    private static class PrefixEntry {
        final Path path;
        final StorageKey key;

        PrefixEntry(Path path, StorageKey key) {
            this.path = path;
            this.key = key;
        }
    }
}
