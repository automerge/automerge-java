package org.automerge;

import java.util.List;
import org.junit.jupiter.api.Assertions;
import org.junit.jupiter.api.Test;

public final class TestDiff {

    @Test
    public void testTransactionAt() {
        Document doc = new Document();
        ChangeHash firstHeads;
        ChangeHash secondHeads;

        try (Transaction tx = doc.startTransaction()) {
            tx.set(ObjectId.ROOT, "key", 1.23);
            firstHeads = tx.commit().get();
        }

        try (Transaction tx = doc.startTransaction()) {
            tx.set(ObjectId.ROOT, "key", 4.56);
            secondHeads = tx.commit().get();
        }

        List<Patch> patches = doc.diff(new ChangeHash[]{firstHeads}, new ChangeHash[]{secondHeads});

        Assertions.assertEquals(patches.size(), 1);
        Assertions.assertEquals(patches.get(0).getObj(), ObjectId.ROOT);
        PatchAction.PutMap action = (PatchAction.PutMap) patches.get(0).getAction();
        Assertions.assertEquals(action.getKey(), "key");
        AmValue.F64 value = ((AmValue.F64) action.getValue());
        Assertions.assertEquals(value.getValue(), 4.56);
    }
}
