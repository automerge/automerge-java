package org.automerge.repo;

import java.util.Objects;

/**
 * Events that can occur on network connections. This is an abstract base class
 * with concrete subclasses for each event type.
 */
public abstract class ConnectionEvent {

    /**
     * Gets the connection ID associated with this event.
     *
     * @return The connection ID
     */
    public abstract ConnectionId getConnectionId();

    /**
     * Gets the connection owner (dialer or listener) for this event.
     *
     * @return The connection owner
     */
    public abstract ConnectionOwner getOwner();

    /**
     * Handshake completed successfully with a peer.
     */
    public static class HandshakeCompleted extends ConnectionEvent {

        private final ConnectionId connectionId;
        private final ConnectionOwner owner;
        private final PeerInfo peerInfo;

        public HandshakeCompleted(ConnectionId connectionId, ConnectionOwner owner, PeerInfo peerInfo) {
            this.connectionId = Objects.requireNonNull(connectionId, "connectionId cannot be null");
            this.owner = Objects.requireNonNull(owner, "owner cannot be null");
            this.peerInfo = Objects.requireNonNull(peerInfo, "peerInfo cannot be null");
        }

        @Override
        public ConnectionId getConnectionId() {
            return connectionId;
        }

        @Override
        public ConnectionOwner getOwner() {
            return owner;
        }

        public PeerInfo getPeerInfo() {
            return peerInfo;
        }

        @Override
        public boolean equals(Object obj) {
            if (this == obj)
                return true;
            if (obj == null || getClass() != obj.getClass())
                return false;
            HandshakeCompleted that = (HandshakeCompleted) obj;
            return (Objects.equals(connectionId, that.connectionId) && Objects.equals(owner, that.owner)
                    && Objects.equals(peerInfo, that.peerInfo));
        }

        @Override
        public int hashCode() {
            return Objects.hash(connectionId, owner, peerInfo);
        }

        @Override
        public String toString() {
            return ("ConnectionEvent.HandshakeCompleted{" + "connectionId=" + connectionId + ", owner=" + owner
                    + ", peerInfo=" + peerInfo + "}");
        }
    }

    /**
     * Connection failed or was disconnected.
     */
    public static class ConnectionFailed extends ConnectionEvent {

        private final ConnectionId connectionId;
        private final ConnectionOwner owner;
        private final String error;

        public ConnectionFailed(ConnectionId connectionId, ConnectionOwner owner, String error) {
            this.connectionId = Objects.requireNonNull(connectionId, "connectionId cannot be null");
            this.owner = Objects.requireNonNull(owner, "owner cannot be null");
            this.error = Objects.requireNonNull(error, "error cannot be null");
        }

        @Override
        public ConnectionId getConnectionId() {
            return connectionId;
        }

        @Override
        public ConnectionOwner getOwner() {
            return owner;
        }

        public String getError() {
            return error;
        }

        @Override
        public boolean equals(Object obj) {
            if (this == obj)
                return true;
            if (obj == null || getClass() != obj.getClass())
                return false;
            ConnectionFailed that = (ConnectionFailed) obj;
            return (Objects.equals(connectionId, that.connectionId) && Objects.equals(owner, that.owner)
                    && Objects.equals(error, that.error));
        }

        @Override
        public int hashCode() {
            return Objects.hash(connectionId, owner, error);
        }

        @Override
        public String toString() {
            return ("ConnectionEvent.ConnectionFailed{" + "connectionId=" + connectionId + ", owner=" + owner
                    + ", error='" + error + '\'' + "}");
        }
    }

    /**
     * Connection state changed.
     */
    public static class StateChanged extends ConnectionEvent {

        private final ConnectionId connectionId;
        private final ConnectionOwner owner;
        private final ConnectionInfo newState;

        public StateChanged(ConnectionId connectionId, ConnectionOwner owner, ConnectionInfo newState) {
            this.connectionId = Objects.requireNonNull(connectionId, "connectionId cannot be null");
            this.owner = Objects.requireNonNull(owner, "owner cannot be null");
            this.newState = Objects.requireNonNull(newState, "newState cannot be null");
        }

        @Override
        public ConnectionId getConnectionId() {
            return connectionId;
        }

        @Override
        public ConnectionOwner getOwner() {
            return owner;
        }

        public ConnectionInfo getNewState() {
            return newState;
        }

        @Override
        public boolean equals(Object obj) {
            if (this == obj)
                return true;
            if (obj == null || getClass() != obj.getClass())
                return false;
            StateChanged that = (StateChanged) obj;
            return (Objects.equals(connectionId, that.connectionId) && Objects.equals(owner, that.owner)
                    && Objects.equals(newState, that.newState));
        }

        @Override
        public int hashCode() {
            return Objects.hash(connectionId, owner, newState);
        }

        @Override
        public String toString() {
            return ("ConnectionEvent.StateChanged{" + "connectionId=" + connectionId + ", owner=" + owner
                    + ", newState=" + newState + "}");
        }
    }
}
