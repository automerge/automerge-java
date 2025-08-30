package org.automerge;

import java.util.Objects;

/**
 * Represents an IO task identifier.
 * IoTaskIds are long counters that uniquely identify IO operations.
 */
public class IoTaskId {

    private final long value;

    /**
     * Creates an IoTaskId with the given long value.
     * Package-private constructor - only called from JNI layer.
     * @param value The IO task ID value
     */
    IoTaskId(long value) {
        this.value = value;
    }

    /**
     * Gets the long representation of this IoTaskId.
     * @return The IO task ID value
     */
    public long getValue() {
        return value;
    }

    @Override
    public boolean equals(Object obj) {
        if (this == obj) return true;
        if (obj == null || getClass() != obj.getClass()) return false;
        IoTaskId other = (IoTaskId) obj;
        return value == other.value;
    }

    @Override
    public int hashCode() {
        return Objects.hash(value);
    }

    @Override
    public String toString() {
        return String.valueOf(value);
    }
}
