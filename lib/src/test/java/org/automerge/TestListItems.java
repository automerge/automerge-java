package org.automerge;

import java.util.Collections;
import java.util.Date;
import org.junit.jupiter.api.Assertions;
import org.junit.jupiter.api.BeforeEach;
import org.junit.jupiter.api.Test;

public class TestListItems {
    private Document doc;
    private ObjectId list;
    private ObjectId subList;
    private ObjectId map;
    private ObjectId text;
    private byte[] bytes;
    private Date date;

    @BeforeEach
    public void setup() {
        doc = new Document();
    }

    @Test
    void testListItems() {
        Transaction tx = doc.startTransaction();
        // Insert a bunch of items
        insertListItems(tx);

        // Check we can read them from a doc with open transaction
        assertListItems(doc);
        // Check we can read them from the transaction
        assertListItems(tx);
        tx.commit();
        // Check we can read them from a doc with closed transaction
        assertListItems(doc);

        // Save the heads
        ChangeHash[] heads = doc.getHeads();

        // Now delete the items we inserted
        tx = doc.startTransaction();
        tx.splice(list, 1, 11, Collections.emptyIterator());

        // Check the current length in the open transaction
        Assertions.assertEquals(1, tx.length(list));
        // Check the current length in the doc with open transaction
        Assertions.assertEquals(1, doc.length(list));

        // Check the current items in the open transaction
        AmValue[] items = tx.listItems(list).get();
        Assertions.assertEquals(1, ((AmValue.Int) items[0]).getValue());
        // Check the current items in the doc with open transaction
        items = doc.listItems(list).get();
        Assertions.assertEquals(1, ((AmValue.Int) items[0]).getValue());

        // Check the length at heads in the open transaction
        Assertions.assertEquals(12, tx.length(list, heads));
        // Check the length at heads in the doc with open transaction
        Assertions.assertEquals(12, doc.length(list, heads));

        // Check the list items at heads in the open transaction
        items = tx.listItems(list, heads).get();
        assertItems(items);

        // Check the list items at heads in the doc with open transaction
        items = doc.listItems(list, heads).get();
        assertItems(items);
        tx.commit();

        // Check the current items in doc with closed transaction
        items = doc.listItems(list).get();
        Assertions.assertEquals(1, ((AmValue.Int) items[0]).getValue());

        // Check the list items at heads in the doc with closed transaction
        items = doc.listItems(list, heads).get();
        assertItems(items);
    }

    void insertListItems(Transaction tx) {
        list = tx.set(ObjectId.ROOT, "list", ObjectType.LIST);

        tx.insert(list, 0, 1);
        tx.insert(list, 1, NewValue.uint(2));
        tx.insert(list, 2, false);
        bytes = "bytes".getBytes();
        tx.insert(list, 3, bytes);
        date = new Date();
        tx.insert(list, 4, date);
        tx.insert(list, 5, 1.2);
        tx.insert(list, 6, "somestring");
        tx.insert(list, 7, NewValue.NULL);
        tx.insert(list, 8, new Counter(10));
        map = tx.insert(list, 9, ObjectType.MAP);
        subList = tx.insert(list, 10, ObjectType.LIST);
        text = tx.insert(list, 11, ObjectType.TEXT);
    }

    <R extends Read> void assertListItems(R read) {
        Assertions.assertEquals(12, read.length(list));
        AmValue[] items = read.listItems(list).get();
        assertItems(items);
    }

    void assertItems(AmValue[] items) {
        Assertions.assertEquals(12, items.length);
        Assertions.assertEquals(1, ((AmValue.Int) items[0]).getValue());
        Assertions.assertEquals(2, ((AmValue.UInt) items[1]).getValue());
        Assertions.assertEquals(false, ((AmValue.Bool) items[2]).getValue());
        Assertions.assertArrayEquals(bytes, ((AmValue.Bytes) items[3]).getValue());
        Assertions.assertEquals(date, ((AmValue.Timestamp) items[4]).getValue());
        Assertions.assertEquals(1.2, ((AmValue.F64) items[5]).getValue());
        Assertions.assertEquals("somestring", ((AmValue.Str) items[6]).getValue());
        Assertions.assertInstanceOf(AmValue.Null.class, items[7]);
        Assertions.assertEquals(10, ((AmValue.Counter) items[8]).getValue());
        Assertions.assertEquals(map, ((AmValue.Map) items[9]).getId());
        Assertions.assertEquals(subList, ((AmValue.List) items[10]).getId());
        Assertions.assertEquals(text, ((AmValue.Text) items[11]).getId());
    }
}
