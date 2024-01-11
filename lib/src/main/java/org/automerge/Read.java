package org.automerge;

import java.util.HashMap;
import java.util.List;
import java.util.Optional;

/** Methods to read from a document */
public interface Read {
	/**
	 * Get a value from the map given by obj
	 *
	 * <p>
	 * Note that if there are multiple conflicting values for the key this method
	 * will arbirtarily return one of them. The choice will be deterministic, in the
	 * sense that any other document with the same set of changes will return the
	 * same value. To get all the values use {@link getAll}
	 *
	 * @param obj
	 *            - The ID of the map to get the value from
	 * @param key
	 *            - The key to get the value for
	 * @return The value of the key or `Optional.empty` if not present
	 * @throws AutomergeException
	 *             if the object ID is not a map
	 */
	public Optional<AmValue> get(ObjectId obj, String key);

	/**
	 * Get a value from the map given by obj as at heads
	 *
	 * <p>
	 * Note that if there are multiple conflicting values for the key this method
	 * will arbirtarily return one of them. The choice will be deterministic, in the
	 * sense that any other document with the same set of changes will return the
	 * same value. To get all the values use {@link getAll}
	 *
	 * @param obj
	 *            - The ID of the map to get the value from
	 * @param key
	 *            - The key to get the value for
	 * @param heads
	 *            - The heads of the version of the document to get the value from
	 * @return The value of the key or `Optional.empty` if not present
	 * @throws AutomergeException
	 *             if the object ID is not a map
	 */
	public Optional<AmValue> get(ObjectId obj, String key, ChangeHash[] heads);

	/**
	 * Get a value from the list given by obj
	 *
	 * <p>
	 * Note that if there are multiple conflicting values for the index this method
	 * will arbirtarily return one of them. The choice will be deterministic, in the
	 * sense that any other document with the same set of changes will return the
	 * same value. To get all the values use {@link getAll}
	 *
	 * @param obj
	 *            - The ID of the list to get the value from
	 * @param idx
	 *            - The index to get the value for
	 * @return The value at the index or `Optional.empty` if the index is out of
	 *         range
	 * @throws AutomergeException
	 *             if the object ID is not a list
	 */
	public Optional<AmValue> get(ObjectId obj, int idx);

	/**
	 * Get a value from the list given by obj as at heads
	 *
	 * <p>
	 * Note that if there are multiple conflicting values for the index this method
	 * will arbirtarily return one of them. The choice will be deterministic, in the
	 * sense that any other document with the same set of changes will return the
	 * same value. To get all the values use {@link getAll}
	 *
	 * @param obj
	 *            - The ID of the list to get the value from
	 * @param idx
	 *            - The index to get the value for
	 * @param heads
	 *            - The heads of the version of the document to get the value from
	 * @return The value at the index or `Optional.empty` if the index is out of
	 *         range
	 * @throws AutomergeException
	 *             if the object ID is not a list
	 */
	public Optional<AmValue> get(ObjectId obj, int idx, ChangeHash[] heads);

	/**
	 * Get all the possibly conflicting values for a key from the map given by obj
	 *
	 * <p>
	 * If there are concurrent set operations to a key in a map there is no way to
	 * resolve that conflict so automerge retains all concurrently set values which
	 * can then be obtained via this method. If you don't care about conflicts and
	 * just want to arbitrarily (but deterministically) choose a value use
	 * {@link get}
	 *
	 * @param obj
	 *            - The ID of the map to get the value from
	 * @param key
	 *            - The key to get the value for
	 * @return The values
	 * @throws AutomergeException
	 *             if the object ID refers to an object which is not a map
	 */
	public Optional<Conflicts> getAll(ObjectId obj, String key);

	/**
	 * Get all the possibly conflicting values for a key from the map given by obj
	 * as at the given heads
	 *
	 * <p>
	 * If there are concurrent set operations to a key in a map there is no way to
	 * resolve that conflict so automerge retains all concurrently set values which
	 * can then be obtained via this method. If you don't care about conflicts and
	 * just want to arbitrarily (but deterministically) choose a value use
	 * {@link get}
	 *
	 * @param obj
	 *            - The ID of the map to get the value from
	 * @param key
	 *            - The key to get the value for
	 * @param heads
	 *            - The heads of the version of the document to get the value from
	 * @return The values
	 * @throws AutomergeException
	 *             if the object ID refers to an object which is not a map
	 */
	public Optional<Conflicts> getAll(ObjectId obj, String key, ChangeHash[] heads);

