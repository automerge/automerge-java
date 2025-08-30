package org.automerge;

import java.util.Objects;

/**
 * Represents a command identifier for tracking command completion.
 * CommandIds are long counters that uniquely identify dispatched commands.
 */
public class CommandId {

    private final long value;

    /**
     * Creates a CommandId with the given long value.
     * Package-private constructor - only called from JNI layer.
     * @param value The command ID value
     */
    CommandId(long value) {
        this.value = value;
    }

    /**
     * Gets the long representation of this CommandId.
     * @return The command ID value
     */
    public long getValue() {
        return value;
    }

    @Override
    public boolean equals(Object obj) {
        if (this == obj) return true;
        if (obj == null || getClass() != obj.getClass()) return false;
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
