package org.automerge;

import java.util.Date;
import java.util.Optional;
import org.junit.jupiter.api.Assertions;
import org.junit.jupiter.api.BeforeEach;
import org.junit.jupiter.api.Test;

public final class TestInsert {
    private Document doc;
    private Transaction tx;
    private ObjectId list;

    public TestInsert() {
        super();
    }

    @BeforeEach
    public void setup() {
        doc = new Document();
        tx = doc.startTransaction();
        list = tx.set(ObjectId.ROOT, "key", ObjectType.LIST);
    }

    @Test
    public void testInsertDoubleInList() {
        tx.insert(list, 0, 1.23);
        Assertions.assertEquals(1.23, ((AmValue.F64) doc.get(list, 0).get()).getValue());
    }

    @Test
    public void testInsertStringInList() {
        tx.insert(list, 0, "something");
        Assertions.assertEquals("something", ((AmValue.Str) doc.get(list, 0).get()).getValue());
    }

    @Test
    public void testInsertIntInList() {
        tx.insert(list, 0, 10);
        Assertions.assertEquals(10, ((AmValue.Int) doc.get(list, 0).get()).getValue());
    }

    @Test
    public void testInsertUingInList() {
        tx.insert(list, 0, NewValue.uint(10));
        Assertions.assertEquals(10, ((AmValue.UInt) doc.get(list, 0).get()).getValue());
    }

    @Test
    public void testInsertBytesInList() {
        byte[] value = "somebytes".getBytes();
        tx.insert(list, 0, value);
        Optional<AmValue> result = doc.get(list, 0);
        Assertions.assertTrue(result.isPresent());
        Assertions.assertArrayEquals(((AmValue.Bytes) result.get()).getValue(), value);
    }

    @Test
    public void testInsertNullInList() {
        tx.insert(list, 0, NewValue.NULL);
        Assertions.assertInstanceOf(AmValue.Null.class, doc.get(list, 0).get());
    }

    @Test
    public void testInsertCounterInList() {
        tx.insert(list, 0, new Counter(10));
        Assertions.assertEquals(10, ((AmValue.Counter) doc.get(list, 0).get()).getValue());
    }

    @Test
    public void testInsertDateInList() {
        Date now = new Date();
        tx.insert(list, 0, now);
        Assertions.assertEquals(now, ((AmValue.Timestamp) doc.get(list, 0).get()).getValue());
    }

    @Test
    public void testBoolInList() {
        tx.insert(list, 0, false);
        tx.insert(list, 1, true);
        Assertions.assertEquals(false, ((AmValue.Bool) doc.get(list, 0).get()).getValue());
        Assertions.assertEquals(true, ((AmValue.Bool) doc.get(list, 1).get()).getValue());
    }

    @Test
    public void testInsertObjInList() {
        ObjectId listId = tx.insert(list, 0, ObjectType.LIST);
        ObjectId textId = tx.insert(list, 1, ObjectType.TEXT);
        ObjectId mapId = tx.insert(list, 2, ObjectType.MAP);
        Assertions.assertEquals(listId, ((AmValue.List) doc.get(list, 0).get()).getId());
        Assertions.assertEquals(textId, ((AmValue.Text) doc.get(list, 1).get()).getId());
        Assertions.assertEquals(mapId, ((AmValue.Map) doc.get(list, 2).get()).getId());
    }
}
