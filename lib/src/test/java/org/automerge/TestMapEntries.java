package org.automerge;

import java.util.Date;
import java.util.HashMap;
import java.util.function.Function;
import org.junit.jupiter.api.Assertions;
import org.junit.jupiter.api.Test;

class TestMapEntries {
    private ObjectId map;
    private ObjectId list;
    private ObjectId text;
    private Date dateValue;

    @Test
    public void testMapEntries() {
        run(doc -> doc.startTransaction());
    }

    void run(Function<Document, Transaction> createTx) {
        Document doc = new Document();
        Transaction tx = createTx.apply(doc);
        insertMapEntries(tx);
        assertMapEntries(tx);
        assertMapEntries(doc);
        tx.commit();
        assertMapEntries(doc);

        ChangeHash[] heads = doc.getHeads();
        tx = createTx.apply(doc);
        for (String key : tx.keys(ObjectId.ROOT).get()) {
            tx.delete(ObjectId.ROOT, key);
        }
        tx.set(ObjectId.ROOT, "newkey", "newvalue");

        assertMapEntriesAfter(tx);
        assertMapEntriesAfter(doc);
        tx.commit();
        assertMapEntriesAfter(doc);

        tx = createTx.apply(doc);
        MapEntry[] txAt = tx.mapEntries(ObjectId.ROOT, heads).get();
        assertMapEntries(txAt);
        MapEntry[] docAtPreCommit = doc.mapEntries(ObjectId.ROOT, heads).get();
        assertMapEntries(docAtPreCommit);
        tx.commit();
        MapEntry[] docAtPostCommit = doc.mapEntries(ObjectId.ROOT, heads).get();
        assertMapEntries(docAtPostCommit);
    }

    void insertMapEntries(Transaction tx) {
        map = tx.set(ObjectId.ROOT, "map", ObjectType.MAP);
        list = tx.set(ObjectId.ROOT, "list", ObjectType.LIST);
        text = tx.set(ObjectId.ROOT, "text", ObjectType.TEXT);

        tx.set(ObjectId.ROOT, "int", 1);
        tx.set(ObjectId.ROOT, "uint", NewValue.uint(1));
        tx.set(ObjectId.ROOT, "float", 1.0);
        tx.set(ObjectId.ROOT, "str", "str");
        tx.set(ObjectId.ROOT, "bytes", "bytes".getBytes());
        tx.set(ObjectId.ROOT, "counter", new Counter(10));
        dateValue = new Date();
        tx.set(ObjectId.ROOT, "date", dateValue);
        tx.set(ObjectId.ROOT, "null", NewValue.NULL);
        tx.set(ObjectId.ROOT, "bool", true);
    }

    void assertMapEntries(Read read) {
        MapEntry[] result = read.mapEntries(ObjectId.ROOT).get();
        assertMapEntries(result);
    }

    void assertMapEntries(MapEntry[] entries) {
        HashMap<String, MapEntry> entrysByKey = new HashMap<String, MapEntry>();
        for (MapEntry entry : entries) {
            entrysByKey.put(entry.getKey(), entry);
        }

        MapEntry mapEntry = entrysByKey.get("map");
        Assertions.assertEquals(map, ((AmValue.Map) mapEntry.getValue()).getId());

        MapEntry listEntry = entrysByKey.get("list");
        Assertions.assertEquals(list, ((AmValue.List) listEntry.getValue()).getId());

        MapEntry textEntry = entrysByKey.get("text");
        Assertions.assertEquals(text, ((AmValue.Text) textEntry.getValue()).getId());

        MapEntry intEntry = entrysByKey.get("int");
        Assertions.assertEquals(1, ((AmValue.Int) intEntry.getValue()).getValue());

        MapEntry uintEntry = entrysByKey.get("uint");
        Assertions.assertEquals(1, ((AmValue.UInt) uintEntry.getValue()).getValue());

        MapEntry floatEntry = entrysByKey.get("float");
        Assertions.assertEquals(1.0, ((AmValue.F64) floatEntry.getValue()).getValue());

        MapEntry strEntry = entrysByKey.get("str");
        Assertions.assertEquals("str", ((AmValue.Str) strEntry.getValue()).getValue());

        MapEntry bytesEntry = entrysByKey.get("bytes");
        Assertions.assertArrayEquals("bytes".getBytes(), ((AmValue.Bytes) bytesEntry.getValue()).getValue());

        MapEntry counterEntry = entrysByKey.get("counter");
        Assertions.assertEquals(10, ((AmValue.Counter) counterEntry.getValue()).getValue());

        MapEntry dateEntry = entrysByKey.get("date");
        Assertions.assertEquals(dateValue, ((AmValue.Timestamp) dateEntry.getValue()).getValue());

        MapEntry nullEntry = entrysByKey.get("null");
        Assertions.assertInstanceOf(AmValue.Null.class, nullEntry.getValue());

        MapEntry boolEntry = entrysByKey.get("bool");
        Assertions.assertEquals(true, ((AmValue.Bool) boolEntry.getValue()).getValue());
    }

    void assertMapEntriesAfter(Read r) {
        MapEntry[] entries = r.mapEntries(ObjectId.ROOT).get();
        Assertions.assertEquals(1, entries.length);
        Assertions.assertEquals("newkey", entries[0].getKey());
        Assertions.assertEquals("newvalue", ((AmValue.Str) entries[0].getValue()).getValue());
    }
}
