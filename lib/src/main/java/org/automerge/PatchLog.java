package org.automerge;

import java.util.Optional;
import java.util.function.Consumer;
import java.util.function.Function;
import org.automerge.AutomergeSys.PatchLogPointer;

class PatchLog {
	private Optional<PatchLogPointer> pointer;

	public PatchLog() {
		pointer = Optional.of(AutomergeSys.createPatchLog());
	}

	synchronized <T> T with(Function<PatchLogPointer, T> f) {
		return f.apply(pointer.get());
	}

	synchronized PatchLogPointer take() {
		if (pointer.isPresent()) {
			PatchLogPointer p = pointer.get();
			pointer = Optional.empty();
			return p;
		} else {
			throw new IllegalStateException("PatchLog already in use");
		}
	}

	synchronized void put(PatchLogPointer p) {
		if (pointer.isPresent()) {
			throw new IllegalStateException("PatchLog already in use");
		} else {
			pointer = Optional.of(p);
		}
	}

	synchronized void with(Consumer<PatchLogPointer> f) {
		f.accept(pointer.get());
	}

	public synchronized void free() {
		pointer.ifPresent(AutomergeSys::freePatchLog);
		pointer = Optional.empty();
	}
}
