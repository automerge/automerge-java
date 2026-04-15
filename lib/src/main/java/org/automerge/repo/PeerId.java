package org.automerge.repo;

import java.util.Objects;
import org.automerge.LoadLibrary;

/**
 * Represents a peer ID in the network protocol
 */
public class PeerId {
    static {
        LoadLibrary.initialize();
    }

    private final String value;

    PeerId(String value) {
        this.value = Objects.requireNonNull(value, "PeerId value cannot be null");
    }

    /**
     * Generates a new random PeerId .
     *
     * @return A new randomly generated PeerId
     */
    public static PeerId generate() {
        return RepoSys.generatePeerId();
    }

    /**
     * Creates a PeerId from a string value.
     *
     * @param value
     *              The peer ID string
     * @return A PeerId instance
     */
    public static PeerId fromString(String value) {
        return new PeerId(value);
    }

    /**
     * Gets the string representation of this PeerId.
     *
     * @return The peer ID string
     */
    public String getValue() {
        return value;
    }

    @Override
    public boolean equals(Object obj) {
        if (this == obj)
            return true;
        if (obj == null || getClass() != obj.getClass())
            return false;
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
