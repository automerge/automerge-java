package org.automerge;

import java.util.Optional;
import java.util.function.Function;
import org.junit.jupiter.api.Assertions;
import org.junit.jupiter.api.Test;

class TestKeys {
	<T> void run(Function<Document, Transaction<T>> createTx) {
		Document doc = new Document();
		Transaction<T> tx = createTx.apply(doc);
		tx.set(ObjectId.ROOT, "key1", ObjectType.MAP);
		tx.set(ObjectId.ROOT, "key2", ObjectType.LIST);
		ObjectId list = tx.set(ObjectId.ROOT, "list", ObjectType.LIST);

		String[] expected = {"key1", "key2", "list",};
		Assertions.assertArrayEquals(expected, tx.keys(ObjectId.ROOT).get());
		Assertions.assertArrayEquals(expected, doc.keys(ObjectId.ROOT).get());

		tx.commit();

		ChangeHash[] heads = doc.getHeads();
		tx = createTx.apply(doc);
		String[] expectedBefore = {"key1", "key2", "list",};
		tx.delete(ObjectId.ROOT, "key1");
		String[] expectedAfter = {"key2", "list",};
		Assertions.assertArrayEquals(expectedAfter, tx.keys(ObjectId.ROOT).get());
		Assertions.assertArrayEquals(expectedBefore, tx.keys(ObjectId.ROOT, heads).get());
		Assertions.assertArrayEquals(expectedBefore, doc.keys(ObjectId.ROOT, heads).get());
		Assertions.assertEquals(Optional.empty(), tx.keys(list));
		tx.commit();
		Assertions.assertArrayEquals(expectedBefore, doc.keys(ObjectId.ROOT, heads).get());
		Assertions.assertEquals(Optional.empty(), doc.keys(list));
	}

	@Test
	public void testKeys() {
		run(doc -> doc.startTransaction());
	}

	@Test
	public void testKeysObserved() {
		run(doc -> doc.startTransactionForPatches());
	}
}
