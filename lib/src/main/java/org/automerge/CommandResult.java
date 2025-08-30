package org.automerge;

import java.util.Objects;

/**
 * Represents the result of a completed command.
 * This is an abstract base class with concrete subclasses for each result type.
 */
public abstract class CommandResult {

    /**
     * Result of a CreateConnection command.
     */
    public static class CreateConnection extends CommandResult {
        private final ConnectionId connectionId;

        CreateConnection(ConnectionId connectionId) {
            this.connectionId = Objects.requireNonNull(connectionId, "connectionId cannot be null");
        }

        public ConnectionId getConnectionId() {
            return connectionId;
        }

        @Override
        public boolean equals(Object obj) {
            if (this == obj) return true;
            if (obj == null || getClass() != obj.getClass()) return false;
            CreateConnection that = (CreateConnection) obj;
            return Objects.equals(connectionId, that.connectionId);
        }

        @Override
        public int hashCode() {
            return Objects.hash(connectionId);
        }

        @Override
        public String toString() {
            return "CommandResult.CreateConnection{connectionId=" + connectionId + "}";
        }
    }

    /**
     * Result of a DisconnectConnection command.
     */
    public static class DisconnectConnection extends CommandResult {

        DisconnectConnection() {
            // Empty result
        }

        @Override
        public boolean equals(Object obj) {
            return this == obj || (obj != null && getClass() == obj.getClass());
        }

        @Override
        public int hashCode() {
            return getClass().hashCode();
        }

        @Override
        public String toString() {
            return "CommandResult.DisconnectConnection{}";
        }
    }

    /**
     * Result of a Receive command.
     */
    public static class Receive extends CommandResult {
        private final ConnectionId connectionId;
        private final String error; // null if no error

        Receive(ConnectionId connectionId, String error) {
            this.connectionId = Objects.requireNonNull(connectionId, "connectionId cannot be null");
            this.error = error; // can be null
        }

        public ConnectionId getConnectionId() {
            return connectionId;
        }

        public String getError() {
            return error;
        }

        public boolean hasError() {
            return error != null;
        }

        @Override
        public boolean equals(Object obj) {
            if (this == obj) return true;
            if (obj == null || getClass() != obj.getClass()) return false;
            Receive receive = (Receive) obj;
            return Objects.equals(connectionId, receive.connectionId) &&
                   Objects.equals(error, receive.error);
        }

        @Override
        public int hashCode() {
            return Objects.hash(connectionId, error);
        }

        @Override
        public String toString() {
            return "CommandResult.Receive{connectionId=" + connectionId +
                   ", error=" + error + "}";
        }
    }

    /**
     * Result of an ActorReady command.
     */
    public static class ActorReady extends CommandResult {

        ActorReady() {
            // Empty result
        }

        @Override
        public boolean equals(Object obj) {
            return this == obj || (obj != null && getClass() == obj.getClass());
        }

        @Override
        public int hashCode() {
            return getClass().hashCode();
        }

        @Override
        public String toString() {
            return "CommandResult.ActorReady{}";
        }
    }

    /**
     * Result of a CreateDocument command.
     */
    public static class CreateDocument extends CommandResult {
        private final DocumentActorId actorId;
        private final DocumentId documentId;

        CreateDocument(DocumentActorId actorId, DocumentId documentId) {
            this.actorId = Objects.requireNonNull(actorId, "actorId cannot be null");
            this.documentId = Objects.requireNonNull(documentId, "documentId cannot be null");
        }

        public DocumentActorId getActorId() {
            return actorId;
        }

        public DocumentId getDocumentId() {
            return documentId;
        }

        @Override
        public boolean equals(Object obj) {
            if (this == obj) return true;
            if (obj == null || getClass() != obj.getClass()) return false;
            CreateDocument that = (CreateDocument) obj;
            return Objects.equals(actorId, that.actorId) &&
                   Objects.equals(documentId, that.documentId);
        }

        @Override
        public int hashCode() {
            return Objects.hash(actorId, documentId);
        }

        @Override
        public String toString() {
            return "CommandResult.CreateDocument{actorId=" + actorId +
                   ", documentId=" + documentId + "}";
        }
    }

    /**
     * Result of a FindDocument command.
     */
    public static class FindDocument extends CommandResult {
        private final DocumentActorId actorId;
        private final boolean found;

        FindDocument(DocumentActorId actorId, boolean found) {
            this.actorId = Objects.requireNonNull(actorId, "actorId cannot be null");
            this.found = found;
        }

        public DocumentActorId getActorId() {
            return actorId;
        }

        public boolean isFound() {
            return found;
        }

        @Override
        public boolean equals(Object obj) {
            if (this == obj) return true;
            if (obj == null || getClass() != obj.getClass()) return false;
            FindDocument that = (FindDocument) obj;
            return Objects.equals(actorId, that.actorId) && found == that.found;
        }

        @Override
        public int hashCode() {
            return Objects.hash(actorId, found);
        }

        @Override
        public String toString() {
            return "CommandResult.FindDocument{actorId=" + actorId +
                   ", found=" + found + "}";
        }
    }
}
