package org.automerge.repo.websocket;

import java.net.URI;
import java.util.Collections;
import java.util.LinkedHashMap;
import java.util.Map;
import javax.net.ssl.SSLContext;

/**
 * Configuration for {@link WebSocketDialer} (a {@link org.automerge.repo.Dialer} implementation).
 *
 * <p>
 * Use the builder to construct instances:
 *
 * <pre>{@code
 * WebSocketClientTransportConfig config = WebSocketClientTransportConfig
 *     .builder(URI.create("wss://sync.example.com"))
 *     .header("Authorization", "Bearer token")
 *     .connectTimeoutMs(5000)
 *     .sslContext(mySSLContext)
 *     .build();
 * }</pre>
 */
public class WebSocketClientTransportConfig {

    private final URI serverUri;
    private final int connectTimeoutMs;
    private final Map<String, String> headers;
    private final SSLContext sslContext;

    private WebSocketClientTransportConfig(Builder builder) {
        this.serverUri = builder.serverUri;
        this.connectTimeoutMs = builder.connectTimeoutMs;
        this.headers = Collections.unmodifiableMap(new LinkedHashMap<>(builder.headers));
        this.sslContext = builder.sslContext;
    }

    public URI getServerUri() {
        return serverUri;
    }

    public int getConnectTimeoutMs() {
        return connectTimeoutMs;
    }

    public Map<String, String> getHeaders() {
        return headers;
    }

    public SSLContext getSslContext() {
        return sslContext;
    }

    /**
     * Creates a new builder with the given server URI.
     *
     * @param serverUri
     *            the WebSocket server URI ({@code ws://} or {@code wss://})
     * @return a new builder
     */
    public static Builder builder(URI serverUri) {
        if (serverUri == null) {
            throw new IllegalArgumentException("serverUri must not be null");
        }
        return new Builder(serverUri);
    }

    public static class Builder {

        private final URI serverUri;
        private int connectTimeoutMs = 10000;
        private final Map<String, String> headers = new LinkedHashMap<>();
        private SSLContext sslContext;

        private Builder(URI serverUri) {
            this.serverUri = serverUri;
        }

        /**
         * Sets the WebSocket handshake timeout in milliseconds.
         *
         * @param connectTimeoutMs
         *            timeout in milliseconds (default 10000)
         * @return this builder
         */
        public Builder connectTimeoutMs(int connectTimeoutMs) {
            this.connectTimeoutMs = connectTimeoutMs;
            return this;
        }

        /**
         * Adds a custom HTTP header for the WebSocket upgrade request.
         *
         * @param name
         *            header name
         * @param value
         *            header value
         * @return this builder
         */
        public Builder header(String name, String value) {
            this.headers.put(name, value);
            return this;
        }

        /**
         * Sets the SSL context for {@code wss://} connections. If null and the URI
         * scheme is {@code wss://}, the JVM default SSL context is used.
         *
         * @param sslContext
         *            the SSL context, or null for JVM default
         * @return this builder
         */
        public Builder sslContext(SSLContext sslContext) {
            this.sslContext = sslContext;
            return this;
        }

        /**
         * Builds the configuration.
         *
         * @return the built config
         */
        public WebSocketClientTransportConfig build() {
            return new WebSocketClientTransportConfig(this);
        }
    }
}
