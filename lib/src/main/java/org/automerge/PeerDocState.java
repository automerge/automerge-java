package org.automerge;

import java.time.Instant;
import java.util.List;
import java.util.Objects;
import java.util.Optional;

/**
 * The state of synchronization for one (peer, document) pair.
 */
public class PeerDocState {

    private final Optional<Instant> lastReceived;
    private final Optional<Instant> lastSent;
    private final Optional<List<ChangeHash>> lastSentHeads;
    private final Optional<List<ChangeHash>> lastAckedHeads;

    PeerDocState(
        Optional<Instant> lastReceived,
        Optional<Instant> lastSent,
        Optional<List<ChangeHash>> lastSentHeads,
        Optional<List<ChangeHash>> lastAckedHeads
    ) {
        this.lastReceived = lastReceived != null
            ? lastReceived
            : Optional.empty();
        this.lastSent = lastSent != null ? lastSent : Optional.empty();
        this.lastSentHeads = lastSentHeads != null
            ? lastSentHeads
            : Optional.empty();
        this.lastAckedHeads = lastAckedHeads != null
            ? lastAckedHeads
            : Optional.empty();
    }

    /**
     * Gets when we last received a message from this peer.
     * @return The timestamp, or empty if never received
     */
    public Optional<Instant> getLastReceived() {
        return lastReceived;
    }

    /**
     * Gets when we last sent a message to this peer.
     * @return The timestamp, or empty if never sent
     */
    public Optional<Instant> getLastSent() {
        return lastSent;
    }

    /**
     * Gets the heads of the document when we last sent a message.
     * @return The change hashes, or empty if never sent
     */
    public Optional<List<ChangeHash>> getLastSentHeads() {
        return lastSentHeads;
    }

    /**
     * Gets the last heads of the document that the peer said they had.
     * @return The change hashes, or empty if never acknowledged
     */
    public Optional<List<ChangeHash>> getLastAckedHeads() {
        return lastAckedHeads;
    }

    @Override
    public boolean equals(Object obj) {
        if (this == obj) return true;
        if (obj == null || getClass() != obj.getClass()) return false;
        PeerDocState that = (PeerDocState) obj;
        return (
            Objects.equals(lastReceived, that.lastReceived) &&
            Objects.equals(lastSent, that.lastSent) &&
            Objects.equals(lastSentHeads, that.lastSentHeads) &&
            Objects.equals(lastAckedHeads, that.lastAckedHeads)
        );
    }

    @Override
    public int hashCode() {
        return Objects.hash(
            lastReceived,
            lastSent,
            lastSentHeads,
            lastAckedHeads
        );
    }

    @Override
    public String toString() {
        return (
            "PeerDocState{" +
            "lastReceived=" +
            lastReceived +
            ", lastSent=" +
            lastSent +
            ", lastSentHeads=" +
            lastSentHeads +
            ", lastAckedHeads=" +
            lastAckedHeads +
            "}"
        );
    }
}
