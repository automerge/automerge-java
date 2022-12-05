package org.automerge;

import java.util.ArrayList;
import java.util.List;
import java.util.Optional;
import org.automerge.AutomergeSys.DocPointer;
import org.automerge.AutomergeSys.UnobservedTransactionPointer;

/**
 * The entry point to the automerge data model
 *
 * <p>
 * The automerge data model is a tree of maps, lists, and primitive values. This
 * hierarchy is represented by the {@link AmValue} class. Every composite object
 * in an automerge document is represented by an {@link ObjectId}. The root
 * object of the document is a map with ID {@link ObjectId#ROOT}.
 *
 * <h2>Reading Values</h2>
 *
 * Both {@link Document} and {@link Transaction} implement
 * {@link org.automerge.Read}, which specify the methods which allow you to read
 * values from an automerge document.
 *
 * <h2>Transactions</h2>
 *
 * Changes to the document are made in a {@link Transaction}. To obtain a
 * transaction call {@link startTransaction} or
 * {@link startTransactionForPatches}. Attempting to modify the document whilst
 * a transaction is in progress will throw a {@link TransactionInProgress}
 * exception. For this reason transactions should probably be short lived and
 * locally scoped.
 *
 * <h2>Heads</h2>
 *
 * Automerge documents have "heads", which identify a particular version of a
 * document. You can get these heads with {@link Read#getHeads}. These heads can
 * then be used in various methods on {@link Read} and elsewhere to identify the
 * version of the document you want.
 *
 * <h2>Syncing</h2>
 *
 * To sync a document over a realiable in-order transport create an
 * {@link SyncState} for the connection and then use
 * {@link generateSyncMessage(SyncState)} and
 * {@link receiveSyncMessage(SyncState, byte[])}.
 *
 * <h2>Patches</h2>
 *
 * In some situations you may need to know what has changed in the document as a
 * result of some action (e.g. when receiving a sync message you may want to
 * update the UI). In these cases there are variants of the relevant methods
 * with a "ForPatches" suffix on the method name which return a list of
 * {@link Patch}es describing the changes which occurred.
 *
 * <h2>Memory Management</h2>
 *
 * Each document actually points to some memory which is not managed by the JVM.
 * This means that you will need to manually call {@link free} when you are done
 * with the document.
 *
 * <h2>Actor IDs</h2>
 *
 * Every automerge document has an associated actor ID. This is a unique
 * identifier which must only be used concurrently. Creating a new document will
 * create a new random actor ID, as will forking the document but you can also
 * pass in an actor ID if you need to. It's fine to create a new actor ID for
 * every interaction with a document, but this will introduce a small amount of
 * overhead in storage as every actor ID has to be stored forever so if you
 * think you'll be storing a document for a long time or interacting with it
 * many times it may be worth reusing actor IDs.
 */
public class Document implements Read {
	private Optional<DocPointer> pointer;
	// Keep actor ID here so we a) don't have to keep passing it across the JNI
	// boundary and b) can access it when a transaction is in progress
	private byte[] actorId;
	// If a transaction is in progress we must forward all calls to the transaction.
	// In rust code the transaction holds a mutable reference to the document, so
	// any
	// calls to the document whilst the transaction exists would be undefined
	// behaviour.
	private Optional<AutomergeSys.TransactionPointer> transactionPtr;

	/** Create a new document with a random actor ID */
	public Document() {
		LoadLibrary.initialize();
		this.pointer = Optional.of(AutomergeSys.createDoc());
		this.actorId = AutomergeSys.getActorId(this.pointer.get());
		this.transactionPtr = Optional.empty();
	}

	/**
	 * Create a new document with a specific actor ID
	 *
	 * @param actorId
	 *            the actor ID to use for this document
	 */
	public Document(byte[] actorId) {
		LoadLibrary.initialize();
		this.actorId = actorId;
		this.pointer = Optional.of(AutomergeSys.createDocWithActor(actorId));
		this.transactionPtr = Optional.empty();
	}

	private Document(DocPointer pointer) {
		LoadLibrary.initialize();
		this.pointer = Optional.of(pointer);
		this.actorId = AutomergeSys.getActorId(this.pointer.get());
		this.transactionPtr = Optional.empty();
	}

	/**
	 * Get the actor ID for this document
	 *
	 * @return the actor ID for this document
	 */
	public byte[] getActorId() {
		return this.actorId;
	}

