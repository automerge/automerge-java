package org.automerge.repo.integration.helpers;

import org.automerge.repo.Transport;

/**
 * In-memory transport factory for testing. Creates two linked transports where
 * messages sent on one arrive on the other via direct synchronous delivery.
 */
public class ChannelAdapter {

    private ChannelAdapter() {}

    /**
     * Creates a linked pair of transports. Sending on one delivers directly to
     * the other's onMessage; closing one delivers onClose to the other.
     */
    public static Transport[] createPair() {
        Transport[] pair = new Transport[2];
        pair[0] = new Transport(data -> pair[1].onMessage(data), () -> pair[1].onClose());
        pair[1] = new Transport(data -> pair[0].onMessage(data), () -> pair[0].onClose());
        return pair;
    }
}
