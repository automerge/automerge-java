package org.automerge;

import java.util.HashSet;
import java.util.function.Function;
import org.junit.jupiter.api.Assertions;
import org.junit.jupiter.api.Test;

class TestGetAll {
	interface TestCase {
		void init(Transaction tx, double value);

		void branch1(Transaction tx, double value);

		void branch2(Transaction tx, double value);

		void merge(Transaction tx, double value);

		Conflicts getConflicts(Read doc);

		Conflicts getConflictsAt(Read doc, ChangeHash[] heads);
	}

	@Test
	public void testGetAllInMap() {
		runMap(doc -> doc.startTransaction());
	}

	@Test
	public void testGetAllInList() {
		runList(doc -> doc.startTransaction());
	}

	public void runMap(Function<Document, Transaction> createTx) {
		run(new TestCase() {
			public void init(Transaction tx, double value) {
				tx.set(ObjectId.ROOT, "key", value);
			}

			public void branch1(Transaction tx, double value) {
				tx.set(ObjectId.ROOT, "key", value);
			}

			public void branch2(Transaction tx, double value) {
				tx.set(ObjectId.ROOT, "key", value);
			}

			public void merge(Transaction tx, double value) {
				tx.set(ObjectId.ROOT, "key", value);
			}

			public Conflicts getConflicts(Read doc) {
				return doc.getAll(ObjectId.ROOT, "key").get();
			}

			public Conflicts getConflictsAt(Read doc, ChangeHash[] heads) {
				return doc.getAll(ObjectId.ROOT, "key", heads).get();
			}
		}, createTx);
	}

	public void runList(Function<Document, Transaction> createTx) {
		run(new TestCase() {
			ObjectId list;

			public void init(Transaction tx, double value) {
				list = tx.set(ObjectId.ROOT, "list", ObjectType.LIST);
				tx.insert(list, 0, value);
			}

			public void branch1(Transaction tx, double value) {
				tx.set(list, 0, value);
			}

			public void branch2(Transaction tx, double value) {
				tx.set(list, 0, value);
			}

			public void merge(Transaction tx, double value) {
				tx.set(list, 0, value);
			}

			public Conflicts getConflicts(Read doc) {
				return doc.getAll(list, 0).get();
			}

			public Conflicts getConflictsAt(Read doc, ChangeHash[] heads) {
				return doc.getAll(list, 0, heads).get();
			}
		}, createTx);
	}

	void run(TestCase testCase, Function<Document, Transaction> createTx) {
		Document doc = new Document();
		try (Transaction tx = createTx.apply(doc)) {
			testCase.init(tx, 1.23);
			tx.commit();
		}
		Document doc2 = doc.fork();
		try (Transaction tx = createTx.apply(doc)) {
			testCase.branch1(tx, 4.56);
			tx.commit();
		}
		try (Transaction tx = createTx.apply(doc2)) {
			testCase.branch2(tx, 7.89);
			tx.commit();
		}
		doc.merge(doc2);
		ChangeHash[] heads = doc.getHeads();
		// Check it works with an open transaction
		Transaction tx = createTx.apply(doc);
		Conflicts conflicts = testCase.getConflicts(doc);
		assertConflicts(conflicts);

		// check the version from the transaction works
		conflicts = testCase.getConflicts(tx);
		assertConflicts(conflicts);

		// Check works without an open transaction
		tx.commit();
		conflicts = testCase.getConflicts(doc);
		assertConflicts(conflicts);

		// create a merge commit
		try (Transaction tx2 = createTx.apply(doc)) {
			testCase.merge(tx2, 2.00);
			tx2.commit();
		}

		// Check there are no conflicts after the merge
		conflicts = testCase.getConflicts(doc);
		Assertions.assertEquals(1, conflicts.values().size());

		// Check getAllAt works with an open transaction
		tx = createTx.apply(doc);
		conflicts = testCase.getConflictsAt(doc, heads);
		assertConflicts(conflicts);

		// check the version from the transaction works
		conflicts = testCase.getConflictsAt(tx, heads);
		assertConflicts(conflicts);

		// Check works without an open transaction
		tx.commit();
		conflicts = testCase.getConflictsAt(doc, heads);
		assertConflicts(conflicts);
	}

	void assertConflicts(Conflicts conflicts) {
		Assertions.assertEquals(2, conflicts.values().size());

		HashSet<Double> expected = new HashSet<Double>();
		expected.add(4.56);
		expected.add(7.89);

		HashSet<Double> values = new HashSet<Double>();
		for (AmValue value : conflicts.values()) {
			values.add(((AmValue.F64) value).getValue());
		}
		Assertions.assertEquals(expected, values);
	}
}
