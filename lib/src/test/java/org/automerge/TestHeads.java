package org.automerge;

import org.junit.jupiter.api.Assertions;
import org.junit.jupiter.api.Test;

public class TestHeads {
    @Test
    public void testGetHeads() {
        Document doc = new Document();
        Transaction tx = doc.startTransaction();
        tx.set(ObjectId.ROOT, "key", "value");
        Assertions.assertEquals(tx.getHeads().length, 0);
        Assertions.assertEquals(doc.getHeads().length, 0);
        tx.commit();
        Assertions.assertEquals(doc.getHeads().length, 1);
        Assertions.assertEquals(doc.getHeads()[0].getBytes().length, 32);
    }
}
