package org.automerge.repo;

/**
 * Result of spawning a DocumentActor.
 *
 * Contains both the newly spawned actor and its initial DocActorResult, which
 * includes important initial messages (e.g., DocumentStatusChanged).
 */
class SpawnedActor {

    private final DocumentActor actor;
    private final DocActorResult initialResult;

    /**
     * Package-private constructor called from JNI.
     *
     * @param actor
     *            The spawned DocumentActor
     * @param initialResult
     *            The initial DocActorResult from spawning
     */
    SpawnedActor(DocumentActor actor, DocActorResult initialResult) {
        this.actor = actor;
        this.initialResult = initialResult;
    }

    /**
     * Gets the spawned DocumentActor.
     *
     * @return The document actor
     */
    DocumentActor getActor() {
        return actor;
    }

    /**
     * Gets the initial DocActorResult from spawning.
     *
     * This result should be processed immediately after spawning to handle any
     * initial messages (such as DocumentStatusChanged).
     *
     * @return The initial result
     */
    DocActorResult getInitialResult() {
        return initialResult;
    }
}
