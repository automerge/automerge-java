package org.automerge.repo;

import java.util.Queue;
import java.util.concurrent.CompletableFuture;
import java.util.concurrent.ConcurrentLinkedQueue;
import java.util.concurrent.atomic.AtomicBoolean;
import java.util.function.Consumer;

/**
 * A bidirectional message transport to a remote peer.
 *
 * Transports represent a bidirectional message oriented transport to a remote
 * peer. The intended use case is for integrations with specific networking
 * libraries to be implemented by creating a Transport and then returning it
 * from
 * {@link Dialer#connect()} or passing it to
 * {@link AcceptorHandle#accept(Transport)}.
 *
 * Networking event sources should call {@link #onMessage} for received data and
 * {@link #onClose} for connection teardown. The constructor arguments are a
 * {@link Sender} and a closer {@link Runnable} which the runtime calls to send
 * data and close the transport, respectively. Messages which are passed to
 * `onMessage` before the transport has been connected to the {@link Repo} will
 * be buffered and delivered once the transport is connected.
 *
 * Usage:
 *
 * <pre>{@code
 * Transport transport = new Transport(
 *         data -> ws.send(data),
 *         () -> ws.close());
 *
 * // event source pushes into the transport:
 * transport.onMessage(data);
 * transport.onClose();
 * }</pre>
 */
public class Transport implements AutoCloseable {

    /**
     * Sends a message to the remote peer over the underlying connection.
     */
    @FunctionalInterface
    public interface Sender {
        void send(byte[] data) throws Exception;
    }

    private final Sender sender;
    private final Runnable closer;
    private final Object lock = new Object();
    private final Queue<byte[]> buffer = new ConcurrentLinkedQueue<>();
    private final AtomicBoolean onCloseDelivered = new AtomicBoolean(false);

    // Guarded by lock
    private boolean connected = false;
    private boolean closed = false;
    private Consumer<byte[]> onMessageCb;
    private Runnable onCloseCb;

    /**
     * Creates a Transport with the given sender and closer.
     *
     * @param sender
     *               sends bytes to the remote peer. Called by the runtime when a
     *               message is ready to send.
     * @param closer
     *               closes the underlying connection. Called by the runtime when
     *               the connection is closed.
     */
    public Transport(Sender sender, Runnable closer) {
        this.sender = sender;
        this.closer = closer;
    }

    /**
     * Called by the event source when a message arrives from the remote peer. If
     * the runtime has connected, the message is delivered immediately; otherwise
     * it is buffered. Messages arriving after close are dropped.
     *
     * @param data
     *             the message bytes
     */
    public void onMessage(byte[] data) {
        synchronized (lock) {
            if (closed) {
                return;
            }
            if (connected) {
                onMessageCb.accept(data);
            } else {
                buffer.add(data);
            }
        }
    }

    /**
     * Called by the event source when the remote peer closes the connection. If
     * the runtime has connected, the onClose callback fires immediately;
     * otherwise the closed flag is set and the callback will fire when the
     * runtime connects.
     */
    public void onClose() {
        synchronized (lock) {
            closed = true;
            if (connected) {
                fireOnClose();
            }
        }
    }

    /**
     * Called by the runtime to start consuming messages. Flushes any buffered
     * messages and switches to direct delivery.
     */
    public void connect(Consumer<byte[]> onMessage, Runnable onClose) {
        synchronized (lock) {
            this.onMessageCb = onMessage;
            this.onCloseCb = onClose;
            this.connected = true;
            byte[] msg;
            while ((msg = buffer.poll()) != null) {
                onMessage.accept(msg);
            }
            if (closed) {
                fireOnClose();
            }
        }
    }

    /**
     * Send a message to the remote peer.
     */
    public CompletableFuture<Void> send(byte[] data) {
        if (closed) {
            CompletableFuture<Void> future = new CompletableFuture<>();
            future.completeExceptionally(new IllegalStateException("Transport is closed"));
            return future;
        }
        try {
            sender.send(data);
            return CompletableFuture.completedFuture(null);
        } catch (Exception e) {
            CompletableFuture<Void> future = new CompletableFuture<>();
            future.completeExceptionally(e);
            return future;
        }
    }

    /**
     * Close the transport and the underlying connection.
     */
    @Override
    public void close() {
        synchronized (lock) {
            closed = true;
            if (connected) {
                fireOnClose();
            }
        }
        try {
            closer.run();
        } catch (Exception e) {
            // ignored — closer may throw if already closed
        }
    }

    private void fireOnClose() {
        if (onCloseDelivered.compareAndSet(false, true)) {
            Runnable cb = onCloseCb;
            if (cb != null) {
                cb.run();
            }
        }
    }
}
