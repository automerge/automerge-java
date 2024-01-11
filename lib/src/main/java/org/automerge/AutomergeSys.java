package org.automerge;

import java.util.ArrayList;
import java.util.Date;
import java.util.HashMap;
import java.util.Iterator;
import java.util.List;
import java.util.Optional;

class AutomergeSys {
	protected class DocPointer {
		private long pointer;
	}

	protected class TransactionPointer {
		private long pointer;
	}

	protected class SyncStatePointer {
		private long pointer;
	}

	protected class PatchLogPointer {
		private long pointer;
	}

	// Get the version of the JNI libs
	public static native String rustLibVersion();

	// Document methods
	public static native DocPointer createDoc();

	public static native DocPointer createDocWithActor(byte[] actorId);

	public static native DocPointer loadDoc(byte[] bytes);

	public static native void freeDoc(DocPointer pointer);

	public static native byte[] saveDoc(DocPointer pointer);

	public static native DocPointer forkDoc(DocPointer pointer);

	public static native DocPointer forkDocWithActor(DocPointer pointer, byte[] actorId);

	public static native DocPointer forkDocAt(DocPointer pointer, ChangeHash[] heads);

	public static native DocPointer forkDocAtWithActor(DocPointer pointer, ChangeHash[] heads, byte[] actorId);

	public static native void mergeDoc(DocPointer pointer, DocPointer other);

	public static native void mergeDocLogPatches(DocPointer pointer, DocPointer other, PatchLogPointer patchLog);

	public static native byte[] getActorId(DocPointer pointer);

	public static native TransactionPointer startTransaction(DocPointer doc);

	public static native TransactionPointer startTransactionLogPatches(DocPointer doc, PatchLogPointer patchLog);

	public static native TransactionPointer startTransactionAt(DocPointer doc, PatchLogPointer patchLog,
			ChangeHash[] heads);

	public static native byte[] encodeChangesSince(DocPointer doc, ChangeHash[] heads);

	public static native void applyEncodedChanges(DocPointer doc, byte[] changes);

	public static native void applyEncodedChangesLogPatches(DocPointer doc, PatchLogPointer patchLog, byte[] changes);

	public static native ArrayList<Patch> makePatches(DocPointer doc, PatchLogPointer patchLog);

	// Read methods
	public static native Optional<AmValue> getInMapInDoc(DocPointer doc, ObjectId obj, String key);

	public static native Optional<AmValue> getInListInDoc(DocPointer doc, ObjectId obj, int index);

	public static native Optional<AmValue> getInMapInTx(TransactionPointer tx, ObjectId obj, String key);

	public static native Optional<AmValue> getInListInTx(TransactionPointer tx, ObjectId obj, int index);

	public static native Optional<AmValue> getAtInMapInDoc(DocPointer doc, ObjectId obj, String key,
			ChangeHash[] heads);

	public static native Optional<AmValue> getAtInListInDoc(DocPointer doc, ObjectId obj, int index,
			ChangeHash[] heads);

	public static native Optional<AmValue> getAtInMapInTx(TransactionPointer tx, ObjectId obj, String key,
			ChangeHash[] heads);

	public static native Optional<AmValue> getAtInListInTx(TransactionPointer tx, ObjectId obj, int index,
			ChangeHash[] heads);

	public static native Optional<Conflicts> getAllInMapInDoc(DocPointer doc, ObjectId obj, String key);

	public static native Optional<Conflicts> getAllInListInDoc(DocPointer doc, ObjectId obj, int idx);

	public static native Optional<Conflicts> getAllInMapInTx(TransactionPointer tx, ObjectId obj, String key);

	public static native Optional<Conflicts> getAllInListInTx(TransactionPointer tx, ObjectId obj, int idx);

	public static native Optional<Conflicts> getAllAtInMapInDoc(DocPointer doc, ObjectId obj, String key,
			ChangeHash[] heads);

	public static native Optional<Conflicts> getAllAtInListInDoc(DocPointer doc, ObjectId obj, int idx,
			ChangeHash[] heads);

	public static native Optional<Conflicts> getAllAtInMapInTx(TransactionPointer tx, ObjectId obj, String key,
			ChangeHash[] heads);

	public static native Optional<Conflicts> getAllAtInListInTx(TransactionPointer tx, ObjectId obj, int idx,
			ChangeHash[] heads);

	// Transaction mutation methods
	// Set in map
	public static native void setDoubleInMap(TransactionPointer tx, ObjectId obj, String key, double value);

	public static native void setBytesInMap(TransactionPointer tx, ObjectId obj, String key, byte[] value);

	public static native void setStringInMap(TransactionPointer tx, ObjectId obj, String key, String value);

	public static native void setIntInMap(TransactionPointer tx, ObjectId obj, String key, long value);

	public static native void setUintInMap(TransactionPointer tx, ObjectId obj, String key, long value);

