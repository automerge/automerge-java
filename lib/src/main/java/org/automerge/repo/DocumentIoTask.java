package org.automerge.repo;

import java.util.Objects;

/**
 * Represents IO tasks that a document actor can request. This is an abstract
 * base class with concrete subclasses for each task type.
 */
abstract class DocumentIoTask {

    /**
     * Storage operation task - delegates to a StorageTask.
     *
     * <p>
     * Note: The field is named {@code value0} because the JNI layer generates tuple
     * variant fields with indexed names ({@code value0}, {@code value1}, etc.).
     */
    static class Storage extends DocumentIoTask {

        private final StorageTask value0;

        Storage(StorageTask value0) {
            this.value0 = Objects.requireNonNull(value0, "storageTask cannot be null");
        }

        public StorageTask getStorageTask() {
            return value0;
        }

        @Override
        public boolean equals(Object obj) {
            if (this == obj)
                return true;
            if (obj == null || getClass() != obj.getClass())
                return false;
            Storage storage = (Storage) obj;
            return Objects.equals(value0, storage.value0);
        }

        @Override
        public int hashCode() {
            return Objects.hash(value0);
        }

        @Override
        public String toString() {
            return "DocumentIoTask.Storage{storageTask=" + value0 + "}";
        }
    }

    /**
     * Check announce policy task - verify if a peer should be announced to.
     */
    static class CheckAnnouncePolicy extends DocumentIoTask {

        private final PeerId peerId;

        CheckAnnouncePolicy(PeerId peerId) {
            this.peerId = Objects.requireNonNull(peerId, "peerId cannot be null");
        }

        public PeerId getPeerId() {
            return peerId;
        }

        @Override
        public boolean equals(Object obj) {
            if (this == obj)
                return true;
            if (obj == null || getClass() != obj.getClass())
                return false;
            CheckAnnouncePolicy that = (CheckAnnouncePolicy) obj;
            return Objects.equals(peerId, that.peerId);
        }

        @Override
        public int hashCode() {
            return Objects.hash(peerId);
        }

        @Override
        public String toString() {
            return "DocumentIoTask.CheckAnnouncePolicy{peerId=" + peerId + "}";
        }
    }
}
