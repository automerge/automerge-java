package org.automerge;

import java.util.ArrayList;
import java.util.List;
import java.util.Objects;

/**
 * The Hub actor manages connections, document lifecycle, and message routing.
 *
 * The Hub is the central coordinator for samod-core functionality. It handles:
 * - Creating and managing network connections
 * - Spawning and managing document actors
 * - Routing messages between peers and documents
 * - Processing commands and IO operations
 *
 * ## Usage Pattern
 *
 * 1. Load a Hub instance using {@link SamodLoader}
 * 2. Process events through {@link #handleEvent(long, HubEvent)}
 * 3. Execute IO tasks returned in {@link HubResults}
 * 4. Route actor messages and spawn actors as needed
 * 5. Notify IO completion via {@link HubEvent#ioComplete(IoResult)}
 */
public class Hub {

    private final AutomergeSys.HubPointer pointer;

    /**
     * Creates a Hub with the given pointer.
     * Package-private constructor - only called from SamodLoader.
     * @param pointer The opaque pointer to the Rust Hub
     */
    Hub(AutomergeSys.HubPointer pointer) {
        this.pointer = Objects.requireNonNull(
            pointer,
            "pointer cannot be null"
        );
    }

    /**
     * Gets the internal pointer.
     * This is used by the JNI layer to access the Rust Hub.
     * @return The opaque pointer
     */
    AutomergeSys.HubPointer getPointer() {
        return pointer;
    }

    /**
     * Processes an event and returns any resulting IO tasks or command completions.
     *
     * This is the main interface for interacting with samod-core. Events can be
     * commands to execute, IO completion notifications, or periodic ticks.
     *
     * @param now The current timestamp (Unix timestamp in milliseconds)
     * @param event The event to process
     * @return HubResults containing IO tasks, completed commands, and other outcomes
     */
    public HubResults handleEvent(long now, HubEvent event) {
        return AutomergeSys.hubHandleEvent(pointer, now, event.getPointer());
    }

    /**
     * Returns the storage ID for this samod instance.
     *
     * The storage ID is a UUID that identifies the storage layer this peer is
     * connected to. Multiple peers may share the same storage ID when they're
     * connected to the same underlying storage (e.g., browser tabs sharing
     * IndexedDB, processes sharing filesystem storage).
     *
     * @return The storage ID for this instance
     */
    public StorageId getStorageId() {
        return AutomergeSys.hubGetStorageId(pointer);
    }

    /**
     * Returns the peer ID for this samod instance.
     *
     * The peer ID is a unique identifier for this specific peer instance.
     * It is generated once at startup and used for all connections.
     *
     * @return The peer ID for this instance
     */
    public PeerId getPeerId() {
        return AutomergeSys.hubGetPeerId(pointer);
    }

    /**
     * Returns a list of all connection information.
     *
     * This includes connections in all states: handshaking, established, and failed.
     *
     * @return A list of all connection information currently managed by this instance
     */
    public List<ConnectionInfo> getConnections() {
        ConnectionInfo[] connections = AutomergeSys.hubGetConnections(pointer);
        List<ConnectionInfo> result = new ArrayList<>(connections.length);
        for (ConnectionInfo info : connections) {
            result.add(info);
        }
        return result;
    }

    /**
     * Returns a list of all established peer connections.
     *
     * This only includes connections that have successfully completed the handshake
     * and are in the established state.
     *
     * @return A list of established peers with their connection and peer IDs
     */
    public List<EstablishedPeer> getEstablishedPeers() {
        EstablishedPeer[] peers = AutomergeSys.hubGetEstablishedPeers(pointer);
        List<EstablishedPeer> result = new ArrayList<>(peers.length);
        for (EstablishedPeer peer : peers) {
            result.add(peer);
        }
        return result;
    }

    /**
     * Checks if this instance is connected to a specific peer.
     *
     * @param peerId The peer ID to check for
     * @return true if there is an established connection to the specified peer, false otherwise
     */
    public boolean isConnectedTo(PeerId peerId) {
        return AutomergeSys.hubIsConnectedTo(pointer, peerId);
    }

    /**
     * Checks if this hub instance has been stopped.
     *
     * @return true if the hub is stopped, false otherwise
     */
    public boolean isStopped() {
        return AutomergeSys.hubIsStopped(pointer);
    }

    /**
     * Manually frees the underlying Rust memory.
     * This must be called when the Hub is no longer needed to prevent memory leaks.
     * Do not use this Hub after calling free().
     */
    public void free() {
        AutomergeSys.freeHub(pointer);
    }

    @Override
    public String toString() {
        return (
            "Hub{peerId=" + getPeerId() + ", storageId=" + getStorageId() + "}"
        );
    }
}
