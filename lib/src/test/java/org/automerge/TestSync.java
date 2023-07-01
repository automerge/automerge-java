package org.automerge;

import java.util.ArrayList;
import java.util.List;
import java.util.Optional;
import org.junit.jupiter.api.Assertions;
import org.junit.jupiter.api.Test;

class TestSync {
	@Test
	public void testSync() {
		Document doc1 = new Document();
		try (Transaction<ChangeHash> tx = doc1.startTransaction()) {
			tx.set(ObjectId.ROOT, "key", "value");
			tx.commit();
		}

		Document doc2 = doc1.fork();
		try (Transaction<ChangeHash> tx = doc2.startTransaction()) {
			tx.set(ObjectId.ROOT, "key2", "value2");
			tx.commit();
		}
		try (Transaction<ChangeHash> tx = doc1.startTransaction()) {
			tx.set(ObjectId.ROOT, "key3", "value3");
			tx.commit();
		}

		sync(doc1, doc2);

		Assertions.assertEquals("value", ((AmValue.Str) doc1.get(ObjectId.ROOT, "key").get()).getValue());
		Assertions.assertEquals("value2", ((AmValue.Str) doc1.get(ObjectId.ROOT, "key2").get()).getValue());
		Assertions.assertEquals("value3", ((AmValue.Str) doc1.get(ObjectId.ROOT, "key3").get()).getValue());
		Assertions.assertEquals("value", ((AmValue.Str) doc2.get(ObjectId.ROOT, "key").get()).getValue());
		Assertions.assertEquals("value2", ((AmValue.Str) doc2.get(ObjectId.ROOT, "key2").get()).getValue());
		Assertions.assertEquals("value3", ((AmValue.Str) doc2.get(ObjectId.ROOT, "key3").get()).getValue());
	}

	@Test
	public void testGenerateSyncMessageThrowsInTransaction() {
		Document doc1 = new Document();
		Transaction<ChangeHash> tx = doc1.startTransaction();
		SyncState syncState = new SyncState();
		Assertions.assertThrows(TransactionInProgress.class, () -> {
			doc1.generateSyncMessage(syncState);
		});
	}

	@Test
	public void testRecieveSyncMessageThrowsInTransaction() {
		Document doc1 = new Document();
		try (Transaction<ChangeHash> tx = doc1.startTransaction()) {
			tx.set(ObjectId.ROOT, "key", "value");
			tx.commit();
		}
		SyncState syncState = new SyncState();
		byte[] message = syncState.generateSyncMessage(doc1).get();
		Document doc2 = new Document();
		Transaction<ChangeHash> tx = doc2.startTransaction();
		SyncState syncState2 = new SyncState();
		Assertions.assertThrows(TransactionInProgress.class, () -> {
			doc2.receiveSyncMessage(syncState2, message);
		});
	}

	@Test
	public void testEncodeDecodeSyncState() {
		SyncState state = new SyncState();
		Document doc = new Document();
		try (Transaction<ChangeHash> tx = doc.startTransaction()) {
			tx.set(ObjectId.ROOT, "key", "value");
			tx.commit();
		}
		byte[] message = doc.generateSyncMessage(state).get();
		byte[] encodedState = state.encode();
		SyncState state2 = SyncState.decode(encodedState);
		byte[] message2 = doc.generateSyncMessage(state2).get();
		Assertions.assertArrayEquals(message, message2);
	}

	@Test
	public void testFree() {
		SyncState state = new SyncState();
		state.free();
	}

	void sync(Document docA, Document docB) {
		SyncState atob = new SyncState();
		SyncState btoa = new SyncState();
		sync(atob, btoa, docA, docB);
	}

	void sync(SyncState atob, SyncState btoa, Document docA, Document docB) {
		int iterations = 0;
		while (true) {
			Optional<byte[]> message1 = docA.generateSyncMessage(atob);
			if (message1.isPresent()) {
				docB.receiveSyncMessage(btoa, message1.get());
			}
			Optional<byte[]> message2 = docB.generateSyncMessage(btoa);
			if (message2.isPresent()) {
				docA.receiveSyncMessage(atob, message2.get());
			}
			if (!message1.isPresent() && !message2.isPresent()) {
				break;
			}
			iterations += 1;
			if (iterations >= 10) {
				throw new RuntimeException("Sync failed to converge");
			}
		}
	}

	@Test
	public void testSyncForPatches() {
		Document doc1 = new Document();
		try (Transaction<ChangeHash> tx = doc1.startTransaction()) {
			tx.set(ObjectId.ROOT, "key", "value");
			tx.commit();
		}

		Document doc2 = new Document();

		List<Patch> patches = syncForPatches(doc1, doc2);

		Assertions.assertEquals(patches.size(), 1);
		Patch patch = patches.get(0);
		Assertions.assertEquals(patch.getPath(), new ArrayList<>());
		Assertions.assertEquals(patch.getObj(), ObjectId.ROOT);
		PatchAction.PutMap action = (PatchAction.PutMap) patch.getAction();
		Assertions.assertEquals(action.getKey(), "key");
		Assertions.assertEquals(((AmValue.Str) action.getValue()).getValue(), "value");
	}

	// Run receiveSyncMessageForPatches on docB and return all patches that were
	// generated
	List<Patch> syncForPatches(Document docA, Document docB) {
		SyncState atob = new SyncState();
		SyncState btoa = new SyncState();
		ArrayList<Patch> patches = new ArrayList<>();
		int iterations = 0;
		while (true) {
			Optional<byte[]> message1 = docA.generateSyncMessage(atob);
			if (message1.isPresent()) {
				patches.addAll(docB.receiveSyncMessageForPatches(btoa, message1.get()));
			}
			Optional<byte[]> message2 = docB.generateSyncMessage(btoa);
			if (message2.isPresent()) {
				docA.receiveSyncMessage(atob, message2.get());
			}
			if (!message1.isPresent() && !message2.isPresent()) {
				break;
			}
			iterations += 1;
			if (iterations >= 10) {
				throw new RuntimeException("Sync failed to converge");
			}
		}
		return patches;
	}

	@Test
	public void testInSync() {
		Document doc1 = new Document();
		try (Transaction<ChangeHash> tx = doc1.startTransaction()) {
			tx.set(ObjectId.ROOT, "key", "value");
			tx.commit();
		}

		Document doc2 = doc1.fork();
		try (Transaction<ChangeHash> tx = doc2.startTransaction()) {
			tx.set(ObjectId.ROOT, "key2", "value2");
			tx.commit();
		}

		SyncState atob = new SyncState();
		SyncState btoa = new SyncState();
		Assertions.assertFalse(atob.isInSync(doc1));
		Assertions.assertFalse(btoa.isInSync(doc2));

		sync(atob, btoa, doc1, doc2);
		Assertions.assertTrue(atob.isInSync(doc1));
		Assertions.assertTrue(btoa.isInSync(doc2));

	}
}
