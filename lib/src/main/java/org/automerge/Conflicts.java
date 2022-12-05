package org.automerge;

import java.util.Collection;
import java.util.HashMap;

class Conflicts {
	private HashMap<String, AmValue> values;

	public Collection<AmValue> values() {
		return this.values.values();
	}
}
