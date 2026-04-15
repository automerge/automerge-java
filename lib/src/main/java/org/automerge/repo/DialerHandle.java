package org.automerge.repo;

import java.util.Objects;
import java.util.Optional;
import java.util.concurrent.CompletableFuture;
import java.util.concurrent.ConcurrentLinkedQueue;
import java.util.concurrent.atomic.AtomicBoolean;

public class DialerHandle implements AutoCloseable {
    private final DialerId dialerId;
    private final RepoRuntime runtime;
    private final CompletableFuture<Void> maxRetriesFuture;
    private final AtomicBoolean closed;
    private final ConcurrentLinkedQueue<CompletableFuture<PeerId>> establishedWaiters;
    private volatile PeerId peerId;
    private volatile ConnectionId connectionId;
    private volatile boolean connected;

    DialerHandle(DialerId dialerId, RepoRuntime runtime) {
        this.dialerId = Objects.requireNonNull(dialerId);
        this.runtime = Objects.requireNonNull(runtime);
        this.maxRetriesFuture = new CompletableFuture<>();
        this.closed = new AtomicBoolean(false);
        this.establishedWaiters = new ConcurrentLinkedQueue<>();
        this.connected = false;
    }

    public DialerId getId() {
        return dialerId;
    }

    /**
     * Returns a future that completes when the dialer has an active connection.
     *
     * If the dialer is already connected, returns a completed future immediately.
     * Otherwise, the future completes on the next successful connection (including
     * reconnections after backoff).
     *
     * Each call returns a fresh future. Callers who want to observe every
     * reconnection should call this again after each completion.
     */
    public CompletableFuture<PeerId> onEstablished() {
        if (closed.get()) {
            CompletableFuture<PeerId> future = new CompletableFuture<>();
            future.completeExceptionally(new IllegalStateException("Dialer is closed"));
            return future;
        }
        if (connected) {
            return CompletableFuture.completedFuture(peerId);
        }
        CompletableFuture<PeerId> future = new CompletableFuture<>();
        establishedWaiters.add(future);
        // Re-check: connection may have been established between the check and add
        if (connected) {
            future.complete(peerId);
        }
        if (closed.get()) {
            future.completeExceptionally(new IllegalStateException("Dialer is closed"));
        }
        return future;
    }

    public Optional<PeerId> getPeerId() {
        return Optional.ofNullable(peerId);
    }

    public boolean isConnected() {
        return connected && !closed.get();
    }

    public CompletableFuture<Void> onMaxRetriesReached() {
        return maxRetriesFuture;
    }

    @Override
    public void close() {
        if (closed.compareAndSet(false, true)) {
            connected = false;
            runtime.removeDialer(dialerId);
            if (!maxRetriesFuture.isDone()) {
                maxRetriesFuture.completeExceptionally(
                        new RuntimeException("Dialer closed before max retries reached"));
            }
            // Fail all waiting futures
            CompletableFuture<PeerId> waiter;
            while ((waiter = establishedWaiters.poll()) != null) {
                waiter.completeExceptionally(new IllegalStateException("Dialer is closed"));
            }
        }
    }

    // Package-private methods called by RepoRuntime

    void notifyEstablished(PeerId peerId, ConnectionId connectionId) {
        this.peerId = peerId;
        this.connectionId = connectionId;
        this.connected = true;
        // Complete all waiting futures
        CompletableFuture<PeerId> waiter;
        while ((waiter = establishedWaiters.poll()) != null) {
            waiter.complete(peerId);
        }
    }

    void notifyDisconnected() {
        this.connected = false;
        this.connectionId = null;
    }

    void notifyMaxRetriesReached() {
        this.connected = false;
        if (!maxRetriesFuture.isDone()) {
            maxRetriesFuture.complete(null);
        }
    }

    ConnectionId getConnectionId() {
        return connectionId;
    }
}