	public static native void setBoolInMap(TransactionPointer tx, ObjectId obj, String key, boolean value);

	public static native void setCounterInMap(TransactionPointer tx, ObjectId obj, String key, long value);

	public static native void setDateInMap(TransactionPointer transactionPointer, ObjectId obj, String key, Date value);

	public static native void setNullInMap(TransactionPointer tx, ObjectId obj, String key);

	public static native ObjectId setObjectInMap(TransactionPointer tx, ObjectId obj, String key, ObjectType objType);

	// Set in list
	public static native void setDoubleInList(TransactionPointer tx, ObjectId obj, long idx, double value);

	public static native void setIntInList(TransactionPointer tx, ObjectId obj, long idx, long value);

	public static native void setUintInList(TransactionPointer tx, ObjectId obj, long idx, long value);

	public static native void setStringInList(TransactionPointer tx, ObjectId obj, long idx, String value);

	public static native void setBytesInList(TransactionPointer tx, ObjectId obj, long idx, byte[] value);

	public static native void setBoolInList(TransactionPointer tx, ObjectId obj, long idx, boolean value);

	public static native void setDateInList(TransactionPointer tx, ObjectId obj, long idx, Date value);

	public static native void setCounterInList(TransactionPointer tx, ObjectId obj, long idx, long value);

	public static native void setNullInList(TransactionPointer tx, ObjectId obj, long idx);

	public static native ObjectId setObjectInList(TransactionPointer tx, ObjectId obj, long idx, ObjectType objType);

	// Insert in list
	public static native void insertDoubleInList(TransactionPointer tx, ObjectId obj, long index, double value);

	public static native void insertStringInList(TransactionPointer tx, ObjectId obj, long index, String value);

	public static native void insertIntInList(TransactionPointer tx, ObjectId obj, long index, long value);

	public static native void insertBytesInList(TransactionPointer tx, ObjectId obj, long index, byte[] value);

	public static native void insertUintInList(TransactionPointer tx, ObjectId obj, long index, long value);

	public static native void insertNullInList(TransactionPointer tx, ObjectId obj, long index);

	public static native void insertCounterInList(TransactionPointer transactionPointer, ObjectId obj, long index,
			long value);

	public static native void insertDateInList(TransactionPointer transactionPointer, ObjectId obj, long index,
			Date value);

	public static native void insertBoolInList(TransactionPointer transactionPointer, ObjectId obj, long index,
			boolean value);

	public static native ObjectId insertObjectInList(TransactionPointer tx, ObjectId obj, long index,
			ObjectType objType);

	// Increment
	public static native void incrementInMap(TransactionPointer tx, ObjectId obj, String key, long value);

	public static native void incrementInList(TransactionPointer tx, ObjectId obj, long idx, long value);

	// Delete
	public static native void deleteInMap(TransactionPointer tx, ObjectId obj, String key);

	public static native void deleteInList(TransactionPointer tx, ObjectId obj, long idx);

	// Splice
	public static native void splice(TransactionPointer tx, ObjectId obj, long start, long deleteCount,
			Iterator<NewValue> values);

	// Text
	public static native void spliceText(TransactionPointer tx, ObjectId obj, long start, long deleteCount,
			String text);

	public static native Optional<String> getTextInDoc(DocPointer pointer, ObjectId obj);

	public static native Optional<String> getTextInTx(TransactionPointer pointer, ObjectId obj);

	public static native Optional<String> getTextAtInDoc(DocPointer pointer, ObjectId obj, ChangeHash[] heads);

	public static native Optional<String> getTextAtInTx(TransactionPointer pointer, ObjectId obj, ChangeHash[] heads);

	// Keys
	public static native Optional<String[]> getKeysInTx(TransactionPointer tx, ObjectId obj);

	public static native Optional<String[]> getKeysInDoc(DocPointer doc, ObjectId obj);

	public static native Optional<String[]> getKeysAtInTx(TransactionPointer tx, ObjectId obj, ChangeHash[] heads);

	public static native Optional<String[]> getKeysAtInDoc(DocPointer doc, ObjectId obj, ChangeHash[] heads);

	// Map entries
	public static native Optional<MapEntry[]> getMapEntriesInTx(TransactionPointer tx, ObjectId obj);

	public static native Optional<MapEntry[]> getMapEntriesInDoc(DocPointer doc, ObjectId obj);

	public static native Optional<MapEntry[]> getMapEntriesAtInTx(TransactionPointer tx, ObjectId obj,
			ChangeHash[] heads);

	public static native Optional<MapEntry[]> getMapEntriesAtInDoc(DocPointer doc, ObjectId obj, ChangeHash[] heads);

	// List items
	public static native Optional<AmValue[]> getListItemsInTx(TransactionPointer tx, ObjectId obj);

	public static native Optional<AmValue[]> getListItemsInDoc(DocPointer doc, ObjectId obj);

	public static native Optional<AmValue[]> getListItemsAtInTx(TransactionPointer tx, ObjectId obj,
			ChangeHash[] heads);

