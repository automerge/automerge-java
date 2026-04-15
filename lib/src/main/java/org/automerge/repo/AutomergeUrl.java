package org.automerge.repo;

import java.util.Objects;
import org.automerge.LoadLibrary;

public class AutomergeUrl {

    private DocumentId id;
    private String urlString;

    AutomergeUrl(DocumentId docId) {
        this.id = docId;
        this.urlString = RepoSys.automergeUrlFromDocumentId(docId);
    }

    /**
     * Parse an automerge URL string (e.g. "automerge:&lt;documentId&gt;").
     *
     * @param url
     *            the URL string to parse
     * @return the parsed AutomergeUrl
     * @throws IllegalArgumentException
     *             if the string is not a valid automerge URL
     */
    public static AutomergeUrl parse(String url) {
        LoadLibrary.initialize();
        return RepoSys.parseAutomergeUrl(url);
    }

    @Override
    public boolean equals(Object obj) {
        if (this == obj)
            return true;
        if (obj == null || getClass() != obj.getClass())
            return false;
        AutomergeUrl that = (AutomergeUrl) obj;
        return Objects.equals(id, that.id);
    }

    @Override
    public int hashCode() {
        return Objects.hash(id);
    }

    public DocumentId getId() {
        return id;
    }

    @Override
    public String toString() {
        return urlString;
    }
}
