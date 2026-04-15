package org.automerge.repo;

import java.util.Objects;

class ListenerConfig {
    private final String url;

    ListenerConfig(String url) {
        this.url = Objects.requireNonNull(url, "url cannot be null");
    }

    String getUrl() {
        return url;
    }

    @Override
    public boolean equals(Object obj) {
        if (this == obj)
            return true;
        if (obj == null || getClass() != obj.getClass())
            return false;
        ListenerConfig that = (ListenerConfig) obj;
        return Objects.equals(url, that.url);
    }

    @Override
    public int hashCode() {
        return Objects.hash(url);
    }

    @Override
    public String toString() {
        return "ListenerConfig{url='" + url + "'}";
    }
}
