package org.automerge;

import java.util.ArrayList;
import java.util.Date;
import org.junit.jupiter.api.Assertions;
import org.junit.jupiter.api.Test;

class TestGet {

	interface ValueCheck {
	}

	interface TestCase extends ValueCheck {
		void set(Transaction tx, ObjectId obj, String key);

		void set(Transaction tx, ObjectId obj, long idx);

		void check(AmValue value);
	}

	ArrayList<TestCase> makeTestCases() {
		ArrayList<TestCase> testCases = new ArrayList<TestCase>();
		// Uint
		testCases.add(new TestCase() {
			@Override
			public void set(Transaction tx, ObjectId obj, String key) {
				tx.set(obj, key, NewValue.uint(1));
				// tx.setUint(obj, key, 1);
			}

			@Override
			public void set(Transaction tx, ObjectId obj, long idx) {
				tx.set(obj, idx, NewValue.uint(1));
			}

			@Override
			public void check(AmValue value) {
				Assertions.assertEquals(1, ((AmValue.UInt) value).getValue());
			}
		});
		// Int
		testCases.add(new TestCase() {
			@Override
			public void set(Transaction tx, ObjectId obj, String key) {
				tx.set(obj, key, 1);
			}

			@Override
			public void set(Transaction tx, ObjectId obj, long idx) {
				tx.set(obj, idx, 1);
			}

			@Override
			public void check(AmValue value) {
				Assertions.assertEquals(1, ((AmValue.Int) value).getValue());
			}
		});
		// F64
		testCases.add(new TestCase() {
			@Override
			public void set(Transaction tx, ObjectId obj, String key) {
				tx.set(obj, key, 2.0);
			}

			@Override
			public void set(Transaction tx, ObjectId obj, long idx) {
				tx.set(obj, idx, 2.0);
			}

			@Override
			public void check(AmValue value) {
				assert ((AmValue.F64) value).getValue() == 2.0;
			}
		});
		// Bool
		testCases.add(new TestCase() {
			@Override
			public void set(Transaction tx, ObjectId obj, String key) {
				tx.set(obj, key, true);
			}

			@Override
			public void set(Transaction tx, ObjectId obj, long idx) {
				tx.set(obj, idx, true);
			}

			@Override
			public void check(AmValue value) {
				Assertions.assertEquals(true, ((AmValue.Bool) value).getValue());
			}
		});
		// Str
		testCases.add(new TestCase() {
			@Override
			public void set(Transaction tx, ObjectId obj, String key) {
				tx.set(obj, key, "hello");
			}

			public void set(Transaction tx, ObjectId obj, long idx) {
				tx.set(obj, idx, "hello");
			}

			@Override
			public void check(AmValue value) {
				Assertions.assertEquals("hello", ((AmValue.Str) value).getValue());
			}
		});
		// Bytes
		testCases.add(new TestCase() {
			@Override
			public void set(Transaction tx, ObjectId obj, String key) {
				tx.set(obj, key, new byte[]{1, 2, 3});
			}

			public void set(Transaction tx, ObjectId obj, long idx) {
				tx.set(obj, idx, new byte[]{1, 2, 3});
			}

			@Override
			public void check(AmValue value) {
				Assertions.assertArrayEquals(new byte[]{1, 2, 3}, ((AmValue.Bytes) value).getValue());
			}
		});
		// Counter
		testCases.add(new TestCase() {
			@Override
			public void set(Transaction tx, ObjectId obj, String key) {
				tx.set(obj, key, new Counter(1));
			}

			public void set(Transaction tx, ObjectId obj, long idx) {
				tx.set(obj, idx, new Counter(1));
			}

			@Override
			public void check(AmValue value) {
				Assertions.assertEquals(1, ((AmValue.Counter) value).getValue());
			}
		});
		// Timestamp
		testCases.add(new TestCase() {
			private Date date = new Date();

			@Override
			public void set(Transaction tx, ObjectId obj, String key) {
				tx.set(obj, key, date);
			}

			public void set(Transaction tx, ObjectId obj, long idx) {
				tx.set(obj, idx, date);
			}

			@Override
			public void check(AmValue value) {
				Assertions.assertEquals(date, ((AmValue.Timestamp) value).getValue());
			}
		});
		// Null
		testCases.add(new TestCase() {
			@Override
			public void set(Transaction tx, ObjectId obj, String key) {
				tx.set(obj, key, NewValue.NULL);
			}

			public void set(Transaction tx, ObjectId obj, long idx) {
				tx.set(obj, idx, NewValue.NULL);
			}

			@Override
			public void check(AmValue value) {
				Assertions.assertInstanceOf(AmValue.Null.class, value);
			}
		});
		// List
		testCases.add(new TestCase() {
			ObjectId list;

			@Override
			public void set(Transaction tx, ObjectId obj, String key) {
				list = tx.set(obj, key, ObjectType.LIST);
			}

			public void set(Transaction tx, ObjectId obj, long idx) {
				list = tx.set(obj, idx, ObjectType.LIST);
			}

			@Override
			public void check(AmValue value) {
				Assertions.assertEquals(list, ((AmValue.List) value).getId());
			}
		});
		// Map
		testCases.add(new TestCase() {
			ObjectId map;

			@Override
			public void set(Transaction tx, ObjectId obj, String key) {
				map = tx.set(obj, key, ObjectType.MAP);
			}

			public void set(Transaction tx, ObjectId obj, long idx) {
				map = tx.set(obj, idx, ObjectType.MAP);
			}

			@Override
			public void check(AmValue value) {
				Assertions.assertEquals(map, ((AmValue.Map) value).getId());
			}
		});
		// Text
		testCases.add(new TestCase() {
			ObjectId text;

			@Override
			public void set(Transaction tx, ObjectId obj, String key) {
				text = tx.set(obj, key, ObjectType.TEXT);
			}

			public void set(Transaction tx, ObjectId obj, long idx) {
				text = tx.set(obj, idx, ObjectType.TEXT);
			}

			@Override
			public void check(AmValue value) {
				Assertions.assertEquals(text, ((AmValue.Text) value).getId());
			}
		});
		return testCases;
	}

	@Test
	public void testGetMap() {
		for (TestCase testCase : makeTestCases()) {
			Document doc = new Document();
			try (Transaction tx = doc.startTransaction()) {
				testCase.set(tx, ObjectId.ROOT, "key");
				testCase.check(tx.get(ObjectId.ROOT, "key").get());
				testCase.check(doc.get(ObjectId.ROOT, "key").get());
				tx.commit();
				testCase.check(doc.get(ObjectId.ROOT, "key").get());
			}
		}
	}

	@Test
	public void testGetList() {
		for (TestCase testCase : makeTestCases()) {
			Document doc = new Document();
			try (Transaction tx = doc.startTransaction()) {
				ObjectId list = tx.set(ObjectId.ROOT, "list", ObjectType.LIST);
				tx.insert(list, 0, NewValue.NULL);
				testCase.set(tx, list, 0);
				testCase.check(tx.get(list, 0).get());
				testCase.check(doc.get(list, 0).get());
				tx.commit();
				testCase.check(doc.get(list, 0).get());
			}
		}
	}
}
