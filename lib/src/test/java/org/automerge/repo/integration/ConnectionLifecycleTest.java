package org.automerge.repo.integration;

import static org.automerge.repo.integration.helpers.TestHelpers.*;
import static org.junit.jupiter.api.Assertions.*;

import java.util.ArrayList;
import java.util.List;
import org.automerge.repo.AcceptorHandle;
import org.automerge.repo.DialerHandle;
import org.automerge.repo.PeerId;
import org.automerge.repo.Repo;
import org.automerge.repo.RepoConfig;
import org.automerge.repo.integration.helpers.ChannelDialer;
import org.automerge.repo.storage.InMemoryStorage;
import org.junit.jupiter.api.Test;

/**
 * Integration tests for connection lifecycle management using the
 * Dialer/Listener API.
 *
 * These tests validate:
 * - Dialer connection establishment and peer ID discovery
 * - Explicit dialer close
 * - Multiple concurrent connections
 * - Connection state tracking via isConnected()
 * - Acceptor close closes connections
 * - Multiple close calls are safe
 *
 * Tests use ChannelDialer/ChannelAdapter for in-memory communication without
 * actual networking.
 */
class ConnectionLifecycleTest {

    @Test
    void testDialerConnectionEstablishment() {
        RepoConfig configA = RepoConfig.builder().storage(new InMemoryStorage()).build();
        RepoConfig configB = RepoConfig.builder().storage(new InMemoryStorage()).build();

        try (Repo repoA = Repo.load(configA); Repo repoB = Repo.load(configB)) {
            AcceptorHandle acceptorB = repoB.makeAcceptor("channel://b");
            DialerHandle dialerA = repoA.dial(
                    new ChannelDialer(acceptorB));

            // Wait for handshake to complete
            PeerId peerB = waitFor(dialerA.onEstablished(), "dialer A establishment");
            assertNotNull(peerB, "Peer ID should not be null");

            // Connection should report established state
            assertTrue(dialerA.isConnected(), "Dialer A should be connected");
            assertEquals(peerB, dialerA.getPeerId().orElse(null),
                    "getPeerId() should return peer ID after establishment");

            dialerA.close();
            acceptorB.close();
        }
    }

    @Test
    void testDialerClose() {
        RepoConfig configA = RepoConfig.builder().storage(new InMemoryStorage()).build();
        RepoConfig configB = RepoConfig.builder().storage(new InMemoryStorage()).build();

        try (Repo repoA = Repo.load(configA); Repo repoB = Repo.load(configB)) {
            AcceptorHandle acceptorB = repoB.makeAcceptor("channel://b");
            DialerHandle dialerA = repoA.dial(
                    new ChannelDialer(acceptorB));

            // Wait for establishment
            waitFor(dialerA.onEstablished(), "dialer establishment");

            // Explicitly close the dialer
            dialerA.close();

            assertFalse(dialerA.isConnected(), "Dialer should not be connected after close");

            acceptorB.close();
        }
    }

    @Test
    void testMultipleConcurrentConnections() {
        RepoConfig configA = RepoConfig.builder().storage(new InMemoryStorage()).build();
        RepoConfig configB = RepoConfig.builder().storage(new InMemoryStorage()).build();

        try (Repo repoA = Repo.load(configA); Repo repoB = Repo.load(configB)) {
            int connectionCount = 5;
            List<DialerHandle> dialers = new ArrayList<>();
            List<AcceptorHandle> acceptors = new ArrayList<>();

            // Create multiple connections
            for (int i = 0; i < connectionCount; i++) {
                AcceptorHandle acceptor = repoB.makeAcceptor("channel://b-" + i);
                DialerHandle dialer = repoA.dial(
                        new ChannelDialer(acceptor));

                acceptors.add(acceptor);
                dialers.add(dialer);
            }

            // Wait for all to establish
            for (int i = 0; i < connectionCount; i++) {
                PeerId peerB = waitFor(
                        dialers.get(i).onEstablished(),
                        "connection " + i + " establishment");
                assertNotNull(peerB, "Peer B ID should not be null for connection " + i);
            }

            // All dialers should be connected
            for (int i = 0; i < connectionCount; i++) {
                assertTrue(dialers.get(i).isConnected(), "Dialer " + i + " should be connected");
            }

            // Close all dialers
            for (DialerHandle dialer : dialers) {
                dialer.close();
            }

            // All should be disconnected
            for (int i = 0; i < connectionCount; i++) {
                assertFalse(dialers.get(i).isConnected(), "Dialer " + i + " should not be connected after close");
            }

            // Close all acceptors
            for (AcceptorHandle acceptor : acceptors) {
                acceptor.close();
            }
        }
    }

    @Test
    void testIsConnectedReflectsState() {
        RepoConfig configA = RepoConfig.builder().storage(new InMemoryStorage()).build();
        RepoConfig configB = RepoConfig.builder().storage(new InMemoryStorage()).build();

        try (Repo repoA = Repo.load(configA); Repo repoB = Repo.load(configB)) {
            AcceptorHandle acceptorB = repoB.makeAcceptor("channel://b");
            DialerHandle dialerA = repoA.dial(
                    new ChannelDialer(acceptorB));

            // Wait for establishment
            waitFor(dialerA.onEstablished(), "dialer establishment");

            // Should be connected after establishment
            assertTrue(dialerA.isConnected(), "Dialer should be connected after establishment");

            // Close
            dialerA.close();

            // Should not be connected after close
            assertFalse(dialerA.isConnected(), "Dialer should not be connected after close()");

            acceptorB.close();
        }
    }

    @Test
    void testMultipleCloseCallsAreSafe() {
        RepoConfig configA = RepoConfig.builder().storage(new InMemoryStorage()).build();
        RepoConfig configB = RepoConfig.builder().storage(new InMemoryStorage()).build();

        try (Repo repoA = Repo.load(configA); Repo repoB = Repo.load(configB)) {
            AcceptorHandle acceptorB = repoB.makeAcceptor("channel://b");
            DialerHandle dialerA = repoA.dial(
                    new ChannelDialer(acceptorB));

            waitFor(dialerA.onEstablished(), "dialer establishment");

            // Close multiple times - should be idempotent
            dialerA.close();
            dialerA.close();
            dialerA.close();

            assertFalse(dialerA.isConnected(), "Dialer should not be connected after close");

            acceptorB.close();
        }
    }

    @Test
    void testAcceptorCloseClosesConnections() {
        RepoConfig configA = RepoConfig.builder().storage(new InMemoryStorage()).build();
        RepoConfig configB = RepoConfig.builder().storage(new InMemoryStorage()).build();

        try (Repo repoA = Repo.load(configA); Repo repoB = Repo.load(configB)) {
            AcceptorHandle acceptorB = repoB.makeAcceptor("channel://b");
            DialerHandle dialerA = repoA.dial(
                    new ChannelDialer(acceptorB));

            // Wait for establishment
            waitFor(dialerA.onEstablished(), "dialer establishment");
            assertTrue(dialerA.isConnected(), "Dialer should be connected");

            // Close the acceptor side
            acceptorB.close();

            // Clean up dialer
            dialerA.close();
            assertFalse(dialerA.isConnected(), "Dialer should not be connected after close");
        }
    }
}
