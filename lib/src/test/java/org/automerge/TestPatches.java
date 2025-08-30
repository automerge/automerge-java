package org.automerge;

import java.util.ArrayList;
import java.util.Date;
import java.util.List;
import org.junit.jupiter.api.Assertions;
import org.junit.jupiter.api.Test;

class TestPatches {

    @Test
    public void testSetInMap() {
        Document doc = new Document();
        PatchLog patchlog = new PatchLog();
        Transaction tx = doc.startTransaction(patchlog);
        tx.set(ObjectId.ROOT, "uint", NewValue.uint(0));
        tx.set(ObjectId.ROOT, "int", 1);
        tx.set(ObjectId.ROOT, "float", 2.0);
        tx.set(ObjectId.ROOT, "string", "string");
        tx.set(ObjectId.ROOT, "bytes", "bytes".getBytes());
        tx.set(ObjectId.ROOT, "bool", true);
        tx.set(ObjectId.ROOT, "null", NewValue.NULL);
        tx.set(ObjectId.ROOT, "counter", new Counter(3));
        Date now = new Date();
        tx.set(ObjectId.ROOT, "timestamp", now);
        ObjectId list = tx.set(ObjectId.ROOT, "list", ObjectType.LIST);
        ObjectId map = tx.set(ObjectId.ROOT, "map", ObjectType.MAP);
        ObjectId text = tx.set(ObjectId.ROOT, "text", ObjectType.TEXT);

        tx.commit();
        List<Patch> patches = doc.makePatches(patchlog);

        assertPutMap(patches.get(0), ObjectId.ROOT, emptyPath(), "uint",
                (AmValue value) -> Assertions.assertEquals(((AmValue.UInt) value).getValue(), 0));

        assertPutMap(patches.get(1), ObjectId.ROOT, emptyPath(), "int", (AmValue value) -> {
            Assertions.assertEquals(((AmValue.Int) value).getValue(), 1);
        });

        assertPutMap(patches.get(2), ObjectId.ROOT, emptyPath(), "float", (AmValue value) -> {
            Assertions.assertEquals(((AmValue.F64) value).getValue(), 2.0);
        });

        assertPutMap(patches.get(3), ObjectId.ROOT, emptyPath(), "string", (AmValue value) -> {
            Assertions.assertEquals(((AmValue.Str) value).getValue(), "string");
        });

        assertPutMap(patches.get(4), ObjectId.ROOT, emptyPath(), "bytes", (AmValue value) -> {
            Assertions.assertArrayEquals(((AmValue.Bytes) value).getValue(), "bytes".getBytes());
        });

        assertPutMap(patches.get(5), ObjectId.ROOT, emptyPath(), "bool", (AmValue value) -> {
            Assertions.assertEquals(((AmValue.Bool) value).getValue(), true);
        });

        assertPutMap(patches.get(6), ObjectId.ROOT, emptyPath(), "null", (AmValue value) -> {
            Assertions.assertInstanceOf(AmValue.Null.class, value);
        });

        assertPutMap(patches.get(7), ObjectId.ROOT, emptyPath(), "counter", (AmValue value) -> {
            Assertions.assertEquals(((AmValue.Counter) value).getValue(), 3);
        });

        assertPutMap(patches.get(8), ObjectId.ROOT, emptyPath(), "timestamp", (AmValue value) -> {
            Assertions.assertEquals(((AmValue.Timestamp) value).getValue(), now);
        });

        assertPutMap(patches.get(9), ObjectId.ROOT, emptyPath(), "list", (AmValue value) -> {
            Assertions.assertEquals(((AmValue.List) value).getId(), list);
        });

        assertPutMap(patches.get(10), ObjectId.ROOT, emptyPath(), "map", (AmValue value) -> {
            Assertions.assertEquals(((AmValue.Map) value).getId(), map);
        });

        assertPutMap(patches.get(11), ObjectId.ROOT, emptyPath(), "text", (AmValue value) -> {
            Assertions.assertEquals(((AmValue.Text) value).getId(), text);
        });
        patchlog.free();
    }

