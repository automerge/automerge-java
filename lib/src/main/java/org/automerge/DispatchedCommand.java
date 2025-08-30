package org.automerge;

import java.util.Objects;

/**
 * Represents a command that has been dispatched with a tracking ID.
 * Contains both the command ID for tracking completion and the event to be processed.
 */
public class DispatchedCommand {
    private final CommandId commandId;
    private final HubEvent event;

    /**
     * Creates a DispatchedCommand with the given command ID and event.
     * Package-private constructor - only called from JNI layer.
     * @param commandId The unique command identifier for tracking
     * @param event The hub event to be processed
     */
    DispatchedCommand(CommandId commandId, HubEvent event) {
        this.commandId = Objects.requireNonNull(commandId, "commandId cannot be null");
        this.event = Objects.requireNonNull(event, "event cannot be null");
    }

    /**
     * Gets the command ID for tracking completion.
     * @return The command ID
     */
    public CommandId getCommandId() {
        return commandId;
    }

    /**
     * Gets the hub event to be processed.
     * @return The hub event
     */
    public HubEvent getEvent() {
        return event;
    }

    @Override
    public boolean equals(Object obj) {
        if (this == obj) return true;
        if (obj == null || getClass() != obj.getClass()) return false;
        DispatchedCommand that = (DispatchedCommand) obj;
        return Objects.equals(commandId, that.commandId) && Objects.equals(event, that.event);
    }

    @Override
    public int hashCode() {
        return Objects.hash(commandId, event);
    }

    @Override
    public String toString() {
        return "DispatchedCommand{commandId=" + commandId + ", event=" + event + "}";
    }
}
