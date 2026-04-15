package org.automerge.repo;

import java.util.Objects;
import java.util.concurrent.CompletableFuture;
import java.util.concurrent.atomic.AtomicBoolean;

/**
 * A handle for accepting new connections on. Created by {@link Repo#makeAcceptor}.
 */
public class AcceptorHandle implements AutoCloseable {
    private final ListenerId listenerId;
    private final RepoRuntime runtime;
    private final AtomicBoolean closed;

    AcceptorHandle(ListenerId listenerId, RepoRuntime runtime) {
        this.listenerId = Objects.requireNonNull(listenerId);
        this.runtime = Objects.requireNonNull(runtime);
        this.closed = new AtomicBoolean(false);
    }

    public ListenerId getId() {
        return listenerId;
    }

    /**
     * Accept a new connection on this acceptor. The returned future will
     * complete when the connection ends
     */
    public CompletableFuture<ConnectionId> accept(Transport transport) {
        if (closed.get()) {
            CompletableFuture<ConnectionId> future = new CompletableFuture<>();
            future.completeExceptionally(new IllegalStateException("Acceptor is closed"));
            return future;
        }
        return runtime.acceptListenerConnection(listenerId, transport);
    }

    /**
     * Close this acceptor, removing it from the repo and closing all connections accepted by this acceptor
     */
    @Override
    public void close() {
        if (closed.compareAndSet(false, true)) {
            runtime.removeListener(listenerId);
        }
    }
}
