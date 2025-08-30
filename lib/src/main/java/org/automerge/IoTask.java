package org.automerge;

import java.util.Objects;

/**
 * Represents an IO task that needs to be executed by the runtime.
 * IoTasks are created by samod-core and contain the operation to be performed.
 * @param <T> The type of action (StorageTask, etc.)
 */
public class IoTask<T> {

    private final IoTaskId taskId;
    private final T action;

    /**
     * Creates an IoTask with the given task ID and action.
     * Package-private constructor - only called from JNI layer.
     * @param taskId The unique task identifier
     * @param action The action to be performed
     */
    IoTask(IoTaskId taskId, T action) {
        this.taskId = Objects.requireNonNull(taskId, "taskId cannot be null");
        this.action = Objects.requireNonNull(action, "action cannot be null");
    }

    /**
     * Gets the unique identifier for this task.
     * @return The task ID
     */
    public IoTaskId getTaskId() {
        return taskId;
    }

    /**
     * Gets the action to be performed.
     * @return The action object
     */
    public T getAction() {
        return action;
    }

    @Override
    public boolean equals(Object obj) {
        if (this == obj) return true;
        if (obj == null || getClass() != obj.getClass()) return false;
        IoTask<?> ioTask = (IoTask<?>) obj;
        return (
            Objects.equals(taskId, ioTask.taskId) &&
            Objects.equals(action, ioTask.action)
        );
    }

    @Override
    public int hashCode() {
        return Objects.hash(taskId, action);
    }

    @Override
    public String toString() {
        return "IoTask{taskId=" + taskId + ", action=" + action + "}";
    }
}
