package org.automerge;

/**
 * The ID of a composite object in an automerge document
 *
 * <p>
 * Composite objects are objects that contain other objects, such as lists and
 * maps. The ID of the object is used to read and write to the object. The root
 * object in automerge is a map, the ID of whic his {@link ObjectId#ROOT}.
 */
public class ObjectId {
    private byte[] raw;

    public static ObjectId ROOT;

    static {
        ROOT = AutomergeSys.rootObjectId();
    }

    private ObjectId(byte[] raw) {
        this.raw = raw;
    }

    public boolean isRoot() {
        return AutomergeSys.isRootObjectId(this);
    }

    public String toString() {
        return AutomergeSys.objectIdToString(this);
    }

    @Override
    public int hashCode() {
        return AutomergeSys.objectIdHash(this);
    }

    @Override
    public boolean equals(Object obj) {
        if (this == obj)
            return true;
        if (obj == null)
            return false;
        if (getClass() != obj.getClass())
            return false;
        ObjectId other = (ObjectId) obj;
        return AutomergeSys.objectIdsEqual(this, other);
    }
}
