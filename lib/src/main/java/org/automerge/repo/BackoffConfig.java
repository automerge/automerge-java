package org.automerge.repo;

import java.time.Duration;
import java.util.Objects;
import java.util.Optional;

public class BackoffConfig {
    private final long initialDelayMs;
    private final long maxDelayMs;
    private final Optional<Integer> maxRetries;

    BackoffConfig(long initialDelayMs, long maxDelayMs, Optional<Integer> maxRetries) {
        this.initialDelayMs = initialDelayMs;
        this.maxDelayMs = maxDelayMs;
        this.maxRetries = Objects.requireNonNull(maxRetries);
    }

    public static BackoffConfig defaults() {
        return new BackoffConfig(100, 30000, Optional.empty());
    }

    public static Builder builder() {
        return new Builder();
    }

    public Duration getInitialDelay() {
        return Duration.ofMillis(initialDelayMs);
    }

    public Duration getMaxDelay() {
        return Duration.ofMillis(maxDelayMs);
    }

    public Optional<Integer> getMaxRetries() {
        return maxRetries;
    }

    @Override
    public boolean equals(Object obj) {
        if (this == obj)
            return true;
        if (obj == null || getClass() != obj.getClass())
            return false;
        BackoffConfig that = (BackoffConfig) obj;
        return initialDelayMs == that.initialDelayMs && maxDelayMs == that.maxDelayMs
                && Objects.equals(maxRetries, that.maxRetries);
    }

    @Override
    public int hashCode() {
        return Objects.hash(initialDelayMs, maxDelayMs, maxRetries);
    }

    @Override
    public String toString() {
        return "BackoffConfig{initialDelay=" + initialDelayMs + "ms, maxDelay=" + maxDelayMs
                + "ms, maxRetries=" + maxRetries + "}";
    }

    public static class Builder {
        private long initialDelayMs = 100;
        private long maxDelayMs = 30000;
        private Optional<Integer> maxRetries = Optional.empty();

        public Builder initialDelay(Duration d) {
            this.initialDelayMs = d.toMillis();
            return this;
        }

        public Builder maxDelay(Duration d) {
            this.maxDelayMs = d.toMillis();
            return this;
        }

        public Builder maxRetries(int n) {
            this.maxRetries = Optional.of(n);
            return this;
        }

        public BackoffConfig build() {
            return new BackoffConfig(initialDelayMs, maxDelayMs, maxRetries);
        }
    }
}