	/**
	 * Get all the possibly conflicting values for an index in the list given by obj
	 *
	 * <p>
	 * If there are concurrent set operations to an index in a list there is no way
	 * to resolve that conflict so automerge retains all concurrently set values
	 * which can then be obtained via this method. If you don't care about conflicts
	 * and just want to arbitrarily (but deterministically) choose a value use
	 * {@link get}
	 *
	 * @param obj
	 *            - The ID of the map to get the value from
	 * @param idx
	 *            - The index to get the value for
	 * @return The values
	 * @throws AutomergeException
	 *             if the object ID refers to an object which is not a map
	 */
	public Optional<Conflicts> getAll(ObjectId obj, int idx);

	/**
	 * Get all the possibly conflicting values for an index in the list given by obj
	 * as at the given heads
	 *
	 * <p>
	 * If there are concurrent set operations to an index in a list there is no way
	 * to resolve that conflict so automerge retains all concurrently set values
	 * which can then be obtained via this method. If you don't care about conflicts
	 * and just want to arbitrarily (but deterministically) choose a value use
	 * {@link get}
	 *
	 * @param obj
	 *            - The ID of the map to get the value from
	 * @param idx
	 *            - The index to get the value for
	 * @param heads
	 *            - The heads of the version of the document to get the value from
	 * @return The values
	 * @throws AutomergeException
	 *             if the object ID refers to an object which is not a map
	 */
	public Optional<Conflicts> getAll(ObjectId obj, int idx, ChangeHash[] heads);

	/**
	 * Get the value of a text object
	 *
	 * @param obj
	 *            - The ID of the text object to get the value from
	 * @return The text or None if no such object exists
	 * @throws AutomergeException
	 *             if the object ID refers to an object which is not a text object
	 */
	public Optional<String> text(ObjectId obj);

	/**
	 * Get the value of a text object as at the given heads
	 *
	 * @param obj
	 *            - The ID of the text object to get the value from
	 * @param heads
	 *            - The heads of the version of the document to get the value from
	 * @return The text or None if it does not exist
	 * @throws AutomergeException
	 *             if the object ID refers to an object which is not a text object
	 */
	public Optional<String> text(ObjectId obj, ChangeHash[] heads);

	/**
	 * Get the keys of the object given by obj
	 *
	 * @param obj
	 *            - The ID of the object to get the keys from
	 * @return The keys of the object or None if the object is not a map
	 */
	public Optional<String[]> keys(ObjectId obj);

	/**
	 * Get the keys of the object given by obj as at the given heads
	 *
	 * @param obj
	 *            - The ID of the object to get the keys from
	 * @param heads
	 *            - The heads of the version of the document to get the keys from
	 * @return The keys of the object or None if the object is not a map
	 */
	public Optional<String[]> keys(ObjectId obj, ChangeHash[] heads);

	/**
	 * Get the entries of the map given by obj
	 *
	 * @param obj
	 *            - The ID of the map to get the entries from
	 * @return The entries of the map or None if the object is not a map
	 */
	public Optional<MapEntry[]> mapEntries(ObjectId obj);

	/**
	 * Get the entries of the map given by obj as at the given heads
	 *
	 * @param obj
	 *            - The ID of the map to get the entries from
	 * @param heads
	 *            - The heads of the version of the document to get the entries from
	 * @return The entries of the map or None if the object is not a map
	 */
	public Optional<MapEntry[]> mapEntries(ObjectId obj, ChangeHash[] heads);

	/**
	 * Get the values in the list given by obj
	 *
	 * @param obj
	 *            - The ID of the list to get the values from
	 * @return The values of the list or None if the object is not a list
	 */
	public Optional<AmValue[]> listItems(ObjectId obj);

	/**
	 * Get the values in the list given by obj as at the given heads
	 *
	 * @param obj
	 *            - The ID of the list to get the values from
	 * @param heads
	 *            - The heads of the version of the document to get the values from
	 * @return The values of the list or None if the object is not a list
	 */
	public Optional<AmValue[]> listItems(ObjectId obj, ChangeHash[] heads);

	/**
	 * Get the length of the list given by obj
	 *
	 * @param obj
	 *            - The ID of the list to get the length of
	 * @return The length of the list (this will be zero if the object is not a
	 *         list)
	 */
	public long length(ObjectId obj);

