package org.automerge.repo;

import java.util.concurrent.CompletableFuture;

/**
 * Announce policy that never announces any documents to any peers.
 *
 * This is the most restrictive policy. Documents can still be synced if the
 * peer explicitly requests them, but they won't be automatically
 * announced/advertised.
 *
 * Useful for scenarios where the peer should only access documents they already
 * know about (e.g., via out-of-band URL sharing).
 */
public class AnnounceNone implements AnnouncePolicy {

    @Override
    public CompletableFuture<Boolean> shouldAnnounce(DocumentId documentId, PeerId peerId) {
        return CompletableFuture.completedFuture(false);
    }
}
