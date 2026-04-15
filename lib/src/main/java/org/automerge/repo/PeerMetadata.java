package org.automerge.repo;

import java.util.Objects;
import java.util.Optional;

/**
 * Metadata about a peer from the handshake. Contains information about the
 * peer's connection behavior and capabilities.
 */
public class PeerMetadata {

    private final boolean isEphemeral;
    private final Optional<StorageId> storageId;

    /**
     * Creates a new PeerMetadata instance.
     *
     * @param isEphemeral
     *            Whether the peer expects to connect again with this storage ID
     * @param storageId
     *            The peer's storage ID, if any
     */
    PeerMetadata(boolean isEphemeral, Optional<StorageId> storageId) {
        this.isEphemeral = isEphemeral;
        this.storageId = storageId != null ? storageId : Optional.empty();
    }

    /**
     * Returns whether the peer expects to connect again with this storage ID.
     *
     * @return true if the peer is ephemeral (won't reconnect with same storage ID)
     */
    public boolean isEphemeral() {
        return isEphemeral;
    }

    /**
     * Returns the peer's storage ID, if any.
     *
     * @return The storage ID, or empty if not provided
     */
    public Optional<StorageId> getStorageId() {
        return storageId;
    }

    @Override
    public boolean equals(Object obj) {
        if (this == obj)
            return true;
        if (obj == null || getClass() != obj.getClass())
            return false;
        PeerMetadata that = (PeerMetadata) obj;
        return isEphemeral == that.isEphemeral && Objects.equals(storageId, that.storageId);
    }

    @Override
    public int hashCode() {
        return Objects.hash(isEphemeral, storageId);
    }

    @Override
    public String toString() {
        return "PeerMetadata{isEphemeral=" + isEphemeral + ", storageId=" + storageId + "}";
    }
}
