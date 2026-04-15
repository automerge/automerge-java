package org.automerge.repo.integration.helpers;

import java.util.concurrent.CompletableFuture;
import org.automerge.repo.AcceptorHandle;
import org.automerge.repo.Dialer;
import org.automerge.repo.Transport;

/**
 * Dialer that creates in-memory channel transports. When connect() is called,
 * it creates a linked pair - one end returned to the runtime, the other pushed
 * into an AcceptorHandle.
 */
public class ChannelDialer implements Dialer {

    private final AcceptorHandle acceptor;
    private final int id;

    public ChannelDialer(AcceptorHandle acceptor) {
        this.acceptor = acceptor;
        // generate a random id for this dialer
        this.id = (int) (Math.random() * Integer.MAX_VALUE);
    }

    @Override
    public String getUrl() {
        return "channel://" + id;
    }

    @Override
    public CompletableFuture<Transport> connect() {
        Transport[] pair = ChannelAdapter.createPair();
        acceptor.accept(pair[1]); // push server end to acceptor
        return CompletableFuture.completedFuture(pair[0]); // return client end
    }
}
