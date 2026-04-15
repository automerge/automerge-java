package org.automerge.repo.integration.helpers;

import java.time.Duration;
import java.util.concurrent.CompletableFuture;
import java.util.concurrent.ExecutionException;
import java.util.concurrent.TimeUnit;
import java.util.concurrent.TimeoutException;
import java.util.function.Supplier;
import org.automerge.AmValue;
import org.automerge.Document;
import org.automerge.ObjectId;
import org.automerge.Transaction;

/**
 * Utility class providing helper methods for integration testing.
 *
 * This class provides: - Better timeout handling with descriptive error
 * messages - Polling utilities for eventual consistency testing - Document
 * content creation and manipulation helpers - Logging and debugging utilities
 */
public class TestHelpers {

    private static final Duration DEFAULT_TIMEOUT = Duration.ofSeconds(5);
    private static final Duration DEFAULT_POLL_INTERVAL = Duration.ofMillis(50);

    /**
     * Fluent API for eventual consistency assertions.
     *
     * <p>
     * Allows configuring timeout and poll interval in a readable way:
     *
     * <pre>{@code
     * eventually(() -> doc.isReady()).succeeds("document ready");
     * eventually(() -> doc.isReady()).timeout(Duration.ofSeconds(10)).succeeds("document ready");
     * eventually(() -> doc.isReady()).pollInterval(Duration.ofMillis(100)).succeeds("document ready");
     * }</pre>
     *
     * <p>
     * <strong>Important:</strong> You must call {@link #succeeds(String)} to
     * execute the assertion. The builder pattern is only complete when
     * {@code succeeds()} is called. If you forget to call it, a warning will be
     * printed when this object is garbage collected.
     */
    public static class EventuallyAssertion {

        private final Supplier<Boolean> condition;
        private Duration timeout = DEFAULT_TIMEOUT;
        private Duration pollInterval = DEFAULT_POLL_INTERVAL;
        private boolean executed = false;

        private EventuallyAssertion(Supplier<Boolean> condition) {
            this.condition = condition;
        }

        /**
         * Sets the maximum time to wait for the condition.
         *
         * @param timeout
         *            The timeout duration
         * @return this for chaining
         */
        public EventuallyAssertion timeout(Duration timeout) {
            this.timeout = timeout;
            return this;
        }

        /**
         * Sets how often to check the condition.
         *
         * @param pollInterval
         *            The poll interval
         * @return this for chaining
         */
        public EventuallyAssertion pollInterval(Duration pollInterval) {
            this.pollInterval = pollInterval;
            return this;
        }

        /**
         * Executes the assertion, waiting for the condition to become true.
         *
         * <p>
         * This method polls the condition until it returns true or the timeout expires.
         * The description parameter is used in error messages to explain what was
         * expected.
         *
         * @param description
         *            A description of what we're waiting for (e.g., "document ready",
         *            "connection established")
         * @throws AssertionError
         *             if the condition doesn't become true within the timeout
         */
        public void succeeds(String description) {
            executed = true;
            long startTime = System.currentTimeMillis();
            long timeoutMs = timeout.toMillis();

            while (System.currentTimeMillis() - startTime < timeoutMs) {
                if (condition.get()) {
                    return;
                }

                try {
                    Thread.sleep(pollInterval.toMillis());
                } catch (InterruptedException e) {
                    Thread.currentThread().interrupt();
                    throw new AssertionError(String.format("Interrupted while waiting for: %s", description), e);
                }
            }

            throw new AssertionError(
                    String.format("Condition not met within timeout: %s (waited %dms, polled every %dms)", description,
                            timeoutMs, pollInterval.toMillis()));
        }

        @Override
        @SuppressWarnings("deprecation")
        protected void finalize() throws Throwable {
            if (!executed) {
                System.err.println("WARNING: EventuallyAssertion was created but never executed. "
                        + "Did you forget to call .succeeds(description)?");
            }
            super.finalize();
        }
    }

    /**
     * Waits for a CompletableFuture to complete with a timeout and provides a
     * descriptive error message if it times out.
     *
     * @param <T>
     *            The type of the future's result
     * @param future
     *            The future to wait for
     * @param timeout
     *            The maximum time to wait
     * @param description
     *            A description of what we're waiting for (for error messages)
     * @return The result of the future
     * @throws AssertionError
     *             if the future times out or completes exceptionally
     */
    public static <T> T waitFor(CompletableFuture<T> future, Duration timeout, String description) {
        try {
            return future.get(timeout.toMillis(), TimeUnit.MILLISECONDS);
        } catch (TimeoutException e) {
            throw new AssertionError(
                    String.format("Timeout waiting for: %s (waited %dms)", description, timeout.toMillis()), e);
        } catch (ExecutionException e) {
            throw new AssertionError(String.format("Future completed exceptionally while waiting for: %s", description),
                    e.getCause());
        } catch (InterruptedException e) {
            Thread.currentThread().interrupt();
            throw new AssertionError(String.format("Interrupted while waiting for: %s", description), e);
        }
    }

    /**
     * Waits for a CompletableFuture with the default timeout (5 seconds).
     *
     * @param <T>
     *            The type of the future's result
     * @param future
     *            The future to wait for
     * @param description
     *            A description of what we're waiting for
     * @return The result of the future
     */
    public static <T> T waitFor(CompletableFuture<T> future, String description) {
        return waitFor(future, DEFAULT_TIMEOUT, description);
    }

