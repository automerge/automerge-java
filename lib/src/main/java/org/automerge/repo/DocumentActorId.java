package org.automerge.repo;

import java.util.Objects;

/**
 * Represents a document actor ID managed by samod-core. DocumentActorIds are
 * created by the Hub actor, not generated directly.
 */
class DocumentActorId {

    private final int value;

    DocumentActorId(int value) {
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
