package org.automerge.repo;

import java.util.concurrent.CompletableFuture;

/**
 * Policy for determining whether to announce documents to peers.
 *
 * When a peer connects, the announce policy determines which documents should
 * be announced (advertised) to that peer. This allows control over which
 * documents are shared with which peers.
 *
 * <p>
 * Implementations must be thread-safe as they may be called from multiple
 * threads concurrently.
 */
public interface AnnouncePolicy {

    /**
     * Determines whether a document should be announced to a peer.
     *
     * @param documentId
     *            The document to potentially announce
     * @param peerId
     *            The peer to potentially announce to
     * @return A future that completes with true if the document should be
     *         announced, false otherwise
     */
    CompletableFuture<Boolean> shouldAnnounce(DocumentId documentId, PeerId peerId);
}
