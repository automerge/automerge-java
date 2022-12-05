package org.automerge;

import java.util.Optional;
import java.util.function.Function;
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
		Transaction<ChangeHash> tx = doc.startTransaction();
		text = tx.set(ObjectId.ROOT, "text", ObjectType.TEXT);
		tx.commit();
	}

	@Test
	public void testGet() {
		Transaction<ChangeHash> tx = doc.startTransaction();
		tx.spliceText(text, 0, 0, "hello");
		Assertions.assertEquals(Optional.of("hello"), tx.text(text));
		Assertions.assertEquals(Optional.of("hello"), doc.text(text));
	}

	@Test
	public void testGetObserved() {
		Transaction<HashAndPatches> tx = doc.startTransactionForPatches();
		tx.spliceText(text, 0, 0, "hello");
		Assertions.assertEquals(Optional.of("hello"), tx.text(text));
		Assertions.assertEquals(Optional.of("hello"), doc.text(text));
		tx.commit();
		Assertions.assertEquals(Optional.of("hello"), doc.text(text));
	}

	@Test
	public void testGetNonTextInDoc() {
		Document otherDoc = new Document();
		Transaction<ChangeHash> otherTx = otherDoc.startTransaction();
		ObjectId map = otherTx.set(ObjectId.ROOT, "map", ObjectType.MAP);
		otherTx.commit();
		Assertions.assertEquals(otherDoc.text(map), Optional.empty());
	}

	@Test
	public void testGetNonTextInTx() {
		Document otherDoc = new Document();
		Transaction<ChangeHash> otherTx = otherDoc.startTransaction();
		ObjectId map = otherTx.set(ObjectId.ROOT, "map", ObjectType.MAP);
		Assertions.assertEquals(otherTx.text(map), Optional.empty());
	}

	@Test
	public void testGetNonTextInObservedTx() {
		Document otherDoc = new Document();
		Transaction<HashAndPatches> otherTx = otherDoc.startTransactionForPatches();
		ObjectId map = otherTx.set(ObjectId.ROOT, "map", ObjectType.MAP);
		Assertions.assertEquals(otherTx.text(map), Optional.empty());
	}

	@Test
	public void testTextAt() {
		runTestTextAt(doc -> doc.startTransaction());
	}

	@Test
	public void testTextAtObserved() {
		runTestTextAt((doc) -> {
			return doc.startTransactionForPatches();
		});
	}

	<T> void runTestTextAt(Function<Document, Transaction<T>> createTx) {
		Transaction<T> tx = createTx.apply(doc);
		tx.spliceText(text, 0, 0, "hello");
		tx.commit();
		ChangeHash[] heads = doc.getHeads();
		tx = createTx.apply(doc);
		tx.spliceText(text, 5, 0, " world");
		Assertions.assertEquals(Optional.of("hello world"), tx.text(text));
		Assertions.assertEquals(Optional.of("hello"), tx.text(text, heads));
		Assertions.assertEquals(Optional.of("hello"), doc.text(text, heads));
		tx.commit();
		Assertions.assertEquals(Optional.of("hello"), doc.text(text, heads));
	}
}
