package org.automerge.repo.websocket;

import java.net.URI;
import java.nio.ByteBuffer;
import java.util.Map;
import java.util.concurrent.CompletableFuture;
import javax.net.ssl.SSLContext;
import javax.net.ssl.SSLSocketFactory;
import org.automerge.repo.Dialer;
import org.automerge.repo.Transport;
import org.java_websocket.client.WebSocketClient;
import org.java_websocket.handshake.ServerHandshake;
import org.slf4j.Logger;
import org.slf4j.LoggerFactory;

/**
 * Dialer that creates WebSocket client transports.
 *
 * <p>
 * Usage with the Dialer/Listener API:
 *
 * <pre>{@code
 * // Simple - just a URL
 * DialerHandle handle = repo.dial(
 *     new WebSocketDialer(URI.create("ws://sync.example.com")));
 *
 * // With config
 * WebSocketClientTransportConfig config = WebSocketClientTransportConfig
 *     .builder(URI.create("wss://sync.example.com"))
 *     .header("Authorization", "Bearer token")
 *     .connectTimeoutMs(5000)
 *     .build();
 * DialerHandle handle = repo.dial(
 *     new WebSocketDialer(config));
 * }</pre>
 */
public class WebSocketDialer implements Dialer {

    private static final Logger logger = LoggerFactory.getLogger(
        WebSocketDialer.class
    );

    private final WebSocketClientTransportConfig config;

    /**
     * Creates a dialer with default configuration for the given URI.
     *
     * @param uri
     *            the WebSocket server URI ({@code ws://} or {@code wss://})
     */
    public WebSocketDialer(URI uri) {
        this(WebSocketClientTransportConfig.builder(uri).build());
    }

    /**
     * Creates a dialer with the given configuration.
     *
     * @param config
     *            the client transport configuration
     */
    public WebSocketDialer(WebSocketClientTransportConfig config) {
        if (config == null) {
            throw new IllegalArgumentException("config must not be null");
        }
        this.config = config;
    }

    @Override
    public String getUrl() {
        return config.getServerUri().toString();
    }

    /**
     * Initiates a non-blocking connection to the WebSocket server. The returned
     * future completes when the WebSocket handshake succeeds, or fails if the
     * connection cannot be established.
     */
    @Override
    public CompletableFuture<Transport> connect() {
        Map<String, String> headers = config.getHeaders();
        CompletableFuture<Transport> result = new CompletableFuture<>();

        // Both the transport and ws client reference each other:
        // - transport.send()/close() delegate to wsClient
        // - wsClient callbacks push into transport.onMessage()/onClose()
        // We break the circular dependency with a final array.
        final WebSocketClient[] clientHolder = new WebSocketClient[1];

        Transport transport = new Transport(
            data -> clientHolder[0].send(data),
            () -> clientHolder[0].close()
        );

        WebSocketClient wsClient = new WebSocketClient(
            config.getServerUri(),
            headers
        ) {
            @Override
            public void onOpen(ServerHandshake handshake) {
                result.complete(transport);
            }

            @Override
            public void onMessage(String message) {
                logger.warn("Received unexpected text message, ignoring");
            }

            @Override
            public void onMessage(ByteBuffer bytes) {
                byte[] data = new byte[bytes.remaining()];
                bytes.get(data);
                transport.onMessage(data);
            }

            @Override
            public void onClose(int code, String reason, boolean remote) {
                logger.debug(
                    "WebSocket client onClose: code={}, reason={}, remote={}",
                    code,
                    reason,
                    remote
                );
                transport.onClose();
                result.completeExceptionally(
                    new RuntimeException(
                        "WebSocket connection closed (code=" + code + ")"
                    )
                );
            }

            @Override
            public void onError(Exception ex) {
                logger.debug("WebSocket client error", ex);
                result.completeExceptionally(ex);
            }
        };
        clientHolder[0] = wsClient;

        wsClient.setConnectionLostTimeout(0);

        // Configure SSL if needed
        String scheme = config.getServerUri().getScheme();
        if ("wss".equalsIgnoreCase(scheme)) {
            try {
                SSLContext sslContext = config.getSslContext();
                if (sslContext == null) {
                    sslContext = SSLContext.getDefault();
                }
                SSLSocketFactory factory = sslContext.getSocketFactory();
                wsClient.setSocketFactory(factory);
            } catch (Exception e) {
                CompletableFuture<Transport> failed = new CompletableFuture<>();
                failed.completeExceptionally(
                    new RuntimeException("Failed to configure SSL", e)
                );
                return failed;
            }
        }

        wsClient.connect();
        return result;
    }
}
