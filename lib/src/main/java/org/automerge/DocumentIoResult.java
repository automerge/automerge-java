package org.automerge;

import java.util.Objects;

public abstract class DocumentIoResult {

    public static final class Storage extends DocumentIoResult {
        private final StorageResult storageResult;

        Storage(StorageResult storageResult) {
            this.storageResult = Objects.requireNonNull(storageResult, "storageResult cannot be null");
        }

        public StorageResult getStorageResult() {
            return storageResult;
        }

        @Override
        public boolean equals(Object o) {
            if (this == o) return true;
            if (o == null || getClass() != o.getClass()) return false;
            Storage storage = (Storage) o;
            return Objects.equals(storageResult, storage.storageResult);
        }

        @Override
        public int hashCode() {
            return Objects.hash(storageResult);
        }

        @Override
        public String toString() {
            return "DocumentIoResult.Storage{" +
                    "storageResult=" + storageResult +
                    '}';
        }
    }

    public static final class CheckAnnouncePolicy extends DocumentIoResult {
        private final boolean shouldAnnounce;

        CheckAnnouncePolicy(boolean shouldAnnounce) {
            this.shouldAnnounce = shouldAnnounce;
        }

        public boolean getShouldAnnounce() {
            return shouldAnnounce;
        }

        @Override
        public boolean equals(Object o) {
            if (this == o) return true;
            if (o == null || getClass() != o.getClass()) return false;
            CheckAnnouncePolicy that = (CheckAnnouncePolicy) o;
            return shouldAnnounce == that.shouldAnnounce;
        }

        @Override
        public int hashCode() {
            return Objects.hash(shouldAnnounce);
        }

        @Override
        public String toString() {
            return "DocumentIoResult.CheckAnnouncePolicy{" +
                    "shouldAnnounce=" + shouldAnnounce +
                    '}';
        }
    }
}
