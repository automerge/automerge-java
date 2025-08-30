package org.automerge;

import java.util.Arrays;
import java.util.Objects;

/**
 * Contains the results of processing an operation through a DocumentActor.
 *
 * DocActorResult represents all the actions and state changes that occurred
 * as a result of processing a single operation on a document actor. The runtime
 * is responsible for executing the IO tasks, routing outgoing messages, handling
 * ephemeral messages, and processing change events.
 */
public class DocActorResult {

    private final IoTask<DocumentIoTask>[] ioTasks;
    private final DocToHubMsg[] outgoingMessages;
    private final byte[][] ephemeralMessages;
    private final DocumentChanged[] changeEvents;
    private final boolean stopped;

    /**
     * Creates a DocActorResult instance.
     * Package-private constructor - only called from JNI layer.
     *
     * @param ioTasks Document I/O tasks that need to be executed by the caller
     * @param outgoingMessages Messages to send back to the hub
     * @param ephemeralMessages New ephemeral messages to broadcast
     * @param changeEvents Document change events
     * @param stopped Whether this document actor is stopped
     */
    @SuppressWarnings("unchecked")
    DocActorResult(
        IoTask<DocumentIoTask>[] ioTasks,
        DocToHubMsg[] outgoingMessages,
        byte[][] ephemeralMessages,
        DocumentChanged[] changeEvents,
        boolean stopped
    ) {
        this.ioTasks = Objects.requireNonNull(ioTasks, "ioTasks cannot be null");
        this.outgoingMessages = Objects.requireNonNull(outgoingMessages, "outgoingMessages cannot be null");
        this.ephemeralMessages = Objects.requireNonNull(ephemeralMessages, "ephemeralMessages cannot be null");
        this.changeEvents = Objects.requireNonNull(changeEvents, "changeEvents cannot be null");
        this.stopped = stopped;
    }

    /**
     * Gets the document I/O tasks that need to be executed by the caller.
     *
     * Each task represents either a storage operation (load, store, delete)
     * or an announce policy check. The caller must execute these operations
     * and notify completion via the appropriate mechanisms.
     *
     * @return Array of IO tasks to execute
     */
    public IoTask<DocumentIoTask>[] getIoTasks() {
        return ioTasks.clone(); // Defensive copy
    }

    /**
     * Gets the messages to send back to the hub.
     *
     * These messages represent communications from the document actor back to
     * the main hub actor. The runtime should route these messages appropriately.
     *
     * @return Array of outgoing messages
     */
    public DocToHubMsg[] getOutgoingMessages() {
        return outgoingMessages.clone(); // Defensive copy
    }

    /**
     * Gets the new ephemeral messages to broadcast.
     *
     * Ephemeral messages are typically sync messages or other transient
     * communications that should be broadcast to connected peers but not
     * persisted to storage.
     *
     * @return Array of ephemeral message byte arrays
     */
    public byte[][] getEphemeralMessages() {
        // Deep copy for byte arrays
        byte[][] copy = new byte[ephemeralMessages.length][];
        for (int i = 0; i < ephemeralMessages.length; i++) {
            copy[i] = ephemeralMessages[i].clone();
        }
        return copy;
    }

    /**
     * Gets the document change events.
     *
     * These events indicate that the document has been modified and contain
     * information about what changes occurred. Applications can use these
     * events to update UI, trigger notifications, or perform other reactions
     * to document changes.
     *
     * @return Array of change events
     */
    public DocumentChanged[] getChangeEvents() {
        return changeEvents.clone(); // Defensive copy
    }

    /**
     * Indicates whether this document actor is stopped.
     *
     * @return true if the document actor is stopped, false otherwise
     */
    public boolean isStopped() {
        return stopped;
    }

    @Override
    public boolean equals(Object obj) {
        if (this == obj) return true;
        if (obj == null || getClass() != obj.getClass()) return false;
        DocActorResult that = (DocActorResult) obj;
        return stopped == that.stopped &&
               Arrays.equals(ioTasks, that.ioTasks) &&
               Arrays.equals(outgoingMessages, that.outgoingMessages) &&
               Arrays.deepEquals(ephemeralMessages, that.ephemeralMessages) &&
               Arrays.equals(changeEvents, that.changeEvents);
    }

    @Override
    public int hashCode() {
        return Objects.hash(
            Arrays.hashCode(ioTasks),
            Arrays.hashCode(outgoingMessages),
            Arrays.deepHashCode(ephemeralMessages),
            Arrays.hashCode(changeEvents),
            stopped
        );
    }

    @Override
    public String toString() {
        return "DocActorResult{" +
               "ioTasks=" + Arrays.toString(ioTasks) +
               ", outgoingMessages=" + Arrays.toString(outgoingMessages) +
               ", ephemeralMessages=" + Arrays.deepToString(ephemeralMessages) +
               ", changeEvents=" + Arrays.toString(changeEvents) +
               ", stopped=" + stopped +
               "}";
    }
}
