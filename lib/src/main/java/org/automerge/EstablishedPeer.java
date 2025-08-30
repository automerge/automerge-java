package org.automerge;

import java.util.Objects;

/**
 * Represents an established peer connection with its connection ID and peer ID.
 * This is a simple tuple-like class returned by Hub.getEstablishedPeers().
 */
public class EstablishedPeer {

    private final ConnectionId connectionId;
    private final PeerId peerId;

    /**
     * Creates an EstablishedPeer instance.
     * Package-private constructor - only called from JNI layer.
     *
     * @param connectionId The connection ID for this peer
     * @param peerId The peer ID
     */
    EstablishedPeer(ConnectionId connectionId, PeerId peerId) {
        this.connectionId = Objects.requireNonNull(connectionId, "connectionId cannot be null");
        this.peerId = Objects.requireNonNull(peerId, "peerId cannot be null");
    }

    /**
     * Gets the connection ID.
     * @return The connection ID
     */
    public ConnectionId getConnectionId() {
        return connectionId;
    }

    /**
     * Gets the peer ID.
     * @return The peer ID
     */
    public PeerId getPeerId() {
        return peerId;
    }

    @Override
    public boolean equals(Object obj) {
        if (this == obj) return true;
        if (obj == null || getClass() != obj.getClass()) return false;
        EstablishedPeer that = (EstablishedPeer) obj;
        return Objects.equals(connectionId, that.connectionId) &&
               Objects.equals(peerId, that.peerId);
    }

    @Override
    public int hashCode() {
        return Objects.hash(connectionId, peerId);
    }

    @Override
    public String toString() {
        return "EstablishedPeer{" +
               "connectionId=" + connectionId +
               ", peerId=" + peerId +
               '}';
    }
}
