package org.automerge;

import java.util.Optional;
import org.junit.jupiter.api.Assertions;
import org.junit.jupiter.api.BeforeEach;
import org.junit.jupiter.api.Test;

public final class TestDelete {
	private Document doc;
	private Transaction tx;

	public TestDelete() {
		super();
	}

	@BeforeEach
	public void setup() {
		doc = new Document();
		tx = doc.startTransaction();
	}

	@Test
	public void testDeleteInMap() {
		tx.set(ObjectId.ROOT, "key", new Counter(10));
		tx.delete(ObjectId.ROOT, "key");
		Assertions.assertEquals(tx.get(ObjectId.ROOT, "key"), Optional.empty());
	}

	@Test
	public void testDeleteInList() {
		ObjectId list = tx.set(ObjectId.ROOT, "list", ObjectType.LIST);
		tx.insert(list, 0, 123);
		tx.delete(list, 0);
		Assertions.assertEquals(tx.get(list, 0), Optional.empty());
	}
}
// Check that committing the transaction clears Document.transactionPtr
