package org.automerge.repo;

import org.automerge.LoadLibrary;

/**
 * Thin wrapper around a {@link RepoSys.SamodLoaderPointer} that exposes the
 * step / provide-io-result / free lifecycle as instance methods.
 *
 * <p>
 * This is an internal helper — users should go through {@link Repo#load}
 * instead. The class is package-private so it's only visible inside the
 * repo package.
 *
 * <p>
 * Not thread-safe; operations must be serialized by the caller.
 */
class RepoLoader {
    static {
        LoadLibrary.initialize();
    }

    private final RepoSys.SamodLoaderPointer pointer;

    RepoLoader(RepoSys.SamodLoaderPointer pointer) {
        if (pointer == null) {
            throw new NullPointerException("Loader pointer cannot be null");
        }
        this.pointer = pointer;
    }

    /** Advance the loader one step. */
    LoaderStepResult step(long timestamp) {
        return RepoSys.stepSamodLoader(pointer, timestamp);
    }

    /** Feed a completed IO result back into the loader. */
    void provideIoResult(IoResult<StorageResult> result) {
        RepoSys.provideSamodLoaderIoResult(pointer, result);
    }

    /** Release the native loader. Must be called when done. */
    void free() {
        RepoSys.freeSamodLoader(pointer);
    }
}
