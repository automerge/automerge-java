package org.automerge;

import java.util.Date;
import java.util.Iterator;
import java.util.Optional;

/**
 * A mutable view of a {@link Document}
 *
 */
public interface Transaction extends Read, AutoCloseable {
    /**
     * Commit the transaction
     *
     * <p>
     * Once a transaction has been committed any attempt to use it will throw an
     * exception
     *
     * @return the result of the commit or {@link Optional#empty()} if the
     *         transaction made no changes
     */
    public Optional<ChangeHash> commit();

    /**
     * Close the transaction and reverse any changes made
     *
     * <p>
     * Once a transaction has been closed any attempt to use it will throw an
     * exception
     */
    public void rollback();

    /**
     * Set a string value in a map
     *
     * @param obj
     *            the object id of the map
     * @param key
     *            the key in the map
     * @param value
     *            the string to set
     * @throws AutomergeException
     *             if the object is not a map
     */
    public void set(ObjectId obj, String key, String value);

    /**
     * Set a string value in a list
     *
     * @param obj
     *            the object id of the list
     * @param index
     *            the index in the list
     * @param value
     *            the string to set
     * @throws AutomergeException
     *             if the object is not a list
     */
    public void set(ObjectId obj, long index, String value);

    /**
     * Set a double value in a map
     *
     * @param obj
     *            the object id of the map
     * @param key
     *            the key in the map
     * @param value
     *            the double to set
     * @throws AutomergeException
     *             if the object is not a map
     */
    public void set(ObjectId obj, String key, double value);

    /**
     * Set a double value in a list
     *
     * @param obj
     *            the object id of the list
     * @param index
     *            the index in the list to set
     * @param value
     *            the double to set
     * @throws AutomergeException
     *             if the object is not a list
     */
    public void set(ObjectId obj, long index, double value);

    /**
     * Set an int value in a list
     *
     * @param obj
     *            the object id of the list
     * @param index
     *            the index in the list to set
     * @param value
     *            the int to set
     * @throws AutomergeException
     *             if the object is not a list
     */
    public void set(ObjectId obj, long index, int value);

    /**
     * Set an int value in a map
     *
     * @param obj
     *            the object id of the map
     * @param key
     *            the key in the map
     * @param value
     *            the int to set
     * @throws AutomergeException
     *             if the object is not a map
     */
    public void set(ObjectId obj, String key, int value);

    /**
     * Set any non-object value in a map
     *
     * @param obj
     *            the object id of the map
     * @param key
     *            the key in the map
     * @param value
     *            the {@link NewValue} to set
     * @throws AutomergeException
     *             if the object is not a map
     */
    public void set(ObjectId obj, String key, NewValue value);

    /**
     * Set any non-object value in a list
     *
     * @param obj
     *            the object id of the list
     * @param index
     *            the index in the list
     * @param value
     *            the {@link NewValue} to set
     * @throws AutomergeException
     *             if the object is not a list
     */
    public void set(ObjectId obj, long index, NewValue value);

    /**
     * Set a byte array in a map
     *
     * @param obj
     *            the object id of the map
     * @param key
     *            the key in the map
     * @param value
     *            the bytes to set
     * @throws AutomergeException
     *             if the object is not a map
     */
    public void set(ObjectId obj, String key, byte[] value);

    /**
     * Set a byte array in a list
     *
     * @param obj
     *            the object id of the list
     * @param index
     *            the index in the list
     * @param value
     *            the bytes to set
     * @throws AutomergeException
     *             if the object is not a list
     */
    public void set(ObjectId obj, long index, byte[] value);

    /**
     * Set a boolean in a map
     *
     * @param obj
     *            the object id of the map
     * @param key
     *            the key in the map
     * @param value
     *            the boolean to set
     * @throws AutomergeException
     *             if the object is not a map
     */
    public void set(ObjectId obj, String key, boolean value);

    /**
     * Set a boolean in a list
     *
     * @param obj
     *            the object id of the list
     * @param index
     *            the index in the list
     * @param value
     *            the boolean to set
     * @throws AutomergeException
     *             if the object is not a list
     */
    public void set(ObjectId obj, long index, boolean value);

    /**
     * Set a counter in a map
     *
     * @param obj
     *            the object id of the map
     * @param key
     *            the key in the map
     * @param value
     *            the counter to set
     * @throws AutomergeException
     *             if the object is not a map
     */
    public void set(ObjectId obj, String key, Counter value);

    /**
     * Set a counter in a list
     *
     * @param obj
     *            the object id of the list
     * @param index
     *            the index in the list
     * @param value
     *            the counter to set
     * @throws AutomergeException
     *             if the object is not a list
     */
    public void set(ObjectId obj, long index, Counter value);

