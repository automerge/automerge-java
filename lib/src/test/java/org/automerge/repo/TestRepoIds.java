package org.automerge.repo;

import static org.junit.jupiter.api.Assertions.*;

import org.junit.jupiter.api.Test;

public class TestRepoIds {
    @Test
    public void peerIdGenerateProducesDistinctValues() {
        PeerId a = PeerId.generate();
        PeerId b = PeerId.generate();
        assertNotNull(a.getValue());
        assertNotEquals(a, b);
    }

    @Test
    public void peerIdFromStringRoundTripsValue() {
        PeerId a = PeerId.fromString("alice");
        assertEquals("alice", a.getValue());
    }
}