    @Test
    public void testSetInList() {
        Document doc = new Document();
        ObjectId list;
        try (Transaction tx = doc.startTransaction()) {
            list = tx.set(ObjectId.ROOT, "list", ObjectType.LIST);
            for (int i = 0; i < 12; i++) {
                tx.insert(list, i, NewValue.NULL);
            }
            tx.commit();
        }
        PatchLog patchLog = new PatchLog();
        Transaction tx = doc.startTransaction(patchLog);
        tx.set(list, 0, NewValue.uint(0));

        tx.set(list, 1, 1);
        tx.set(list, 2, 2.0);
        tx.set(list, 3, "string");
        tx.set(list, 4, "bytes".getBytes());
        tx.set(list, 5, true);

        // Have to set index 6 to non-null otherwise the setNull is a noop and so no
        // observer method is called
        tx.set(list, 6, 4);
        tx.set(list, 6, NewValue.NULL);

        tx.set(list, 7, new Counter(3));
        Date now = new Date();
        tx.set(list, 8, now);
        ObjectId innerList = tx.set(list, 9, ObjectType.LIST);
        ObjectId map = tx.set(list, 10, ObjectType.MAP);
        ObjectId text = tx.set(list, 11, ObjectType.TEXT);

        tx.commit();
        List<Patch> patches = doc.makePatches(patchLog);

        assertSetInList(patches.get(0), list, PathBuilder.root("list").build(), 0, (AmValue value) -> {
            Assertions.assertEquals(((AmValue.UInt) value).getValue(), 0);
        });

        assertSetInList(patches.get(1), list, PathBuilder.root("list").build(), 1, (AmValue value) -> {
            Assertions.assertEquals(((AmValue.Int) value).getValue(), 1);
        });

        assertSetInList(patches.get(2), list, PathBuilder.root("list").build(), 2, (AmValue value) -> {
            Assertions.assertEquals(((AmValue.F64) value).getValue(), 2.0);
        });

        assertSetInList(patches.get(3), list, PathBuilder.root("list").build(), 3, (AmValue value) -> {
            Assertions.assertEquals(((AmValue.Str) value).getValue(), "string");
        });

        assertSetInList(patches.get(4), list, PathBuilder.root("list").build(), 4, (AmValue value) -> {
            Assertions.assertArrayEquals(((AmValue.Bytes) value).getValue(), "bytes".getBytes());
        });

        assertSetInList(patches.get(5), list, PathBuilder.root("list").build(), 5, (AmValue value) -> {
            Assertions.assertEquals(((AmValue.Bool) value).getValue(), true);
        });

        // Note we skip an index here due to the patch which sets the 6th index to
        // non-null
        assertSetInList(patches.get(7), list, PathBuilder.root("list").build(), 6, (AmValue value) -> {
            Assertions.assertInstanceOf(AmValue.Null.class, value);
        });

        assertSetInList(patches.get(8), list, PathBuilder.root("list").build(), 7, (AmValue value) -> {
            Assertions.assertEquals(((AmValue.Counter) value).getValue(), 3);
        });

        assertSetInList(patches.get(9), list, PathBuilder.root("list").build(), 8, (AmValue value) -> {
            Assertions.assertEquals(((AmValue.Timestamp) value).getValue(), now);
        });

        assertSetInList(patches.get(10), list, PathBuilder.root("list").build(), 9, (AmValue value) -> {
            Assertions.assertEquals(((AmValue.List) value).getId(), innerList);
        });

        assertSetInList(patches.get(11), list, PathBuilder.root("list").build(), 10, (AmValue value) -> {
            Assertions.assertEquals(((AmValue.Map) value).getId(), map);
        });

        assertSetInList(patches.get(12), list, PathBuilder.root("list").build(), 11, (AmValue value) -> {
            Assertions.assertEquals(((AmValue.Text) value).getId(), text);
        });
    }

