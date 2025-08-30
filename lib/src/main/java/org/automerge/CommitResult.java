package org.automerge;

import java.util.Optional;

class CommitResult {
    private Optional<ChangeHash> hash;
    private AutomergeSys.PatchLogPointer patchLog;

    protected CommitResult(Optional<ChangeHash> hash, AutomergeSys.PatchLogPointer patchLog) {
        this.hash = hash;
        this.patchLog = patchLog;
    }

    protected Optional<ChangeHash> getHash() {
        return hash;
    }

    protected AutomergeSys.PatchLogPointer getPatchLog() {
        return patchLog;
    }

}
