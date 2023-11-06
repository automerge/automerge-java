package org.automerge;

import java.util.Date;
import java.util.List;
import java.util.function.Function;
import org.junit.jupiter.api.Assertions;
import org.junit.jupiter.api.Test;

class TestMarks {

	@Test
	public void testCreateMarks() {
		Document doc = new Document();
		ObjectId text;
		Date now = new Date();
		try (Transaction tx = doc.startTransaction()) {
			text = tx.set(ObjectId.ROOT, "text", ObjectType.TEXT);
			tx.spliceText(text, 0, 0, "Hello");

			tx.mark(text, 1, 3, "bool", true, ExpandMark.BOTH);
			tx.mark(text, 1, 3, "string", "string", ExpandMark.BOTH);
			tx.mark(text, 1, 3, "bytes", "bytes".getBytes(), ExpandMark.BOTH);
			tx.mark(text, 1, 3, "int", -1, ExpandMark.BOTH);
			tx.markUint(text, 1, 3, "uint", 1, ExpandMark.BOTH);
			tx.mark(text, 1, 3, "float", 2.5, ExpandMark.BOTH);
			tx.mark(text, 1, 3, "date", now, ExpandMark.BOTH);
			tx.mark(text, 1, 3, "counter", new Counter(5), ExpandMark.BOTH);
			tx.spliceText(text, 1, 0, "oo");
			tx.commit();
		}

		List<Mark> marks = doc.marks(text);
		Assertions.assertEquals(8, marks.size());

		// Note that the marks are returned in alphabetical order

		Mark boolMark = marks.get(0);
		assertMark(boolMark, 1, 5, "bool", value -> Assertions.assertTrue(((AmValue.Bool) value).getValue()));

		Mark bytesMark = marks.get(1);
		assertMark(bytesMark, 1, 5, "bytes",
				value -> Assertions.assertArrayEquals(((AmValue.Bytes) value).getValue(), "bytes".getBytes()));

		Mark counterMark = marks.get(2);
		assertMark(counterMark, 1, 5, "counter",
				value -> Assertions.assertEquals(((AmValue.Counter) value).getValue(), 5));

		Mark dateMark = marks.get(3);
		assertMark(dateMark, 1, 5, "date",
				value -> Assertions.assertEquals(((AmValue.Timestamp) value).getValue(), now));

		Mark floatMark = marks.get(4);
		assertMark(floatMark, 1, 5, "float", value -> Assertions.assertEquals(((AmValue.F64) value).getValue(), 2.5));

		Mark intMark = marks.get(5);
		assertMark(intMark, 1, 5, "int", value -> Assertions.assertEquals(((AmValue.Int) value).getValue(), -1));

		Mark stringMark = marks.get(6);
		assertMark(stringMark, 1, 5, "string",
				value -> Assertions.assertEquals(((AmValue.Str) value).getValue(), "string"));

		Mark uintMark = marks.get(7);
		assertMark(uintMark, 1, 5, "uint", value -> Assertions.assertEquals(((AmValue.UInt) value).getValue(), 1));

	}

	@Test
	public void testNullMark() {
		testMarkNull(doc -> doc.startTransaction());
	}

	@Test
	public void testMarkPatch() {
		Document doc = new Document();
		ObjectId text;
		try (Transaction tx = doc.startTransaction()) {
			text = tx.set(ObjectId.ROOT, "text", ObjectType.TEXT);
			tx.spliceText(text, 0, 0, "Hello");
			tx.commit();
		}

		PatchLog patchLog = new PatchLog();
		try (Transaction tx = doc.startTransaction(patchLog)) {
			tx.mark(text, 0, 5, "bold", true, ExpandMark.BOTH);
			tx.commit();
		}
		List<Patch> patches = doc.makePatches(patchLog);

		Assertions.assertEquals(patches.size(), 1);
		Patch patch = patches.get(0);
		PatchAction.Mark action = (PatchAction.Mark) patch.getAction();

		Assertions.assertEquals(action.getMarks().length, 1);
		Mark mark = action.getMarks()[0];
		Assertions.assertEquals(mark.getName(), "bold");
		Assertions.assertEquals(mark.getStart(), 0);
		Assertions.assertEquals(mark.getEnd(), 5);
		Assertions.assertEquals(true, ((AmValue.Bool) mark.getValue()).getValue());
	}

	@Test
	public void testUnmark() {
		testUnmark(doc -> doc.startTransaction());
	}

	void testUnmark(Function<Document, Transaction> createTx) {
		Document doc = new Document();
		ObjectId text;
		try (Transaction tx = createTx.apply(doc)) {
			text = tx.set(ObjectId.ROOT, "text", ObjectType.TEXT);
			tx.spliceText(text, 0, 0, "Hello");
			tx.mark(text, 0, 5, "bold", true, ExpandMark.NONE);
			tx.commit();
		}

		try (Transaction tx = createTx.apply(doc)) {
			tx.unmark(text, "bold", 0, 5, ExpandMark.BOTH);
			tx.commit();
		}

		List<Mark> marks = doc.marks(text);
		Assertions.assertEquals(marks.size(), 0);
	}

	void testMarkNull(Function<Document, Transaction> createTx) {
		Document doc = new Document();
		ObjectId text;
		try (Transaction tx = createTx.apply(doc)) {
			text = tx.set(ObjectId.ROOT, "text", ObjectType.TEXT);
			tx.spliceText(text, 0, 0, "Hello");
			tx.mark(text, 0, 5, "comment", "1234", ExpandMark.NONE);
			tx.commit();
		}
		try (Transaction tx = createTx.apply(doc)) {
			tx.markNull(text, 0, 2, "comment", ExpandMark.BOTH);
			tx.commit();
		}

		List<Mark> marks = doc.marks(text);
		Assertions.assertEquals(1, marks.size());
		Mark mark = marks.get(0);
		assertMark(mark, 2, 5, "comment", value -> Assertions.assertEquals("1234", ((AmValue.Str) value).getValue()));
	}

	void assertMark(Mark mark, long start, long end, String name, MarkValueAssertion assertion) {
		Assertions.assertEquals(start, mark.getStart());
		Assertions.assertEquals(end, mark.getEnd());
		Assertions.assertEquals(name, mark.getName());
		assertion.assertMarkValue(mark.getValue());
	}

	private interface MarkValueAssertion {
		void assertMarkValue(AmValue value);
	}
}