	/**
	 * Free the memory associated with this document
	 *
	 * <p>
	 * Once this method has been called any further attempts to interact with the
	 * document will raise an exception. This call itself is idempotent though, so
	 * it's safe to call mutliple times.
	 */
	public synchronized void free() {
		if (this.transactionPtr.isPresent()) {
			throw new TransactionInProgress();
		}
		if (this.pointer.isPresent()) {
			AutomergeSys.freeDoc(this.pointer.get());
			this.pointer = Optional.empty();
		}
	}

	/**
	 * Load a document from disk
	 *
	 * <p>
	 * This can be used to load bytes produced by {@link save} or
	 * {@link encodeChangesSince}
	 *
	 * @param bytes
	 *            The bytes of the document to load
	 * @return The loaded document
	 */
	public static Document load(byte[] bytes) {
		LoadLibrary.initialize();
		return new Document(AutomergeSys.loadDoc(bytes));
	}

	/**
	 * Save a document
	 *
	 * <p>
	 * The saved document can be loaded again with {@link load}
	 *
	 * @return The bytes of the saved document
	 */
	public synchronized byte[] save() {
		if (this.transactionPtr.isPresent()) {
			throw new TransactionInProgress();
		}
		return AutomergeSys.saveDoc(this.pointer.get());
	}

	/**
	 * Create a copy of this document with a new random actor ID
	 *
	 * @return The new document
	 */
	public synchronized Document fork() {
		if (this.transactionPtr.isPresent()) {
			throw new TransactionInProgress();
		}
		return new Document(AutomergeSys.forkDoc(this.pointer.get()));
	}

	/**
	 * Create a copy of this document with the given actor ID
	 *
	 * @param newActor
	 *            The actor ID to use for the new document
	 * @return The new document
	 */
	public synchronized Document fork(byte[] newActor) {
		if (this.transactionPtr.isPresent()) {
			throw new TransactionInProgress();
		}
		return new Document(AutomergeSys.forkDocWithActor(this.pointer.get(), newActor));
	}

	/**
	 * Create a copy of this document as at the given heads
	 *
	 * @param heads
	 *            The heads to fork the document at
	 * @return The new document
	 */
	public synchronized Document fork(ChangeHash[] heads) {
		if (this.transactionPtr.isPresent()) {
			throw new TransactionInProgress();
		}
		return new Document(AutomergeSys.forkDocAt(this.pointer.get(), heads));
	}

	/**
	 * Create a copy of this document as at the given heads with the given actor ID
	 *
	 * @param heads
	 *            The heads to fork the document at
	 * @param newActor
	 *            The actor ID to use for the new document
	 * @return The new document
	 */
	public synchronized Document fork(ChangeHash[] heads, byte[] newActor) {
		if (this.transactionPtr.isPresent()) {
			throw new TransactionInProgress();
		}
		return new Document(AutomergeSys.forkDocAtWithActor(this.pointer.get(), heads, newActor));
	}

	/**
	 * Merge another document into this one
	 *
	 * @param other
	 *            The document to merge into this one
	 * @throws TransactionInProgress
	 *             if there is a transaction in progress on this document or on the
	 *             other document
	 */
	public synchronized void merge(Document other) {
		if (this.transactionPtr.isPresent() || other.transactionPtr.isPresent()) {
			throw new TransactionInProgress();
		}
		AutomergeSys.mergeDoc(this.pointer.get(), other.pointer.get());
	}

	/**
	 * Merge another document into this one returning patches
	 *
	 * @param other
	 *            The document to merge into this one
	 * @return The patches representing how this document changed as a result of the
	 *         merge
	 * @throws TransactionInProgress
	 *             if there is a transaction in progress on this document or on the
	 *             other document
	 */
	public synchronized ArrayList<Patch> mergeForPatches(Document other) {
		if (this.transactionPtr.isPresent() || other.transactionPtr.isPresent()) {
			throw new TransactionInProgress();
		}
		return AutomergeSys.mergeDocObserved(this.pointer.get(), other.pointer.get());
	}

	/**
	 * Encode changes since the given heads
	 *
	 * <p>
	 * The encoded changes this method returns can be used in
	 * {@link applyEncodedChanges}
	 *
	 * @param heads
	 *            The heads to encode changes since
	 * @return The encoded changes
	 */
	public synchronized byte[] encodeChangesSince(ChangeHash[] heads) {
		if (this.transactionPtr.isPresent()) {
			throw new TransactionInProgress();
		}
		return AutomergeSys.encodeChangesSince(this.pointer.get(), heads);
	}

