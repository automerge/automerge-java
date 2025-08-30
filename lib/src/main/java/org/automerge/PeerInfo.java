package org.automerge;

import java.util.Objects;
import java.util.Optional;

/**
 * Information about a connected peer after successful handshake.
 * Contains the peer's identity, metadata, and protocol version.
 */
public class PeerInfo {

    private final PeerId peerId;
    private final Optional<PeerMetadata> metadata;
    private final String protocolVersion;

    /**
     * Creates a new PeerInfo instance.
     * @param peerId Unique identifier for the peer in the sync network
     * @param metadata Optional metadata about the peer from the handshake
     * @param protocolVersion Protocol version used by the peer
     */
    PeerInfo(
        PeerId peerId,
        Optional<PeerMetadata> metadata,
        String protocolVersion
    ) {
        this.peerId = Objects.requireNonNull(peerId, "peerId cannot be null");
        this.metadata = metadata != null ? metadata : Optional.empty();
        this.protocolVersion = Objects.requireNonNull(
            protocolVersion,
            "protocolVersion cannot be null"
        );
    }

    /**
     * Gets the unique identifier for the peer.
     * @return The peer ID
     */
    public PeerId getPeerId() {
        return peerId;
    }

    /**
     * Gets the optional metadata about the peer from the handshake.
     * @return The peer metadata, or empty if none was provided
     */
    public Optional<PeerMetadata> getMetadata() {
        return metadata;
    }

    /**
     * Returns whether this peer has metadata.
     * @return true if metadata is present, false otherwise
     */
    public boolean hasMetadata() {
        return metadata.isPresent();
    }

    /**
     * Gets the protocol version used by the peer.
     * @return The protocol version string
     */
    public String getProtocolVersion() {
        return protocolVersion;
    }

    @Override
    public boolean equals(Object obj) {
        if (this == obj) return true;
        if (obj == null || getClass() != obj.getClass()) return false;
        PeerInfo peerInfo = (PeerInfo) obj;
        return (
            Objects.equals(peerId, peerInfo.peerId) &&
            Objects.equals(metadata, peerInfo.metadata) &&
            Objects.equals(protocolVersion, peerInfo.protocolVersion)
        );
    }

    @Override
    public int hashCode() {
        return Objects.hash(peerId, metadata, protocolVersion);
    }

    @Override
    public String toString() {
        return (
            "PeerInfo{" +
            "peerId=" +
            peerId +
            ", metadata=" +
            metadata +
            ", protocolVersion='" +
            protocolVersion +
            '\'' +
            '}'
        );
    }
}
