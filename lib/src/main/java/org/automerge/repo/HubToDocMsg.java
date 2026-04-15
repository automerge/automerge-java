package org.automerge.repo;

import org.automerge.LoadLibrary;

/**
 * Messages sent from hub to document actors.
 */
class HubToDocMsg {
    static {
        LoadLibrary.initialize();
    }

    @SuppressWarnings("unused") // Set and read by JNI via pointer pattern
    private long pointer;

    HubToDocMsg() {}
}