    @Test
    public void testInsertInList() {
        Document doc = new Document();
        ObjectId list;
        try (Transaction tx = doc.startTransaction()) {
            list = tx.set(ObjectId.ROOT, "list", ObjectType.LIST);
            tx.commit();
        }
        PatchLog patchLog = new PatchLog();
        Transaction tx = doc.startTransaction(patchLog);

        tx.insert(list, 0, NewValue.uint(0));
        tx.insert(list, 1, 1);
        tx.insert(list, 2, 2.0);
        tx.insert(list, 3, "string");
        tx.insert(list, 4, "bytes".getBytes());
        tx.insert(list, 5, true);
        tx.insert(list, 6, NewValue.NULL);
        tx.insert(list, 7, new Counter(3));
        Date now = new Date();
        tx.insert(list, 8, now);
        ObjectId innerList = tx.insert(list, 9, ObjectType.LIST);
        ObjectId map = tx.insert(list, 10, ObjectType.MAP);
        ObjectId text = tx.insert(list, 11, ObjectType.TEXT);

        tx.commit();
        List<Patch> patches = doc.makePatches(patchLog);
        Assertions.assertEquals(patches.size(), 1);

        Patch patch = patches.get(0);
        Assertions.assertEquals(patch.getObj(), list);
        // Assertions.assertEquals(patch.getPath(), path(new PathElement(ObjectId.ROOT,
        // new
        // Prop("list")));
        Assertions.assertEquals(patch.getPath(), PathBuilder.root("list").build());
        ArrayList<AmValue> values = ((PatchAction.Insert) patch.getAction()).getValues();

        Assertions.assertEquals(((AmValue.UInt) values.get(0)).getValue(), 0);
        Assertions.assertEquals(((AmValue.Int) values.get(1)).getValue(), 1);
        Assertions.assertEquals(((AmValue.F64) values.get(2)).getValue(), 2.0);
        Assertions.assertEquals(((AmValue.Str) values.get(3)).getValue(), "string");
        Assertions.assertArrayEquals(((AmValue.Bytes) values.get(4)).getValue(), "bytes".getBytes());
        Assertions.assertEquals(((AmValue.Bool) values.get(5)).getValue(), true);
        Assertions.assertInstanceOf(AmValue.Null.class, values.get(6));
        Assertions.assertEquals(((AmValue.Counter) values.get(7)).getValue(), 3);
        Assertions.assertEquals(((AmValue.Timestamp) values.get(8)).getValue(), now);
        Assertions.assertEquals(((AmValue.List) values.get(9)).getId(), innerList);
        Assertions.assertEquals(((AmValue.Map) values.get(10)).getId(), map);
        Assertions.assertEquals(((AmValue.Text) values.get(11)).getId(), text);
    }

    @Test
    public void testSpliceText() {
        Document doc = new Document();
        ObjectId text;
        try (Transaction tx = doc.startTransaction()) {
            text = tx.set(ObjectId.ROOT, "text", ObjectType.TEXT);
            tx.spliceText(text, 0, 0, "Hello");
            tx.commit();
        }

        PatchLog patchLog = new PatchLog();
        Transaction tx = doc.startTransaction(patchLog);
        tx.spliceText(text, 5, 0, " world");

        tx.commit();
        List<Patch> patches = doc.makePatches(patchLog);

        Assertions.assertEquals(patches.size(), 1);
        Patch patch = patches.get(0);

        Assertions.assertEquals(patch.getObj(), text);
        Assertions.assertEquals(patch.getPath(), PathBuilder.root("text").build());

        PatchAction.SpliceText action = (PatchAction.SpliceText) patch.getAction();
        Assertions.assertEquals(action.getIndex(), 5);
        Assertions.assertEquals(action.getText(), " world");
    }

    @Test
    public void testConflictedPutInMap() {
        Document doc1 = new Document("bbbb".getBytes());
        Document doc2 = new Document("aaaa".getBytes());
        try (Transaction tx = doc1.startTransaction()) {
            tx.set(ObjectId.ROOT, "key", "value_1");
            tx.commit();
        }
        try (Transaction tx = doc2.startTransaction()) {
            tx.set(ObjectId.ROOT, "key", "value_2");
            tx.commit();
        }
        PatchLog patchLog = new PatchLog();
        doc2.merge(doc1, patchLog);
        List<Patch> patches = doc2.makePatches(patchLog);

        Assertions.assertEquals(patches.size(), 1);
        Patch patch = patches.get(0);
        Assertions.assertEquals(patch.getObj(), ObjectId.ROOT);
        Assertions.assertEquals(patch.getPath(), emptyPath());

        PatchAction.PutMap action = (PatchAction.PutMap) patch.getAction();
        Assertions.assertTrue(action.isConflict());
    }

    @Test
    public void testFlagConflictMap() {
        Document doc1 = new Document("bbbb".getBytes());
        Document doc2 = new Document("aaaa".getBytes());
        try (Transaction tx = doc1.startTransaction()) {
            tx.set(ObjectId.ROOT, "key", "value_1");
            tx.commit();
        }
        try (Transaction tx = doc2.startTransaction()) {
            tx.set(ObjectId.ROOT, "key", "value_2");
            tx.commit();
        }
        PatchLog patchLog = new PatchLog();
        doc1.merge(doc2, patchLog);
        List<Patch> patches = doc1.makePatches(patchLog);

        Assertions.assertEquals(patches.size(), 1);
        Patch patch = patches.get(0);
        Assertions.assertEquals(patch.getObj(), ObjectId.ROOT);
        Assertions.assertEquals(patch.getPath(), emptyPath());

        PatchAction.FlagConflict action = (PatchAction.FlagConflict) patch.getAction();
    }

