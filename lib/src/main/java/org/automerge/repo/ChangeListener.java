package org.automerge.repo;

/**
 * A listener that is called when a document changes.
 *
 * <p>
 * <strong>Important:</strong> do not call {@link
 * DocHandle#withDocument(java.util.function.Function)} from within a listener
 * as this
 * will cause deadlocks.
 * 
 * <p>
 * Change listeners are called from within a task on the threadpool which
 * runs all the documents. Calling {@link
 * DocHandle#withDocument(java.util.function.Function)} and blocking on the
 * result
 * from within a listener <strong>will deadlock</strong> — those methods queue
 * tasks on the same executor, which cannot run until the current task (which is
 * calling the listener) completes.
 *
 * <p>
 * Listeners must treat the event as a notification/trigger only. They should
 * return quickly — blocking delays document processing for the same document.
 */
@FunctionalInterface
public interface ChangeListener {

    /**
     * Called when the document has changed.
     *
     * @param event
     *              The change event containing the new heads
     */
    void onDocumentChanged(DocumentChanged event);
}
