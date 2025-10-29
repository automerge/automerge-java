package org.automerge.testapp;

import androidx.test.ext.junit.runners.AndroidJUnit4;
import org.junit.Test;
import org.junit.runner.RunWith;
import static org.junit.Assert.*;

import org.automerge.Document;
import org.automerge.Transaction;
import org.automerge.ObjectId;
import org.automerge.AmValue;

import java.util.Optional;

/**
 * Instrumented test that verifies the full Android integration:
 * 1. The lib JAR is on the classpath (Java code available)
 * 2. The android AAR is on the classpath (native libs available)
 * 3. Android's System.loadLibrary() can find and load the native library
 * 4. The JNI bindings work correctly
 *
 * This test runs on an actual Android device/emulator and simulates exactly
 * how a real Android app would use the automerge library.
 */
@RunWith(AndroidJUnit4.class)
public class LibraryLoadingTest {

    @Test
    public void testLibraryLoadsAndBasicOperationsWork() {
        // This will trigger LoadLibrary.initialize() which calls System.loadLibrary()
        // If the AAR is not properly set up, this will throw UnsatisfiedLinkError
        Document doc = new Document();

        // Verify basic operations work (tests JNI bindings)
        try (Transaction tx = doc.startTransaction()) {
            tx.set(ObjectId.ROOT, "test_key", "test_value");
            tx.commit();
        }

        Optional<AmValue> result = doc.get(ObjectId.ROOT, "test_key");
        assertTrue("Value should be present", result.isPresent());
        assertEquals("test_value", ((AmValue.Str) result.get()).getValue());
    }

    @Test
    public void testMultipleDocuments() {
        // Test that library loading works correctly with multiple instances
        Document doc1 = new Document();
        Document doc2 = new Document();

        try (Transaction tx1 = doc1.startTransaction()) {
            tx1.set(ObjectId.ROOT, "doc", 1);
            tx1.commit();
        }

        try (Transaction tx2 = doc2.startTransaction()) {
            tx2.set(ObjectId.ROOT, "doc", 2);
            tx2.commit();
        }

        Optional<AmValue> result1 = doc1.get(ObjectId.ROOT, "doc");
        Optional<AmValue> result2 = doc2.get(ObjectId.ROOT, "doc");

        assertEquals(1L, ((AmValue.Int) result1.get()).getValue());
        assertEquals(2L, ((AmValue.Int) result2.get()).getValue());
    }

    @Test
    public void testSaveAndLoad() {
        // Test serialization (exercises more JNI functionality)
        Document doc = new Document();

        try (Transaction tx = doc.startTransaction()) {
            tx.set(ObjectId.ROOT, "key", "value");
            tx.set(ObjectId.ROOT, "number", 42);
            tx.commit();
        }

        byte[] bytes = doc.save();
        Document doc2 = Document.load(bytes);

        Optional<AmValue> result1 = doc2.get(ObjectId.ROOT, "key");
        Optional<AmValue> result2 = doc2.get(ObjectId.ROOT, "number");

        assertEquals("value", ((AmValue.Str) result1.get()).getValue());
        assertEquals(42L, ((AmValue.Int) result2.get()).getValue());
    }
}
