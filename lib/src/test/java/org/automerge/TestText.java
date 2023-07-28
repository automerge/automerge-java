package org.automerge;

import java.util.Optional;
import org.junit.jupiter.api.Assertions;
import org.junit.jupiter.api.BeforeEach;
import org.junit.jupiter.api.Test;

class TestText {
	private Document doc;
	private ObjectId text;

	public TestText() {
		super();
	}

	@BeforeEach
	public void setup() {
		doc = new Document();
		Transaction tx = doc.startTransaction();
		text = tx.set(ObjectId.ROOT, "text", ObjectType.TEXT);
		tx.commit();
	}

	@Test
	public void testGet() {
		Transaction tx = doc.startTransaction();
		tx.spliceText(text, 0, 0, "hello");
		Assertions.assertEquals(Optional.of("hello"), tx.text(text));
		Assertions.assertEquals(Optional.of("hello"), doc.text(text));
	}

	@Test
	public void testGetNonTextInDoc() {
		Document otherDoc = new Document();
		Transaction otherTx = otherDoc.startTransaction();
		ObjectId map = otherTx.set(ObjectId.ROOT, "map", ObjectType.MAP);
		otherTx.commit();
		Assertions.assertEquals(otherDoc.text(map), Optional.empty());
	}

	@Test
	public void testGetNonTextInTx() {
		Document otherDoc = new Document();
		Transaction otherTx = otherDoc.startTransaction();
		ObjectId map = otherTx.set(ObjectId.ROOT, "map", ObjectType.MAP);
		Assertions.assertEquals(otherTx.text(map), Optional.empty());
	}

	@Test
	public void testTextAt() {
		Transaction tx = doc.startTransaction();
		tx.spliceText(text, 0, 0, "hello");
		tx.commit();
		ChangeHash[] heads = doc.getHeads();
		tx = doc.startTransaction();
		tx.spliceText(text, 5, 0, " world");
		Assertions.assertEquals(Optional.of("hello world"), tx.text(text));
		Assertions.assertEquals(Optional.of("hello"), tx.text(text, heads));
		Assertions.assertEquals(Optional.of("hello"), doc.text(text, heads));
		tx.commit();
		Assertions.assertEquals(Optional.of("hello"), doc.text(text, heads));
	}
}
