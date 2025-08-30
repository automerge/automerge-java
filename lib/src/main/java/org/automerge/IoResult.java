package org.automerge;

import java.util.Objects;

/**
 * Represents the result of an IO task execution.
 * IoResults are created by the runtime and passed back to samod-core.
 * @param <T> The type of payload (StorageResult, etc.)
 */
public class IoResult<T> {

    private final IoTaskId taskId;
    private final T payload;

    /**
     * Creates an IoResult with the given task ID and payload.
     * @param taskId The task ID this result corresponds to
     * @param payload The result payload - must not be null
     */
    public IoResult(IoTaskId taskId, T payload) {
        this.taskId = Objects.requireNonNull(taskId, "taskId cannot be null");
        this.payload = Objects.requireNonNull(
            payload,
            "payload cannot be null"
        );
    }

    /**
     * Gets the task ID this result corresponds to.
     * @return The task ID
     */
    public IoTaskId getTaskId() {
        return taskId;
    }

    /**
     * Gets the result payload.
     * @return The result payload
     */
    public T getPayload() {
        return payload;
    }

    @Override
    public boolean equals(Object obj) {
        if (this == obj) return true;
        if (obj == null || getClass() != obj.getClass()) return false;
        IoResult<?> ioResult = (IoResult<?>) obj;
        return (
            Objects.equals(taskId, ioResult.taskId) &&
            Objects.equals(payload, ioResult.payload)
        );
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
