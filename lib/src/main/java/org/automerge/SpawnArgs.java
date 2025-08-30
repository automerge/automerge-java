package org.automerge;

import java.util.Arrays;
import java.util.Objects;

/**
 * Arguments for spawning document actors.
 *
 * This class uses byte array serialization to cross the JNI boundary,
 * as SpawnArgs implements to_bytes() -> Vec<u8> and TryFrom<&[u8]> in Rust.
 */
public class SpawnArgs {

    private final byte[] serializedData;

    // Package-private constructor used by native methods
    SpawnArgs(byte[] serializedData) {
        this.serializedData = Arrays.copyOf(
            serializedData,
            serializedData.length
        );
    }

    /**
     * Create a SpawnArgs from serialized bytes.
     *
     * @param bytes The serialized bytes
     * @return A new SpawnArgs instance
     * @throws IllegalArgumentException if the bytes do not represent a valid SpawnArgs
     */
    public static SpawnArgs fromBytes(byte[] bytes) {
        return AutomergeSys.createSpawnArgsFromBytes(bytes);
    }

    /**
     * Get the serialized byte representation of these spawn arguments.
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
        SpawnArgs that = (SpawnArgs) obj;
        return Arrays.equals(serializedData, that.serializedData);
    }

    @Override
    public int hashCode() {
        return Arrays.hashCode(serializedData);
    }

    @Override
    public String toString() {
        return AutomergeSys.displaySpawnArgs(this);
    }
}
