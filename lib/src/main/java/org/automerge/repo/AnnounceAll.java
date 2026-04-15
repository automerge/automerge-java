package org.automerge.repo;

import java.util.concurrent.CompletableFuture;

/**
 * Announce policy that announces all documents to all peers.
 *
 * This is the most permissive policy and is useful for public sharing scenarios
 * where all connected peers should see all documents.
 */
public class AnnounceAll implements AnnouncePolicy {

    @Override
    public CompletableFuture<Boolean> shouldAnnounce(DocumentId documentId, PeerId peerId) {
        return CompletableFuture.completedFuture(true);
    }
}
