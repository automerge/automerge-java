package org.automerge;

import java.util.Arrays;
import java.util.Objects;

/**
 * Represents the result of calling SamodLoader.step().
 *
 * This corresponds to the Rust LoaderState enum and indicates whether the loader
 * needs IO operations to be performed or if loading is complete.
 */
public abstract class LoaderStepResult {

    /**
     * The loader needs IO operations to be performed.
     *
     * The caller should execute all provided IO tasks and call provideSamodLoaderIoResult
     * for each completed task, then call stepSamodLoader again.
     */
    public static class NeedIo extends LoaderStepResult {
        private final IoTask[] ioTasks;

        NeedIo(IoTask[] ioTasks) {
            this.ioTasks = Objects.requireNonNull(ioTasks, "ioTasks cannot be null");
        }

        /**
         * Gets the IO tasks that need to be executed.
         *
         * Each task should be executed and its result provided back to the loader
         * via provideSamodLoaderIoResult before calling step again.
         *
         * @return Array of IO tasks to execute
         */
        public IoTask[] getIoTasks() {
            return ioTasks.clone(); // defensive copy
        }

        @Override
        public boolean equals(Object obj) {
            if (this == obj) return true;
            if (obj == null || getClass() != obj.getClass()) return false;
            NeedIo needIo = (NeedIo) obj;
            return Arrays.equals(ioTasks, needIo.ioTasks);
        }

        @Override
        public int hashCode() {
            return Arrays.hashCode(ioTasks);
        }

        @Override
        public String toString() {
            return "LoaderStepResult.NeedIo{ioTasks=" + Arrays.toString(ioTasks) + "}";
        }
    }

    /**
     * Loading is complete and the samod repository Hub is ready to use.
     *
     * The Hub can be used to handle events and manage the repository.
     */
    public static class Loaded extends LoaderStepResult {
        private final AutomergeSys.HubPointer hub;

        Loaded(AutomergeSys.HubPointer hub) {
            this.hub = Objects.requireNonNull(hub, "hub cannot be null");
        }

        /**
         * Gets the loaded Hub instance.
         *
         * This Hub is ready to handle events and manage documents in the repository.
         *
         * @return The loaded Hub pointer
         */
        public AutomergeSys.HubPointer getHub() {
            return hub;
        }

        @Override
        public boolean equals(Object obj) {
            if (this == obj) return true;
            if (obj == null || getClass() != obj.getClass()) return false;
            Loaded loaded = (Loaded) obj;
            return Objects.equals(hub, loaded.hub);
        }

        @Override
        public int hashCode() {
            return Objects.hash(hub);
        }

        @Override
        public String toString() {
            return "LoaderStepResult.Loaded{hub=" + hub + "}";
        }
    }
}
