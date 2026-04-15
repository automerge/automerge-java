package org.automerge.repo;

import java.util.List;
import java.util.Map;
import java.util.Objects;

/**
 * Contains the results of processing an operation through a DocumentActor.
 *
 * <p>
 * DocActorResult represents all the actions and state changes that occurred as
 * a
 * result of processing a single operation on a document actor. The runtime is
 * responsible for executing the IO tasks, routing outgoing messages, handling
 * ephemeral messages, and processing change events.
 *
 * <p>
 * Note: Fields use {@code List} types (not arrays) because the Rust JNI layer
 * maps {@code Vec<T>} to {@code java.util.List}.
 */
class DocActorResult {

    private final List<IoTask<DocumentIoTask>> ioTasks;
    private final List<DocToHubMsg> outgoingMessages;
    private final List<byte[]> ephemeralMessages;
    private final List<DocumentChanged> changeEvents;
    private final boolean stopped;
    private final Map<ConnectionId, PeerDocState> peerStateChanges;

    /**
     * Creates a DocActorResult instance. Package-private constructor - only called
     * from JNI layer via alloc_object + set_field.
     */
    DocActorResult(List<IoTask<DocumentIoTask>> ioTasks, List<DocToHubMsg> outgoingMessages,
            List<byte[]> ephemeralMessages, List<DocumentChanged> changeEvents, boolean stopped,
            Map<ConnectionId, PeerDocState> peerStateChanges) {
        this.ioTasks = Objects.requireNonNull(ioTasks, "ioTasks cannot be null");
        this.outgoingMessages = Objects.requireNonNull(outgoingMessages, "outgoingMessages cannot be null");
        this.ephemeralMessages = Objects.requireNonNull(ephemeralMessages, "ephemeralMessages cannot be null");
        this.changeEvents = Objects.requireNonNull(changeEvents, "changeEvents cannot be null");
        this.stopped = stopped;
        this.peerStateChanges = peerStateChanges != null ? peerStateChanges : java.util.Collections.emptyMap();
    }

    /**
     * Gets the document I/O tasks that need to be executed by the caller.
     *
     * @return List of IO tasks to execute
     */
    public List<IoTask<DocumentIoTask>> getIoTasks() {
        return ioTasks;
    }

    /**
     * Gets the messages to send back to the hub.
     *
     * @return List of outgoing messages
     */
    public List<DocToHubMsg> getOutgoingMessages() {
        return outgoingMessages;
    }

    /**
     * Gets the new ephemeral messages to broadcast.
     *
     * @return List of ephemeral message byte arrays
     */
    public List<byte[]> getEphemeralMessages() {
        return ephemeralMessages;
    }

    /**
     * Gets the document change events.
     *
     * @return List of change events
     */
    public List<DocumentChanged> getChangeEvents() {
        return changeEvents;
    }

    /**
     * Indicates whether this document actor is stopped.
     *
     * @return true if the document actor is stopped, false otherwise
     */
    public boolean isStopped() {
        return stopped;
    }

    /**
     * Gets the peer document state changes.
     *
     * @return Map from connection ID to peer document state
     */
    public Map<ConnectionId, PeerDocState> getPeerStateChanges() {
        return peerStateChanges;
    }

    @Override
    public boolean equals(Object obj) {
        if (this == obj)
            return true;
        if (obj == null || getClass() != obj.getClass())
            return false;
        DocActorResult that = (DocActorResult) obj;
        return (stopped == that.stopped && Objects.equals(ioTasks, that.ioTasks)
                && Objects.equals(outgoingMessages, that.outgoingMessages)
                && Objects.equals(ephemeralMessages, that.ephemeralMessages)
                && Objects.equals(changeEvents, that.changeEvents)
                && Objects.equals(peerStateChanges, that.peerStateChanges));
    }

    @Override
    public int hashCode() {
        return Objects.hash(ioTasks, outgoingMessages, ephemeralMessages, changeEvents, stopped, peerStateChanges);
    }

    @Override
    public String toString() {
        return ("DocActorResult{" + "ioTasks=" + ioTasks + ", outgoingMessages=" + outgoingMessages
                + ", ephemeralMessages=" + ephemeralMessages + ", changeEvents=" + changeEvents + ", stopped=" + stopped
                + ", peerStateChanges=" + peerStateChanges + "}");
    }
}
