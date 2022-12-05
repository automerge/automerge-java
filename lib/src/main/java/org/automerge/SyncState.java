package org.automerge;

import java.util.List;
import java.util.Optional;

/**
 * The state of an active sync connection
 *
 * <p>
 * The sync protocol is designed to run over a reliable in order stream. When
 * the stream begins you should create a {@link SyncState} and then use
 * {@link Document#receiveSyncMessage(SyncState, byte[])} and
 * {@link Document#generateSyncMessage(SyncState)} to send and receive messages.
 *
 * <h2>Memory Management</h2>
 *
 * The backing memory for the sync state is not managed by the JVM so you must
 * call {@link #free} when you are done with the sync state to avoid leaking.
 *
 * <h2>Persisting sync states</h2>
 *
 * If you are able to identify the peers you are syncing with and know you may
 * reconnect with them later then you can persist the sync state to storage
 * using {@link encode} and then restore it later using {@link decode}. This
 * will mean re-syncing later may require less round trips.
 */
public class SyncState {
	private Optional<AutomergeSys.SyncStatePointer> pointer;

	private SyncState(AutomergeSys.SyncStatePointer pointer) {
		this.pointer = Optional.of(pointer);
	}

	/** Create a new sync state for a new connection */
	public SyncState() {
		this(AutomergeSys.createSyncState());
	}

	/**
	 * Decode a previously encoded sync state
	 *
	 * @param encoded
	 *            The encoded sync state
	 * @return The decoded sync state
	 * @throws AutomergeException
	 *             if the encoded sync state is not valid
	 */
	public static SyncState decode(byte[] encoded) {
		return new SyncState(AutomergeSys.decodeSyncState(encoded));
	}

	protected synchronized Optional<byte[]> generateSyncMessage(Document doc) {
		return doc.generateSyncMessage(this.pointer.get());
	}

	protected synchronized void receiveSyncMessage(Document doc, byte[] message) {
		doc.receiveSyncMessage(this.pointer.get(), message);
	}

	protected synchronized List<Patch> receiveSyncMessageForPatches(Document doc, byte[] message) {
		return doc.receiveSyncMessageForPatches(this.pointer.get(), message);
	}

	/**
	 * Encode the sync state for storage
	 *
	 * @return The encoded sync state
	 */
	public synchronized byte[] encode() {
		return AutomergeSys.encodeSyncState(this.pointer.get());
	}

	/** Free the memory associated with this sync state */
	public synchronized void free() {
		if (this.pointer.isPresent()) {
			AutomergeSys.freeSyncState(this.pointer.get());
			this.pointer = Optional.empty();
		}
	}
}
