package org.automerge;

import java.util.Objects;

/**
 * Represents a message that should be sent to a document actor.
 * Contains the target actor ID and the message to be delivered.
 */
public class ActorMessage {
    private final DocumentActorId actorId;
    private final HubToDocMsg message;

    /**
     * Creates an ActorMessage with the given actor ID and message.
     * Package-private constructor - only called from JNI layer.
     * @param actorId The target document actor ID
     * @param message The message to be delivered
     */
    ActorMessage(DocumentActorId actorId, HubToDocMsg message) {
        this.actorId = Objects.requireNonNull(actorId, "actorId cannot be null");
        this.message = Objects.requireNonNull(message, "message cannot be null");
    }

    /**
     * Gets the target document actor ID.
     * @return The actor ID
     */
    public DocumentActorId getActorId() {
        return actorId;
    }

    /**
     * Gets the message to be delivered to the actor.
     * @return The message
     */
    public HubToDocMsg getMessage() {
        return message;
    }

    @Override
    public boolean equals(Object obj) {
        if (this == obj) return true;
        if (obj == null || getClass() != obj.getClass()) return false;
        ActorMessage that = (ActorMessage) obj;
        return Objects.equals(actorId, that.actorId) &&
               Objects.equals(message, that.message);
    }

    @Override
    public int hashCode() {
        return Objects.hash(actorId, message);
    }

    @Override
    public String toString() {
        return "ActorMessage{actorId=" + actorId + ", message=" + message + "}";
    }
}
