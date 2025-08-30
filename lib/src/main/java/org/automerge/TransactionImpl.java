package org.automerge;

import java.util.Date;
import java.util.HashMap;
import java.util.Iterator;
import java.util.List;
import java.util.Optional;
import java.util.function.Consumer;

public class TransactionImpl implements Transaction {
    private Optional<AutomergeSys.TransactionPointer> pointer;
    private Document doc;
    private Optional<Consumer<AutomergeSys.PatchLogPointer>> finish;

    protected TransactionImpl(Document doc, AutomergeSys.TransactionPointer pointer) {
        this.pointer = Optional.of(pointer);
        this.doc = doc;
        this.finish = Optional.empty();
    }

    protected TransactionImpl(Document doc, AutomergeSys.TransactionPointer pointer,
            Consumer<AutomergeSys.PatchLogPointer> finish) {
        this.pointer = Optional.of(pointer);
        this.doc = doc;
        this.finish = Optional.of(finish);
    }

    public synchronized Optional<ChangeHash> commit() {
        CommitResult result = AutomergeSys.commitTransaction(pointer.get());
        this.pointer = Optional.empty();
        this.doc.clearTransaction();
        if (finish.isPresent()) {
            finish.get().accept(result.getPatchLog());
        }
        return result.getHash();
    }

    public synchronized void rollback() {
        AutomergeSys.rollbackTransaction(this.pointer.get());
        this.pointer = Optional.empty();
        this.doc.clearTransaction();
    }

    public synchronized Optional<AmValue> get(ObjectId obj, String key) {
        return AutomergeSys.getInMapInTx(this.pointer.get(), obj, key);
    }

    public synchronized Optional<AmValue> get(ObjectId obj, long key) {
        return AutomergeSys.getInListInTx(this.pointer.get(), obj, key);
    }

    public synchronized Optional<Conflicts> getAll(ObjectId obj, String key) {
        return AutomergeSys.getAllInMapInTx(this.pointer.get(), obj, key);
    }

    public synchronized Optional<Conflicts> getAll(ObjectId obj, long idx) {
        return AutomergeSys.getAllInListInTx(this.pointer.get(), obj, idx);
    }

    public synchronized Optional<Conflicts> getAll(ObjectId obj, String key, ChangeHash[] heads) {
        return AutomergeSys.getAllAtInMapInTx(this.pointer.get(), obj, key, heads);
    }

    public synchronized Optional<Conflicts> getAll(ObjectId obj, long idx, ChangeHash[] heads) {
        return AutomergeSys.getAllAtInListInTx(this.pointer.get(), obj, idx, heads);
    }

    public Optional<AmValue> get(ObjectId obj, String key, ChangeHash[] heads) {
        return AutomergeSys.getAtInMapInTx(this.pointer.get(), obj, key, heads);
    }

    public Optional<AmValue> get(ObjectId obj, long idx, ChangeHash[] heads) {
        return AutomergeSys.getAtInListInTx(this.pointer.get(), obj, idx, heads);
    }

    public void set(ObjectId obj, String key, String value) {
        AutomergeSys.setStringInMap(this.pointer.get(), obj, key, value);
    }

    public void set(ObjectId obj, long idx, String value) {
        AutomergeSys.setStringInList(this.pointer.get(), obj, idx, value);
    }

    public void set(ObjectId obj, String key, double value) {
        AutomergeSys.setDoubleInMap(this.pointer.get(), obj, key, value);
    }

    public void set(ObjectId obj, long idx, double value) {
        AutomergeSys.setDoubleInList(this.pointer.get(), obj, idx, value);
    }

    public void set(ObjectId obj, long idx, int value) {
        AutomergeSys.setIntInList(this.pointer.get(), obj, idx, value);
    }

    public void set(ObjectId obj, String key, int value) {
        AutomergeSys.setIntInMap(this.pointer.get(), obj, key, value);
    }

    public void set(ObjectId obj, String key, NewValue value) {
        value.set(this, obj, key);
    }

    public void set(ObjectId obj, long index, NewValue value) {
        value.set(this, obj, index);
    }

    public void setUint(ObjectId obj, String key, long value) {
        AutomergeSys.setUintInMap(this.pointer.get(), obj, key, value);
    }

    public void setUint(ObjectId obj, long idx, long value) {
        AutomergeSys.setUintInList(this.pointer.get(), obj, idx, value);
    }

    public void set(ObjectId obj, String key, byte[] value) {
        AutomergeSys.setBytesInMap(this.pointer.get(), obj, key, value);
    }

    public void set(ObjectId obj, long idx, byte[] value) {
        AutomergeSys.setBytesInList(this.pointer.get(), obj, idx, value);
    }

    public void set(ObjectId obj, String key, boolean value) {
        AutomergeSys.setBoolInMap(this.pointer.get(), obj, key, value);
    }

    public void set(ObjectId obj, long idx, boolean value) {
        AutomergeSys.setBoolInList(this.pointer.get(), obj, idx, value);
    }

    public void set(ObjectId obj, String key, Counter value) {
        AutomergeSys.setCounterInMap(this.pointer.get(), obj, key, value.getValue());
    }

