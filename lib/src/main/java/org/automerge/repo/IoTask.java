package org.automerge.repo;

import java.util.Objects;

/**
 * An IO task that needs to be executed by the runtime. Produced by
 * samod-core and carries a task identifier together with the operation to
 * perform (a {@link StorageTask}, a document IO task, etc.).
 *
 * @param <T>
 *            the action type
 */
class IoTask<T> {

    private final IoTaskId taskId;
    private final T action;

    IoTask(IoTaskId taskId, T action) {
        this.taskId = Objects.requireNonNull(taskId, "taskId cannot be null");
        this.action = Objects.requireNonNull(action, "action cannot be null");
    }

    IoTaskId getTaskId() {
        return taskId;
    }

    T getAction() {
        return action;
    }

    @Override
    public boolean equals(Object obj) {
        if (this == obj)
            return true;
        if (obj == null || getClass() != obj.getClass())
            return false;
        IoTask<?> ioTask = (IoTask<?>) obj;
        return Objects.equals(taskId, ioTask.taskId) && Objects.equals(action, ioTask.action);
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
