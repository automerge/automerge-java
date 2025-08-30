package org.automerge;

import java.util.List;
import java.util.Objects;

/**
 * Event emitted when a document changes.
 * Contains the new heads (change hashes) after the change.
 */
public class DocumentChanged {
    private final List<ChangeHash> newHeads;

    DocumentChanged(List<ChangeHash> newHeads) {
        this.newHeads = Objects.requireNonNull(newHeads, "newHeads cannot be null");
    }

    /**
     * Gets the new heads (change hashes) after the document change.
     * @return The list of new change hashes
     */
    public List<ChangeHash> getNewHeads() {
        return newHeads;
    }

    @Override
    public boolean equals(Object obj) {
        if (this == obj) return true;
        if (obj == null || getClass() != obj.getClass()) return false;
        DocumentChanged that = (DocumentChanged) obj;
        return Objects.equals(newHeads, that.newHeads);
    }

    @Override
    public int hashCode() {
        return Objects.hash(newHeads);
    }

    @Override
    public String toString() {
        return "DocumentChanged{" +
               "newHeads=" + newHeads +
               "}";
    }
}
