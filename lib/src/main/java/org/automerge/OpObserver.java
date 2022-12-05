package org.automerge;

interface OpObserver {
	default void set(ObjectId obj, String key, AmValue value, boolean isConflict) {
	}

	default void set(ObjectId obj, long idx, AmValue value, boolean isConflict) {
	}

	default void insert(ObjectId obj, long idx, AmValue value) {
	}

	default void spliceText(ObjectId obj, long start, String value) {
	}

	default void expose(ObjectId obj, String key, AmValue value, boolean isConflict) {
	}

	default void expose(ObjectId obj, long idx, AmValue value, boolean isConflict) {
	}

	default void flagConflict(ObjectId obj, String key) {
	}

	default void flagConflict(ObjectId obj, long idx) {
	}

	default void increment(ObjectId obj, String key, long value) {
	}

	default void increment(ObjectId obj, long index, long value) {
	}

	default void delete(ObjectId obj, String key) {
	}

	default void delete(ObjectId obj, long index, long num) {
	}
}
