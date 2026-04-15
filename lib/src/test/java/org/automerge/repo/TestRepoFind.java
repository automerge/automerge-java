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

public class TestRepoFind {

    @Test
    public void findResolvesDocumentInStorage() throws Exception {
        InMemoryStorage storage = new InMemoryStorage();

        Document doc = new Document();
        try (Transaction tx = doc.startTransaction()) {
            tx.set(ObjectId.ROOT, "key", "hello");
            tx.commit();
        }
        byte[] initialContent = doc.save();

        DocumentId documentId;
        try (Repo repo = Repo.load(repoConfig(storage))) {
            DocHandle handle = repo.create(initialContent).get(5, TimeUnit.SECONDS);
            documentId = handle.getDocumentId();
        }

        try (Repo repo = Repo.load(repoConfig(storage))) {
            DocHandle handle = repo.find(documentId).get(5, TimeUnit.SECONDS).get();
            assertNotNull(handle, "find should resolve a handle for a document in storage");
            assertEquals(documentId, handle.getDocumentId());

            String value = handle.withDocument(view -> {
                Optional<AmValue> v = view.get(ObjectId.ROOT, "key");
                return v.map(amv -> ((AmValue.Str) amv).getValue()).orElse(null);
            }).get(5, TimeUnit.SECONDS);

            assertEquals("hello", value);
        }
    }

    private static RepoConfig repoConfig(InMemoryStorage storage) {
        return RepoConfig.builder().storage(storage).peerId(PeerId.generate()).build();
    }
}
