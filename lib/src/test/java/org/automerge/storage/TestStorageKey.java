package org.automerge;

import org.junit.jupiter.api.Assertions;
import org.junit.jupiter.api.Test;

class TestStorageKey {

	@Test
	public void testCreate() {
		StorageKey key = new StorageKey("one", "two");
	}

	@Test
	public void testEquals() {
		StorageKey key = new StorageKey("one", "two");
		StorageKey other = new StorageKey("one", "two");
		Assertions.assertTrue(key.equals(other));
	}

	@Test
	public void testHashCode() {
		StorageKey key1 = new StorageKey("one", "two");
		StorageKey key2 = new StorageKey("one", "two");
		Assertions.assertEquals(key1.hashCode(), key2.hashCode());
	}

	@Test
	public void testIsPrefixOf() {
		StorageKey key = new StorageKey("one", "two");
		StorageKey prefix = new StorageKey("one");
		Assertions.assertTrue(prefix.isPrefixOf(key));
		Assertions.assertFalse(key.isPrefixOf(prefix));
	}
}