    @Test
    public void testConflictedPutInList() {
        Document doc1 = new Document("bbbb".getBytes());
        ObjectId list;
        try (Transaction tx = doc1.startTransaction()) {
            list = tx.set(ObjectId.ROOT, "list", ObjectType.LIST);
            tx.insert(list, 0, NewValue.NULL);
            tx.commit();
        }

        Document doc2 = doc1.fork("aaaa".getBytes());
        try (Transaction tx = doc2.startTransaction()) {
            tx.set(list, 0, "value_2");
            tx.commit();
        }
        try (Transaction tx = doc1.startTransaction()) {
            tx.set(list, 0, "value_1");
            tx.commit();
        }

        PatchLog patchLog = new PatchLog();
        doc2.merge(doc1, patchLog);
        List<Patch> patches = doc2.makePatches(patchLog);

        Assertions.assertEquals(patches.size(), 1);
        Patch patch = patches.get(0);
        Assertions.assertEquals(patch.getObj(), list);
        Assertions.assertEquals(patch.getPath(), PathBuilder.root("list").build());

        PatchAction.PutList action = (PatchAction.PutList) patch.getAction();
        Assertions.assertTrue(action.isConflict());
    }

    @Test
    public void testFlagConflictList() {
        Document doc1 = new Document("bbbb".getBytes());
        ObjectId list;
        try (Transaction tx = doc1.startTransaction()) {
            list = tx.set(ObjectId.ROOT, "list", ObjectType.LIST);
            tx.insert(list, 0, NewValue.NULL);
            tx.commit();
        }

        Document doc2 = doc1.fork("aaaa".getBytes());
        try (Transaction tx = doc1.startTransaction()) {
            tx.set(list, 0, "value_2");
            tx.commit();
        }
        try (Transaction tx = doc2.startTransaction()) {
            tx.set(list, 0, "value_1");
            tx.commit();
        }

        PatchLog patchLog = new PatchLog();
        doc1.merge(doc2, patchLog);
        List<Patch> patches = doc1.makePatches(patchLog);

        Assertions.assertEquals(patches.size(), 1);
        Patch patch = patches.get(0);
        Assertions.assertEquals(patch.getObj(), list);
        Assertions.assertEquals(patch.getPath(), PathBuilder.root("list").build());

        PatchAction.FlagConflict action = (PatchAction.FlagConflict) patch.getAction();
    }

    @Test
    public void testIncrementInMap() {
        Document doc = new Document();
        try (Transaction tx = doc.startTransaction()) {
            tx.set(ObjectId.ROOT, "counter", new Counter(0));
            tx.commit();
        }

        PatchLog patchLog = new PatchLog();
        try (Transaction tx = doc.startTransaction(patchLog)) {
            tx.increment(ObjectId.ROOT, "counter", 5);
            tx.commit();
        }
        List<Patch> patches = doc.makePatches(patchLog);

        Assertions.assertEquals(patches.size(), 1);
        Patch patch = patches.get(0);

        Assertions.assertEquals(patch.getObj(), ObjectId.ROOT);
        Assertions.assertEquals(patch.getPath(), emptyPath());
        Assertions.assertEquals(((PatchAction.Increment) patch.getAction()).getValue(), 5);
    }

    @Test
    public void testIncrementInList() {
        Document doc = new Document();
        ObjectId list;
        try (Transaction tx = doc.startTransaction()) {
            list = tx.set(ObjectId.ROOT, "list", ObjectType.LIST);
            tx.insert(list, 0, new Counter(10));
            tx.commit();
        }

        PatchLog patchLog = new PatchLog();
        try (Transaction tx = doc.startTransaction(patchLog)) {
            tx.increment(list, 0, 5);
            tx.commit();
        }
        List<Patch> patches = doc.makePatches(patchLog);

        Assertions.assertEquals(patches.size(), 1);
        Patch patch = patches.get(0);

        Assertions.assertEquals(patch.getObj(), list);
        Assertions.assertEquals(patch.getPath(), PathBuilder.root("list").build());
        Assertions.assertEquals(((PatchAction.Increment) patch.getAction()).getValue(), 5);
    }

