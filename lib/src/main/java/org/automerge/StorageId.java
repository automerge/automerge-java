package org.automerge;

import java.util.Objects;

/**
 * Represents a storage ID in the samod network protocol.
 * StorageId is a UUID string wrapper that is only created by Rust code.
 */
public class StorageId {

    private final String value;

    /**
     * Creates a StorageId with the given string value.
     * Package-private constructor - only called from JNI layer.
     * @param value The storage ID string
     */
    StorageId(String value) {
        this.value = Objects.requireNonNull(
            value,
            "StorageId value cannot be null"
        );
    }

    /**
     * Creates a StorageId from a string value.
     * @param value The storage ID string
     * @return A StorageId instance
     */
    public static StorageId fromString(String value) {
        return new StorageId(value);
    }

    /**
     * Gets the string representation of this StorageId.
     * @return The storage ID string
     */
    public String getValue() {
        return value;
    }

    @Override
    public boolean equals(Object obj) {
        if (this == obj) return true;
        if (obj == null || getClass() != obj.getClass()) return false;
        StorageId other = (StorageId) obj;
        return Objects.equals(value, other.value);
    }

    @Override
    public int hashCode() {
        return Objects.hash(value);
    }

    @Override
    public String toString() {
        return value;
    }
}
