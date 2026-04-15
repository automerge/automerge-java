package org.automerge.repo;

import org.automerge.LoadLibrary;
import org.automerge.repo.storage.InMemoryStorage;

/**
 * Configuration for creating a Repo instance.
 *
 * Use the Builder to construct instances:
 *
 * <pre>{@code
 * RepoConfig config = RepoConfig.builder().storage(new InMemoryStorage()).peerId(PeerId.generate()).build();
 * }</pre>
 */
public class RepoConfig {

    private final Storage storage;
    private final PeerId peerId;
    private final AnnouncePolicy announcePolicy;

    private RepoConfig(Builder builder) {
        if (builder.storage == null) {
            this.storage = new InMemoryStorage();
        } else {
            this.storage = builder.storage;
        }

        if (builder.peerId == null) {
            this.peerId = PeerId.generate();
        } else {
            this.peerId = builder.peerId;
        }

        if (builder.announcePolicy == null) {
            this.announcePolicy = new AnnounceAll();
        } else {
            this.announcePolicy = builder.announcePolicy;
        }
    }

    public Storage getStorage() {
        return storage;
    }

    public PeerId getPeerId() {
        return peerId;
    }

    public AnnouncePolicy getAnnouncePolicy() {
        return announcePolicy;
    }

    public static Builder builder() {
        LoadLibrary.initialize();
        return new Builder();
    }

    public static class Builder {

        private Storage storage;
        private PeerId peerId;
        private AnnouncePolicy announcePolicy;

        private Builder() {}

        /**
         * Sets the storage implementation (optional, defaults to InMemoryStorage).
         * @param storage the storage implementation to set
         * @return the builder to chain further config calls agains
         */
        public Builder storage(Storage storage) {
            this.storage = storage;
            return this;
        }

        /**
         * Sets the peer ID (optional, defaults to a randomly generated PeerId).
         * @param peerId the peer ID to set
         * @return the builder to chain further config calls agains
         */
        public Builder peerId(PeerId peerId) {
            this.peerId = peerId;
            return this;
        }

        /**
         * Sets the announce policy (optional, defaults to AnnounceAll).
         *
         * @param announcePolicy the announce policy to set
         * @return the builder to chain further config calls agains
         */
        public Builder announcePolicy(AnnouncePolicy announcePolicy) {
            this.announcePolicy = announcePolicy;
            return this;
        }

        /**
         * Builds the RepoConfig
         * @return the built RepoConfig
         */
        public RepoConfig build() {
            return new RepoConfig(this);
        }
    }
}
