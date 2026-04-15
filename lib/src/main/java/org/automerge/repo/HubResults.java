package org.automerge.repo;

import java.util.Collections;
import java.util.HashMap;
import java.util.List;
import java.util.Map;
import java.util.Objects;

/**
 * Contains the results of processing an event through the Hub actor.
 *
 * <p>
 * HubResults represents all the actions and state changes that occurred as a
 * result of processing a single event. The runtime is responsible for executing
 * the IO tasks, routing actor messages, spawning new actors, and handling
 * connection events.
 */
class HubResults {

    private final List<IoTask<HubIoAction>> newTasks;
    private final Map<CommandId, CommandResult> completedCommands;
    private final List<SpawnedActor> spawnActors;
    private final List<ActorMessage> actorMessages;
    private final List<ConnectionEvent> connectionEvents;
    private final List<DialRequest> dialRequests;
    private final List<DialerEvent> dialerEvents;
    private final boolean stopped;

    HubResults(List<IoTask<HubIoAction>> newTasks, Map<CommandId, CommandResult> completedCommands,
            List<SpawnedActor> spawnActors, List<ActorMessage> actorMessages, List<ConnectionEvent> connectionEvents,
            List<DialRequest> dialRequests, List<DialerEvent> dialerEvents, boolean stopped) {
        this.newTasks = Objects.requireNonNull(newTasks, "newTasks cannot be null");
        this.completedCommands = Collections.unmodifiableMap(
                new HashMap<>(Objects.requireNonNull(completedCommands, "completedCommands cannot be null")));
        this.spawnActors = Objects.requireNonNull(spawnActors, "spawnActors cannot be null");
        this.actorMessages = Objects.requireNonNull(actorMessages, "actorMessages cannot be null");
        this.connectionEvents = Objects.requireNonNull(connectionEvents, "connectionEvents cannot be null");
        this.dialRequests = Objects.requireNonNull(dialRequests, "dialRequests cannot be null");
        this.dialerEvents = Objects.requireNonNull(dialerEvents, "dialerEvents cannot be null");
        this.stopped = stopped;
    }

    List<IoTask<HubIoAction>> getNewTasks() {
        return newTasks;
    }

    Map<CommandId, CommandResult> getCompletedCommands() {
        return completedCommands;
    }

    List<SpawnedActor> getSpawnActors() {
        return spawnActors;
    }

    List<ActorMessage> getActorMessages() {
        return actorMessages;
    }

    List<ConnectionEvent> getConnectionEvents() {
        return connectionEvents;
    }

    List<DialRequest> getDialRequests() {
        return dialRequests;
    }

    List<DialerEvent> getDialerEvents() {
        return dialerEvents;
    }

    boolean isStopped() {
        return stopped;
    }

    @Override
    public boolean equals(Object obj) {
        if (this == obj)
            return true;
        if (obj == null || getClass() != obj.getClass())
            return false;
        HubResults that = (HubResults) obj;
        return (stopped == that.stopped && Objects.equals(newTasks, that.newTasks)
                && Objects.equals(completedCommands, that.completedCommands)
                && Objects.equals(spawnActors, that.spawnActors) && Objects.equals(actorMessages, that.actorMessages)
                && Objects.equals(connectionEvents, that.connectionEvents)
                && Objects.equals(dialRequests, that.dialRequests) && Objects.equals(dialerEvents, that.dialerEvents));
    }

    @Override
    public int hashCode() {
        return Objects.hash(newTasks, completedCommands, spawnActors, actorMessages, connectionEvents, dialRequests,
                dialerEvents, stopped);
    }

    @Override
    public String toString() {
        return ("HubResults{" + "newTasks=" + newTasks + ", completedCommands=" + completedCommands + ", spawnActors="
                + spawnActors + ", actorMessages=" + actorMessages + ", connectionEvents=" + connectionEvents
                + ", dialRequests=" + dialRequests + ", dialerEvents=" + dialerEvents + ", stopped=" + stopped + "}");
    }
}
