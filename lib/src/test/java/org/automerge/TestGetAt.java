package org.automerge;

import java.util.function.Function;
import org.junit.jupiter.api.Assertions;
import org.junit.jupiter.api.Test;

class TestGetAt {

	interface TestCase<T> {
		void init(Transaction<T> doc, String value);

		void update(Transaction<T> tx, String value);

		AmValue get(Read r);

		AmValue getAt(Read r, ChangeHash[] heads);

		Transaction<T> createTransaction(Document doc);
	}

	<T> void run(TestCase<T> testcase) {
		// Steps
		// Create the document
		Document doc = new Document();
		// Create the first state and commit
		Transaction<T> tx = testcase.createTransaction(doc);
		testcase.init(tx, "value1");
		tx.commit();
		// save the heads
		ChangeHash[] heads = doc.getHeads();
		// Create a new transaction
		tx = testcase.createTransaction(doc);
		// Run the second update
		testcase.update(tx, "value2");
		tx.commit();
		// Check that get returns current value
		tx = testcase.createTransaction(doc);
		// Check that get and getAt on open tx work as expected
		Assertions.assertEquals("value2", ((AmValue.Str) testcase.get(tx)).getValue());
		Assertions.assertEquals("value1", ((AmValue.Str) testcase.getAt(tx, heads)).getValue());

		// Check that get and getAt on document with open tx work
		Assertions.assertEquals("value2", ((AmValue.Str) testcase.get(doc)).getValue());
		Assertions.assertEquals("value1", ((AmValue.Str) testcase.getAt(doc, heads)).getValue());

		tx.commit();

		// Check that get and getAt on doc with closed tx work as expected
		Assertions.assertEquals("value2", ((AmValue.Str) testcase.get(doc)).getValue());
		Assertions.assertEquals("value1", ((AmValue.Str) testcase.getAt(doc, heads)).getValue());
	}

	<T> void runMap(Function<Document, Transaction<T>> c) {
		run(new TestCase<T>() {
			@Override
			public void init(Transaction<T> tx, String value) {
				tx.set(ObjectId.ROOT, "key", value);
			}

			@Override
			public void update(Transaction<T> tx, String value) {
				tx.set(ObjectId.ROOT, "key", value);
			}

			@Override
			public AmValue get(Read r) {
				return r.get(ObjectId.ROOT, "key").get();
			}

			@Override
			public AmValue getAt(Read r, ChangeHash[] heads) {
				return r.get(ObjectId.ROOT, "key", heads).get();
			}

			@Override
			public Transaction<T> createTransaction(Document doc) {
				return c.apply(doc);
			}
		});
	}

	<T> void runList(Function<Document, Transaction<T>> c) {
		run(new TestCase<T>() {
			ObjectId list;

			@Override
			public void init(Transaction<T> tx, String value) {
				list = tx.set(ObjectId.ROOT, "list", ObjectType.LIST);
				tx.insert(list, 0, value);
			}

			@Override
			public void update(Transaction<T> tx, String value) {
				tx.set(list, 0, value);
			}

			@Override
			public AmValue get(Read r) {
				return r.get(list, 0).get();
			}

			@Override
			public AmValue getAt(Read r, ChangeHash[] heads) {
				return r.get(list, 0, heads).get();
			}

			@Override
			public Transaction<T> createTransaction(Document doc) {
				return c.apply(doc);
			}
		});
	}

	@Test
	public void testGetAtInMap() {
		runMap(doc -> doc.startTransaction());
	}

	@Test
	public void testGetAtInList() {
		runList(doc -> doc.startTransaction());
	}

	@Test
	public void testGetAtInMapInObservedTx() {
		runMap(doc -> {
			return doc.startTransactionForPatches();
		});
	}

	@Test
	public void testGetAtInListInObservedTx() {
		runList(doc -> {
			return doc.startTransactionForPatches();
		});
	}
}
