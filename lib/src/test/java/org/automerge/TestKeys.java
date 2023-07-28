package org.automerge;

import java.util.Optional;
import org.junit.jupiter.api.Assertions;
import org.junit.jupiter.api.Test;

class TestKeys {
	@Test
	public void testKeys() {
		Document doc = new Document();
		Transaction tx = doc.startTransaction();
		tx.set(ObjectId.ROOT, "key1", ObjectType.MAP);
		tx.set(ObjectId.ROOT, "key2", ObjectType.LIST);
		ObjectId list = tx.set(ObjectId.ROOT, "list", ObjectType.LIST);

		String[] expected = {"key1", "key2", "list",};
		Assertions.assertArrayEquals(expected, tx.keys(ObjectId.ROOT).get());
		Assertions.assertArrayEquals(expected, doc.keys(ObjectId.ROOT).get());

		tx.commit();

		ChangeHash[] heads = doc.getHeads();
		tx = doc.startTransaction();
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
}
