package org.automerge.repo;

import java.util.Objects;

/**
 * Represents the source of a connection.
 */
public abstract class ConnectionOwner {

    /**
     * A connection that was created by a {@link Dialer}.
     */
    public static class DialerOwner extends ConnectionOwner {
        private final DialerId dialerId;

        public DialerOwner(DialerId dialerId) {
            this.dialerId = Objects.requireNonNull(dialerId);
        }

        public DialerId getDialerId() {
            return dialerId;
        }

        @Override
        public boolean equals(Object obj) {
            if (this == obj)
                return true;
            if (obj == null || getClass() != obj.getClass())
                return false;
            DialerOwner that = (DialerOwner) obj;
            return Objects.equals(dialerId, that.dialerId);
        }

        @Override
        public int hashCode() {
            return Objects.hash(dialerId);
        }

        @Override
        public String toString() {
            return "ConnectionOwner.DialerOwner{dialerId=" + dialerId + "}";
        }
    }

    /**
     * A connection that was created by a {@link AcceptorHandle}.
     */
    public static class ListenerOwner extends ConnectionOwner {
        private final ListenerId listenerId;

        public ListenerOwner(ListenerId listenerId) {
            this.listenerId = Objects.requireNonNull(listenerId);
        }

        public ListenerId getListenerId() {
            return listenerId;
        }

        @Override
        public boolean equals(Object obj) {
            if (this == obj)
                return true;
            if (obj == null || getClass() != obj.getClass())
                return false;
            ListenerOwner that = (ListenerOwner) obj;
            return Objects.equals(listenerId, that.listenerId);
        }

        @Override
        public int hashCode() {
            return Objects.hash(listenerId);
        }

        @Override
        public String toString() {
            return "ConnectionOwner.ListenerOwner{listenerId=" + listenerId + "}";
        }
    }
}
