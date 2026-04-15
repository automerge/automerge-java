package org.automerge.repo;

import java.util.Objects;

/**
 * Represents a command identifier for tracking command completion. CommandIds
 * are int counters that uniquely identify dispatched commands.
 */
class CommandId {

    private final int value;

    /**
     * Creates a CommandId with the given int value. Package-private constructor -
     * only called from JNI layer.
     *
     * @param value
     *            The command ID value
     */
    CommandId(int value) {
        this.value = value;
    }

    public int getValue() {
        return value;
    }

    @Override
    public boolean equals(Object obj) {
        if (this == obj)
            return true;
        if (obj == null || getClass() != obj.getClass())
            return false;
        CommandId other = (CommandId) obj;
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