    /**
     * Set a date in a map
     *
     * @param obj
     *            the object id of the map
     * @param key
     *            the key in the map
     * @param value
     *            the date to set
     * @throws AutomergeException
     *             if the object is not a map
     */
    public void set(ObjectId obj, String key, Date value);

    /**
     * Set a date in a list
     *
     * @param obj
     *            the object id of the list
     * @param index
     *            the index in the list
     * @param value
     *            the date to set
     * @throws AutomergeException
     *             if the object is not a list
     */
    public void set(ObjectId obj, long index, Date value);

    /**
     * Set an unsigned integer in a map
     *
     * @param obj
     *            the object id of the map
     * @param key
     *            the key in the map
     * @param value
     *            the integer to set
     * @throws AutomergeException
     *             if the object is not a map
     * @throws IllegalArgumentException
     *             if the value is negative
     */
    public void setUint(ObjectId obj, String key, long value);

    /**
     * Set an unsigned integer in a list
     *
     * @param obj
     *            the object id of the list
     * @param index
     *            the index in the list
     * @param value
     *            the integer to set
     * @throws AutomergeException
     *             if the object is not a list
     * @throws IllegalArgumentException
     *             if the value is negative
     */
    public void setUint(ObjectId obj, long index, long value);

    /**
     * Set a key in a map to null
     *
     * @param obj
     *            the object id of the map
     * @param key
     *            the key in the map
     * @throws AutomergeException
     *             if the object is not a map
     * @throws IllegalArgumentException
     *             if the value is negative
     */
    public void setNull(ObjectId obj, String key);

    /**
     * Set an index in a list to null
     *
     * @param obj
     *            the object id of the list
     * @param index
     *            the index in the list
     * @throws AutomergeException
     *             if the object is not a map
     * @throws IllegalArgumentException
     *             if the value is negative
     */
    public void setNull(ObjectId obj, long index);

    /**
     * Create a new object at the given key in a map
     *
     * @param parent
     *            the object id of the map to set the key in
     * @param key
     *            the key in the map
     * @param objType
     *            the type of object to create
     * @return the object id of the new object
     * @throws AutomergeException
     *             if the object is not a map
     */
    public ObjectId set(ObjectId parent, String key, ObjectType objType);

    /**
     * Create a new object at the given index in a list
     *
     * @param parent
     *            the object id of the list to set inside
     * @param index
     *            the index in the list
     * @param objType
     *            the type of object to create
     * @return the object id of the new object
     * @throws AutomergeException
     *             if the object is not a list or the index is out of range
     */
    public ObjectId set(ObjectId parent, long index, ObjectType objType);

    /**
     * Insert a double into a list
     *
     * @param obj
     *            the object id of the list
     * @param index
     *            the index in the list
     * @param value
     *            the double to insert
     * @throws AutomergeException
     *             if the object is not a list or the index is out of range
     */
    public void insert(ObjectId obj, long index, double value);

    /**
     * Insert a string into a list
     *
     * @param obj
     *            the object id of the list
     * @param index
     *            the index in the list
     * @param value
     *            the string to insert
     * @throws AutomergeException
     *             if the object is not a list or the index is out of range
     */
    public void insert(ObjectId obj, long index, String value);

    /**
     * Insert an int into a list
     *
     * @param obj
     *            the object id of the list
     * @param index
     *            the index in the list
     * @param value
     *            the integer to insert
     * @throws AutomergeException
     *             if the object is not a list or the index is out of range
     */
    public void insert(ObjectId obj, long index, int value);

    /**
     * Insert a byte array into a list
     *
     * @param obj
     *            the object id of the list
     * @param index
     *            the index in the list
     * @param value
     *            the bytes to insert
     * @throws AutomergeException
     *             if the object is not a list or the index is out of range
     */
    public void insert(ObjectId obj, long index, byte[] value);

    /**
     * Insert a counter into a list
     *
     * @param obj
     *            the object id of the list
     * @param index
     *            the index in the list
     * @param value
     *            the counter to insert
     * @throws AutomergeException
     *             if the object is not a list or the index is out of range
     */
    public void insert(ObjectId obj, long index, Counter value);

    /**
     * Insert a date into a list
     *
     * @param obj
     *            the object id of the list
     * @param index
     *            the index in the list
     * @param value
     *            the date to insert
     * @throws AutomergeException
     *             if the object is not a list or the index is out of range
     */
    public void insert(ObjectId obj, long index, Date value);

    /**
     * Insert a boolean into a list
     *
     * @param obj
     *            the object id of the list
     * @param index
     *            the index in the list
     * @param value
     *            the boolean to insert
     * @throws AutomergeException
     *             if the object is not a list or the index is out of range
     */
    public void insert(ObjectId obj, long index, boolean value);