    public void set(ObjectId obj, long idx, Counter value) {
        AutomergeSys.setCounterInList(this.pointer.get(), obj, idx, value.getValue());
    }

    public void set(ObjectId obj, String key, Date value) {
        AutomergeSys.setDateInMap(this.pointer.get(), obj, key, value);
    }

    public void set(ObjectId obj, long idx, Date value) {
        AutomergeSys.setDateInList(this.pointer.get(), obj, idx, value);
    }

    public ObjectId set(ObjectId parent, String key, ObjectType objType) {
        return AutomergeSys.setObjectInMap(this.pointer.get(), parent, key, objType);
    }

    public ObjectId set(ObjectId parent, long idx, ObjectType objType) {
        return AutomergeSys.setObjectInList(this.pointer.get(), parent, idx, objType);
    }

    public void setNull(ObjectId obj, String key) {
        AutomergeSys.setNullInMap(this.pointer.get(), obj, key);
    }

    public void setNull(ObjectId obj, long idx) {
        AutomergeSys.setNullInList(this.pointer.get(), obj, idx);
    }

    public void insert(ObjectId obj, long index, double value) {
        AutomergeSys.insertDoubleInList(this.pointer.get(), obj, index, value);
    }

    public void insert(ObjectId obj, long index, String value) {
        AutomergeSys.insertStringInList(this.pointer.get(), obj, index, value);
    }

    public void insert(ObjectId obj, long index, int value) {
        AutomergeSys.insertIntInList(this.pointer.get(), obj, index, value);
    }

    public void insert(ObjectId obj, long index, byte[] value) {
        AutomergeSys.insertBytesInList(this.pointer.get(), obj, index, value);
    }

    public void insert(ObjectId obj, long index, Counter value) {
        AutomergeSys.insertCounterInList(this.pointer.get(), obj, index, value.getValue());
    }

    public void insert(ObjectId obj, long index, Date value) {
        AutomergeSys.insertDateInList(this.pointer.get(), obj, index, value);
    }

    public void insert(ObjectId obj, long index, boolean value) {
        AutomergeSys.insertBoolInList(this.pointer.get(), obj, index, value);
    }

    public void insertNull(ObjectId obj, long index) {
        AutomergeSys.insertNullInList(this.pointer.get(), obj, index);
    }

    public void insertUint(ObjectId obj, long index, long value) {
        AutomergeSys.insertUintInList(this.pointer.get(), obj, index, value);
    }

    public ObjectId insert(ObjectId parent, long index, ObjectType objType) {
        return AutomergeSys.insertObjectInList(this.pointer.get(), parent, index, objType);
    }

    public void insert(ObjectId obj, long index, NewValue value) {
        value.insert(this, obj, index);
    }

    public void increment(ObjectId obj, String key, long amount) {
        AutomergeSys.incrementInMap(this.pointer.get(), obj, key, amount);
    }

    public void increment(ObjectId obj, long idx, long amount) {
        AutomergeSys.incrementInList(this.pointer.get(), obj, idx, amount);
    }

    public void delete(ObjectId obj, String key) {
        AutomergeSys.deleteInMap(this.pointer.get(), obj, key);
    }

    public void delete(ObjectId obj, long idx) {
        AutomergeSys.deleteInList(this.pointer.get(), obj, idx);
    }

    public void splice(ObjectId obj, long start, long deleteCount, Iterator<NewValue> items) {
        AutomergeSys.splice(this.pointer.get(), obj, start, deleteCount, items);
    }

    public void spliceText(ObjectId obj, long start, long deleteCount, String text) {
        AutomergeSys.spliceText(this.pointer.get(), obj, start, deleteCount, text);
    }

    public synchronized Optional<String> text(ObjectId obj) {
        return AutomergeSys.getTextInTx(this.pointer.get(), obj);
    }

    public synchronized Optional<String> text(ObjectId obj, ChangeHash[] heads) {
        return AutomergeSys.getTextAtInTx(this.pointer.get(), obj, heads);
    }

    public synchronized Optional<String[]> keys(ObjectId obj) {
        return AutomergeSys.getKeysInTx(this.pointer.get(), obj);
    }

    public synchronized Optional<String[]> keys(ObjectId obj, ChangeHash[] heads) {
        return AutomergeSys.getKeysAtInTx(this.pointer.get(), obj, heads);
    }

    public synchronized Optional<MapEntry[]> mapEntries(ObjectId obj) {
        return AutomergeSys.getMapEntriesInTx(this.pointer.get(), obj);
    }

    public synchronized Optional<MapEntry[]> mapEntries(ObjectId obj, ChangeHash[] heads) {
        return AutomergeSys.getMapEntriesAtInTx(this.pointer.get(), obj, heads);
    }

    public synchronized Optional<AmValue[]> listItems(ObjectId obj) {
        return AutomergeSys.getListItemsInTx(this.pointer.get(), obj);
    }

