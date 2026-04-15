package org.automerge.repo;

import java.util.List;
import java.util.Objects;
import org.automerge.LoadLibrary;

/**
 * Represents the result of calling SamodLoader.step().
 *
 * This corresponds to the Rust LoaderState enum and indicates whether the
 * loader needs IO operations to be performed or if loading is complete.
 */
abstract class LoaderStepResult {
    static {
        LoadLibrary.initialize();
    }

    /**
     * The loader needs IO operations to be performed.
     *
     * The caller should execute all provided IO tasks and call
     * provideSamodLoaderIoResult for each completed task, then call stepSamodLoader
     * again.
     *
     * <p>
     * Note: The field is named {@code value0} because the JNI layer generates tuple
     * variant fields with indexed names. The field type is {@code List} because the
     * Rust {@code Vec<T>} maps to {@code java.util.List} in the JNI layer.
     */
    static class NeedIo extends LoaderStepResult {

        private final List<IoTask<StorageTask>> value0;

        NeedIo(List<IoTask<StorageTask>> value0) {
            this.value0 = Objects.requireNonNull(value0, "ioTasks cannot be null");
        }

        /**
         * Gets the IO tasks that need to be executed.
         *
         * @return List of IO tasks to execute
         */
        public List<IoTask<StorageTask>> getIoTasks() {
            return value0;
        }

        @Override
        public boolean equals(Object obj) {
            if (this == obj)
                return true;
            if (obj == null || getClass() != obj.getClass())
                return false;
            NeedIo needIo = (NeedIo) obj;
            return Objects.equals(value0, needIo.value0);
        }

        @Override
        public int hashCode() {
            return Objects.hash(value0);
        }

        @Override
        public String toString() {
            return ("LoaderStepResult.NeedIo{ioTasks=" + value0 + "}");
        }
    }

    /**
     * Loading is complete and the samod repository Hub is ready to use.
     *
     * The Hub can be used to handle events and manage the repository.
     *
     * <p>
     * Note: The field is named {@code value0} because the JNI layer generates tuple
     * variant fields with indexed names. The JNI layer creates a {@code Hub} object
     * directly (using the pointer pattern) and sets it as this field.
     */
    static class Loaded extends LoaderStepResult {

        private final RepoSys.HubPointer value0;

        Loaded(RepoSys.HubPointer value0) {
            this.value0 = Objects.requireNonNull(value0, "hub cannot be null");
        }

        /**
         * Gets the loaded Hub pointer.
         *
         * @return The loaded Hub pointer
         */
        public RepoSys.HubPointer getHub() {
            return value0;
        }

        @Override
        public boolean equals(Object obj) {
            if (this == obj)
                return true;
            if (obj == null || getClass() != obj.getClass())
                return false;
            Loaded loaded = (Loaded) obj;
            return Objects.equals(value0, loaded.value0);
        }

        @Override
        public int hashCode() {
            return Objects.hash(value0);
        }

        @Override
        public String toString() {
            return "LoaderStepResult.Loaded{hub=" + value0 + "}";
        }
    }
}
