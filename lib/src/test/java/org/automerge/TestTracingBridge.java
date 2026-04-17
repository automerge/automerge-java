package org.automerge;

import ch.qos.logback.classic.Level;
import ch.qos.logback.classic.LoggerContext;
import ch.qos.logback.classic.spi.ILoggingEvent;
import ch.qos.logback.core.read.ListAppender;
import org.junit.jupiter.api.AfterEach;
import org.junit.jupiter.api.Assertions;
import org.junit.jupiter.api.BeforeEach;
import org.junit.jupiter.api.Test;
import org.slf4j.LoggerFactory;

/**
 * Tests for the Rust tracing → SLF4J bridge.
 *
 * <p>The bridge is initialised automatically by {@link LoadLibrary#initialize()},
 * which calls {@code AutomergeSys.initTracing}. These tests verify that
 * SLF4J level configuration controls which Rust tracing events are forwarded.
 */
public final class TestTracingBridge {

    static {
        LoadLibrary.initialize();
    }

    private ListAppender<ILoggingEvent> appender;
    private ch.qos.logback.classic.Logger testLogger;

    @BeforeEach
    public void setUp() {
        LoggerContext ctx = (LoggerContext) LoggerFactory.getILoggerFactory();
        // Rust tracing target "automerge::test" is mapped into the
        // SLF4J name space as "org.automerge.automerge.test".
        testLogger = ctx.getLogger("org.automerge.automerge.test");

        // Attach a ListAppender so we can inspect what the bridge forwarded.
        appender = new ListAppender<>();
        appender.setContext(ctx);
        appender.start();
        testLogger.addAppender(appender);

        // Enable all levels so we can test selective filtering.
        testLogger.setLevel(Level.TRACE);
    }

    @AfterEach
    public void tearDown() {
        testLogger.detachAppender(appender);
        appender.stop();
    }

    /**
     * Emit a tracing event at each level and verify it is forwarded to SLF4J.
     */
    @Test
    public void allLevelsAreForwarded() {
        AutomergeSys.testEmitTracingEvent("trace", "trace msg");
        AutomergeSys.testEmitTracingEvent("debug", "debug msg");
        AutomergeSys.testEmitTracingEvent("info", "info msg");
        AutomergeSys.testEmitTracingEvent("warn", "warn msg");
        AutomergeSys.testEmitTracingEvent("error", "error msg");

        Assertions.assertEquals(5, appender.list.size());

        Assertions.assertEquals(Level.TRACE, appender.list.get(0).getLevel());
        Assertions.assertEquals("trace msg", appender.list.get(0).getMessage());

        Assertions.assertEquals(Level.DEBUG, appender.list.get(1).getLevel());
        Assertions.assertEquals("debug msg", appender.list.get(1).getMessage());

        Assertions.assertEquals(Level.INFO, appender.list.get(2).getLevel());
        Assertions.assertEquals("info msg", appender.list.get(2).getMessage());

        Assertions.assertEquals(Level.WARN, appender.list.get(3).getLevel());
        Assertions.assertEquals("warn msg", appender.list.get(3).getMessage());

        Assertions.assertEquals(Level.ERROR, appender.list.get(4).getLevel());
        Assertions.assertEquals("error msg", appender.list.get(4).getMessage());
    }

    /**
     * Verify that when the SLF4J logger is set to WARN, trace/debug/info
     * events are not forwarded — they are filtered out before crossing the
     * JNI boundary.
     */
    @Test
    public void slf4jLevelFilteringIsRespected() {
        testLogger.setLevel(Level.WARN);

        AutomergeSys.testEmitTracingEvent("trace", "should be filtered");
        AutomergeSys.testEmitTracingEvent("debug", "should be filtered");
        AutomergeSys.testEmitTracingEvent("info", "should be filtered");
        AutomergeSys.testEmitTracingEvent("warn", "should pass");
        AutomergeSys.testEmitTracingEvent("error", "should pass");

        Assertions.assertEquals(2, appender.list.size());
        Assertions.assertEquals(Level.WARN, appender.list.get(0).getLevel());
        Assertions.assertEquals("should pass", appender.list.get(0).getMessage());
        Assertions.assertEquals(Level.ERROR, appender.list.get(1).getLevel());
        Assertions.assertEquals("should pass", appender.list.get(1).getMessage());
    }

    /**
     * Verify that raising the SLF4J level after initial filtering takes
     * effect — the bridge uses {@code Interest::sometimes} so tracing
     * re-evaluates {@code enabled} when levels change.
     */
    @Test
    public void levelChangesTakeEffect() {
        // Start with INFO — trace and debug should be filtered out.
        testLogger.setLevel(Level.INFO);

        AutomergeSys.testEmitTracingEvent("trace", "filtered at INFO");
        AutomergeSys.testEmitTracingEvent("debug", "filtered at INFO");
        AutomergeSys.testEmitTracingEvent("info", "passes at INFO");

        Assertions.assertEquals(1, appender.list.size());
        Assertions.assertEquals("passes at INFO", appender.list.get(0).getMessage());

        // Now lower the threshold to TRACE — all events should come through.
        appender.list.clear();
        testLogger.setLevel(Level.TRACE);

        AutomergeSys.testEmitTracingEvent("trace", "now passes");
        AutomergeSys.testEmitTracingEvent("debug", "now passes");
        AutomergeSys.testEmitTracingEvent("info", "still passes");

        Assertions.assertEquals(3, appender.list.size());
    }
}
