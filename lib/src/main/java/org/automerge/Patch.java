package org.automerge;

import java.util.ArrayList;
import java.util.Arrays;

/**
 * A single change made to a document
 *
 * <p>
 * This type is returned by the various "*ForPatches" methods on
 * {@link Document} and describes a change that was made to the document.
 */
public class Patch {
    private final ObjectId obj;
    private final ArrayList<PathElement> path;
    private final PatchAction action;

    protected Patch(ObjectId obj, PathElement[] path, PatchAction action) {
        this.obj = obj;
        this.path = new ArrayList<>(Arrays.asList(path));
        this.action = action;
    }

    /**
     * The object this patch modifies
     *
     * @return The object this patch modifies
     */
    public ObjectId getObj() {
        return obj;
    }

    /**
     * The path to the object this patch modifies
     *
     * @return The path to the object this patch modifies
     */
    public ArrayList<PathElement> getPath() {
        return path;
    }

    /**
     * The modification this patch makes
     *
     * @return The modification this patch makes
     */
    public PatchAction getAction() {
        return action;
    }
}
