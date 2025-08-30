package org.automerge;

import java.util.Objects;

/**
 * Metadata about a peer from the handshake.
 * Contains information about the peer's connection behavior and capabilities.
 */
public class PeerMetadata {

    private final boolean isEphemeral;

    /**
     * Creates a new PeerMetadata instance.
     * @param isEphemeral Whether the peer expects to connect again with this storage ID
     */
    public PeerMetadata(boolean isEphemeral) {
        this.isEphemeral = isEphemeral;
    }

    /**
     * Returns whether the peer expects to connect again with this storage ID.
     * @return true if the peer is ephemeral (won't reconnect with same storage ID)
     */
    public boolean isEphemeral() {
        return isEphemeral;
    }

    @Override
    public boolean equals(Object obj) {
        if (this == obj) return true;
        if (obj == null || getClass() != obj.getClass()) return false;
        PeerMetadata that = (PeerMetadata) obj;
        return isEphemeral == that.isEphemeral;
    }

    @Override
    public int hashCode() {
        return Objects.hash(isEphemeral);
    }

    @Override
    public String toString() {
        return "PeerMetadata{isEphemeral=" + isEphemeral + "}";
    }
}
