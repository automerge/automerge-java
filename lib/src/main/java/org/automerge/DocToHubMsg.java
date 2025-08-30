package org.automerge;

import java.util.Arrays;
import java.util.Objects;

/**
 * Messages sent from document actors to hub.
 *
 * This class uses byte array serialization to cross the JNI boundary,
 * as DocToHubMsg implements to_bytes() -> Vec<u8> and TryFrom<&[u8]> in Rust.
 */
public class DocToHubMsg {
    private final byte[] serializedData;

    // Package-private constructor used by native methods
    DocToHubMsg(byte[] serializedData) {
        this.serializedData = Arrays.copyOf(serializedData, serializedData.length);
    }

    /**
     * Create a DocToHubMsg from serialized bytes.
     *
     * @param bytes The serialized bytes
     * @return A new DocToHubMsg instance
     * @throws IllegalArgumentException if the bytes do not represent a valid DocToHubMsg
     */
    public static DocToHubMsg fromBytes(byte[] bytes) {
        return AutomergeSys.createDocToHubMsgFromBytes(bytes);
    }

    /**
     * Get the serialized byte representation of this message.
     *
     * @return A copy of the serialized bytes
     */
    public byte[] toBytes() {
        return Arrays.copyOf(serializedData, serializedData.length);
    }

    @Override
    public boolean equals(Object obj) {
        if (this == obj) return true;
        if (obj == null || getClass() != obj.getClass()) return false;
        DocToHubMsg that = (DocToHubMsg) obj;
        return Arrays.equals(serializedData, that.serializedData);
    }

    @Override
    public int hashCode() {
        return Arrays.hashCode(serializedData);
    }

    @Override
    public String toString() {
        return AutomergeSys.displayDocToHubMsg(this);
    }
}
