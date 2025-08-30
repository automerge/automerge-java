package org.automerge;

import java.util.Objects;

/**
 * Events that can occur on network connections.
 * This is an abstract base class with concrete subclasses for each event type.
 */
public abstract class ConnectionEvent {

    /**
     * Gets the connection ID associated with this event.
     * @return The connection ID
     */
    public abstract ConnectionId getConnectionId();

    /**
     * Handshake completed successfully with a peer.
     *
     * This event is emitted when the connection handshake process
     * finishes successfully and the connection transitions to the
     * established state. After this event, the connection is ready
     * for document synchronization.
     */
    public static class HandshakeCompleted extends ConnectionEvent {
        private final ConnectionId connectionId;
        private final PeerInfo peerInfo;

        public HandshakeCompleted(ConnectionId connectionId, PeerInfo peerInfo) {
            this.connectionId = Objects.requireNonNull(connectionId, "connectionId cannot be null");
            this.peerInfo = Objects.requireNonNull(peerInfo, "peerInfo cannot be null");
        }

        @Override
        public ConnectionId getConnectionId() {
            return connectionId;
        }

        /**
         * Gets information about the peer that completed the handshake.
         * @return The peer information
         */
        public PeerInfo getPeerInfo() {
            return peerInfo;
        }

        @Override
        public boolean equals(Object obj) {
            if (this == obj) return true;
            if (obj == null || getClass() != obj.getClass()) return false;
            HandshakeCompleted that = (HandshakeCompleted) obj;
            return Objects.equals(connectionId, that.connectionId) &&
                   Objects.equals(peerInfo, that.peerInfo);
        }

        @Override
        public int hashCode() {
            return Objects.hash(connectionId, peerInfo);
        }

        @Override
        public String toString() {
            return "ConnectionEvent.HandshakeCompleted{" +
                   "connectionId=" + connectionId +
                   ", peerInfo=" + peerInfo +
                   "}";
        }
    }

    /**
     * Connection failed or was disconnected.
     *
     * This event is emitted when a connection fails or when a connection is
     * explicitly disconnected. This can happen due to network errors, protocol
     * violations, or explicit disconnection.
     */
    public static class ConnectionFailed extends ConnectionEvent {
        private final ConnectionId connectionId;
        private final String error;

        public ConnectionFailed(ConnectionId connectionId, String error) {
            this.connectionId = Objects.requireNonNull(connectionId, "connectionId cannot be null");
            this.error = Objects.requireNonNull(error, "error cannot be null");
        }

        @Override
        public ConnectionId getConnectionId() {
            return connectionId;
        }

        /**
         * Gets the error message describing why the connection failed.
         * @return The error message
         */
        public String getError() {
            return error;
        }

        @Override
        public boolean equals(Object obj) {
            if (this == obj) return true;
            if (obj == null || getClass() != obj.getClass()) return false;
            ConnectionFailed that = (ConnectionFailed) obj;
            return Objects.equals(connectionId, that.connectionId) &&
                   Objects.equals(error, that.error);
        }

        @Override
        public int hashCode() {
            return Objects.hash(connectionId, error);
        }

        @Override
        public String toString() {
            return "ConnectionEvent.ConnectionFailed{" +
                   "connectionId=" + connectionId +
                   ", error='" + error + '\'' +
                   "}";
        }
    }

    /**
     * Connection state changed.
     *
     * This event is emitted whenever some part of the connection state changes.
     */
    public static class StateChanged extends ConnectionEvent {
        private final ConnectionId connectionId;
        private final ConnectionInfo newState;

        public StateChanged(ConnectionId connectionId, ConnectionInfo newState) {
            this.connectionId = Objects.requireNonNull(connectionId, "connectionId cannot be null");
            this.newState = Objects.requireNonNull(newState, "newState cannot be null");
        }

        @Override
        public ConnectionId getConnectionId() {
            return connectionId;
        }

        /**
         * Gets the new connection state.
         * @return The new connection state information
         */
        public ConnectionInfo getNewState() {
            return newState;
        }

        @Override
        public boolean equals(Object obj) {
            if (this == obj) return true;
            if (obj == null || getClass() != obj.getClass()) return false;
            StateChanged that = (StateChanged) obj;
            return Objects.equals(connectionId, that.connectionId) &&
                   Objects.equals(newState, that.newState);
        }

        @Override
        public int hashCode() {
            return Objects.hash(connectionId, newState);
        }

        @Override
        public String toString() {
            return "ConnectionEvent.StateChanged{" +
                   "connectionId=" + connectionId +
                   ", newState=" + newState +
                   "}";
        }
    }
}
