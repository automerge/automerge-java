package org.automerge;

import org.junit.jupiter.api.Assertions;
import org.junit.jupiter.api.Test;

class TestMerge {
	@Test
	public void testMerge() {
		Document doc1 = new Document();
		try (Transaction tx = doc1.startTransaction()) {
			tx.set(ObjectId.ROOT, "key1", 1.23);
			tx.commit();
		}
		Document doc2 = new Document();
		try (Transaction tx = doc2.startTransaction()) {
			tx.set(ObjectId.ROOT, "key2", 4.56);
			tx.commit();
		}
		doc1.merge(doc2);
		Assertions.assertEquals(1.23, ((AmValue.F64) doc1.get(ObjectId.ROOT, "key1").get()).getValue());
		Assertions.assertEquals(4.56, ((AmValue.F64) doc1.get(ObjectId.ROOT, "key2").get()).getValue());
	}

	@Test
	public void testMergeThrowsIfTransactionInProgress() {
		Document doc1 = new Document();
		doc1.startTransaction();
		Document doc2 = new Document();
		Assertions.assertThrows(TransactionInProgress.class, () -> {
			doc1.merge(doc2);
		});
	}

	@Test
	public void testMergeThrowsIfOtherTransactionInProgress() {
		Document doc1 = new Document();
		Document doc2 = new Document();
		doc2.startTransaction();
		Assertions.assertThrows(TransactionInProgress.class, () -> {
			doc1.merge(doc2);
		});
	}
}
