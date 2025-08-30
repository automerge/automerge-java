package org.automerge;

import java.util.Objects;

/**
 * Represents a storage operation task in the samod-core system.
 * This is an abstract base class with concrete subclasses for each operation type.
 */
public abstract class StorageTask {

    /**
     * A storage task to load a single value by key.
     */
    public static class Load extends StorageTask {
        private final StorageKey key;

        Load(StorageKey key) {
            this.key = Objects.requireNonNull(key, "key cannot be null");
        }

        public StorageKey getKey() {
            return key;
        }

        @Override
        public boolean equals(Object obj) {
            if (this == obj) return true;
            if (obj == null || getClass() != obj.getClass()) return false;
            Load load = (Load) obj;
            return Objects.equals(key, load.key);
        }

        @Override
        public int hashCode() {
            return Objects.hash(key);
        }

        @Override
        public String toString() {
            return "StorageTask.Load{key=" + key + "}";
        }
    }

    /**
     * A storage task to load a range of values with keys matching a prefix.
     */
    public static class LoadRange extends StorageTask {
        private final StorageKey prefix;

        LoadRange(StorageKey prefix) {
            this.prefix = Objects.requireNonNull(prefix, "prefix cannot be null");
        }

        public StorageKey getPrefix() {
            return prefix;
        }

        @Override
        public boolean equals(Object obj) {
            if (this == obj) return true;
            if (obj == null || getClass() != obj.getClass()) return false;
            LoadRange loadRange = (LoadRange) obj;
            return Objects.equals(prefix, loadRange.prefix);
        }

        @Override
        public int hashCode() {
            return Objects.hash(prefix);
        }

        @Override
        public String toString() {
            return "StorageTask.LoadRange{prefix=" + prefix + "}";
        }
    }

    /**
     * A storage task to store a key-value pair.
     */
    public static class Put extends StorageTask {
        private final StorageKey key;
        private final byte[] value;

        Put(StorageKey key, byte[] value) {
            this.key = Objects.requireNonNull(key, "key cannot be null");
            this.value = Objects.requireNonNull(value, "value cannot be null").clone();
        }

        public StorageKey getKey() {
            return key;
        }

        public byte[] getValue() {
            return value.clone();
        }

        @Override
        public boolean equals(Object obj) {
            if (this == obj) return true;
            if (obj == null || getClass() != obj.getClass()) return false;
            Put put = (Put) obj;
            return Objects.equals(key, put.key) && java.util.Arrays.equals(value, put.value);
        }

        @Override
        public int hashCode() {
            return Objects.hash(key, java.util.Arrays.hashCode(value));
        }

        @Override
        public String toString() {
            return "StorageTask.Put{key=" + key + ", valueLength=" + value.length + "}";
        }
    }

    /**
     * A storage task to delete a key-value pair.
     */
    public static class Delete extends StorageTask {
        private final StorageKey key;

        Delete(StorageKey key) {
            this.key = Objects.requireNonNull(key, "key cannot be null");
        }

        public StorageKey getKey() {
            return key;
        }

        @Override
        public boolean equals(Object obj) {
            if (this == obj) return true;
            if (obj == null || getClass() != obj.getClass()) return false;
            Delete delete = (Delete) obj;
            return Objects.equals(key, delete.key);
        }

        @Override
        public int hashCode() {
            return Objects.hash(key);
        }

        @Override
        public String toString() {
            return "StorageTask.Delete{key=" + key + "}";
        }
    }
}
