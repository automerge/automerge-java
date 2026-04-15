package org.automerge.repo;

import java.util.Objects;
import java.util.concurrent.CompletableFuture;
import java.util.function.Function;
import org.automerge.Document;

/**
 * A handle to a document in a repository.
 *
 * A DocHandle represents an Automerge document that the repository is managing.
 * Changes made to the document will be saved to storage, propagated to other
 * local processes, and replicated over the network. Thus the main purpose of
 * the handle is to enqueue new changes to the document via {@link
 * #withDocument}, and listen for changes via {@link #addChangeListener}.
 *
 * <p>
 * {@link #withDocument} hands the callback a {@link Document} that is only
 * valid for the duration of the callback; the underlying native pointer is
 * evicted once the callback returns.
 */
public class DocHandle {

    private final RepoRuntime runtime;
    private final DocumentId documentId;
    private final DocumentActorId actorId;
    private final AutomergeUrl url;

    /**
     * Creates a new DocHandle. Package-private - typically created by Repo methods.
     *
     * @param runtime
     *                   The repository runtime
     * @param documentId
     *                   The document ID
     * @param actorId
     *                   The actor ID for this document
     * @param url
     *                   The automerge URL for this document
     */
    DocHandle(RepoRuntime runtime, DocumentId documentId, DocumentActorId actorId, AutomergeUrl url) {
        this.runtime = Objects.requireNonNull(runtime, "runtime cannot be null");
        this.documentId = Objects.requireNonNull(documentId, "documentId cannot be null");
        this.actorId = Objects.requireNonNull(actorId, "actorId cannot be null");
        this.url = Objects.requireNonNull(url, "url cannot be null");
    }

    /**
     * Access the document by enqueuing execution of a function that receives a
     * {@link Document}. Changes made within the callback (e.g. via {@link
     * Document#startTransaction()}) are automatically tracked and may trigger
     * storage operations and sync messages to connected peers.
     *
     * @param <T>
     *            The return type of the function
     * @param fn
     *            The function to execute with document access
     * @return A CompletableFuture that completes with the function's return value
     */
    public <T> CompletableFuture<T> withDocument(Function<Document, T> fn) {
        return runtime.withDocument(actorId, fn);
    }

    /**
     * Registers a change listener that is called whenever this document changes.
     *
     * <p>
     * <strong>Important:</strong> The listener is called from within a task on the
     * document's {@code SerializingExecutor}. Calling
     * {@link #withDocument(Function)}
     * and blocking on the result from within the listener <strong>will
     * deadlock</strong>. Listeners must return quickly and treat the event as a
     * notification only.
     *
     * @param listener
     *                 The listener to register
     * @return A {@link ListenerRegistration} that can be used to remove the
     *         listener
     */
    public ListenerRegistration addChangeListener(ChangeListener listener) {
        return runtime.addChangeListener(actorId, listener);
    }

    /**
     * Gets the document ID.
     *
     * @return The document ID
     */
    public DocumentId getDocumentId() {
        return documentId;
    }

    /**
     * Gets the actor ID for this document.
     *
     * @return The document actor ID
     */
    DocumentActorId getActorId() {
        return actorId;
    }

    /**
     * Gets the automerge URL for this document.
     *
     * @return The automerge URL
     */
    public AutomergeUrl getUrl() {
        return url;
    }

    @Override
    public boolean equals(Object obj) {
        if (this == obj) {
            return true;
        }
        if (obj == null || getClass() != obj.getClass()) {
            return false;
        }
        DocHandle docHandle = (DocHandle) obj;
        return (Objects.equals(documentId, docHandle.documentId) && Objects.equals(actorId, docHandle.actorId));
    }

    @Override
    public int hashCode() {
        return Objects.hash(documentId, actorId);
    }

    @Override
    public String toString() {
        return ("DocHandle{" + "url=" + url + ", documentId=" + documentId + ", actorId=" + actorId + '}');
    }
}
