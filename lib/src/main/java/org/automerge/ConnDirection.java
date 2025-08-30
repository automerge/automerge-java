package org.automerge;

/**
 * Represents the direction of a connection in the samod network protocol.
 */
public enum ConnDirection {
    /**
     * An outgoing connection (initiated by this peer).
     */
    OUTGOING,

    /**
     * An incoming connection (initiated by a remote peer).
     */
    INCOMING,
}
