package org.automerge;

import java.util.Objects;

/**
 * Represents a peer ID in the samod network protocol.
 * PeerId is a string wrapper that delegates to samod-core for generation and validation.
 */
public class PeerId {

    private final String value;

    /**
     * Creates a PeerId with the given string value.
     * Package-private constructor - only called from JNI layer.
     * @param value The peer ID string
     */
    PeerId(String value) {
        this.value = Objects.requireNonNull(
            value,
            "PeerId value cannot be null"
        );
    }

    /**
     * Generates a new random PeerId using samod-core's generation logic.
     * @return A new randomly generated PeerId
     */
    public static PeerId generate() {
        return AutomergeSys.generatePeerId();
    }

    /**
     * Creates a PeerId from a string value.
     * @param value The peer ID string
     * @return A PeerId instance
     */
    public static PeerId fromString(String value) {
        return new PeerId(value);
    }

    /**
     * Gets the string representation of this PeerId.
     * @return The peer ID string
     */
    public String getValue() {
        return value;
    }

    @Override
    public boolean equals(Object obj) {
        if (this == obj) return true;
        if (obj == null || getClass() != obj.getClass()) return false;
        PeerId other = (PeerId) obj;
        return Objects.equals(value, other.value);
    }

    @Override
    public int hashCode() {
        return Objects.hash(value);
    }

    @Override
    public String toString() {
        return value;
    }
}
