package org.automerge;

import java.time.Instant;
import java.util.Map;
import java.util.Objects;
import java.util.Optional;

/**
 * Information about a network connection including state and timing data.
 */
public class ConnectionInfo {

    private final ConnectionId id;
    private final Optional<Instant> lastReceived;
    private final Optional<Instant> lastSent;
    private final Map<DocumentId, PeerDocState> docs;
    private final ConnectionState state;

    ConnectionInfo(
        ConnectionId id,
        Optional<Instant> lastReceived,
        Optional<Instant> lastSent,
        Map<DocumentId, PeerDocState> docs,
        ConnectionState state
    ) {
        this.id = Objects.requireNonNull(id, "id cannot be null");
        this.lastReceived = lastReceived != null
            ? lastReceived
            : Optional.empty();
        this.lastSent = lastSent != null ? lastSent : Optional.empty();
        this.docs = Objects.requireNonNull(docs, "docs cannot be null");
        this.state = Objects.requireNonNull(state, "state cannot be null");
    }

    /**
     * Gets the connection ID.
     * @return The connection ID
     */
    public ConnectionId getId() {
        return id;
    }

    /**
     * Gets the timestamp of the last received message.
     * @return The last received timestamp, or empty if none
     */
    public Optional<Instant> getLastReceived() {
        return lastReceived;
    }

    /**
     * Gets the timestamp of the last sent message.
     * @return The last sent timestamp, or empty if none
     */
    public Optional<Instant> getLastSent() {
        return lastSent;
    }

    /**
     * Gets the map of document states being synchronized with this peer.
     * @return Map of document IDs to their synchronization states
     */
    public Map<DocumentId, PeerDocState> getDocs() {
        return docs;
    }

    /**
     * Gets the current connection state.
     * @return The connection state
     */
    public ConnectionState getState() {
        return state;
    }

    @Override
    public boolean equals(Object obj) {
        if (this == obj) return true;
        if (obj == null || getClass() != obj.getClass()) return false;
        ConnectionInfo that = (ConnectionInfo) obj;
        return (
            Objects.equals(id, that.id) &&
            Objects.equals(lastReceived, that.lastReceived) &&
            Objects.equals(lastSent, that.lastSent) &&
            Objects.equals(docs, that.docs) &&
            Objects.equals(state, that.state)
        );
    }

    @Override
    public int hashCode() {
        return Objects.hash(id, lastReceived, lastSent, docs, state);
    }

    @Override
    public String toString() {
        return (
            "ConnectionInfo{" +
            "id=" +
            id +
            ", lastReceived=" +
            lastReceived +
            ", lastSent=" +
            lastSent +
            ", docs=" +
            docs +
            ", state=" +
            state +
            "}"
        );
    }
}
