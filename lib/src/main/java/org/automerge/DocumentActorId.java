package org.automerge;

import java.util.Objects;

/**
 * Represents a document actor ID in the samod network protocol.
 * DocumentActorId is a u32 counter wrapper managed internally by samod-core.
 * DocumentActorIds are created by the Hub actor, not generated directly.
 */
public class DocumentActorId {

    private final int value;

    /**
     * Creates a DocumentActorId with the given int value.
     * Package-private constructor - only called from JNI layer.
     * @param value The document actor ID value
     */
    DocumentActorId(int value) {
        this.value = value;
    }

    /**
     * Gets the int representation of this DocumentActorId.
     * @return The document actor ID value
     */
    public int getValue() {
        return value;
    }

    @Override
    public boolean equals(Object obj) {
        if (this == obj) return true;
        if (obj == null || getClass() != obj.getClass()) return false;
        DocumentActorId other = (DocumentActorId) obj;
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
