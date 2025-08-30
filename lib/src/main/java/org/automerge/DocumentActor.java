package org.automerge;

import java.util.Objects;
import java.util.function.Function;

/**
 * The DocumentActor manages a single Automerge document within the samod-core system.
 *
 * DocumentActors are spawned by the Hub actor and handle all operations for a specific
 * document, including:
 * - Processing messages from the Hub
 * - Executing document modifications via closures
 * - Generating sync messages for connected peers
 * - Managing document storage operations
 *
 * ## Usage Pattern
 *
 * 1. DocumentActors are created by spawning via the Hub (using SpawnArgs)
 * 2. Process hub messages through {@link #handleMsg(HubToDocMsg)}
 * 3. Modify the document using {@link #withDocument(Function)}
 * 4. Execute IO tasks and route messages returned in results
 * 5. Free the actor when done using {@link #free()}
 */
public class DocumentActor {

    private final AutomergeSys.DocumentActorPointer pointer;

    /**
     * Creates a DocumentActor with the given pointer.
     * Package-private constructor - typically created via spawning from Hub.
     * @param pointer The opaque pointer to the Rust DocumentActor
     */
    DocumentActor(AutomergeSys.DocumentActorPointer pointer) {
        this.pointer = Objects.requireNonNull(
            pointer,
            "pointer cannot be null"
        );
    }

    /**
     * Gets the internal pointer.
     * This is used by the JNI layer to access the Rust DocumentActor.
     * @return The opaque pointer
     */
    AutomergeSys.DocumentActorPointer getPointer() {
        return pointer;
    }

    /**
     * Processes a message from the Hub and returns any resulting actions.
     *
     * Messages from the Hub include sync messages from peers, storage results,
     * and other coordination events. Processing these messages may generate
     * IO tasks (storage operations, announce policy checks) and outgoing
     * messages (sync messages to peers, responses to the Hub).
     *
     * @param now The current timestamp (Unix timestamp in milliseconds)
     * @param msg The message from the Hub to process
     * @return DocActorResult containing IO tasks, messages, and change events
     */
    public DocActorResult handleMsg(long now, HubToDocMsg msg) {
        return AutomergeSys.documentActorHandleMsg(pointer, now, msg);
    }

    /**
     * Executes a function with mutable access to the document.
     *
     * This is the primary way to modify a document. The provided function
     * receives a {@link Document} and can perform any operations on it.
     * The function's return value is wrapped in a {@link WithDocResult}
     * along with any side effects (storage operations, sync messages, etc.).
     *
     * Example:
     * <pre>{@code
     * long now = System.currentTimeMillis();
     * WithDocResult<String> result = docActor.withDocument(now, doc -> {
     *     Transaction tx = doc.startTransaction();
     *     tx.set(new ObjId(), "key", "value");
     *     tx.commit();
     *     return doc.getActorId().toString();
     * });
     * String actorId = result.getValue();
     * DocActorResult sideEffects = result.getActorResult();
     * }</pre>
     *
     * @param <T> The return type of the function
     * @param now The current timestamp (Unix timestamp in milliseconds)
     * @param fn The function to execute with document access
     * @return WithDocResult containing the function's return value and side effects
     */
    public <T> WithDocResult<T> withDocument(
        long now,
        Function<Document, T> fn
    ) {
        return AutomergeSys.documentActorWithDocument(pointer, now, fn);
    }

    /**
     * Gets the document ID for this actor's document.
     *
     * @return The document ID
     */
    public DocumentId getDocumentId() {
        return AutomergeSys.documentActorGetDocumentId(pointer);
    }

    /**
     * Checks if this document actor is stopped.
     *
     * A stopped actor will not process further messages or operations.
     *
     * @return true if the actor is stopped, false otherwise
     */
    public boolean isStopped() {
        return AutomergeSys.documentActorIsStopped(pointer);
    }

    /**
     * Manually frees the underlying Rust memory.
     * This must be called when the DocumentActor is no longer needed to prevent memory leaks.
     * Do not use this DocumentActor after calling free().
     */
    public void free() {
        AutomergeSys.freeDocumentActor(pointer);
    }

    @Override
    public String toString() {
        return "DocumentActor{documentId=" + getDocumentId() + "}";
    }
}
