package org.automerge;

import org.junit.jupiter.api.Assertions;
import org.junit.jupiter.api.Test;

class TestGetAt {

	interface TestCase {
		void init(Transaction doc, String value);

		void update(Transaction tx, String value);

		AmValue get(Read r);

		AmValue getAt(Read r, ChangeHash[] heads);
	}

	void run(TestCase testcase) {
		// Steps
		// Create the document
		Document doc = new Document();
		// Create the first state and commit
		Transaction tx = doc.startTransaction();
		testcase.init(tx, "value1");
		tx.commit();
		// save the heads
		ChangeHash[] heads = doc.getHeads();
		// Create a new transaction
		tx = doc.startTransaction();
		// Run the second update
		testcase.update(tx, "value2");
		tx.commit();
		// Check that get returns current value
		tx = doc.startTransaction();
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

	@Test
	public void testGetAtInMap() {
		run(new TestCase() {
			@Override
			public void init(Transaction tx, String value) {
				tx.set(ObjectId.ROOT, "key", value);
			}

			@Override
			public void update(Transaction tx, String value) {
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
		});
	}

	@Test
	public void testGetAtInList() {
		run(new TestCase() {
			ObjectId list;

			@Override
			public void init(Transaction tx, String value) {
				list = tx.set(ObjectId.ROOT, "list", ObjectType.LIST);
				tx.insert(list, 0, value);
			}

			@Override
			public void update(Transaction tx, String value) {
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
		});
	}

}