	/**
	 * Incorporate changes from another document into this document
	 *
	 * @param changes
	 *            The changes to incorporate. Produced by {@link encodeChangesSince}
	 *            or {@link save}
	 * @throws TransactionInProgress
	 *             if a transaction is in progress
	 * @throws AutomergeException
	 *             if the changes are not valid
	 */
	public synchronized void applyEncodedChanges(byte[] changes) {
		if (this.transactionPtr.isPresent()) {
			throw new TransactionInProgress();
		}
		AutomergeSys.applyEncodedChanges(this.pointer.get(), changes);
	}

	/**
	 * The same as {@link applyEncodedChanges} but returns some patches that
	 * describe any change that occurred
	 *
	 * <p>
	 * Creating patches does imply a performance penalty, so if you don't need them
	 * you should use {@link applyEncodedChanges}
	 *
	 * @param changes
	 *            The changes to incorporate. Produced by {@link encodeChangesSince}
	 *            or {@link save}
	 * @return The patches that describe the changes that occurred when merging the
	 *         changes
	 * @throws TransactionInProgress
	 *             if a transaction is in progress
	 * @throws AutomergeException
	 *             if the changes are not valid
	 */
	public synchronized ArrayList<Patch> applyEncodedChangesForPatches(byte[] changes) {
		if (this.transactionPtr.isPresent()) {
			throw new TransactionInProgress();
		}
		return AutomergeSys.applyEncodedChangesObserved(this.pointer.get(), changes);
	}

	public synchronized Optional<AmValue> get(ObjectId obj, String key) {
		if (this.transactionPtr.isPresent()) {
			return AutomergeSys.getInMapInTx(this.transactionPtr.get(), obj, key);
		} else {
			return AutomergeSys.getInMapInDoc(this.pointer.get(), obj, key);
		}
	}

	public synchronized Optional<AmValue> get(ObjectId obj, int key) {
		if (this.transactionPtr.isPresent()) {
			return AutomergeSys.getInListInTx(this.transactionPtr.get(), obj, key);
		} else {
			return AutomergeSys.getInListInDoc(this.pointer.get(), obj, key);
		}
	}

	public Optional<AmValue> get(ObjectId obj, String key, ChangeHash[] heads) {
		if (this.transactionPtr.isPresent()) {
			return AutomergeSys.getAtInMapInTx(this.transactionPtr.get(), obj, key, heads);
		} else {
			return AutomergeSys.getAtInMapInDoc(this.pointer.get(), obj, key, heads);
		}
	}

	public Optional<AmValue> get(ObjectId obj, int idx, ChangeHash[] heads) {
		if (this.transactionPtr.isPresent()) {
			return AutomergeSys.getAtInListInTx(this.transactionPtr.get(), obj, idx, heads);
		} else {
			return AutomergeSys.getAtInListInDoc(this.pointer.get(), obj, idx, heads);
		}
	}

	public Optional<Conflicts> getAll(ObjectId obj, String key) {
		if (this.transactionPtr.isPresent()) {
			return AutomergeSys.getAllInMapInTx(this.transactionPtr.get(), obj, key);
		} else {
			return AutomergeSys.getAllInMapInDoc(this.pointer.get(), obj, key);
		}
	}

	public Optional<Conflicts> getAll(ObjectId obj, int idx) {
		if (this.transactionPtr.isPresent()) {
			return AutomergeSys.getAllInListInTx(this.transactionPtr.get(), obj, idx);
		} else {
			return AutomergeSys.getAllInListInDoc(this.pointer.get(), obj, idx);
		}
	}

	public Optional<Conflicts> getAll(ObjectId obj, String key, ChangeHash[] heads) {
		if (this.transactionPtr.isPresent()) {
			return AutomergeSys.getAllAtInMapInTx(this.transactionPtr.get(), obj, key, heads);
		} else {
			return AutomergeSys.getAllAtInMapInDoc(this.pointer.get(), obj, key, heads);
		}
	}

	public Optional<Conflicts> getAll(ObjectId obj, int idx, ChangeHash[] heads) {
		if (this.transactionPtr.isPresent()) {
			return AutomergeSys.getAllAtInListInTx(this.transactionPtr.get(), obj, idx, heads);
		} else {
			return AutomergeSys.getAllAtInListInDoc(this.pointer.get(), obj, idx, heads);
		}
	}

