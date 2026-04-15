package org.automerge.repo;

/**
 * Represents the result types for Hub IO operations in the samod protocol.
 */
abstract class HubIoResult {

    /**
     * Indicates a successful send operation.
     */
    static final class Send extends HubIoResult {

        /** Singleton instance for convenience in Java code. */
        public static final Send INSTANCE = new Send();

        @Override
        public String toString() {
            return "HubIoResult.Send";
        }
    }

    /**
     * Indicates a disconnect operation.
     */
    static final class Disconnect extends HubIoResult {

        /** Singleton instance for convenience in Java code. */
        public static final Disconnect INSTANCE = new Disconnect();

        @Override
        public String toString() {
            return "HubIoResult.Disconnect";
        }
    }

    /** Convenience constant for send result. */
    static final Send SEND = Send.INSTANCE;

    /** Convenience constant for disconnect result. */
    static final Disconnect DISCONNECT = Disconnect.INSTANCE;
}
