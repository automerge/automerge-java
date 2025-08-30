package org.automerge;

import java.util.Objects;

/**
 * Represents a network connection identifier.
 * ConnectionIds are integer counters that uniquely identify network connections.
 */
public class ConnectionId {

    private final int value;

    /**
     * Creates a ConnectionId with the given int value.
     * Package-private constructor - only called from JNI layer.
     * @param value The connection ID value
     */
    ConnectionId(int value) {
        this.value = value;
    }

    /**
     * Gets the int representation of this ConnectionId.
     * @return The connection ID value
     */
    public int getValue() {
        return value;
    }

    @Override
    public boolean equals(Object obj) {
        if (this == obj) return true;
        if (obj == null || getClass() != obj.getClass()) return false;
        ConnectionId other = (ConnectionId) obj;
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
