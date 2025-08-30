package org.automerge;

import java.util.Objects;

/**
 * The state of a network connection.
 * This is an abstract base class with concrete subclasses for each state.
 */
public abstract class ConnectionState {

    /**
     * We're still exchanging peer IDs.
     */
    public static class Handshaking extends ConnectionState {

        Handshaking() {
            // Empty state - just handshaking
        }

        @Override
        public boolean equals(Object obj) {
            return this == obj || (obj != null && getClass() == obj.getClass());
        }

        @Override
        public int hashCode() {
            return getClass().hashCode();
        }

        @Override
        public String toString() {
            return "ConnectionState.Handshaking{}";
        }
    }

    /**
     * We have exchanged peer IDs and we're now synchronizing documents.
     */
    public static class Connected extends ConnectionState {

        private final PeerId theirPeerId;

        Connected(PeerId theirPeerId) {
            this.theirPeerId = Objects.requireNonNull(
                theirPeerId,
                "theirPeerId cannot be null"
            );
        }

        /**
         * Gets the peer ID of the connected peer.
         * @return The peer ID
         */
        public PeerId getTheirPeerId() {
            return theirPeerId;
        }

        @Override
        public boolean equals(Object obj) {
            if (this == obj) return true;
            if (obj == null || getClass() != obj.getClass()) return false;
            Connected connected = (Connected) obj;
            return Objects.equals(theirPeerId, connected.theirPeerId);
        }

        @Override
        public int hashCode() {
            return Objects.hash(theirPeerId);
        }

        @Override
        public String toString() {
            return (
                "ConnectionState.Connected{" +
                "theirPeerId=" +
                theirPeerId +
                "}"
            );
        }
    }
}
