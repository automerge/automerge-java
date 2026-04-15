package org.automerge.repo;

import java.util.concurrent.CopyOnWriteArrayList;
import java.util.concurrent.atomic.AtomicBoolean;

/**
 * A registration handle returned when adding a change listener via {@link DocHandle#addChangeListener(ChangeListener)}.
 *
 * <p>
 * Call {@link #remove()} to unregister the listener. The remove operation is
 * idempotent and thread-safe — calling it multiple times has no additional
 * effect.
 */
public class ListenerRegistration {

    private final AtomicBoolean removed = new AtomicBoolean(false);
    private final ChangeListener listener;
    private final CopyOnWriteArrayList<ChangeListener> listenerList;

    ListenerRegistration(ChangeListener listener, CopyOnWriteArrayList<ChangeListener> listenerList) {
        this.listener = listener;
        this.listenerList = listenerList;
    }

    /**
     * Removes the listener. This method is idempotent and thread-safe.
     */
    public void remove() {
        if (removed.compareAndSet(false, true)) {
            listenerList.remove(listener);
        }
    }
}
