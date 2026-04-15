package org.automerge.repo;

import java.util.Objects;

public abstract class DialerEvent {

    public static class MaxRetriesReached extends DialerEvent {
        private final DialerId dialerId;
        private final String url;

        public MaxRetriesReached(DialerId dialerId, String url) {
            this.dialerId = Objects.requireNonNull(dialerId);
            this.url = Objects.requireNonNull(url);
        }

        public DialerId getDialerId() {
            return dialerId;
        }

        public String getUrl() {
            return url;
        }

        @Override
        public boolean equals(Object obj) {
            if (this == obj)
                return true;
            if (obj == null || getClass() != obj.getClass())
                return false;
            MaxRetriesReached that = (MaxRetriesReached) obj;
            return Objects.equals(dialerId, that.dialerId) && Objects.equals(url, that.url);
        }

        @Override
        public int hashCode() {
            return Objects.hash(dialerId, url);
        }

        @Override
        public String toString() {
            return "DialerEvent.MaxRetriesReached{dialerId=" + dialerId + ", url='" + url + "'}";
        }
    }
}