    /**
     * Creates an eventual consistency assertion for the given condition.
     *
     * <p>
     * This is a fluent API that allows configuring timeout and poll interval:
     *
     * <pre>{@code
     * // With defaults (5s timeout, 50ms poll)
     * eventually(() -> doc.isReady()).succeeds("document ready");
     *
     * // With custom timeout
     * eventually(() -> doc.isReady()).timeout(Duration.ofSeconds(10)).succeeds("document ready");
     *
     * // With custom poll interval
     * eventually(() -> doc.isReady()).pollInterval(Duration.ofMillis(100)).succeeds("document ready");
     *
     * // With both
     * eventually(() -> doc.isReady()).timeout(Duration.ofSeconds(10)).pollInterval(Duration.ofMillis(100))
     * 		.succeeds("document ready");
     * }</pre>
     *
     * @param condition
     *            A supplier that returns true when the condition is met
     * @return An EventuallyAssertion for configuring and executing the check
     */
    public static EventuallyAssertion eventually(Supplier<Boolean> condition) {
        return new EventuallyAssertion(condition);
    }

    /**
     * Creates a simple document with a text property for testing.
     *
     * @param key
     *            The property key
     * @param value
     *            The text value
     * @return A document with the specified property
     */
    public static byte[] createSimpleDocument(String key, String value) {
        Document doc = new Document();
        try (Transaction tx = doc.startTransaction()) {
            tx.set(ObjectId.ROOT, key, value);
            tx.commit();
        }
        return doc.save();
    }

    /**
     * Creates a document with multiple string properties.
     *
     * @param properties
     *            Key-value pairs to add to the document
     * @return A document with the specified properties
     */
    public static byte[] createDocumentWithProperties(String... properties) {
        if (properties.length % 2 != 0) {
            throw new IllegalArgumentException("Properties must be key-value pairs");
        }

        Document doc = new Document();
        try (Transaction tx = doc.startTransaction()) {
            for (int i = 0; i < properties.length; i += 2) {
                tx.set(ObjectId.ROOT, properties[i], properties[i + 1]);
            }
            tx.commit();
        }
        return doc.save();
    }

    /**
     * Extracts a string property from a document's bytes.
     *
     * @param documentBytes
     *            The serialized document
     * @param key
     *            The property key to extract
     * @return The string value, or null if not present
     */
    public static String getStringFromDocument(byte[] documentBytes, String key) {
        Document doc = Document.load(documentBytes);
        return doc.get(ObjectId.ROOT, key).map(v -> v instanceof AmValue.Str ? ((AmValue.Str) v).getValue() : null)
                .orElse(null);
    }

    /**
     * Sleeps for the specified duration, wrapping InterruptedException.
     *
     * @param duration
     *            How long to sleep
     */
    public static void sleep(Duration duration) {
        try {
            Thread.sleep(duration.toMillis());
        } catch (InterruptedException e) {
            Thread.currentThread().interrupt();
            throw new RuntimeException("Sleep interrupted", e);
        }
    }

    /**
     * Logs a test message with timestamp.
     *
     * @param message
     *            The message to log
     */
    public static void log(String message) {
        System.out.printf("[%d] %s%n", System.currentTimeMillis(), message);
    }

    /**
     * Logs a formatted test message with timestamp.
     *
     * @param format
     *            The format string
     * @param args
     *            The format arguments
     */
    public static void logf(String format, Object... args) {
        log(String.format(format, args));
    }

    /**
     * Creates a large document for testing with the specified number of properties.
     *
     * @param propertyCount
     *            Number of properties to add
     * @return A document with many properties
     */
    public static byte[] createLargeDocument(int propertyCount) {
        Document doc = new Document();
        try (Transaction tx = doc.startTransaction()) {
            for (int i = 0; i < propertyCount; i++) {
                StringBuilder sb = new StringBuilder();
                for (int j = 0; j < 100; j++) {
                    sb.append("x");
                }
                String repeatedX = sb.toString();
                tx.set(ObjectId.ROOT, "property_" + i, "value_" + i + "_" + repeatedX);
            }
            tx.commit();
        }
        return doc.save();
    }

    /**
     * Asserts that a future completes exceptionally with a specific exception type.
     *
     * @param <T>
     *            The future's result type
     * @param future
     *            The future that should fail
     * @param expectedExceptionClass
     *            The expected exception class
     * @param description
     *            Description for error messages
     * @return The exception that was thrown
     */
    public static <T> Throwable assertFails(CompletableFuture<T> future,
            Class<? extends Throwable> expectedExceptionClass, String description) {
        try {
            T result = future.get(DEFAULT_TIMEOUT.toMillis(), TimeUnit.MILLISECONDS);
            throw new AssertionError(String.format("Expected %s to fail with %s, but it succeeded with result: %s",
                    description, expectedExceptionClass.getSimpleName(), result));
        } catch (ExecutionException e) {
            Throwable cause = e.getCause();
            if (expectedExceptionClass.isInstance(cause)) {
                return cause;
            } else {
                throw new AssertionError(String.format("Expected %s to fail with %s, but it failed with %s: %s",
                        description, expectedExceptionClass.getSimpleName(), cause.getClass().getSimpleName(),
                        cause.getMessage()), cause);
            }
        } catch (TimeoutException e) {
            throw new AssertionError(String.format("Expected %s to fail with %s, but it timed out", description,
                    expectedExceptionClass.getSimpleName()), e);
        } catch (InterruptedException e) {
            Thread.currentThread().interrupt();
            throw new AssertionError(String.format("Interrupted while waiting for %s to fail", description), e);
        }
    }

    /**
     * Asserts that a future completes exceptionally (with any exception).
     *
     * @param <T>
     *            The future's result type
     * @param future
     *            The future that should fail
     * @param description
     *            Description for error messages
     * @return The exception that was thrown
     */
    public static <T> Throwable assertFails(CompletableFuture<T> future, String description) {
        return assertFails(future, Throwable.class, description);
    }

    private TestHelpers() {
        // Utility class - no instantiation
    }
}
