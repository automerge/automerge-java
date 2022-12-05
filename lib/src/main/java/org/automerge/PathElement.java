package org.automerge;

/** A single element in a path to a property in a document */
public class PathElement {
	private final ObjectId objectId;
	private final Prop prop;

	protected PathElement(ObjectId objectId, Prop prop) {
		this.objectId = objectId;
		this.prop = prop;
	}

	/**
	 * The object this element points at
	 *
	 * @return The object this element points at
	 */
	public ObjectId getObjectId() {
		return objectId;
	}

	/**
	 * The property within the object that this path points at
	 *
	 * @return The property within the object that this path points at
	 */
	public Prop getProp() {
		return prop;
	}

	@Override
	public int hashCode() {
		final int prime = 31;
		int result = 1;
		result = prime * result + ((objectId == null) ? 0 : objectId.hashCode());
		result = prime * result + ((prop == null) ? 0 : prop.hashCode());
		return result;
	}

	@Override
	public boolean equals(Object obj) {
		if (this == obj)
			return true;
		if (obj == null)
			return false;
		if (getClass() != obj.getClass())
			return false;
		PathElement other = (PathElement) obj;
		if (objectId == null) {
			if (other.objectId != null)
				return false;
		} else if (!objectId.equals(other.objectId))
			return false;
		if (prop == null) {
			if (other.prop != null)
				return false;
		} else if (!prop.equals(other.prop))
			return false;
		return true;
	}
}
