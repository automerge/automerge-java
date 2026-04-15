package org.automerge.repo.integration;

import static org.automerge.repo.integration.helpers.TestHelpers.*;
import static org.junit.jupiter.api.Assertions.*;

import java.util.ArrayList;
import java.util.Collections;
import java.util.List;
import java.util.concurrent.CountDownLatch;
import java.util.concurrent.TimeUnit;
import org.automerge.ObjectId;
import org.automerge.Transaction;
import org.automerge.repo.DocHandle;
import org.automerge.repo.DocumentChanged;
import org.automerge.repo.ListenerRegistration;
import org.automerge.repo.Repo;
import org.automerge.repo.RepoConfig;
import org.automerge.repo.storage.InMemoryStorage;
import org.junit.jupiter.api.Test;

/**
 * Integration tests for the change listener functionality on DocHandle.
 */
class ChangeListenerTest {

    @Test
    void testListenerCalledOnChange() throws Exception {
        RepoConfig config = RepoConfig.builder().storage(new InMemoryStorage()).build();

        try (Repo repo = Repo.load(config)) {
            DocHandle handle = waitFor(repo.create(), "document creation");

            CountDownLatch latch = new CountDownLatch(3);
            List<DocumentChanged> events = Collections.synchronizedList(new ArrayList<>());

            handle.addChangeListener(event -> {
                events.add(event);
                latch.countDown();
            });

            // Make three changes
            for (int i = 0; i < 3; i++) {
                final int idx = i;
                waitFor(handle.withDocument(doc -> {
                    try (Transaction tx = doc.startTransaction()) {
                        tx.set(ObjectId.ROOT, "key" + idx, "value" + idx);
                        tx.commit();
                    }
                    return null;
                }), "document change " + i);
            }

            assertTrue(latch.await(5, TimeUnit.SECONDS), "Listener should be called 3 times");
            assertEquals(3, events.size(), "Should have received three events");
        }
    }

    @Test
    void testMultipleListeners() throws Exception {
        try (Repo repo = Repo.load()) {
            DocHandle handle = waitFor(repo.create(), "document creation");

            CountDownLatch latch1 = new CountDownLatch(1);
            CountDownLatch latch2 = new CountDownLatch(1);
            List<DocumentChanged> events1 = Collections.synchronizedList(new ArrayList<>());
            List<DocumentChanged> events2 = Collections.synchronizedList(new ArrayList<>());

            handle.addChangeListener(event -> {
                events1.add(event);
                latch1.countDown();
            });
            handle.addChangeListener(event -> {
                events2.add(event);
                latch2.countDown();
            });

            // Make a change
            waitFor(handle.withDocument(doc -> {
                try (Transaction tx = doc.startTransaction()) {
                    tx.set(ObjectId.ROOT, "key", "value");
                    tx.commit();
                }
                return null;
            }), "document change");

            assertTrue(latch1.await(5, TimeUnit.SECONDS), "First listener should be called");
            assertTrue(latch2.await(5, TimeUnit.SECONDS), "Second listener should be called");
            assertEquals(1, events1.size());
            assertEquals(1, events2.size());
        }
    }

    @Test
    void testRemoveListener() throws Exception {
        try (Repo repo = Repo.load()) {
            DocHandle handle = waitFor(repo.create(), "document creation");

            CountDownLatch latch = new CountDownLatch(1);
            List<DocumentChanged> events = Collections.synchronizedList(new ArrayList<>());

            ListenerRegistration registration = handle.addChangeListener(event -> {
                events.add(event);
                latch.countDown();
            });

            // Make a change - listener should fire
            waitFor(handle.withDocument(doc -> {
                try (Transaction tx = doc.startTransaction()) {
                    tx.set(ObjectId.ROOT, "key", "value1");
                    tx.commit();
                }
                return null;
            }), "first change");

            assertTrue(latch.await(5, TimeUnit.SECONDS), "Listener should be called");
            assertEquals(1, events.size());

            // Remove the listener
            registration.remove();

            // Make another change - listener should NOT fire
            waitFor(handle.withDocument(doc -> {
                try (Transaction tx = doc.startTransaction()) {
                    tx.set(ObjectId.ROOT, "key", "value2");
                    tx.commit();
                }
                return null;
            }), "second change");

            // Wait a bit to make sure no extra event arrives
            Thread.sleep(200);
            assertEquals(1, events.size(), "Listener should not be called after removal");
        }
    }

    @Test
    void testRemoveListenerIdempotent() throws Exception {
        try (Repo repo = Repo.load()) {
            DocHandle handle = waitFor(repo.create(), "document creation");

            ListenerRegistration registration = handle.addChangeListener(event -> {});

            // Remove twice - should not throw
            registration.remove();
            registration.remove();
        }
    }

    @Test
    void testListenerExceptionDoesNotBreakOtherListeners() throws Exception {
        try (Repo repo = Repo.load()) {
            DocHandle handle = waitFor(repo.create(), "document creation");

            CountDownLatch latch = new CountDownLatch(1);
            List<DocumentChanged> events = Collections.synchronizedList(new ArrayList<>());

            // First listener throws
            handle.addChangeListener(event -> {
                throw new RuntimeException("Test exception");
            });

            // Second listener should still be called
            handle.addChangeListener(event -> {
                events.add(event);
                latch.countDown();
            });

            // Make a change
            waitFor(handle.withDocument(doc -> {
                try (Transaction tx = doc.startTransaction()) {
                    tx.set(ObjectId.ROOT, "key", "value");
                    tx.commit();
                }
                return null;
            }), "document change");

            assertTrue(latch.await(5, TimeUnit.SECONDS),
                    "Second listener should be called despite first throwing");
            assertEquals(1, events.size());
        }
    }

    @Test
    void testListenerOnDifferentDocuments() throws Exception {
        try (Repo repo = Repo.load()) {
            DocHandle handle1 = waitFor(repo.create(), "doc1 creation");
            DocHandle handle2 = waitFor(repo.create(), "doc2 creation");

            CountDownLatch latch1 = new CountDownLatch(1);
            CountDownLatch latch2 = new CountDownLatch(1);
            List<DocumentChanged> events1 = Collections.synchronizedList(new ArrayList<>());
            List<DocumentChanged> events2 = Collections.synchronizedList(new ArrayList<>());

            handle1.addChangeListener(event -> {
                events1.add(event);
                latch1.countDown();
            });
            handle2.addChangeListener(event -> {
                events2.add(event);
                latch2.countDown();
            });

            // Change only doc1
            waitFor(handle1.withDocument(doc -> {
                try (Transaction tx = doc.startTransaction()) {
                    tx.set(ObjectId.ROOT, "key", "value");
                    tx.commit();
                }
                return null;
            }), "doc1 change");

            assertTrue(latch1.await(5, TimeUnit.SECONDS), "Doc1 listener should be called");
            assertEquals(1, events1.size());

            // Doc2 listener should NOT have been called
            Thread.sleep(200);
            assertEquals(0, events2.size(), "Doc2 listener should not be called for doc1 change");

            // Now change doc2
            waitFor(handle2.withDocument(doc -> {
                try (Transaction tx = doc.startTransaction()) {
                    tx.set(ObjectId.ROOT, "key", "value");
                    tx.commit();
                }
                return null;
            }), "doc2 change");

            assertTrue(latch2.await(5, TimeUnit.SECONDS), "Doc2 listener should be called");
            assertEquals(1, events2.size());
            // Doc1 should still have only 1 event
            assertEquals(1, events1.size());
        }
    }
}