    public synchronized Optional<AmValue[]> listItems(ObjectId obj, ChangeHash[] heads) {
        return AutomergeSys.getListItemsAtInTx(this.pointer.get(), obj, heads);
    }

    public synchronized long length(ObjectId obj) {
        return AutomergeSys.getListLengthInTx(this.pointer.get(), obj);
    }

    public synchronized long length(ObjectId obj, ChangeHash[] heads) {
        return AutomergeSys.getListLengthAtInTx(this.pointer.get(), obj, heads);
    }

    public synchronized ChangeHash[] getHeads() {
        return AutomergeSys.getHeadsInTx(this.pointer.get());
    }

    @Override
    public synchronized void mark(ObjectId obj, long start, long end, String markName, NewValue value,
            ExpandMark expand) {
        value.mark(this, obj, start, end, markName, expand);
    }

    @Override
    public synchronized void mark(ObjectId obj, long start, long end, String markName, String value,
            ExpandMark expand) {
        AutomergeSys.markString(this.pointer.get(), obj, markName, start, end, value, expand);
    }

    @Override
    public synchronized void mark(ObjectId obj, long start, long end, String markName, long value, ExpandMark expand) {
        AutomergeSys.markInt(this.pointer.get(), obj, markName, start, end, value, expand);
    }

    @Override
    public synchronized void markUint(ObjectId obj, long start, long end, String markName, long value,
            ExpandMark expand) {
        AutomergeSys.markUint(this.pointer.get(), obj, markName, start, end, value, expand);
    }

    @Override
    public synchronized void mark(ObjectId obj, long start, long end, String markName, double value,
            ExpandMark expand) {
        AutomergeSys.markDouble(this.pointer.get(), obj, markName, start, end, value, expand);
    }

    @Override
    public synchronized void mark(ObjectId obj, long start, long end, String markName, byte[] value,
            ExpandMark expand) {
        AutomergeSys.markBytes(this.pointer.get(), obj, markName, start, end, value, expand);
    }

    @Override
    public synchronized void mark(ObjectId obj, long start, long end, String markName, Counter value,
            ExpandMark expand) {
        AutomergeSys.markCounter(this.pointer.get(), obj, markName, start, end, value.getValue(), expand);
    }

    @Override
    public synchronized void mark(ObjectId obj, long start, long end, String markName, Date value, ExpandMark expand) {
        AutomergeSys.markDate(this.pointer.get(), obj, markName, start, end, value, expand);
    }

    @Override
    public synchronized void mark(ObjectId obj, long start, long end, String markName, boolean value,
            ExpandMark expand) {
        AutomergeSys.markBool(this.pointer.get(), obj, markName, start, end, value, expand);
    }

    @Override
    public synchronized void markNull(ObjectId obj, long start, long end, String markName, ExpandMark expand) {
        AutomergeSys.markNull(this.pointer.get(), obj, markName, start, end, expand);
    }

    public synchronized void unmark(ObjectId obj, String markName, long start, long end, ExpandMark expand) {
        AutomergeSys.unMark(this.pointer.get(), obj, markName, start, end, expand);
    }

    public synchronized List<Mark> marks(ObjectId obj) {
        return AutomergeSys.getMarksInTx(this.pointer.get(), obj, Optional.empty());
    }

    public synchronized List<Mark> marks(ObjectId obj, ChangeHash[] heads) {
        return AutomergeSys.getMarksInTx(this.pointer.get(), obj, Optional.of(heads));
    }

    public synchronized void close() {
        if (this.pointer.isPresent()) {
            this.rollback();
        }
    }

    @Override
    public synchronized HashMap<String, AmValue> getMarksAtIndex(ObjectId obj, long index) {
        return AutomergeSys.getMarksAtIndexInTx(this.pointer.get(), obj, index, Optional.empty());
    }

    @Override
    public synchronized HashMap<String, AmValue> getMarksAtIndex(ObjectId obj, long index, ChangeHash[] heads) {
        return AutomergeSys.getMarksAtIndexInTx(this.pointer.get(), obj, index, Optional.of(heads));
    }

    @Override
    public synchronized Cursor makeCursor(ObjectId obj, long index) {
        return AutomergeSys.makeCursorInTx(this.pointer.get(), obj, index, Optional.empty());
    }

    @Override
    public synchronized Cursor makeCursor(ObjectId obj, long index, ChangeHash[] heads) {
        return AutomergeSys.makeCursorInTx(this.pointer.get(), obj, index, Optional.of(heads));
    }

    @Override
    public synchronized long lookupCursorIndex(ObjectId obj, Cursor cursor) {
        return AutomergeSys.lookupCursorIndexInTx(this.pointer.get(), obj, cursor, Optional.empty());
    }

    @Override
    public synchronized long lookupCursorIndex(ObjectId obj, Cursor cursor, ChangeHash[] heads) {
        return AutomergeSys.lookupCursorIndexInTx(this.pointer.get(), obj, cursor, Optional.of(heads));
    }

    @Override
    public synchronized Optional<ObjectType> getObjectType(ObjectId obj) {
        return AutomergeSys.getObjectTypeInTx(this.pointer.get(), obj);
    }

}