	public static native Optional<AmValue[]> getListItemsAtInDoc(DocPointer doc, ObjectId obj, ChangeHash[] heads);

	public static native long getListLengthInTx(TransactionPointer tx, ObjectId obj);

	public static native long getListLengthInDoc(DocPointer doc, ObjectId obj);

	public static native long getListLengthAtInTx(TransactionPointer tx, ObjectId obj, ChangeHash[] heads);

	public static native long getListLengthAtInDoc(DocPointer doc, ObjectId obj, ChangeHash[] heads);

	// Marks
	public static native List<Mark> getMarksInDoc(DocPointer doc, ObjectId obj, Optional<ChangeHash[]> heads);

	public static native List<Mark> getMarksInTx(TransactionPointer tx, ObjectId obj, Optional<ChangeHash[]> heads);

	public static native HashMap<String, AmValue> getMarksAtIndexInDoc(DocPointer doc, ObjectId obj, long index,
			Optional<ChangeHash[]> heads);

	public static native HashMap<String, AmValue> getMarksAtIndexInTx(TransactionPointer tx, ObjectId obj, long index,
			Optional<ChangeHash[]> heads);

	public static native void markUint(TransactionPointer tx, ObjectId obj, String name, long start, long end,
			long value, ExpandMark expand);

	public static native void markInt(TransactionPointer tx, ObjectId obj, String name, long start, long end,
			long value, ExpandMark expand);

	public static native void markDouble(TransactionPointer tx, ObjectId obj, String name, long start, long end,
			double value, ExpandMark expand);

	public static native void markBytes(TransactionPointer tx, ObjectId obj, String name, long start, long end,
			byte[] value, ExpandMark expand);

	public static native void markString(TransactionPointer tx, ObjectId obj, String name, long start, long end,
			String value, ExpandMark expand);

	public static native void markCounter(TransactionPointer tx, ObjectId obj, String name, long start, long end,
			long value, ExpandMark expand);

	public static native void markDate(TransactionPointer tx, ObjectId obj, String name, long start, long end,
			Date value, ExpandMark expand);

	public static native void markBool(TransactionPointer tx, ObjectId obj, String name, long start, long end,
			boolean value, ExpandMark expand);

	public static native void markNull(TransactionPointer tx, ObjectId obj, String name, long start, long end,
			ExpandMark expand);

	public static native void unMark(TransactionPointer tx, ObjectId obj, String name, long start, long end,
			ExpandMark expand);

	// Transactions
	public static native CommitResult commitTransaction(TransactionPointer tx);

	public static native void rollbackTransaction(TransactionPointer tx);

	// Heads
	public static native ChangeHash[] getHeadsInDoc(DocPointer doc);

	public static native ChangeHash[] getHeadsInTx(TransactionPointer tx);

	// Object ID methods
	public static native ObjectId rootObjectId();

	public static native boolean isRootObjectId(ObjectId obj);

	public static native String objectIdToString(ObjectId obj);

	public static native boolean objectIdsEqual(ObjectId left, ObjectId right);

	public static native int objectIdHash(ObjectId left);

	// Sync
	public static native SyncStatePointer createSyncState();

	public static native Optional<byte[]> generateSyncMessage(SyncStatePointer syncState, DocPointer doc);

	public static native void receiveSyncMessage(SyncStatePointer syncState, DocPointer doc, byte[] message);

	public static native void receiveSyncMessageLogPatches(SyncStatePointer syncState, DocPointer doc,
			PatchLogPointer patchLog, byte[] message);

	public static native SyncStatePointer decodeSyncState(byte[] encoded);

	public static native byte[] encodeSyncState(SyncStatePointer syncState);

	public static native void freeSyncState(SyncStatePointer syncState);

	public static native ChangeHash[] syncStateSharedHeads(SyncStatePointer syncState);

	public static native PatchLogPointer createPatchLog();

	public static native void freePatchLog(PatchLogPointer pointer);

	public static native ArrayList<Patch> diff(DocPointer doc, ChangeHash[] before, ChangeHash[] after);

	public static native Cursor makeCursorInDoc(DocPointer doc, ObjectId obj, long index, Optional<ChangeHash[]> heads);

	public static native Cursor makeCursorInTx(TransactionPointer tx, ObjectId obj, long index,
			Optional<ChangeHash[]> heads);

	public static native long lookupCursorIndexInDoc(DocPointer doc, ObjectId obj, Cursor cursor,
			Optional<ChangeHash[]> heads);

	public static native long lookupCursorIndexInTx(TransactionPointer tx, ObjectId obj, Cursor cursor,
			Optional<ChangeHash[]> heads);

	public static native Optional<ObjectType> getObjectTypeInDoc(DocPointer doc, ObjectId obj);

	public static native Optional<ObjectType> getObjectTypeInTx(TransactionPointer tx, ObjectId obj);
}
