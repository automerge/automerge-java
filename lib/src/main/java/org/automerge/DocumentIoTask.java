package org.automerge;

import java.util.Objects;

/**
 * Represents IO tasks that a document actor can request.
 * This is an abstract base class with concrete subclasses for each task type.
 */
public abstract class DocumentIoTask {

    /**
     * Storage operation task - delegates to a StorageTask.
     */
    public static class Storage extends DocumentIoTask {

        private final StorageTask storageTask;

        Storage(StorageTask storageTask) {
            this.storageTask = Objects.requireNonNull(
                storageTask,
                "storageTask cannot be null"
            );
        }

        public StorageTask getStorageTask() {
            return storageTask;
        }

        @Override
        public boolean equals(Object obj) {
            if (this == obj) return true;
            if (obj == null || getClass() != obj.getClass()) return false;
            Storage storage = (Storage) obj;
            return Objects.equals(storageTask, storage.storageTask);
        }

        @Override
        public int hashCode() {
            return Objects.hash(storageTask);
        }

        @Override
        public String toString() {
            return "DocumentIoTask.Storage{storageTask=" + storageTask + "}";
        }
    }

    /**
     * Check announce policy task - verify if a peer should be announced to.
     */
    public static class CheckAnnouncePolicy extends DocumentIoTask {

        private final PeerId peerId;

        CheckAnnouncePolicy(PeerId peerId) {
            this.peerId = Objects.requireNonNull(
                peerId,
                "peerId cannot be null"
            );
        }

        public PeerId getPeerId() {
            return peerId;
        }

        @Override
        public boolean equals(Object obj) {
            if (this == obj) return true;
            if (obj == null || getClass() != obj.getClass()) return false;
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
