package org.automerge.repo;

import java.util.Objects;

/**
 * Represents an IO task identifier. IoTaskIds are int counters that uniquely
 * identify IO operations.
 */
class IoTaskId {

    private final int value;

    IoTaskId(int value) {
        this.value = value;
    }

    int getValue() {
        return value;
    }

    @Override
    public boolean equals(Object obj) {
        if (this == obj)
            return true;
        if (obj == null || getClass() != obj.getClass())
            return false;
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
