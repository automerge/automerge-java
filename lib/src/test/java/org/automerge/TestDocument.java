package org.automerge;

import java.util.Optional;
import org.junit.jupiter.api.Assertions;
import org.junit.jupiter.api.Test;

public final class TestDocument {

	public TestDocument() {
		super();
	}

	@Test
	public void testConstructWithActorId() {
		Document doc = new Document("actorId".getBytes());
		Assertions.assertArrayEquals("actorId".getBytes(), doc.getActorId());
	}

	@Test
	public final void startAndCommitTransaction() {
		Document doc = new Document();
		try (Transaction tx = doc.startTransaction()) {
			tx.set(ObjectId.ROOT, "key", 1.23);
			tx.commit();
		}
		Optional<AmValue> result = doc.get(ObjectId.ROOT, "key");
		Assertions.assertEquals(1.23, ((AmValue.F64) result.get()).getValue());
	}

	@Test
	public void testThrowsOnSaveWhileTransactionInProgress() {
		Document doc = new Document();
		Transaction tx = doc.startTransaction();
		tx.set(ObjectId.ROOT, "key", 1.23);
		Assertions.assertThrows(TransactionInProgress.class, () -> {
			doc.save();
		});
	}

	@Test
	public void testSaveAndLoad() {
		Document doc = new Document();
		try (Transaction tx = doc.startTransaction()) {
			tx.set(ObjectId.ROOT, "key", 1.23);
			tx.commit();
		}
		byte[] bytes = doc.save();
		Document doc2 = Document.load(bytes);
		Optional<AmValue> result = doc2.get(ObjectId.ROOT, "key");
		Assertions.assertEquals(1.23, ((AmValue.F64) result.get()).getValue());
	}

	@Test
	public void testThrowsAutomergeExceptionOnInvalidBytes() {
		byte[] badBytes = "badbytes".getBytes();
		Assertions.assertThrows(AutomergeException.class, () -> {
			Document.load(badBytes);
		});
	}

	@Test
	public void testFree() {
		Document doc = new Document();
		doc.free();
	}
}
