package org.automerge.repo;

import java.util.Objects;
import org.automerge.LoadLibrary;

public class DocumentId {
    static {
        LoadLibrary.initialize();
    }

    private byte[] bytes;

    /**
     * Creates a DocumentId with the given bytes. Package-private constructor - only
     * called from JNI layer.
     *
     * @param bytes
     *            The document ID bytes
     */
    private DocumentId(byte[] bytes) {
        this.bytes = Objects.requireNonNull(bytes, "bytes cannot be null").clone();
    }

    public static DocumentId fromBytes(byte[] bytes) {
        LoadLibrary.initialize();
        return RepoSys.documentIdFromBytes(bytes);
    }

    public static DocumentId generate() {
        LoadLibrary.initialize();
        return RepoSys.generateDocumentId();
    }

    /**
     * @return a defensive copy of the raw bytes backing this document id.
     */
    public byte[] getBytes() {
        return bytes.clone();
    }

    public boolean equals(Object obj) {
        if (this == obj)
            return true;
        if (obj == null || getClass() != obj.getClass())
            return false;
        DocumentId other = (DocumentId) obj;
        if (bytes.length != other.bytes.length) {
            return false;
        }
        for (int i = 0; i < bytes.length; i++) {
            if (bytes[i] != other.bytes[i]) {
                return false;
            }
        }
        return true;
    }

    public int hashCode() {
        int result = 1;
        for (byte b : bytes) {
            result = 31 * result + b;
        }
        return result;
    }
}
