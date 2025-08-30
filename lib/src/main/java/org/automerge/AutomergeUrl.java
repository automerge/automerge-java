package org.automerge;

import java.util.Objects;

class AutomergeUrl {
    private DocumentId id;

    @Override
    public boolean equals(Object obj) {
        if (this == obj) return true;
        if (obj == null || getClass() != obj.getClass()) return false;
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

}
