package org.automerge;

import java.util.List;
import java.util.Optional;
import org.junit.jupiter.api.Assertions;
import org.junit.jupiter.api.Test;

public final class TestTransactionAt {

    @Test
    public void testTransactionAt() {
        Document doc = new Document();
        ChangeHash firstHeads;

        try (Transaction tx = doc.startTransaction()) {
            tx.set(ObjectId.ROOT, "key", 1.23);
            firstHeads = tx.commit().get();
        }

        try (Transaction tx = doc.startTransaction()) {
            tx.set(ObjectId.ROOT, "key", 4.56);
            tx.commit().get();
        }

        PatchLog patchLog = new PatchLog();
        try (Transaction tx = doc.startTransactionAt(patchLog, new ChangeHash[]{firstHeads})) {
            Optional<AmValue> resulut = tx.get(ObjectId.ROOT, "key");
            Assertions.assertEquals(1.23, ((AmValue.F64) resulut.get()).getValue());
            tx.set(ObjectId.ROOT, "key", 7.89);
            tx.commit();
        }

        List<Patch> patches = doc.makePatches(patchLog);

        Assertions.assertEquals(patches.size(), 1);
        Assertions.assertEquals(patches.get(0).getObj(), ObjectId.ROOT);
        PatchAction.PutMap action = (PatchAction.PutMap) patches.get(0).getAction();
        Assertions.assertEquals(action.getKey(), "key");
        AmValue.F64 value = ((AmValue.F64) action.getValue());
        Assertions.assertEquals(value.getValue(), 7.89);
    }
}
