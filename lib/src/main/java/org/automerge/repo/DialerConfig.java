package org.automerge.repo;

import java.util.Objects;

public class DialerConfig {
    private final BackoffConfig backoff;

    public DialerConfig(BackoffConfig backoff) {
        this.backoff = Objects.requireNonNull(backoff, "backoff cannot be null");
    }

    public static DialerConfig defaults() {
        return new DialerConfig(BackoffConfig.defaults());
    }

    public BackoffConfig getBackoff() {
        return backoff;
    }

    @Override
    public boolean equals(Object obj) {
        if (this == obj)
            return true;
        if (obj == null || getClass() != obj.getClass())
            return false;
        DialerConfig that = (DialerConfig) obj;
        return Objects.equals(backoff, that.backoff);
    }

    @Override
    public int hashCode() {
        return Objects.hash(backoff);
    }

    @Override
    public String toString() {
        return "DialerConfig{, backoff=" + backoff + "}";
    }
}
