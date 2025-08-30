package org.automerge.storage;

import java.util.HashMap;
import java.util.Optional;
import org.automerge.Storage;
import org.automerge.StorageKey;

public class InMemoryStorage implements Storage {

	private HashMap<StorageKey, byte[]> data = new HashMap<>();

	@Override
	public Optional<byte[]> load(StorageKey key) {
		return Optional.ofNullable(data.get(key));
	}

	@Override
	public HashMap<StorageKey, byte[]> loadRange(StorageKey prefix) {
		HashMap<StorageKey, byte[]> result = new HashMap<>();
		for (StorageKey key : data.keySet()) {
			if (prefix.isPrefixOf(key)) {
				result.put(key, data.get(key));
			}
		}
		return result;
	}

	@Override
	public void put(StorageKey key, byte[] value) {
		data.put(key, value);
	}

	@Override
	public void delete(StorageKey key) {
		data.remove(key);
	}
}
