package org.automerge.repo;

import java.util.Objects;
import org.automerge.LoadLibrary;

/**
 * The DocumentActor manages a single Automerge document within the samod-core
 * system.
 *
 * DocumentActors are spawned by the Hub actor and handle all operations for a
 * specific document, including: - Processing messages from the Hub - Executing
 * document modifications via closures - Generating sync messages for connected
 * peers - Managing document storage operations
 *
 * ## Usage Pattern
 *
 * 1. DocumentActors are spawned automatically by the Rust layer when the Hub
 * requests them.
 * 2. Execute IO tasks and route messages returned in results.
 * 3. Free the actor when done using {@link #free()}
 */
class DocumentActor {
    static {
        LoadLibrary.initialize();
    }

    private final RepoSys.DocumentActorPointer pointer;
    private final DocumentActorId actorId;
    private final DocumentId documentId;

    /**
     * Creates a DocumentActor with the given pointer. Package-private
     * constructor — created via spawning from Hub. The caller
     * supplies {@code actorId} and {@code documentId} because the
     * underlying samod-core DocumentActor does not expose getters for
     * them; we stash them on the Java side at spawn time.
     */
    DocumentActor(RepoSys.DocumentActorPointer pointer, DocumentActorId actorId,
            DocumentId documentId) {
        this.pointer = Objects.requireNonNull(pointer, "pointer cannot be null");
        this.actorId = Objects.requireNonNull(actorId, "actorId cannot be null");
        this.documentId = Objects.requireNonNull(documentId, "documentId cannot be null");
    }

    /**
     * Gets the internal pointer. This is used by the JNI layer to access the Rust
     * DocumentActor.
     *
     * @return The opaque pointer
     */
    RepoSys.DocumentActorPointer getPointer() {
        return pointer;
    }

    /**
     * Gets the actor ID for this document actor.
     *
     * @return The document actor ID
     */
    public DocumentActorId getActorId() {
        return actorId;
    }

    /**
     * Gets the document ID for this actor's document.
     *
     * @return The document ID
     */
    public DocumentId getDocumentId() {
        return documentId;
    }

    /**
     * Checks if this document actor is stopped.
     *
     * A stopped actor will not process further messages or operations.
     *
     * @return true if the actor is stopped, false otherwise
     */
    public boolean isStopped() {
        return RepoSys.documentActorIsStopped(pointer);
    }

    /**
     * Manually frees the underlying Rust memory. This must be called when the
     * DocumentActor is no longer needed to prevent memory leaks. Do not use this
     * DocumentActor after calling free().
     */
    public void free() {
        RepoSys.freeDocumentActor(pointer);
    }

    @Override
    public String toString() {
        return "DocumentActor{documentId=" + getDocumentId() + "}";
    }
}
