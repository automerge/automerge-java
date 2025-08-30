package org.automerge;

import java.util.Arrays;
import java.util.Collections;
import java.util.HashMap;
import java.util.Map;
import java.util.Objects;

/**
 * Contains the results of processing an event through the Hub actor.
 *
 * HubResults represents all the actions and state changes that occurred
 * as a result of processing a single event. The runtime is responsible
 * for executing the IO tasks, routing actor messages, spawning new actors,
 * and handling connection events.
 */
public class HubResults {

    private final IoTask<HubIoAction>[] newTasks;
    private final Map<CommandId, CommandResult> completedCommands;
    private final SpawnArgs[] spawnActors;
    private final ActorMessage[] actorMessages;
    private final ConnectionEvent[] connectionEvents;
    private final boolean stopped;

    /**
     * Creates a HubResults instance.
     * Package-private constructor - only called from JNI layer.
     *
     * @param newTasks IO tasks that must be executed by the calling application
     * @param completedCommands Commands that have completed execution
     * @param spawnActors Requests to spawn new document actors
     * @param actorMessages Messages to send to document actors
     * @param connectionEvents Connection events emitted during processing
     * @param stopped Indicates whether the hub is currently stopped
     */
    @SuppressWarnings("unchecked")
    HubResults(
        IoTask<HubIoAction>[] newTasks,
        Map<CommandId, CommandResult> completedCommands,
        SpawnArgs[] spawnActors,
        ActorMessage[] actorMessages,
        ConnectionEvent[] connectionEvents,
        boolean stopped
    ) {
        this.newTasks = Objects.requireNonNull(newTasks, "newTasks cannot be null");
        this.completedCommands = Collections.unmodifiableMap(
            new HashMap<>(Objects.requireNonNull(completedCommands, "completedCommands cannot be null"))
        );
        this.spawnActors = Objects.requireNonNull(spawnActors, "spawnActors cannot be null");
        this.actorMessages = Objects.requireNonNull(actorMessages, "actorMessages cannot be null");
        this.connectionEvents = Objects.requireNonNull(connectionEvents, "connectionEvents cannot be null");
        this.stopped = stopped;
    }

    /**
     * Gets the IO tasks that must be executed by the calling application.
     *
     * Each task represents either a storage operation (load, store, delete)
     * or a network operation (send message). The caller must execute these
     * operations and notify completion via Event::io_complete.
     *
     * Tasks are identified by their IoTaskId which must be included
     * in the completion notification to match results with requests.
     *
     * @return Array of IO tasks to execute
     */
    public IoTask<HubIoAction>[] getNewTasks() {
        return newTasks.clone(); // Defensive copy
    }

    /**
     * Gets the commands that have completed execution.
     *
     * This map contains command results keyed by their CommandId.
     * Applications can use this to retrieve the results of commands
     * they initiated using Event methods.
     *
     * Common command results include:
     * - CreateConnection: Returns the new connection ID
     * - DisconnectConnection: Confirms disconnection
     * - Receive: Confirms message processing
     *
     * @return Immutable map of completed commands
     */
    public Map<CommandId, CommandResult> getCompletedCommands() {
        return completedCommands;
    }

    /**
     * Gets the requests to spawn new document actors.
     *
     * The caller should create document actor instances for these requests
     * and begin managing their lifecycle. Each entry contains the unique
     * actor ID and the document ID it should manage.
     *
     * @return Array of spawn requests
     */
    public SpawnArgs[] getSpawnActors() {
        return spawnActors.clone(); // Defensive copy
    }

    /**
     * Gets the messages to send to document actors.
     *
     * The caller should route these messages to the appropriate document
     * actor instances. Each entry contains the target actor ID and the
     * message to deliver.
     *
     * @return Array of actor messages
     */
    public ActorMessage[] getActorMessages() {
        return actorMessages.clone(); // Defensive copy
    }

    /**
     * Gets the connection events emitted during processing.
     *
     * These events indicate changes in connection state, such as successful
     * handshake completion, handshake failures, or connection disconnections.
     * Applications can use these events to track network connectivity and
     * respond to connection state changes.
     *
     * Events include:
     * - HandshakeCompleted: Connection successfully established with peer
     * - HandshakeFailed: Handshake failed due to protocol or format errors
     * - ConnectionEstablished: Connection ready for document sync
     * - ConnectionFailed: Connection failed or was disconnected
     *
     * @return Array of connection events
     */
    public ConnectionEvent[] getConnectionEvents() {
        return connectionEvents.clone(); // Defensive copy
    }

    /**
     * Indicates whether the hub is currently stopped.
     *
     * @return true if the hub is stopped, false otherwise
     */
    public boolean isStopped() {
        return stopped;
    }

    @Override
    public boolean equals(Object obj) {
        if (this == obj) return true;
        if (obj == null || getClass() != obj.getClass()) return false;
        HubResults that = (HubResults) obj;
        return stopped == that.stopped &&
               Arrays.equals(newTasks, that.newTasks) &&
               Objects.equals(completedCommands, that.completedCommands) &&
               Arrays.equals(spawnActors, that.spawnActors) &&
               Arrays.equals(actorMessages, that.actorMessages) &&
               Arrays.equals(connectionEvents, that.connectionEvents);
    }

    @Override
    public int hashCode() {
        return Objects.hash(
            Arrays.hashCode(newTasks),
            completedCommands,
            Arrays.hashCode(spawnActors),
            Arrays.hashCode(actorMessages),
            Arrays.hashCode(connectionEvents),
            stopped
        );
    }

    @Override
    public String toString() {
        return "HubResults{" +
               "newTasks=" + Arrays.toString(newTasks) +
               ", completedCommands=" + completedCommands +
               ", spawnActors=" + Arrays.toString(spawnActors) +
               ", actorMessages=" + Arrays.toString(actorMessages) +
               ", connectionEvents=" + Arrays.toString(connectionEvents) +
               ", stopped=" + stopped +
               "}";
    }
}