	public synchronized Optional<String> text(ObjectId obj) {
		if (this.transactionPtr.isPresent()) {
			return AutomergeSys.getTextInTx(this.transactionPtr.get(), obj);
		} else {
			return AutomergeSys.getTextInDoc(this.pointer.get(), obj);
		}
	}

	public synchronized Optional<String> text(ObjectId obj, ChangeHash[] heads) {
		if (this.transactionPtr.isPresent()) {
			return AutomergeSys.getTextAtInTx(this.transactionPtr.get(), obj, heads);
		} else {
			return AutomergeSys.getTextAtInDoc(this.pointer.get(), obj, heads);
		}
	}

	public synchronized Optional<String[]> keys(ObjectId obj) {
		if (this.transactionPtr.isPresent()) {
			return AutomergeSys.getKeysInTx(this.transactionPtr.get(), obj);
		} else {
			return AutomergeSys.getKeysInDoc(this.pointer.get(), obj);
		}
	}

	public synchronized Optional<String[]> keys(ObjectId obj, ChangeHash[] heads) {
		if (this.transactionPtr.isPresent()) {
			return AutomergeSys.getKeysAtInTx(this.transactionPtr.get(), obj, heads);
		} else {
			return AutomergeSys.getKeysAtInDoc(this.pointer.get(), obj, heads);
		}
	}

	public synchronized Optional<MapEntry[]> mapEntries(ObjectId obj) {
		if (this.transactionPtr.isPresent()) {
			return AutomergeSys.getMapEntriesInTx(this.transactionPtr.get(), obj);
		} else {
			return AutomergeSys.getMapEntriesInDoc(this.pointer.get(), obj);
		}
	}

	public synchronized Optional<MapEntry[]> mapEntries(ObjectId obj, ChangeHash[] heads) {
		if (this.transactionPtr.isPresent()) {
			return AutomergeSys.getMapEntriesAtInTx(this.transactionPtr.get(), obj, heads);
		} else {
			return AutomergeSys.getMapEntriesAtInDoc(this.pointer.get(), obj, heads);
		}
	}

	public synchronized long length(ObjectId obj) {
		if (this.transactionPtr.isPresent()) {
			return AutomergeSys.getListLengthInTx(this.transactionPtr.get(), obj);
		} else {
			return AutomergeSys.getListLengthInDoc(this.pointer.get(), obj);
		}
	}

	public synchronized long length(ObjectId obj, ChangeHash[] heads) {
		if (this.transactionPtr.isPresent()) {
			return AutomergeSys.getListLengthAtInTx(this.transactionPtr.get(), obj, heads);
		} else {
			return AutomergeSys.getListLengthAtInDoc(this.pointer.get(), obj, heads);
		}
	}

	/**
	 * Start a transaction to change this document
	 *
	 * <p>
	 * There can only be one active transaction per document. Any method which
	 * mutates the document (e.g. {@link merge} or {@link receiveSyncMessage} will
	 * throw an exception if a transaction is in progress. Therefore keep
	 * transactions short lived.
	 *
	 * @return a new transaction
	 * @throws TransactionInProgress
	 *             if a transaction is already in progress
	 */
	public synchronized Transaction<ChangeHash> startTransaction() {
		if (this.transactionPtr.isPresent()) {
			throw new TransactionInProgress();
		}
		UnobservedTransactionPointer ptr = AutomergeSys.startTransaction(this.pointer.get());
		this.transactionPtr = Optional.of(ptr);
		return new TransactionWithoutPatches(this, ptr);
	}

	/**
	 * Start a transaction to change this document which returns patches
	 *
	 * <p>
	 * This returns a transaction which will return a {@link HashAndPatches} from
	 * it's {@link Transaction#commit} method. The returned patches represent the
	 * changes made during the transaction.
	 *
	 * <p>
	 * There can only be one active transaction per document. Any method which
	 * mutates the document (e.g. {@link merge} or {@link receiveSyncMessage} will
	 * throw an exception if a transaction is in progress. Therefore keep
	 * transactions short lived.
	 *
	 * @return a new transaction
	 * @throws TransactionInProgress
	 *             if a transaction is already in progress
	 */
	public synchronized Transaction<HashAndPatches> startTransactionForPatches() {
		if (this.transactionPtr.isPresent()) {
			throw new TransactionInProgress();
		}
		AutomergeSys.ObservedTransactionPointer ptr = AutomergeSys.startObservedTransaction(this.pointer.get());
		this.transactionPtr = Optional.of(ptr);
		return new TransactionWithPatches(this, ptr);
	}

