package org.automerge;

import java.util.Optional;

/**
 * A transaction which tracks {@link Patch}es and returns them on commit
 *
 * <p>
 * This type is created by {@link Document#startTransactionForPatches} and
 */
public class TransactionWithPatches extends AbstractTransaction<HashAndPatches> {
	protected TransactionWithPatches(Document doc, AutomergeSys.ObservedTransactionPointer pointer) {
		super(doc, pointer);
	}

	@Override
	protected Optional<HashAndPatches> doCommit(AutomergeSys.TransactionPointer tx) {
		return AutomergeSys.commitObservedTransaction((AutomergeSys.ObservedTransactionPointer) tx);
	}
}
