package org.automerge;

import java.util.HashMap;
import java.util.Optional;

public interface Storage {
	Optional<byte[]> load(StorageKey key);
	HashMap<StorageKey, byte[]> loadRange(StorageKey prefix);
	void put(StorageKey key, byte[] value);
	void delete(StorageKey key);
}
