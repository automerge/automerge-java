package org.automerge.repo;

import static org.junit.jupiter.api.Assertions.*;

import org.junit.jupiter.api.Test;

public class TestAutomergeUrl {
    @Test
    public void automergeUrlRoundTrip() {
        DocumentId docId = DocumentId.generate();
        AutomergeUrl url = AutomergeUrl.parse("automerge:" + RepoSys.automergeUrlFromDocumentId(docId)
                .substring("automerge:".length()));
        assertEquals(docId, url.getId());
    }

    @Test
    public void automergeUrlFromKnownString() {
        String urlString = RepoSys.automergeUrlFromDocumentId(DocumentId.generate());
        AutomergeUrl parsed = AutomergeUrl.parse(urlString);
        assertEquals(urlString, parsed.toString());
    }
}
