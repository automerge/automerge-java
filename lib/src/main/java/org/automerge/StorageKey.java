package org.automerge;

import java.util.Arrays;

public class StorageKey {

	private String[] parts;

	public StorageKey(String... parts) {
		for (String part : parts) {
			if (part == null || part.isEmpty() || part.contains("/")) {
				throw new IllegalArgumentException("Parts cannot be null or empty or contain slashes");
			}
		}
		this.parts = parts;
	}

	public boolean isPrefixOf(StorageKey other) {
		if (other == null || other.parts == null || other.parts.length == 0) {
			return false;
		}
		if (parts.length > other.parts.length) {
			return false;
		}
		for (int i = 0; i < parts.length; i++) {
			if (!parts[i].equals(other.parts[i])) {
				return false;
			}
		}
		return true;
	}

	@Override
	public boolean equals(Object obj) {
		if (this == obj)
			return true;
		if (obj == null || getClass() != obj.getClass())
			return false;
		StorageKey other = (StorageKey) obj;
		return Arrays.equals(parts, other.parts);
	}

	@Override
	public int hashCode() {
		return Arrays.hashCode(parts);
	}
}