    /**
     * Insert any non-object value into a list
     *
     * @param obj
     *            the object id of the list
     * @param index
     *            the index in the list
     * @param value
     *            the new value to insert
     * @throws AutomergeException
     *             if the object is not a list or the index is out of range
     */
    public void insert(ObjectId obj, long index, NewValue value);

    /**
     * Insert a new object in a list
     *
     * @param parent
     *            the object id of the list to insert into
     * @param index
     *            the index in the list
     * @param objType
     *            the object type to insert
     * @return the object id of the new object
     * @throws AutomergeException
     *             if the object is not a list or the index is out of range
     */
    public ObjectId insert(ObjectId parent, long index, ObjectType objType);

    /**
     * Insert a null into a list
     *
     * @param obj
     *            the object id of the list
     * @param index
     *            the index in the list
     * @throws AutomergeException
     *             if the object is not a list or the index is out of range
     */
    public void insertNull(ObjectId obj, long index);

    /**
     * Insert an unsigned integer into a list
     *
     * @param obj
     *            the object id of the list
     * @param index
     *            the index in the list
     * @param value
     *            the integer to insert
     * @throws AutomergeException
     *             if the object is not a list or the index is out of range
     * @throws IllegalArgumentException
     *             if the value is negative
     */
    public void insertUint(ObjectId obj, long index, long value);

    /**
     * Increment a counter in a map
     *
     * @param obj
     *            the object id of the map
     * @param key
     *            the key in the map where the counter is
     * @param amount
     *            the amount to increment by
     * @throws AutomergeException
     *             if the object is not a map or the key is not a counter
     */
    public void increment(ObjectId obj, String key, long amount);

    /**
     * Increment a counter in a list
     *
     * @param obj
     *            the object id of the list
     * @param index
     *            the idx in the list where the counter is
     * @param amount
     *            the amount to increment by
     * @throws AutomergeException
     *             if the object is not a list or the index is not a counter
     */
    public void increment(ObjectId obj, long index, long amount);

    /**
     * Delete a key from a map
     *
     * @param obj
     *            the object id of the map
     * @param key
     *            the key to delete
     * @throws AutomergeException
     *             if the object is not a map or the key does not exist
     */
    public void delete(ObjectId obj, String key);

    /**
     * Delete an element from a list
     *
     * @param obj
     *            the object id of the map
     * @param index
     *            the index of the element to delete
     * @throws AutomergeException
     *             if the object is not a list or the index is out of bounds
     */
    public void delete(ObjectId obj, long index);

    /**
     * Splice multiple non-object values into a list
     *
     * @param obj
     *            the object id of the list
     * @param start
     *            the index in the list to start splicing
     * @param deleteCount
     *            the number of elements to delete
     * @param items
     *            the new values to insert
     * @throws AutomergeException
     *             if the object is not a list or the start index is out of range
     */
    public void splice(ObjectId obj, long start, long deleteCount, Iterator<NewValue> items);

    /**
     * Splice text into a text object
     *
     * @param obj
     *            the object id of the text object
     * @param start
     *            the index in the text to start splicing
     * @param deleteCount
     *            the number of characters to delete
     * @param text
     *            the new text to insert
     * @throws AutomergeException
     *             if the object is not a text object or the start index is out of
     *             range
     */
    public void spliceText(ObjectId obj, long start, long deleteCount, String text);

    /**
     * Create a mark
     *
     * @param obj
     *            the object id of the text object to create the mark on
     * @param start
     *            the index in the text object to start the mark at
     * @param end
     *            the index in the text object to end the mark at
     * @param markName
     *            the name of the mark
     * @param value
     *            the value to associate with the mark
     * @param expand
     *            how to expand the mark
     * @throws AutomergeException
     *             if the object is not a text object or the start or end index is
     *             out of range
     */
    public void mark(ObjectId obj, long start, long end, String markName, NewValue value, ExpandMark expand);

    /**
     * Create a mark with a string value
     *
     * @param obj
     *            the object id of the text object to create the mark on
     * @param start
     *            the index in the text object to start the mark at
     * @param end
     *            the index in the text object to end the mark at
     * @param markName
     *            the name of the mark
     * @param value
     *            the string to associate with the mark
     * @param expand
     *            how to expand the mark
     * @throws AutomergeException
     *             if the object is not a text object or the start or end index is
     *             out of range
     */
    public void mark(ObjectId obj, long start, long end, String markName, String value, ExpandMark expand);

