package org.automerge.repo;

import static org.junit.jupiter.api.Assertions.*;

import java.util.Optional;
import java.util.concurrent.TimeUnit;
import org.automerge.AmValue;
import org.automerge.Document;
import org.automerge.ObjectId;
import org.automerge.Transaction;
import org.automerge.repo.storage.InMemoryStorage;
import org.junit.jupiter.api.Test;

public class TestRepoChange {

    @Test
    public void changePersistsAcrossReopen() throws Exception {
        InMemoryStorage storage = new InMemoryStorage();

        // Seed an empty document so we have something to mutate.
        byte[] initialContent = new Document().save();

        DocumentId documentId;
        try (Repo repo = Repo.load(repoConfig(storage))) {
            DocHandle handle = repo.create(initialContent).get(5, TimeUnit.SECONDS);
            documentId = handle.getDocumentId();

            handle.withDocument(doc -> {
                try (Transaction tx = doc.startTransaction()) {
                    tx.set(ObjectId.ROOT, "key", "world");
                    tx.commit();
                }
                return null;
            }).get(5, TimeUnit.SECONDS);
        }

        try (Repo repo = Repo.load(repoConfig(storage))) {
            DocHandle handle = repo.find(documentId).get(5, TimeUnit.SECONDS).get();
            assertNotNull(handle);

            String value = handle.withDocument(doc -> {
                Optional<AmValue> v = doc.get(ObjectId.ROOT, "key");
                return v.map(amv -> ((AmValue.Str) amv).getValue()).orElse(null);
            }).get(5, TimeUnit.SECONDS);

            assertEquals("world", value, "value written via change should survive close/reopen");
        }
    }

    @Test
    public void documentBecomesUnusableAfterCallback() throws Exception {
        try (Repo repo = Repo.load()) {
            DocHandle handle = repo.create(new Document().save()).get(5, TimeUnit.SECONDS);

            Document leaked = handle.withDocument(doc -> doc).get(5, TimeUnit.SECONDS);

            assertThrows(Exception.class, () -> leaked.get(ObjectId.ROOT, "key"),
                    "stashed Document reference should not be usable after the callback returns");
        }
    }

    private static RepoConfig repoConfig(InMemoryStorage storage) {
        return RepoConfig.builder().storage(storage).peerId(PeerId.generate()).build();
    }
}
