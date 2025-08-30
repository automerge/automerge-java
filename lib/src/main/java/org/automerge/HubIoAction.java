package org.automerge;

import java.util.Arrays;
import java.util.Objects;

/**
 * Represents IO actions that the Hub can request.
 * This is an abstract base class with concrete subclasses for each action type.
 */
public abstract class HubIoAction {

    /**
     * Send a message to a specific connection.
     */
    public static class Send extends HubIoAction {
        private final ConnectionId connectionId;
        private final byte[] message;

        Send(ConnectionId connectionId, byte[] message) {
            this.connectionId = Objects.requireNonNull(connectionId, "connectionId cannot be null");
            this.message = Objects.requireNonNull(message, "message cannot be null");
        }

        public ConnectionId getConnectionId() {
            return connectionId;
        }

        public byte[] getMessage() {
            return message.clone(); // Defensive copy
        }

        @Override
        public boolean equals(Object obj) {
            if (this == obj) return true;
            if (obj == null || getClass() != obj.getClass()) return false;
            Send send = (Send) obj;
            return Objects.equals(connectionId, send.connectionId) &&
                   Arrays.equals(message, send.message);
        }

        @Override
        public int hashCode() {
            return Objects.hash(connectionId, Arrays.hashCode(message));
        }

        @Override
        public String toString() {
            return "HubIoAction.Send{connectionId=" + connectionId +
                   ", messageLength=" + message.length + "}";
        }
    }

    /**
     * Disconnect a specific connection.
     */
    public static class Disconnect extends HubIoAction {
        private final ConnectionId connectionId;

        Disconnect(ConnectionId connectionId) {
            this.connectionId = Objects.requireNonNull(connectionId, "connectionId cannot be null");
        }

        public ConnectionId getConnectionId() {
            return connectionId;
        }

        @Override
        public boolean equals(Object obj) {
            if (this == obj) return true;
            if (obj == null || getClass() != obj.getClass()) return false;
            Disconnect that = (Disconnect) obj;
            return Objects.equals(connectionId, that.connectionId);
        }

        @Override
        public int hashCode() {
            return Objects.hash(connectionId);
        }

        @Override
        public String toString() {
            return "HubIoAction.Disconnect{connectionId=" + connectionId + "}";
        }
    }
}
