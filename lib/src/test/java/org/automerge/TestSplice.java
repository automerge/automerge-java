package org.automerge;

import java.util.Arrays;
import java.util.Date;
import org.junit.jupiter.api.Assertions;
import org.junit.jupiter.api.BeforeEach;
import org.junit.jupiter.api.Test;

public final class TestSplice {
	private Document doc;
	private Transaction tx;
	private ObjectId list;

	@FunctionalInterface
	interface InsertedAssertions {
		void assertInserted(Object elem1, Object elem2);
	}

	public TestSplice() {
		super();
	}

	@BeforeEach
	public void setup() {
		doc = new Document();
		tx = doc.startTransaction();
		list = tx.set(ObjectId.ROOT, "list", ObjectType.LIST);
	}

	void testSplice2(NewValue val1, NewValue val2, InsertedAssertions assertions) {
		tx.insert(list, 0, 1);
		tx.insert(list, 1, 2);
		tx.insert(list, 2, 3);
		NewValue[] toInsert = {val1, val2,};
		tx.splice(list, 1, 1, Arrays.asList(toInsert).iterator());
		Assertions.assertEquals(1l, ((AmValue.Int) tx.get(list, 0).get()).getValue());

		Object elem1 = tx.get(list, 1).get();
		Object elem2 = tx.get(list, 2).get();
		assertions.assertInserted(elem1, elem2);

		Assertions.assertEquals(3l, ((AmValue.Int) tx.get(list, 3).get()).getValue());
	}

	@Test
	public void testSpliceInt() {
		testSplice2(NewValue.integer(4), NewValue.integer(5), (elem1, elem2) -> {
			Assertions.assertEquals(4l, ((AmValue.Int) elem1).getValue());
			Assertions.assertEquals(5l, ((AmValue.Int) elem2).getValue());
		});
	}

	@Test
	public void testSpliceUint() {
		testSplice2(NewValue.uint(1), NewValue.uint(2), (elem1, elem2) -> {
			Assertions.assertEquals(1, ((AmValue.UInt) elem1).getValue());
			Assertions.assertEquals(2, ((AmValue.UInt) elem2).getValue());
		});
	}

	@Test
	public void testSpliceDouble() {
		testSplice2(NewValue.f64(4.0d), NewValue.f64(5.0d), (elem1, elem2) -> {
			Assertions.assertEquals(4.0, ((AmValue.F64) elem1).getValue());
			Assertions.assertEquals(5.0, ((AmValue.F64) elem2).getValue());
		});
	}

	@Test
	public void testSpliceBytes() {
		testSplice2(NewValue.bytes("4".getBytes()), NewValue.bytes("5".getBytes()), (elem1, elem2) -> {
			Assertions.assertArrayEquals("4".getBytes(), ((AmValue.Bytes) elem1).getValue());
			Assertions.assertArrayEquals("5".getBytes(), ((AmValue.Bytes) elem2).getValue());
		});
	}

	@Test
	public void testInsertBool() {
		testSplice2(NewValue.bool(true), NewValue.bool(false), (elem1, elem2) -> {
			Assertions.assertEquals(true, ((AmValue.Bool) elem1).getValue());
			Assertions.assertEquals(false, ((AmValue.Bool) elem2).getValue());
		});
	}

	@Test
	public void testInsertNull() {
		testSplice2(NewValue.NULL, NewValue.NULL, (elem1, elem2) -> {
			Assertions.assertInstanceOf(AmValue.Null.class, elem1);
			Assertions.assertInstanceOf(AmValue.Null.class, elem2);
		});
	}

	@Test
	public void testInsertString() {
		testSplice2(NewValue.str("4"), NewValue.str("5"), (elem1, elem2) -> {
			Assertions.assertEquals("4", ((AmValue.Str) elem1).getValue());
			Assertions.assertEquals("5", ((AmValue.Str) elem2).getValue());
		});
	}

	@Test
	public void testSpliceDate() {
		Date d1 = new Date();
		Date d2 = new Date();
		testSplice2(NewValue.timestamp(d1), NewValue.timestamp(d2), (elem1, elem2) -> {
			Assertions.assertEquals(d1, ((AmValue.Timestamp) elem1).getValue());
			Assertions.assertEquals(d2, ((AmValue.Timestamp) elem2).getValue());
		});
	}

	@Test
	public void testSpliceCounter() {
		testSplice2(NewValue.counter(1), NewValue.counter(2), (elem1, elem2) -> {
			Assertions.assertEquals(1, ((AmValue.Counter) elem1).getValue());
			Assertions.assertEquals(2, ((AmValue.Counter) elem2).getValue());
		});
	}
}
// Check that committing the transaction clears Document.transactionPtr
