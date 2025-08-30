package org.automerge;

import java.util.HashMap;
import java.util.Optional;
import org.automerge.storage.InMemoryStorage;
import org.junit.jupiter.api.Assertions;
import org.junit.jupiter.api.Test;

class TestInMemoryStorage {

	@Test
	public void testCreate() {
		InMemoryStorage storage = new InMemoryStorage();
	}

	@Test
	public void testPut() {
		InMemoryStorage storage = new InMemoryStorage();
		StorageKey key = new StorageKey("one", "two");
		byte[] data = "hello".getBytes();
		storage.put(key, data);
		Optional<byte[]> result = storage.load(key);
		Assertions.assertTrue(result.isPresent());
		Assertions.assertArrayEquals(data, result.get());
	}

	@Test
	public void testDelete() {
		InMemoryStorage storage = new InMemoryStorage();
		StorageKey key = new StorageKey("one", "two");
		byte[] data = "hello".getBytes();
		storage.put(key, data);
		storage.delete(key);
		Optional<byte[]> result = storage.load(key);
		Assertions.assertFalse(result.isPresent());
	}

	@Test
	public void testLoadRange() {
		InMemoryStorage storage = new InMemoryStorage();
		StorageKey key1 = new StorageKey("one", "two");
		StorageKey key2 = new StorageKey("one", "three");
		StorageKey key3 = new StorageKey("two", "four");
		byte[] data1 = "hello".getBytes();
		byte[] data2 = "world".getBytes();
		storage.put(key1, data1);
		storage.put(key2, data2);
		storage.put(key3, data1);
		HashMap<StorageKey, byte[]> results = storage.loadRange(new StorageKey("one"));
		Assertions.assertEquals(2, results.size());
		Assertions.assertArrayEquals(data1, results.get(key1));
		Assertions.assertArrayEquals(data2, results.get(key2));
	}
}
