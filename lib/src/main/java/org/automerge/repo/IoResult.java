package org.automerge.repo;

import java.util.Objects;

/**
 * Result of an IO task execution. Created by the runtime and passed back to
 * samod-core by the runtime.
 *
 * @param <T>
 *            the payload type (e.g. {@link StorageResult})
 */
class IoResult<T> {

    private final IoTaskId taskId;
    private final T payload;

    IoResult(IoTaskId taskId, T payload) {
        this.taskId = Objects.requireNonNull(taskId, "taskId cannot be null");
        this.payload = Objects.requireNonNull(payload, "payload cannot be null");
    }

    IoTaskId getTaskId() {
        return taskId;
    }

    T getPayload() {
        return payload;
    }

    @Override
    public boolean equals(Object obj) {
        if (this == obj)
            return true;
        if (obj == null || getClass() != obj.getClass())
            return false;
        IoResult<?> ioResult = (IoResult<?>) obj;
        return Objects.equals(taskId, ioResult.taskId) && Objects.equals(payload, ioResult.payload);
    }

    @Override
    public int hashCode() {
        return Objects.hash(taskId, payload);
    }

    @Override
    public String toString() {
        return "IoResult{taskId=" + taskId + ", payload=" + payload + "}";
    }
}
