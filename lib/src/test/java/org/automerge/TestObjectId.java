package org.automerge;

import org.junit.jupiter.api.Assertions;
import org.junit.jupiter.api.Test;

public final class TestObjectId {

	public TestObjectId() {
		super();
	}

	@Test
	public final void rootObj() {
		ObjectId root = ObjectId.ROOT;
		Assertions.assertTrue(root.isRoot());
	}
}
