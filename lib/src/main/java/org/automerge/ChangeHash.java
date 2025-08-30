package org.automerge;

import java.util.Arrays;

/** The hash of a single change to an automerge document */
public class ChangeHash {
    private byte[] hash;

    protected ChangeHash(byte[] hash) {
        this.hash = hash;
    }

    /**
     * @return the bytes of the hash
     */
    public byte[] getBytes() {
        return hash.clone();
    }

    @Override
    public int hashCode() {
        final int prime = 31;
        int result = 1;
        result = prime * result + Arrays.hashCode(hash);
        return result;
    }

    @Override
    public boolean equals(Object obj) {
        if (this == obj)
            return true;
        if (obj == null)
            return false;
        if (getClass() != obj.getClass())
            return false;
        ChangeHash other = (ChangeHash) obj;
        if (!Arrays.equals(hash, other.hash))
            return false;
        return true;
    }
}
