package org.automerge.repo;

import static org.junit.jupiter.api.Assertions.*;

import org.junit.jupiter.api.Test;

public class TestDocumentId {
    @Test
    public void generateProducesDistinctIds() {
        DocumentId a = DocumentId.generate();
        DocumentId b = DocumentId.generate();
        assertNotEquals(a, b);
    }

    @Test
    public void bytesRoundTrip() {
        DocumentId original = DocumentId.generate();
        byte[] bytes = original.getBytes();
        DocumentId restored = DocumentId.fromBytes(bytes);
        assertEquals(original, restored);
        assertArrayEquals(bytes, restored.getBytes());
    }

    @Test
    public void getBytesIsDefensiveCopy() {
        DocumentId id = DocumentId.generate();
        byte[] a = id.getBytes();
        byte[] b = id.getBytes();
        assertNotSame(a, b);
        a[0] = (byte) ~a[0];
        assertNotEquals(a[0], b[0]);
    }
}
