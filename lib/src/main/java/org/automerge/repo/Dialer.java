package org.automerge.repo;

import java.util.concurrent.CompletableFuture;

/**
 * Creates transports to a remote peer. Called by the runtime on initial
 * connection and on each reconnection attempt after backoff.
 *
 * <p>
 * {@link #connect()} should return a future that completes when the connection
 * is established.
 */
public interface Dialer {
    /**
     * A URL identifying the remote endpoint. Used for logging and debugging
     *
     * @return the URL of the remote endpoint
     */
    String getUrl();

    /**
     * Connect to the remote endpoint and return a future that completes with the
     * transport when the connection is established.
     *
     * @return a future that completes with the transport when the connection is
     *         established
     */
    CompletableFuture<Transport> connect();
}
