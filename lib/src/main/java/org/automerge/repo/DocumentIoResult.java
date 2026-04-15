package org.automerge.repo;

import java.util.Objects;

abstract class DocumentIoResult {

    static final class Storage extends DocumentIoResult {

        private final StorageResult value0;

        Storage(StorageResult value0) {
            this.value0 = Objects.requireNonNull(value0, "storageResult cannot be null");
        }

        public StorageResult getStorageResult() {
            return value0;
        }

        @Override
        public boolean equals(Object o) {
            if (this == o)
                return true;
            if (o == null || getClass() != o.getClass())
                return false;
            Storage storage = (Storage) o;
            return Objects.equals(value0, storage.value0);
        }

        @Override
        public int hashCode() {
            return Objects.hash(value0);
        }

        @Override
        public String toString() {
            return ("DocumentIoResult.Storage{" + "storageResult=" + value0 + '}');
        }
    }

    static final class CheckAnnouncePolicy extends DocumentIoResult {

        private final boolean value0;

        CheckAnnouncePolicy(boolean value0) {
            this.value0 = value0;
        }

        public boolean getShouldAnnounce() {
            return value0;
        }

        @Override
        public boolean equals(Object o) {
            if (this == o)
                return true;
            if (o == null || getClass() != o.getClass())
                return false;
            CheckAnnouncePolicy that = (CheckAnnouncePolicy) o;
            return value0 == that.value0;
        }

        @Override
        public int hashCode() {
            return Objects.hash(value0);
        }

        @Override
        public String toString() {
            return ("DocumentIoResult.CheckAnnouncePolicy{" + "shouldAnnounce=" + value0 + '}');
        }
    }
}