    @Test
    public void testDeleteInMap() {
        Document doc = new Document();
        try (Transaction tx = doc.startTransaction()) {
            tx.set(ObjectId.ROOT, "key", "value");
            tx.commit();
        }

        PatchLog patchLog = new PatchLog();
        try (Transaction tx = doc.startTransaction(patchLog)) {
            tx.delete(ObjectId.ROOT, "key");
            tx.commit();
        }

        List<Patch> patches = doc.makePatches(patchLog);

        Assertions.assertEquals(patches.size(), 1);
        Patch patch = patches.get(0);
        Assertions.assertEquals(patch.getObj(), ObjectId.ROOT);
        Assertions.assertEquals(patch.getPath(), emptyPath());
        PatchAction.DeleteMap action = (PatchAction.DeleteMap) patch.getAction();
        Assertions.assertEquals(action.getKey(), "key");
    }

    @Test
    public void testDeleteInList() {
        Document doc = new Document();
        ObjectId list;
        try (Transaction tx = doc.startTransaction()) {
            list = tx.set(ObjectId.ROOT, "list", ObjectType.LIST);
            tx.insert(list, 0, "value");
            tx.insert(list, 1, "value2");
            tx.commit();
        }

        PatchLog patchLog = new PatchLog();
        try (Transaction tx = doc.startTransaction(patchLog)) {
            tx.delete(list, 0);
            tx.delete(list, 0);
            tx.commit();
        }
        List<Patch> patches = doc.makePatches(patchLog);

        Assertions.assertEquals(patches.size(), 1);
        Patch patch1 = patches.get(0);
        Assertions.assertEquals(patch1.getObj(), list);
        Assertions.assertEquals(patch1.getPath(), PathBuilder.root("list").build());
        PatchAction.DeleteList action1 = (PatchAction.DeleteList) patch1.getAction();
        Assertions.assertEquals(action1.getIndex(), 0);
        Assertions.assertEquals(action1.getLength(), 2);
    }

    @Test
    public void testApplyEncodedChangesForPatches() {
        Document doc = new Document();
        try (Transaction tx = doc.startTransaction()) {
            tx.set(ObjectId.ROOT, "key", "value");
            tx.commit();
        }
        byte[] changes = doc.encodeChangesSince(new ChangeHash[]{});
        Document doc2 = new Document();
        PatchLog patchLog = new PatchLog();
        doc2.applyEncodedChanges(changes, patchLog);

        List<Patch> patches = doc2.makePatches(patchLog);

        Assertions.assertEquals(patches.size(), 1);
        Patch patch = patches.get(0);

        Assertions.assertEquals(patch.getObj(), ObjectId.ROOT);
        Assertions.assertEquals(patch.getPath(), emptyPath());

        PatchAction.PutMap action = (PatchAction.PutMap) patch.getAction();
        Assertions.assertEquals(action.getKey(), "key");
    }

    public ArrayList<PathElement> emptyPath() {
        return PathBuilder.empty();
    }

    void assertPutMap(Patch patch, ObjectId expectedObj, ArrayList<PathElement> expectedPath, String expectedKey,
            ValueAssertion value) {
        Assertions.assertEquals(patch.getPath(), expectedPath);
        Assertions.assertEquals(patch.getObj(), expectedObj);
        Assertions.assertEquals(((PatchAction.PutMap) patch.getAction()).getKey(), expectedKey);
        value.assertValue(((PatchAction.PutMap) patch.getAction()).getValue());
    }

    void assertSetInList(Patch patch, ObjectId expectedObj, ArrayList<PathElement> expectedPath, long expectedIndex,
            ValueAssertion value) {
        Assertions.assertEquals(patch.getPath(), expectedPath);
        Assertions.assertEquals(patch.getObj(), expectedObj);
        Assertions.assertEquals(((PatchAction.PutList) patch.getAction()).getIndex(), expectedIndex);
        value.assertValue(((PatchAction.PutList) patch.getAction()).getValue());
    }

    void assertInsertInList(Patch patch, ObjectId expectedObj, ArrayList<PathElement> expectedPath, long expectedIndex,
            ValuesAssertion value) {
        Assertions.assertEquals(patch.getPath(), expectedPath);
        Assertions.assertEquals(patch.getObj(), expectedObj);
        Assertions.assertEquals(((PatchAction.Insert) patch.getAction()).getIndex(), expectedIndex);
        value.assertValues(((PatchAction.Insert) patch.getAction()).getValues());
    }

    public interface ValueAssertion {
        void assertValue(AmValue value);
    }

    public interface ValuesAssertion {
        void assertValues(ArrayList<AmValue> values);
    }
}
