package org.automerge.repo;

import java.util.Objects;

class DialRequest {
    private final DialerId dialerId;
    private final String url;

    DialRequest(DialerId dialerId, String url) {
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
        DialRequest that = (DialRequest) obj;
        return Objects.equals(dialerId, that.dialerId) && Objects.equals(url, that.url);
    }

    @Override
    public int hashCode() {
        return Objects.hash(dialerId, url);
    }

    @Override
    public String toString() {
        return "DialRequest{dialerId=" + dialerId + ", url='" + url + "'}";
    }
}
