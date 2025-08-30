package org.automerge;

import java.util.Objects;

public class DocHandle {
    private AutomergeUrl url;

    public AutomergeUrl getUrl() {
        return url;
    }

    @Override
    public boolean equals(Object obj) {
        if (this == obj) {
            return true;
        }
        if (obj == null || getClass() != obj.getClass()) {
            return false;
        }
        DocHandle docHandle = (DocHandle) obj;
        return Objects.equals(url, docHandle.url);
    }

    @Override
    public int hashCode() {
        return Objects.hash(url);
    }
}
