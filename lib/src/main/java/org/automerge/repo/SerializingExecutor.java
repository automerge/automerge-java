package org.automerge.repo;

import java.util.concurrent.ConcurrentHashMap;
import java.util.concurrent.ConcurrentLinkedQueue;
import java.util.concurrent.Executor;
import java.util.concurrent.atomic.AtomicBoolean;

/**
 * An executor wrapper that ensures tasks for the same key never run
 * concurrently, while allowing tasks with different keys to run in parallel on
 * the underlying executor.
 *
 * This is used to ensure thread-safety for DocumentActors (which are not
 * thread-safe) while maintaining parallelism across different documents using a
 * work-stealing pool.
 *
 * @param <K>
 *            The key type (e.g., DocumentActorId)
 */
class SerializingExecutor<K> {

    private final Executor delegate;
    private final ConcurrentHashMap<K, ActorQueue> queues = new ConcurrentHashMap<>();

    /**
     * Creates a SerializingExecutor that wraps the given executor.
     *
     * @param delegate
     *            The underlying executor (typically a WorkStealingPool)
     */
    SerializingExecutor(Executor delegate) {
        this.delegate = delegate;
    }

    /**
     * Submits a task to be executed, ensuring it doesn't run concurrently with
     * other tasks for the same key.
     *
     * @param key
     *            The key identifying the resource (e.g., DocumentActorId)
     * @param task
     *            The task to execute
     */
    void execute(K key, Runnable task) {
        ActorQueue queue = queues.computeIfAbsent(key, k -> new ActorQueue());
        queue.submit(task);
    }

    /**
     * Removes the queue for a key when the actor is freed. Should be called when
     * the actor is no longer needed.
     *
     * @param key
     *            The key to clean up
     */
    void cleanup(K key) {
        queues.remove(key);
    }

    /**
     * Per-actor queue that ensures only one task runs at a time for this actor.
     */
    private class ActorQueue {

        private final ConcurrentLinkedQueue<Runnable> pending = new ConcurrentLinkedQueue<>();
        private final AtomicBoolean scheduled = new AtomicBoolean(false);

        void submit(Runnable task) {
            pending.offer(task);
            trySchedule();
        }

        private void trySchedule() {
            // Try to become the "scheduler" thread
            if (!scheduled.compareAndSet(false, true)) {
                // Someone else is already scheduled/running
                return;
            }

            // Submit a task that will drain the entire queue
            delegate.execute(this::drainQueue);
        }

        private void drainQueue() {
            // Drain all pending tasks sequentially
            while (true) {
                Runnable task = pending.poll();
                if (task == null) {
                    // Queue is empty, release the scheduler lock
                    scheduled.set(false);
                    // Check for race: task might have been added just before we released
                    if (!pending.isEmpty() && scheduled.compareAndSet(false, true)) {
                        // Continue draining
                        continue;
                    }
                    // Done
                    return;
                }

                // Execute task directly (we're already in the executor thread)
                // Don't catch exceptions - let them propagate to the task's own handler
                task.run();
            }
        }
    }
}
