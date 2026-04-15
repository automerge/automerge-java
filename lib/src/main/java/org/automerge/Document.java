package org.automerge;

import java.util.HashMap;
import java.util.List;
import java.util.Optional;
import java.util.function.Function;
import org.automerge.AutomergeSys.DocPointer;

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
 * transaction call {@link startTransaction}.Attempting to modify the document
 * whilst a transaction is in progress will throw a
 * {@link TransactionInProgress} exception. For this reason transactions should
 * probably be short lived and locally scoped.
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
    static {
        LoadLibrary.initialize();
    }

    private Optional<DocPointer> pointer;
    // Keep actor ID here so we a) don't have to keep passing it across the JNI
    // boundary and b) can access it when a transaction is in progress
    private byte[] actorId;
    // If a transaction is in progress we must forward all calls to the transaction.
    // In rust code the transaction holds a mutable reference to the document, so
    // any
    // calls to the document whilst the transaction exists would be unsafe
    private Optional<AutomergeSys.TransactionPointer> transactionPtr;

    /** Create a new document with a random actor ID */
    public Document() {
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
        this.actorId = actorId;
        this.pointer = Optional.of(AutomergeSys.createDocWithActor(actorId));
        this.transactionPtr = Optional.empty();
    }

    private Document(DocPointer pointer) {
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
        requireNoTransaction();
        if (this.pointer.isPresent()) {
            AutomergeSys.freeDoc(this.pointer.get());
            this.pointer = Optional.empty();
        }
    }

    /**
     * Drop this Document's reference to its native pointer without freeing
     * the underlying memory. Used by the {@code org.automerge.repo} layer
     * after a {@code DocumentActor.withDocument} callback returns and the
     * native side has already reclaimed the underlying {@code Automerge}.
     * Subsequent operations on this Document will fail.
     */
    synchronized void invalidate() {
        this.pointer = Optional.empty();
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
        requireNoTransaction();
        return AutomergeSys.saveDoc(this.pointer.get());
    }

    /**
     * Create a copy of this document with a new random actor ID
     *
     * @return The new document
     */
    public synchronized Document fork() {
        requireNoTransaction();
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
        requireNoTransaction();
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
        requireNoTransaction();
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
        requireNoTransaction();
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
        requireNoTransaction();
        other.requireNoTransaction();
        AutomergeSys.mergeDoc(this.pointer.get(), other.pointer.get());
    }

    /**
     * Merge another document into this one logging patches
     *
     * @param other
     *            The document to merge into this one
     * @param patchLog
     *            The patch log in which to record any changes to the current state
     *            which occur as a result of the merge
     * @throws TransactionInProgress
     *             if there is a transaction in progress on this document or on the
     *             other document
     */
    public synchronized void merge(Document other, PatchLog patchLog) {
        requireNoTransaction();
        other.requireNoTransaction();
        patchLog.with((pointer) -> {
            AutomergeSys.mergeDocLogPatches(this.pointer.get(), other.pointer.get(), pointer);
        });
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
        requireNoTransaction();
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
        requireNoTransaction();
        AutomergeSys.applyEncodedChanges(this.pointer.get(), changes);
    }

    /**
     * The same as {@link applyEncodedChanges} but logs any changes to the current
     * state that result from applying the change in the given patch log
     *
     * <p>
     * Creating patches does imply a performance penalty, so if you don't need them
     * you should use {@link applyEncodedChanges}
     *
     * @param changes
     *            The changes to incorporate. Produced by {@link encodeChangesSince}
     *            or {@link save}
     * @param patchLog
     *            The patch log in which to record any changes to the current state
     * @throws TransactionInProgress
     *             if a transaction is in progress
     * @throws AutomergeException
     *             if the changes are not valid
     */
    public synchronized void applyEncodedChanges(byte[] changes, PatchLog patchLog) {
        requireNoTransaction();
        patchLog.with((AutomergeSys.PatchLogPointer patchLogPointer) -> AutomergeSys
                .applyEncodedChangesLogPatches(this.pointer.get(), patchLogPointer, changes));
    }

    public synchronized Optional<AmValue> get(ObjectId obj, String key) {
        return read(
                tx -> AutomergeSys.getInMapInTx(tx, obj, key),
                doc -> AutomergeSys.getInMapInDoc(doc, obj, key));
    }

    public synchronized Optional<AmValue> get(ObjectId obj, long key) {
        return read(
                tx -> AutomergeSys.getInListInTx(tx, obj, key),
                doc -> AutomergeSys.getInListInDoc(doc, obj, key));
    }

    public synchronized Optional<AmValue> get(ObjectId obj, String key, ChangeHash[] heads) {
        return read(
                tx -> AutomergeSys.getAtInMapInTx(tx, obj, key, heads),
                doc -> AutomergeSys.getAtInMapInDoc(doc, obj, key, heads));
    }

    public synchronized Optional<AmValue> get(ObjectId obj, long idx, ChangeHash[] heads) {
        return read(
                tx -> AutomergeSys.getAtInListInTx(tx, obj, idx, heads),
                doc -> AutomergeSys.getAtInListInDoc(doc, obj, idx, heads));
    }

    public synchronized Optional<Conflicts> getAll(ObjectId obj, String key) {
        return read(
                tx -> AutomergeSys.getAllInMapInTx(tx, obj, key),
                doc -> AutomergeSys.getAllInMapInDoc(doc, obj, key));
    }

    public synchronized Optional<Conflicts> getAll(ObjectId obj, long idx) {
        return read(
                tx -> AutomergeSys.getAllInListInTx(tx, obj, idx),
                doc -> AutomergeSys.getAllInListInDoc(doc, obj, idx));
    }

    public synchronized Optional<Conflicts> getAll(ObjectId obj, String key, ChangeHash[] heads) {
        return read(
                tx -> AutomergeSys.getAllAtInMapInTx(tx, obj, key, heads),
                doc -> AutomergeSys.getAllAtInMapInDoc(doc, obj, key, heads));
    }

    public synchronized Optional<Conflicts> getAll(ObjectId obj, long idx, ChangeHash[] heads) {
        return read(
                tx -> AutomergeSys.getAllAtInListInTx(tx, obj, idx, heads),
                doc -> AutomergeSys.getAllAtInListInDoc(doc, obj, idx, heads));
    }

    public synchronized Optional<String> text(ObjectId obj) {
        return read(
                tx -> AutomergeSys.getTextInTx(tx, obj),
                doc -> AutomergeSys.getTextInDoc(doc, obj));
    }

    public synchronized Optional<String> text(ObjectId obj, ChangeHash[] heads) {
        return read(
                tx -> AutomergeSys.getTextAtInTx(tx, obj, heads),
                doc -> AutomergeSys.getTextAtInDoc(doc, obj, heads));
    }

    public synchronized Optional<String[]> keys(ObjectId obj) {
        return read(
                tx -> AutomergeSys.getKeysInTx(tx, obj),
                doc -> AutomergeSys.getKeysInDoc(doc, obj));
    }

    public synchronized Optional<String[]> keys(ObjectId obj, ChangeHash[] heads) {
        return read(
                tx -> AutomergeSys.getKeysAtInTx(tx, obj, heads),
                doc -> AutomergeSys.getKeysAtInDoc(doc, obj, heads));
    }

    public synchronized Optional<MapEntry[]> mapEntries(ObjectId obj) {
        return read(
                tx -> AutomergeSys.getMapEntriesInTx(tx, obj),
                doc -> AutomergeSys.getMapEntriesInDoc(doc, obj));
    }

    public synchronized Optional<MapEntry[]> mapEntries(ObjectId obj, ChangeHash[] heads) {
        return read(
                tx -> AutomergeSys.getMapEntriesAtInTx(tx, obj, heads),
                doc -> AutomergeSys.getMapEntriesAtInDoc(doc, obj, heads));
    }

    public synchronized long length(ObjectId obj) {
        return read(
                tx -> AutomergeSys.getListLengthInTx(tx, obj),
                doc -> AutomergeSys.getListLengthInDoc(doc, obj));
    }

    public synchronized long length(ObjectId obj, ChangeHash[] heads) {
        return read(
                tx -> AutomergeSys.getListLengthAtInTx(tx, obj, heads),
                doc -> AutomergeSys.getListLengthAtInDoc(doc, obj, heads));
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
    public synchronized Transaction startTransaction() {
        requireNoTransaction();
        AutomergeSys.TransactionPointer ptr = AutomergeSys.startTransaction(this.pointer.get());
        this.transactionPtr = Optional.of(ptr);
        return new TransactionImpl(this, ptr);
    }

    /**
     * Start a transaction to change this document which logs changes in a patch log
     *
     * <p>
     * There can only be one active transaction per document. Any method which
     * mutates the document (e.g. {@link merge} or {@link receiveSyncMessage} will
     * throw an exception if a transaction is in progress. Therefore keep
     * transactions short lived.
     *
     * @param patchLog
     *            the {@link PatchLog} to log changes to
     * @return a new transaction
     * @throws TransactionInProgress
     *             if a transaction is already in progress
     */
    public synchronized Transaction startTransaction(PatchLog patchLog) {
        requireNoTransaction();
        AutomergeSys.PatchLogPointer patchLogPointer = patchLog.take();
        AutomergeSys.TransactionPointer ptr = AutomergeSys.startTransactionLogPatches(this.pointer.get(),
                patchLogPointer);
        this.transactionPtr = Optional.of(ptr);
        return new TransactionImpl(this, ptr, (AutomergeSys.PatchLogPointer returnedPointer) -> {
            patchLog.put(returnedPointer);
        });
    }

    /**
     * Start a transaction to change this document based on the document at a given
     * heads
     *
     * <p>
     * There can only be one active transaction per document. Any method which
     * mutates the document (e.g. {@link merge} or {@link receiveSyncMessage} will
     * throw an exception if a transaction is in progress. Therefore keep
     * transactions short lived.
     *
     * @param patchLog
     *            the {@link PatchLog} to log changes to. Note that the the changes
     *            logged here will represent changes from the state as at the given
     *            heads, not the state of the document when calling this method.
     * @param heads
     *            the heads to begin the transaction at
     *
     * @return a new transaction
     * @throws TransactionInProgress
     *             if a transaction is already in progress
     */
    public synchronized Transaction startTransactionAt(PatchLog patchLog, ChangeHash[] heads) {
        requireNoTransaction();
        AutomergeSys.TransactionPointer ptr = AutomergeSys.startTransactionAt(this.pointer.get(), patchLog.take(),
                heads);
        return new TransactionImpl(this, ptr, (AutomergeSys.PatchLogPointer returnedPointer) -> {
            patchLog.put(returnedPointer);
        });
    }

    public synchronized Optional<AmValue[]> listItems(ObjectId obj) {
        return read(
                tx -> AutomergeSys.getListItemsInTx(tx, obj),
                doc -> AutomergeSys.getListItemsInDoc(doc, obj));
    }

    public synchronized Optional<AmValue[]> listItems(ObjectId obj, ChangeHash[] heads) {
        return read(
                tx -> AutomergeSys.getListItemsAtInTx(tx, obj, heads),
                doc -> AutomergeSys.getListItemsAtInDoc(doc, obj, heads));
    }

    public synchronized ChangeHash[] getHeads() {
        return read(
                tx -> AutomergeSys.getHeadsInTx(tx),
                doc -> AutomergeSys.getHeadsInDoc(doc));
    }

    public synchronized List<Mark> marks(ObjectId obj) {
        return read(
                tx -> AutomergeSys.getMarksInTx(tx, obj, Optional.empty()),
                doc -> AutomergeSys.getMarksInDoc(doc, obj, Optional.empty()));
    }

    public synchronized List<Mark> marks(ObjectId obj, ChangeHash[] heads) {
        return read(
                tx -> AutomergeSys.getMarksInTx(tx, obj, Optional.of(heads)),
                doc -> AutomergeSys.getMarksInDoc(doc, obj, Optional.of(heads)));
    }

    protected synchronized void clearTransaction() {
        this.transactionPtr = Optional.empty();
    }

    DocPointer getDocPointer() {
        return this.pointer.get();
    }

    /** Throw if a transaction is in progress. Used by mutating operations. */
    private void requireNoTransaction() {
        if (this.transactionPtr.isPresent()) {
            throw new TransactionInProgress();
        }
    }

    /** Route a read operation through the transaction (if active) or the document. */
    private <T> T read(
            Function<AutomergeSys.TransactionPointer, T> txFn,
            Function<DocPointer, T> docFn) {
        if (this.transactionPtr.isPresent()) {
            return txFn.apply(this.transactionPtr.get());
        } else {
            return docFn.apply(this.pointer.get());
        }
    }

    protected synchronized Optional<byte[]> generateSyncMessage(AutomergeSys.SyncStatePointer syncState) {
        requireNoTransaction();
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
        requireNoTransaction();
        AutomergeSys.receiveSyncMessage(syncState, this.pointer.get(), message);
    }

    /**
     * Applies a sync message to the document.
     *
     * <p>
     * If you need to know what changes happened as a result of the message use
     * {@link receiveSyncMessage(SyncState,PatchLog,byte[])} instead.
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
     * Applies a sync message to the document logging any changes in a PatchLog.
     *
     * @param syncState
     *            the {@link SyncState} for the connection you are syncing with
     * @param patchLog
     *            the {@link PatchLog} to log changes to
     * @param message
     *            The sync message to apply.
     * @throws TransactionInProgress
     *             if a transaction is already in progress
     */
    public synchronized void receiveSyncMessage(SyncState syncState, PatchLog patchLog, byte[] message) {
        syncState.receiveSyncMessageLogPatches(this, patchLog, message);
    }

    public synchronized List<Patch> makePatches(PatchLog patchLog) {
        requireNoTransaction();
        return patchLog.with((AutomergeSys.PatchLogPointer p) -> AutomergeSys.makePatches(this.pointer.get(), p));
    }

    protected synchronized void receiveSyncMessageLogPatches(AutomergeSys.SyncStatePointer syncState,
            AutomergeSys.PatchLogPointer patchLog, byte[] message) {
        AutomergeSys.receiveSyncMessageLogPatches(syncState, this.pointer.get(), patchLog, message);
    }

    /**
     * Return the patches that would be required to modify the state at `before` to
     * become the state at `after`
     *
     * @param before
     *            The heads of the statre to start from
     * @param after
     *            The heads of the state to end at
     * @return The patches required to transform the state at `before` to the state
     *         at `after`
     */
    public synchronized List<Patch> diff(ChangeHash[] before, ChangeHash[] after) {
        return AutomergeSys.diff(this.pointer.get(), before, after);
    }

    @Override
    public synchronized HashMap<String, AmValue> getMarksAtIndex(ObjectId obj, long index) {
        return read(
                tx -> AutomergeSys.getMarksAtIndexInTx(tx, obj, index, Optional.empty()),
                doc -> AutomergeSys.getMarksAtIndexInDoc(doc, obj, index, Optional.empty()));
    }

    @Override
    public synchronized HashMap<String, AmValue> getMarksAtIndex(ObjectId obj, long index, ChangeHash[] heads) {
        return read(
                tx -> AutomergeSys.getMarksAtIndexInTx(tx, obj, index, Optional.of(heads)),
                doc -> AutomergeSys.getMarksAtIndexInDoc(doc, obj, index, Optional.of(heads)));
    }

    @Override
    public synchronized Cursor makeCursor(ObjectId obj, long index) {
        return read(
                tx -> AutomergeSys.makeCursorInTx(tx, obj, index, Optional.empty()),
                doc -> AutomergeSys.makeCursorInDoc(doc, obj, index, Optional.empty()));
    }

    @Override
    public synchronized Cursor makeCursor(ObjectId obj, long index, ChangeHash[] heads) {
        return read(
                tx -> AutomergeSys.makeCursorInTx(tx, obj, index, Optional.of(heads)),
                doc -> AutomergeSys.makeCursorInDoc(doc, obj, index, Optional.of(heads)));
    }

    @Override
    public synchronized long lookupCursorIndex(ObjectId obj, Cursor cursor) {
        return read(
                tx -> AutomergeSys.lookupCursorIndexInTx(tx, obj, cursor, Optional.empty()),
                doc -> AutomergeSys.lookupCursorIndexInDoc(doc, obj, cursor, Optional.empty()));
    }

    @Override
    public synchronized long lookupCursorIndex(ObjectId obj, Cursor cursor, ChangeHash[] heads) {
        return read(
                tx -> AutomergeSys.lookupCursorIndexInTx(tx, obj, cursor, Optional.of(heads)),
                doc -> AutomergeSys.lookupCursorIndexInDoc(doc, obj, cursor, Optional.of(heads)));
    }

    @Override
    public synchronized Optional<ObjectType> getObjectType(ObjectId obj) {
        return read(
                tx -> AutomergeSys.getObjectTypeInTx(tx, obj),
                doc -> AutomergeSys.getObjectTypeInDoc(doc, obj));
    }
}
