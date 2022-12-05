package org.automerge;

import java.util.Optional;

/**
 * A transaction which does not track patches and only return the hash on commit
 *
 * <p>
 * This type is created by {@link Document#startTransaction} and
 */
public class TransactionWithoutPatches extends AbstractTransaction<ChangeHash> {
	protected TransactionWithoutPatches(Document doc, AutomergeSys.UnobservedTransactionPointer pointer) {
		super(doc, pointer);
	}

	@Override
	protected Optional<ChangeHash> doCommit(AutomergeSys.TransactionPointer tx) {
		return AutomergeSys.commitUnobservedTransaction((AutomergeSys.UnobservedTransactionPointer) tx);
	}
}
