package org.automerge.repo.websocket;

import java.net.InetSocketAddress;
import java.nio.ByteBuffer;
import java.util.concurrent.ConcurrentHashMap;
import org.automerge.repo.AcceptorHandle;
import org.automerge.repo.Transport;
import org.java_websocket.WebSocket;
import org.java_websocket.handshake.ClientHandshake;
import org.java_websocket.server.WebSocketServer;
import org.slf4j.Logger;
import org.slf4j.LoggerFactory;

/**
 * WebSocket server that accepts incoming connections and bridges each to the
 * {@link Transport} class.
 *
 * <p>
 * Usage with the Dialer/Listener API:
 *
 * <pre>{@code
 * AcceptorHandle acceptor = repo.makeAcceptor("ws://0.0.0.0:8080");
 * WebSocketServerTransport server = new WebSocketServerTransport(
 *     new InetSocketAddress(8080),
 *     acceptor
 * );
 * server.start();
 *
 * // Later...
 * server.stop();
 * }</pre>
 */
public class WebSocketServerTransport {

    private static final Logger logger = LoggerFactory.getLogger(
        WebSocketServerTransport.class
    );

    private final WebSocketServer server;
    private final ConcurrentHashMap<WebSocket, Transport> connections =
        new ConcurrentHashMap<>();
    private final AcceptorHandle acceptorHandle;

    /**
     * Creates a WebSocket server transport.
     *
     * @param address
     *            the address to bind to (use port 0 for a random available port)
     * @param acceptorHandle
     *            the {@link AcceptorHandle} to which incoming connections are
     *            handed off via {@code acceptorHandle.accept(transport)}
     */
    public WebSocketServerTransport(
        InetSocketAddress address,
        AcceptorHandle acceptorHandle
    ) {
        this.acceptorHandle = acceptorHandle;
        this.server = new WebSocketServer(address) {
            @Override
            public void onOpen(WebSocket conn, ClientHandshake handshake) {
                Transport transport = new Transport(
                    data -> conn.send(data),
                    () -> conn.close()
                );
                connections.put(conn, transport);
                acceptorHandle.accept(transport);
            }

            @Override
            public void onClose(
                WebSocket conn,
                int code,
                String reason,
                boolean remote
            ) {
                Transport transport = connections.remove(conn);
                if (transport != null) {
                    transport.onClose();
                }
            }

            @Override
            public void onMessage(WebSocket conn, String message) {
                logger.warn(
                    "Received unexpected text message from {}, ignoring",
                    conn.getRemoteSocketAddress()
                );
            }

            @Override
            public void onMessage(WebSocket conn, ByteBuffer message) {
                Transport transport = connections.get(conn);
                if (transport != null) {
                    byte[] data = new byte[message.remaining()];
                    message.get(data);
                    transport.onMessage(data);
                }
            }

            @Override
            public void onError(WebSocket conn, Exception ex) {
                if (conn != null) {
                    logger.debug(
                        "WebSocket server error on connection {}",
                        conn.getRemoteSocketAddress(),
                        ex
                    );
                } else {
                    logger.debug("WebSocket server error", ex);
                }
            }

            @Override
            public void onStart() {
                logger.debug("WebSocket server started on port {}", getPort());
            }
        };
        this.server.setReuseAddr(true);
    }

    /**
     * Starts the WebSocket server.
     */
    public void start() {
        server.start();
    }

    /**
     * Stops the WebSocket server and closes all connections.
     *
     * @throws InterruptedException
     *             if interrupted while waiting for the server to stop
     */
    public void stop() throws InterruptedException {
        server.stop();
        acceptorHandle.close();
    }

    /**
     * Returns the port the server is listening on. Useful when the server was
     * started with port 0 to get the actual assigned port.
     *
     * @return the server port
     */
    public int getPort() {
        return server.getPort();
    }
}
