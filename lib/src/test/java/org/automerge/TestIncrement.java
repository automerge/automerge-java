package org.automerge;

import org.junit.jupiter.api.Assertions;
import org.junit.jupiter.api.BeforeEach;
import org.junit.jupiter.api.Test;

public final class TestIncrement {
	private Document doc;
	private Transaction tx;

	public TestIncrement() {
		super();
	}

	@BeforeEach
	public void setup() {
		doc = new Document();
		tx = doc.startTransaction();
	}

	@Test
	public void testIncrementInMap() {
		tx.set(ObjectId.ROOT, "key", new Counter(10));
		tx.increment(ObjectId.ROOT, "key", 5);
		Assertions.assertEquals(15, ((AmValue.Counter) doc.get(ObjectId.ROOT, "key").get()).getValue());
	}

	@Test
	public void testIncrementInList() {
		ObjectId list = tx.set(ObjectId.ROOT, "list", ObjectType.LIST);
		tx.insert(list, 0, new Counter(10));
		tx.increment(list, 0, 5);
		Assertions.assertEquals(15, ((AmValue.Counter) doc.get(list, 0).get()).getValue());
	}
}