	public synchronized Optional<AmValue[]> listItems(ObjectId obj) {
		if (this.transactionPtr.isPresent()) {
			return AutomergeSys.getListItemsInTx(this.transactionPtr.get(), obj);
		} else {
			return AutomergeSys.getListItemsInDoc(this.pointer.get(), obj);
		}
	}

	public synchronized Optional<AmValue[]> listItems(ObjectId obj, ChangeHash[] heads) {
		if (this.transactionPtr.isPresent()) {
			return AutomergeSys.getListItemsAtInTx(this.transactionPtr.get(), obj, heads);
		} else {
			return AutomergeSys.getListItemsAtInDoc(this.pointer.get(), obj, heads);
		}
	}

	public synchronized ChangeHash[] getHeads() {
		if (this.transactionPtr.isPresent()) {
			return AutomergeSys.getHeadsInTx(this.transactionPtr.get());
		} else {
			return AutomergeSys.getHeadsInDoc(this.pointer.get());
		}
	}

	public synchronized List<Mark> marks(ObjectId obj) {
		if (this.transactionPtr.isPresent()) {
			return AutomergeSys.getMarksInTx(this.transactionPtr.get(), obj, Optional.empty());
		} else {
			return AutomergeSys.getMarksInDoc(this.pointer.get(), obj, Optional.empty());
		}
	}

	public synchronized List<Mark> marks(ObjectId obj, ChangeHash[] heads) {
		if (this.transactionPtr.isPresent()) {
			return AutomergeSys.getMarksInTx(this.transactionPtr.get(), obj, Optional.of(heads));
		} else {
			return AutomergeSys.getMarksInDoc(this.pointer.get(), obj, Optional.of(heads));
		}
	}

	protected synchronized void clearTransaction() {
		this.transactionPtr = Optional.empty();
	}

	protected synchronized Optional<byte[]> generateSyncMessage(AutomergeSys.SyncStatePointer syncState) {
		if (this.transactionPtr.isPresent()) {
			throw new TransactionInProgress();
		}
		return AutomergeSys.generateSyncMessage(syncState, this.pointer.get());
	}

	/**
	 * Generate a sync message
	 *
	 * @param syncState
	 *            the {@link SyncState} for the connection you are syncing with
	 * @return the sync message to send to the other side, or {@link Optional#empty}
	 *         if there is nothing to send
	 */
	public synchronized Optional<byte[]> generateSyncMessage(SyncState syncState) {
		return syncState.generateSyncMessage(this);
	}

	protected synchronized void receiveSyncMessage(AutomergeSys.SyncStatePointer syncState, byte[] message) {
		if (this.transactionPtr.isPresent()) {
			throw new TransactionInProgress();
		}
		AutomergeSys.receiveSyncMessage(syncState, this.pointer.get(), message);
	}

	/**
	 * Applies a sync message to the document.
	 *
	 * <p>
	 * If you need to know what changes happened as a result of the message use
	 * {@link receiveSyncMessageForPatches} instead.
	 *
	 * @param syncState
	 *            the {@link SyncState} for the connection you are syncing with
	 * @param message
	 *            The sync message to apply.
	 * @throws TransactionInProgress
	 *             if a transaction is already in progress
	 */
	public synchronized void receiveSyncMessage(SyncState syncState, byte[] message) {
		syncState.receiveSyncMessage(this, message);
	}

	/**
	 * Applies a sync message to the document.
	 *
	 * @param syncState
	 *            the {@link SyncState} for the connection you are syncing with
	 * @param message
	 *            The sync message to apply.
	 * @return A list of {@link Patch}es representing the changes that occurred as a
	 *         result of the message
	 * @throws TransactionInProgress
	 *             if a transaction is already in progress
	 */
	public synchronized List<Patch> receiveSyncMessageForPatches(SyncState syncState, byte[] message) {
		return syncState.receiveSyncMessageForPatches(this, message);
	}

	protected synchronized List<Patch> receiveSyncMessageForPatches(AutomergeSys.SyncStatePointer syncState,
			byte[] message) {
		return AutomergeSys.receiveSyncMessageForPatches(syncState, this.pointer.get(), message);
	}
}
