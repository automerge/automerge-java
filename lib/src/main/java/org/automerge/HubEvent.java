package org.automerge;

import java.util.Objects;

/**
 * Represents an event that can be processed by the Hub actor.
 * HubEvents are opaque pointers to Rust HubEvent structures.
 */
public class HubEvent {

    private final AutomergeSys.HubEventPointer pointer;

    /**
     * Creates a HubEvent with the given pointer.
     * Package-private constructor - only called from JNI layer.
     * @param pointer The opaque pointer to the Rust HubEvent
     */
    HubEvent(AutomergeSys.HubEventPointer pointer) {
        this.pointer = Objects.requireNonNull(
            pointer,
            "pointer cannot be null"
        );
    }

    /**
     * Gets the internal pointer.
     * This is used by the JNI layer to access the Rust HubEvent.
     * @return The opaque pointer
     */
    AutomergeSys.HubEventPointer getPointer() {
        return pointer;
    }

    @Override
    public boolean equals(Object obj) {
        if (this == obj) return true;
        if (obj == null || getClass() != obj.getClass()) return false;
        HubEvent hubEvent = (HubEvent) obj;
        return AutomergeSys.hubEventEquals(this.pointer, hubEvent.pointer);
    }

    @Override
    public int hashCode() {
        return AutomergeSys.hubEventHashCode(this.pointer);
    }

    @Override
    public String toString() {
        return AutomergeSys.hubEventToString(pointer);
    }

    /**
     * Manually frees the underlying Rust memory.
     * This must be called when the HubEvent is no longer needed to prevent memory leaks.
     * Do not use this HubEvent after calling free().
     */
    public void free() {
        AutomergeSys.freeHubEvent(pointer);
    }
}