    /**
     * Create a mark with an integer value
     *
     * @param obj
     *            the object id of the text object to create the mark on
     * @param start
     *            the index in the text object to start the mark at
     * @param end
     *            the index in the text object to end the mark at
     * @param markName
     *            the name of the mark
     * @param value
     *            the integer to associate with the mark
     * @param expand
     *            how to expand the mark
     * @throws AutomergeException
     *             if the object is not a text object or the start or end index is
     *             out of range
     */
    public void mark(ObjectId obj, long start, long end, String markName, long value, ExpandMark expand);

    /**
     * Create a mark with an unsigned integer value
     *
     * @param obj
     *            the object id of the text object to create the mark on
     * @param start
     *            the index in the text object to start the mark at
     * @param end
     *            the index in the text object to end the mark at
     * @param markName
     *            the name of the mark
     * @param value
     *            the integer to associate with the mark
     * @param expand
     *            how to expand the mark
     * @throws AutomergeException
     *             if the object is not a text object or the start or end index is
     *             out of range
     * @throws IllegalArgumentException
     *             if the value is negative
     */
    public void markUint(ObjectId obj, long start, long end, String markName, long value, ExpandMark expand);

    /**
     * Create a mark with a double value
     *
     * @param obj
     *            the object id of the text object to create the mark on
     * @param start
     *            the index in the text object to start the mark at
     * @param end
     *            the index in the text object to end the mark at
     * @param markName
     *            the name of the mark
     * @param value
     *            the double to associate with the mark
     * @param expand
     *            how to expand the mark
     * @throws AutomergeException
     *             if the object is not a text object or the start or end index is
     *             out of range
     */
    public void mark(ObjectId obj, long start, long end, String markName, double value, ExpandMark expand);

    /**
     * Create a mark with a byte array
     *
     * @param obj
     *            the object id of the text object to create the mark on
     * @param start
     *            the index in the text object to start the mark at
     * @param end
     *            the index in the text object to end the mark at
     * @param markName
     *            the name of the mark
     * @param value
     *            the byte array to associate with the mark
     * @param expand
     *            how to expand the mark
     * @throws AutomergeException
     *             if the object is not a text object or the start or end index is
     *             out of range
     */
    public void mark(ObjectId obj, long start, long end, String markName, byte[] value, ExpandMark expand);

    /**
     * Create a mark with a counter value
     *
     * @param obj
     *            the object id of the text object to create the mark on
     * @param start
     *            the index in the text object to start the mark at
     * @param end
     *            the index in the text object to end the mark at
     * @param markName
     *            the name of the mark
     * @param value
     *            the counter to associate with the mark
     * @param expand
     *            how to expand the mark
     * @throws AutomergeException
     *             if the object is not a text object or the start or end index is
     *             out of range
     */
    public void mark(ObjectId obj, long start, long end, String markName, Counter value, ExpandMark expand);

    /**
     * Create a mark with a {@link Date} value
     *
     * @param obj
     *            the object id of the text object to create the mark on
     * @param start
     *            the index in the text object to start the mark at
     * @param end
     *            the index in the text object to end the mark at
     * @param markName
     *            the name of the mark
     * @param value
     *            the date to associate with the mark
     * @param expand
     *            how to expand the mark
     * @throws AutomergeException
     *             if the object is not a text object or the start or end index is
     *             out of range
     */
    public void mark(ObjectId obj, long start, long end, String markName, Date value, ExpandMark expand);

    /**
     * Create a mark with a boolean value
     *
     * @param obj
     *            the object id of the text object to create the mark on
     * @param start
     *            the index in the text object to start the mark at
     * @param end
     *            the index in the text object to end the mark at
     * @param markName
     *            the name of the mark
     * @param value
     *            the boolean to associate with the mark
     * @param expand
     *            how to expand the mark
     * @throws AutomergeException
     *             if the object is not a text object or the start or end index is
     *             out of range
     */
    public void mark(ObjectId obj, long start, long end, String markName, boolean value, ExpandMark expand);

    /**
     * Create a mark with a null value
     *
     * @param obj
     *            the object id of the text object to create the mark on
     * @param start
     *            the index in the text object to start the mark at
     * @param end
     *            the index in the text object to end the mark at
     * @param markName
     *            the name of the mark
     * @param expand
     *            how to expand the mark
     * @throws AutomergeException
     *             if the object is not a text object or the start or end index is
     *             out of range
     */
    public void markNull(ObjectId obj, long start, long end, String markName, ExpandMark expand);

    /**
     * remove a mark from a range of characters in a text object
     *
     * @param obj
     *            the object id of the text object to remove the mark from
     * @param start
     *            the index in the text object to start removing from
     * @param end
     *            the index in the text object to end removing at
     * @param markName
     *            the name of the mark to remove
     * @param expand
     *            how the removed span should expand
     */
    public void unmark(ObjectId obj, String markName, long start, long end, ExpandMark expand);

    @Override
    public void close();
}
