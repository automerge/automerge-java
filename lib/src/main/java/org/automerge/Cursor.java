package org.automerge;

public class Cursor {
	private byte[] raw;

	public static Cursor fromBytes(byte[] encoded) {
		return AutomergeSys.cursorFromBytes(encoded);
	}

	public static Cursor fromString(String encoded) {
		return AutomergeSys.cursorFromString(encoded);
	}

	@Override
	public String toString() {
		return AutomergeSys.cursorToString(this);
	}

	public byte[] toBytes() {
		return raw.clone();
	}
}