	/**
	 * Get the length of the list given by obj as at the given heads
	 *
	 * @param obj
	 *            - The ID of the list to get the length of
	 * @param heads
	 *            - The heads of the version of the document to get the length from
	 * @return The length of the list (this will be zero if the object is not a
	 *         list)
	 */
	public long length(ObjectId obj, ChangeHash[] heads);

	/**
	 * Get the marks for the text object given by obj
	 *
	 * @param obj
	 *            - The ID of the text object to get the marks from
	 * @return The marks of the text object or None if the object is not a text
	 *         object
	 */
	public List<Mark> marks(ObjectId obj);

	/**
	 * Get the marks for the text object given by obj as at the given heads
	 *
	 * @param obj
	 *            - The ID of the text object to get the marks from
	 * @param heads
	 *            - The heads of the version of the document to get the marks from
	 * @return The marks of the text object or None if the object is not a text
	 *         object
	 */
	public List<Mark> marks(ObjectId obj, ChangeHash[] heads);

	/**
	 * Get the marks defined at the given index in a text object
	 *
	 * @param obj
	 *            - The ID of the text object to get the marks from
	 * @param index
	 *            - The index to get the marks at
	 * @return The marks at the given index or None if the object is not a text
	 *         object
	 */
	public HashMap<String, AmValue> getMarksAtIndex(ObjectId obj, int index);

	/**
	 * Get the marks defined at the given index in a text object
	 *
	 * @param obj
	 *            - The ID of the text object to get the marks from
	 * @param index
	 *            - The index to get the marks at
	 * @param heads
	 *            - The heads of the version of the document to get the marks from
	 * @return The marks at the given index or None if the object is not a text
	 *         object
	 */
	public HashMap<String, AmValue> getMarksAtIndex(ObjectId obj, int index, ChangeHash[] heads);

	/**
	 * Get the heads of the object
	 *
	 * @return The heads of the document
	 *         <p>
	 *         The returned heads represent the current version of the document and
	 *         can be passed to many other methods to refer to the document as at
	 *         this moment.
	 */
	public ChangeHash[] getHeads();

	/**
	 * Get a cursor which refers to the given index in a list or text object
	 *
	 * @param obj
	 *            - The ID of the list or text object to get the cursor for
	 * @param index
	 *            - The index to get the cursor for
	 *
	 * @return The cursor
	 *
	 * @throws AutomergeException
	 *             if the object ID refers to an object which is not a list or text
	 *             object or if the index is out of range
	 */
	public Cursor makeCursor(ObjectId obj, long index);

	/**
	 * Get a cursor which refers to the given index in a list or text object as at
	 * the given heads
	 *
	 * @param obj
	 *            - The ID of the list or text object to get the cursor for
	 * @param index
	 *            - The index to get the cursor for
	 * @param heads
	 *            - The heads of the version of the document to make the cursor from
	 *
	 * @return The cursor
	 *
	 * @throws AutomergeException
	 *             if the object ID refers to an object which is not a list or text
	 *             object or if the index is out of range
	 */
	public Cursor makeCursor(ObjectId obj, long index, ChangeHash[] heads);

	/**
	 * Given a cursor for an object, get the index the cursor points at
	 *
	 * @param obj
	 *            - The ID of the object the cursor refers into
	 * @param cursor
	 *            - The cursor
	 *
	 * @return The index the cursor points at
	 * @throws AutomergeException
	 *             if the object ID refers to an object which is not a list or text
	 *             object or if the cursor does not refer to an element in the
	 *             object
	 */
	public long lookupCursorIndex(ObjectId obj, Cursor cursor);

	/**
	 * Given a cursor for an object, get the index the cursor points at as at the
	 * given heads
	 *
	 * @param obj
	 *            - The ID of the object the cursor refers into
	 * @param cursor
	 *            - The cursor
	 * @param heads
	 *            - The heads of the version of the document to make the cursor from
	 *
	 * @return The index the cursor points at
	 * @throws AutomergeException
	 *             if the object ID refers to an object which is not a list or text
	 *             object or if the cursor does not refer to an element in the
	 *             object
	 */
	public long lookupCursorIndex(ObjectId obj, Cursor cursor, ChangeHash[] heads);

	/**
	 * Get the object type of the object given by obj
	 *
	 * @param obj
	 *            - The ID of the object to get the type of
	 *
	 * @return The type of the object or Optional.empty if the object does not exist
	 *         in this document
	 */
	public Optional<ObjectType> getObjectType(ObjectId obj);
}
