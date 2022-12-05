package org.automerge;

import org.junit.jupiter.api.Assertions;
import org.junit.jupiter.api.Test;

class TestChanges {
	@Test
	public void testApplyEncodedChanges() {
		Document doc = new Document();
		try (Transaction tx = doc.startTransaction()) {
			tx.set(ObjectId.ROOT, "key", "value");
			tx.commit();
		}
		ChangeHash[] heads = doc.getHeads();
		Document doc2 = doc.fork();
		try (Transaction tx = doc2.startTransaction()) {
			tx.set(ObjectId.ROOT, "key", "value2");
			tx.commit();
		}
		byte[] changes = doc2.encodeChangesSince(heads);
		doc.applyEncodedChanges(changes);
		Assertions.assertEquals("value2", ((AmValue.Str) doc.get(ObjectId.ROOT, "key").get()).getValue());
	}
}
