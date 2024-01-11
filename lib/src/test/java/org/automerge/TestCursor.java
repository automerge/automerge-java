package org.automerge;

import org.junit.jupiter.api.Assertions;
import org.junit.jupiter.api.BeforeEach;
import org.junit.jupiter.api.Test;

public final class TestCursor {
	private Document doc;
	private ObjectId text;

	@BeforeEach
	public void setup() {
		doc = new Document();
		try (Transaction tx = doc.startTransaction()) {
			text = tx.set(ObjectId.ROOT, "text", ObjectType.TEXT);
			tx.spliceText(text, 0, 0, "hello world");
			tx.commit();
		}
	}

	@Test
	public void testCursorInDoc() {
		Cursor cursor = doc.makeCursor(text, 3);
		Assertions.assertEquals(doc.lookupCursorIndex(text, cursor), 3);

		ChangeHash[] heads = doc.getHeads();

		try (Transaction tx = doc.startTransaction()) {
			tx.spliceText(text, 3, 0, "!");
			tx.commit();
		}

		Assertions.assertEquals(doc.lookupCursorIndex(text, cursor), 4);
		Assertions.assertEquals(doc.lookupCursorIndex(text, cursor, heads), 3);

		Cursor oldCursor = doc.makeCursor(text, 3, heads);
		Assertions.assertEquals(doc.lookupCursorIndex(text, oldCursor), 4);
		Assertions.assertEquals(doc.lookupCursorIndex(text, oldCursor, heads), 3);

	}

	@Test
	public void testCursorInTx() {
		ChangeHash[] heads = doc.getHeads();
		Cursor cursor;
		try (Transaction tx = doc.startTransaction()) {
			cursor = tx.makeCursor(text, 3);
			Assertions.assertEquals(tx.lookupCursorIndex(text, cursor), 3);
			tx.spliceText(text, 3, 0, "!");
			Assertions.assertEquals(tx.lookupCursorIndex(text, cursor), 4);
			tx.commit();
		}

		try (Transaction tx = doc.startTransaction()) {
			Cursor oldCursor = tx.makeCursor(text, 3, heads);
			Assertions.assertEquals(tx.lookupCursorIndex(text, oldCursor), 4);
			Assertions.assertEquals(tx.lookupCursorIndex(text, oldCursor, heads), 3);
			tx.commit();
		}
	}
}
