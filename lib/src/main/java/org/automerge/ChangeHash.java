package org.automerge;

/** The hash of a single change to an automerge document */
public class ChangeHash {
	private byte[] hash;

	protected ChangeHash(byte[] hash) {
		this.hash = hash;
	}

	/**
	 * @return the bytes of the hash
	 */
	public byte[] getBytes() {
		return hash.clone();
	}
}
