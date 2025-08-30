package org.automerge;

import java.util.Optional;
import org.junit.jupiter.api.Assertions;
import org.junit.jupiter.api.Test;

class TestFork {

    @Test
    public void testFork() {
        Document doc = new Document();
        try (Transaction tx = doc.startTransaction()) {
            tx.set(ObjectId.ROOT, "key", 1.23);
            tx.commit();
        }
        Document doc2 = doc.fork();
        try (Transaction tx = doc2.startTransaction()) {
            tx.set(ObjectId.ROOT, "key", 4.56);
            tx.commit();
        }
        Optional<AmValue> result = doc.get(ObjectId.ROOT, "key");
        Assertions.assertEquals(1.23, ((AmValue.F64) result.get()).getValue());
        Optional<AmValue> result2 = doc2.get(ObjectId.ROOT, "key");
        Assertions.assertEquals(4.56, ((AmValue.F64) result2.get()).getValue());
    }

    @Test
    public void testForkWithActor() {
        Document doc = new Document();
        try (Transaction tx = doc.startTransaction()) {
            tx.set(ObjectId.ROOT, "key", 1.23);
            tx.commit();
        }
        Document doc2 = doc.fork("actor2".getBytes());
        Optional<AmValue> result = doc.get(ObjectId.ROOT, "key");
        Assertions.assertEquals(1.23, ((AmValue.F64) result.get()).getValue());
        Assertions.assertArrayEquals("actor2".getBytes(), doc2.getActorId());
    }

    @Test
    public void testForkWhileTransactionInProgressThrows() {
        Document doc = new Document();
        Transaction tx = doc.startTransaction();
        Assertions.assertThrows(TransactionInProgress.class, () -> {
            doc.fork();
        });
    }

    @Test
    public void testForkWithActorWhileTransactionInProgressThrows() {
        Document doc = new Document();
        Transaction tx = doc.startTransaction();
        Assertions.assertThrows(TransactionInProgress.class, () -> {
            doc.fork("actor2".getBytes());
        });
    }

    @Test
    public void testForkAt() {
        Document doc = new Document();
        try (Transaction tx = doc.startTransaction()) {
            tx.set(ObjectId.ROOT, "key", 1.23);
            tx.commit();
        }
        ChangeHash[] heads = doc.getHeads();
        try (Transaction tx = doc.startTransaction()) {
            tx.set(ObjectId.ROOT, "key", 4.56);
            tx.commit();
        }
        Document doc2 = doc.fork(heads);
        Optional<AmValue> result = doc2.get(ObjectId.ROOT, "key");
        Assertions.assertEquals(1.23, ((AmValue.F64) result.get()).getValue());
    }

    @Test
    public void testForkAtWithActor() {
        Document doc = new Document();
        try (Transaction tx = doc.startTransaction()) {
            tx.set(ObjectId.ROOT, "key", 1.23);
            tx.commit();
        }
        ChangeHash[] heads = doc.getHeads();
        try (Transaction tx = doc.startTransaction()) {
            tx.set(ObjectId.ROOT, "key", 4.56);
            tx.commit();
        }
        Document doc2 = doc.fork(heads, "actor2".getBytes());
        Optional<AmValue> result = doc2.get(ObjectId.ROOT, "key");
        Assertions.assertEquals(1.23, ((AmValue.F64) result.get()).getValue());
        Assertions.assertArrayEquals("actor2".getBytes(), doc2.getActorId());
    }

    @Test
    public void testForkAtWhileTransactionInProgressThrows() {
        Document doc = new Document();
        Transaction tx = doc.startTransaction();
        Assertions.assertThrows(TransactionInProgress.class, () -> {
            doc.fork(new ChangeHash[]{});
        });
    }

    @Test
    public void testForkAtWithActorWhileTransactionInProgressThrows() {
        Document doc = new Document();
        Transaction tx = doc.startTransaction();
        Assertions.assertThrows(TransactionInProgress.class, () -> {
            doc.fork(new ChangeHash[]{}, "actor2".getBytes());
        });
    }
}
