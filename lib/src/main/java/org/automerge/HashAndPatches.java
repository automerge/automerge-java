package org.automerge;

import java.util.ArrayList;
import java.util.Arrays;

/**
 * The return value of the transaction type created by
 * {@link Document#startTransactionForPatches}
 */
public class HashAndPatches {
	/** The {@link Patch}es representing the changes made during the transaction */
	public final ArrayList<Patch> patches;
	/** The hash of the change the transaction created */
	public final ChangeHash changeHash;

	protected HashAndPatches(Patch[] patches, ChangeHash changeHash) {
		this.patches = new ArrayList<>(Arrays.asList(patches));
		this.changeHash = changeHash;
	}
}
