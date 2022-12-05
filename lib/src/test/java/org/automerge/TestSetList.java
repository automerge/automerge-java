package org.automerge;

import java.util.Date;
import org.junit.jupiter.api.Assertions;
import org.junit.jupiter.api.BeforeEach;
import org.junit.jupiter.api.Test;

public final class TestSetList {
	private Document doc;
	private Transaction tx;
	private ObjectId list;

	public TestSetList() {
		super();
	}

	@BeforeEach
	public void setup() {
		doc = new Document();
		tx = doc.startTransaction();
		list = tx.set(ObjectId.ROOT, "list", ObjectType.LIST);
		tx.insert(list, 0, "something");
	}

	@Test
	public void testSetDoubleInList() {
		tx.set(list, 0, 1.23);
		Assertions.assertEquals(1.23, ((AmValue.F64) doc.get(list, 0).get()).getValue());
	}

	@Test
	public void testSetIntInList() {
		tx.set(list, 0, 123);
		Assertions.assertEquals(123, ((AmValue.Int) doc.get(list, 0).get()).getValue());
	}

	@Test
	public void testSetUintInList() {
		tx.set(list, 0, NewValue.uint(123));
		Assertions.assertEquals(123, ((AmValue.UInt) doc.get(list, 0).get()).getValue());
		Assertions.assertInstanceOf(AmValue.UInt.class, doc.get(list, 0).get());
	}

	@Test
	public void testSetStringInList() {
		tx.set(list, 0, "hello");
		Assertions.assertEquals("hello", ((AmValue.Str) doc.get(list, 0).get()).getValue());
	}

	@Test
	public void testSetBytesInList() {
		byte[] value = "some bytes".getBytes();
		tx.set(list, 0, value);
		Assertions.assertArrayEquals(value, ((AmValue.Bytes) doc.get(list, 0).get()).getValue());
	}

	@Test
	public void testSetBooleanInList() {
		tx.set(list, 0, true);
		Assertions.assertEquals(true, ((AmValue.Bool) doc.get(list, 0).get()).getValue());
		tx.set(list, 0, false);
		Assertions.assertEquals(false, ((AmValue.Bool) doc.get(list, 0).get()).getValue());
	}

	@Test
	public void testSetDateInList() {
		Date date = new Date();
		tx.set(list, 0, date);
		Assertions.assertEquals(date, ((AmValue.Timestamp) doc.get(list, 0).get()).getValue());
	}

	@Test
	public void testSetCounterInList() {
		tx.set(list, 0, new Counter(10));
		Assertions.assertEquals(new Counter(10), ((AmValue.Counter) doc.get(list, 0).get()).getValue());
	}

	@Test
	public void testSetNullInList() {
		tx.set(list, 0, NewValue.NULL);
		Assertions.assertInstanceOf(AmValue.Null.class, doc.get(list, 0).get());
	}

	@Test
	public void setObjectInList() {
		ObjectId listId = tx.set(list, 0, ObjectType.LIST);
		Assertions.assertEquals(listId, ((AmValue.List) doc.get(list, 0).get()).getId());
		ObjectId textId = tx.set(list, 0, ObjectType.TEXT);
		Assertions.assertEquals(textId, ((AmValue.Text) doc.get(list, 0).get()).getId());
		ObjectId mapId = tx.set(list, 0, ObjectType.MAP);
		Assertions.assertEquals(mapId, ((AmValue.Map) doc.get(list, 0).get()).getId());
	}
}
