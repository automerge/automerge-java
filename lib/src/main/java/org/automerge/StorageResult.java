package org.automerge;

import java.util.Map;
import java.util.Objects;
import java.util.Optional;

/**
 * Represents the result of a storage operation task.
 * This is an abstract base class with concrete subclasses for each result type.
 */
public abstract class StorageResult {

    /**
     * Result of a Load storage task.
     */
    public static class Load extends StorageResult {

        private final Optional<byte[]> value;

        Load(byte[] value) {
            this.value = value != null
                ? Optional.of(value.clone())
                : Optional.empty();
        }

        /**
         * Gets the loaded value.
         * @return The value bytes, or empty if not found
         */
        public Optional<byte[]> getValue() {
            return value.map(bytes -> bytes.clone());
        }

        /**
         * Returns true if a value was found for the key.
         * @return true if value exists, false if key not found
         */
        public boolean isFound() {
            return value.isPresent();
        }

        @Override
        public boolean equals(Object obj) {
            if (this == obj) return true;
            if (obj == null || getClass() != obj.getClass()) return false;
            Load load = (Load) obj;
            if (value.isPresent() && load.value.isPresent()) {
                return java.util.Arrays.equals(value.get(), load.value.get());
            }
            return value.equals(load.value);
        }

        @Override
        public int hashCode() {
            return value.map(java.util.Arrays::hashCode).orElse(0);
        }

        @Override
        public String toString() {
            return (
                "StorageResult.Load{found=" +
                isFound() +
                (value.isPresent()
                        ? ", valueLength=" + value.get().length
                        : "") +
                "}"
            );
        }
    }

    /**
     * Result of a LoadRange storage task.
     */
    public static class LoadRange extends StorageResult {

        private final Map<StorageKey, byte[]> values;

        LoadRange(Map<StorageKey, byte[]> values) {
            this.values = Objects.requireNonNull(
                values,
                "values cannot be null"
            );
        }

        /**
         * Gets the map of loaded key-value pairs.
         * @return Map of StorageKey to byte array values
         */
        public Map<StorageKey, byte[]> getValues() {
            return values;
        }

        @Override
        public boolean equals(Object obj) {
            if (this == obj) return true;
            if (obj == null || getClass() != obj.getClass()) return false;
            LoadRange loadRange = (LoadRange) obj;
            return Objects.equals(values, loadRange.values);
        }

        @Override
        public int hashCode() {
            return Objects.hash(values);
        }

        @Override
        public String toString() {
            return "StorageResult.LoadRange{count=" + values.size() + "}";
        }
    }

    /**
     * Result of a Put storage task (empty - operation succeeded).
     */
    public static class Put extends StorageResult {

        Put() {
            // Empty result - Put operation succeeded
        }

        @Override
        public boolean equals(Object obj) {
            return this == obj || (obj != null && getClass() == obj.getClass());
        }

        @Override
        public int hashCode() {
            return getClass().hashCode();
        }

        @Override
        public String toString() {
            return "StorageResult.Put{}";
        }
    }

    /**
     * Result of a Delete storage task (empty - operation succeeded).
     */
    public static class Delete extends StorageResult {

        Delete() {
            // Empty result - Delete operation succeeded
        }

        @Override
        public boolean equals(Object obj) {
            return this == obj || (obj != null && getClass() == obj.getClass());
        }

        @Override
        public int hashCode() {
            return getClass().hashCode();
        }

        @Override
        public String toString() {
            return "StorageResult.Delete{}";
        }
    }
}
