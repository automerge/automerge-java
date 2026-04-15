package org.automerge.repo;

import org.automerge.LoadLibrary;

/**
 * Messages sent from document actors to hub.
 *
 * <p>
 * Note: The {@code pointer} field is set directly by the JNI layer using
 * {@code set_rust_field}. The JNI layer creates DocToHubMsg instances via
 * {@code alloc_object} and stores the native Rust pointer in this field.
 */
class DocToHubMsg {
    static {
        LoadLibrary.initialize();
    }

    @SuppressWarnings("unused") // Set and read by JNI via pointer pattern
    private long pointer;

    /**
     * Package-private constructor - instances are created by JNI via alloc_object.
     */
    DocToHubMsg() {}
}
