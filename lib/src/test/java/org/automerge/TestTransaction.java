package org.automerge;

import java.util.Date;
import java.util.Optional;
import org.junit.jupiter.api.Assertions;
import org.junit.jupiter.api.BeforeEach;
import org.junit.jupiter.api.Test;

public final class TestTransaction {
	private Document doc;
	private Transaction tx;

	public TestTransaction() {
		super();
	}

	@BeforeEach
	public void setup() {
		doc = new Document();
		tx = doc.startTransaction();
	}

	@Test
	public void commitTransaction() {
		// Check that committing the transaction clears Document.transactionPtr
		tx.set(ObjectId.ROOT, "key", 1.23);
		tx.commit();
		tx = doc.startTransaction();
		tx.set(ObjectId.ROOT, "key", 4.56);
		tx.commit();
		Optional<AmValue> result = doc.get(ObjectId.ROOT, "key");
		Assertions.assertEquals(4.56, ((AmValue.F64) result.get()).getValue());
	}

	@Test
	public final void exceptionCaughtWhenSetting() {
		// Create an object in another document, then use that object in this document,
		// yielding an error in Rust
		// then check the error is caught and thrown as an AutomergeException.
		Document otherDoc = new Document();
		final ObjectId otherObj;
		try (Transaction tx = otherDoc.startTransaction()) {
			otherObj = tx.set(ObjectId.ROOT, "key", ObjectType.MAP);
		}
		// Close the already running transaction from `setup`
		tx.commit();
		Assertions.assertThrows(AutomergeException.class, () -> {
			try (Transaction tx = doc.startTransaction()) {
				tx.set(otherObj, "key", 1.23);
			}
		});
	}

	@Test
	public void testRollBackRemovesOps() {
		tx.set(ObjectId.ROOT, "key", 1.23);
		tx.rollback();
		Assertions.assertTrue(doc.get(ObjectId.ROOT, "key").isEmpty());
	}

	@Test
	public void testRollbackClearsTransaction() {
		tx.set(ObjectId.ROOT, "key", 1.23);
		tx.rollback();
		doc.startTransaction();
	}

	@Test
	public void testCloseWithoutCommitRollsback() {
		tx.set(ObjectId.ROOT, "key", 1.23);
		tx.close();
		Assertions.assertTrue(doc.get(ObjectId.ROOT, "key").isEmpty());
		// Check we can start a new transaction because the last one was closed
		doc.startTransaction();
	}

	@Test
	public void testCloseAfterCommitIsNoOp() {
		tx.set(ObjectId.ROOT, "key", 1.23);
		tx.commit();
		tx.close();
		Assertions.assertEquals(1.23, ((AmValue.F64) doc.get(ObjectId.ROOT, "key").get()).getValue());
		// Check we can start a new transaction because the last one was closed
		doc.startTransaction();
	}

	// Basic tests for setting properties in maps
	@Test
	public final void setDoubleInMap() {
		tx.set(ObjectId.ROOT, "key", 1.23);
		tx.commit();
		Assertions.assertEquals(1.23, ((AmValue.F64) doc.get(ObjectId.ROOT, "key").get()).getValue());
	}

	@Test
	public final void setBytesInMap() {
		byte[] somebytes = "some bytes".getBytes();
		tx.set(ObjectId.ROOT, "bytes", somebytes);
		tx.commit();
		Assertions.assertArrayEquals(somebytes, ((AmValue.Bytes) doc.get(ObjectId.ROOT, "bytes").get()).getValue());
	}

	@Test
	public final void setStringInMap() {
		tx.set(ObjectId.ROOT, "key", "some string");
		Assertions.assertEquals("some string", ((AmValue.Str) doc.get(ObjectId.ROOT, "key").get()).getValue());
	}

	@Test
	public final void setIntInMap() {
		tx.set(ObjectId.ROOT, "key", 10);
		Assertions.assertEquals(10, ((AmValue.Int) doc.get(ObjectId.ROOT, "key").get()).getValue());
	}

	@Test
	public final void setUintInMap() {
		tx.set(ObjectId.ROOT, "key", NewValue.uint(10));
		Assertions.assertEquals(10, ((AmValue.UInt) doc.get(ObjectId.ROOT, "key").get()).getValue());
	}

	@Test
	public final void setBoolInMap() {
		tx.set(ObjectId.ROOT, "false", false);
		tx.set(ObjectId.ROOT, "true", true);
		Assertions.assertEquals(false, ((AmValue.Bool) doc.get(ObjectId.ROOT, "false").get()).getValue());
		Assertions.assertEquals(true, ((AmValue.Bool) doc.get(ObjectId.ROOT, "true").get()).getValue());
	}

	@Test
	public final void setNullInMap() {
		tx.set(ObjectId.ROOT, "key", NewValue.NULL);
		Assertions.assertInstanceOf(AmValue.Null.class, doc.get(ObjectId.ROOT, "key").get());
	}

	@Test
	public final void setCounterInMap() {
		tx.set(ObjectId.ROOT, "counter", new Counter(10));
		Assertions.assertEquals(10, ((AmValue.Counter) doc.get(ObjectId.ROOT, "counter").get()).getValue());
	}

	@Test
	public final void setDateInMap() {
		Date date = new Date();
		tx.set(ObjectId.ROOT, "key", date);
		Assertions.assertEquals(date, ((AmValue.Timestamp) doc.get(ObjectId.ROOT, "key").get()).getValue());
	}

	@Test
	public final void getNone() {
		Assertions.assertTrue(doc.get(ObjectId.ROOT, "nonexistent").isEmpty());
	}

	// Test for creating objects
	@Test
	public final void putMapInMap() {
		ObjectId nested = tx.set(ObjectId.ROOT, "nested", ObjectType.MAP);
		tx.set(nested, "key", 1.23);
		tx.commit();
		Assertions.assertEquals(nested, ((AmValue.Map) doc.get(ObjectId.ROOT, "nested").get()).getId());
	}

	@Test
	public final void putListInMap() {
		ObjectId nested = tx.set(ObjectId.ROOT, "nested", ObjectType.LIST);
		Assertions.assertEquals(nested, ((AmValue.List) doc.get(ObjectId.ROOT, "nested").get()).getId());
	}

	@Test
	public void testGetInListInTx() {
		ObjectId nested = tx.set(ObjectId.ROOT, "nested", ObjectType.LIST);
		tx.insert(nested, 0, 123);
		Assertions.assertEquals(123, ((AmValue.Int) doc.get(nested, 0).get()).getValue());
		Assertions.assertEquals(123, ((AmValue.Int) tx.get(nested, 0).get()).getValue());
		tx.commit();
		Assertions.assertEquals(123, ((AmValue.Int) doc.get(nested, 0).get()).getValue());
	}

	@Test
	public void testGetInMapInTx() {
		tx.set(ObjectId.ROOT, "key", 123);
		Assertions.assertEquals(123, ((AmValue.Int) doc.get(ObjectId.ROOT, "key").get()).getValue());
		Assertions.assertEquals(123, ((AmValue.Int) tx.get(ObjectId.ROOT, "key").get()).getValue());
		tx.commit();
		Assertions.assertEquals(123, ((AmValue.Int) doc.get(ObjectId.ROOT, "key").get()).getValue());
	}
}
