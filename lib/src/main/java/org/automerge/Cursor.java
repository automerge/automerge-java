package org.automerge;

/**
 * A stable reference to a position in a sequence
 * <p>
 * When working with sequences it is often useful to refer to a position in the
 * sequence without having to keep track of how that position changes as changes
 * are made to the document. A cursor is such a reference. You create a cursor
 * with {@link Read#makeCursor(ObjectId, long)} and then dereference it with
 * {@link Read#lookupCursorIndex(ObjectId, Cursor)}.
 * <p>
 * Cursors are intended to be serialized and can be sent across the network or
 * stored in a database or an automerge document. The {@link toString()} and
 * {@link fromString(String)} methods are interoperable with the JavaScript
 * implementation. The {@link toBytes()} and {@link fromBytes(byte[])} methods
 * do not have equivalents in the JavaScript implementation so if interop is
 * important you should use the string methods.
 */
public class Cursor {
	private byte[] raw;

	/**
	 * Parse the output of {@link toBytes()}
	 *
	 * @param encoded
	 *            the output of {@link toBytes()}
	 *
	 * @throws IllegalArgumentException
	 *             if the input is not a valid cursor
	 *
	 * @return the parsed cursor
	 */
	public static Cursor fromBytes(byte[] encoded) {
		return AutomergeSys.cursorFromBytes(encoded);
	}

	/**
	 * Parse the output of {@link toString()}
	 *
	 * @param encoded
	 *            the output of {@link toString()}
	 *
	 * @throws IllegalArgumentException
	 *             if the input is not a valid cursor
	 *
	 * @return the parsed cursor
	 */
	public static Cursor fromString(String encoded) {
		return AutomergeSys.cursorFromString(encoded);
	}

	/**
	 * Encode a cursor as a string
	 * <p>
	 * This encoding is interoperable with cursors produced by the JavaScript
	 * implementation.
	 *
	 * @return the encoded cursor
	 */
	@Override
	public String toString() {
		return AutomergeSys.cursorToString(this);
	}

	/**
	 * Encode a cursor as a byte array
	 *
	 * @return the encoded cursor
	 */
	public byte[] toBytes() {
		return raw.clone();
	}
}
