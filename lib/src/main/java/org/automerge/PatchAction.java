package org.automerge;

import java.util.ArrayList;
import java.util.Arrays;

/** The set of possible patches */
public abstract class PatchAction {

	/** A property was set in a map */
	public static class PutMap extends PatchAction {
		private final String key;
		private final AmValue value;
		private final boolean conflict;

		protected PutMap(String key, AmValue value, boolean conflict) {
			this.key = key;
			this.value = value;
			this.conflict = conflict;
		}

		/**
		 * The key in the map
		 *
		 * @return The key in the map
		 */
		public String getKey() {
			return key;
		}

		/**
		 * The new value that was set
		 *
		 * @return The new value that was set
		 */
		public AmValue getValue() {
			return value;
		}

		/**
		 * Whether there is now a conflict on this property
		 *
		 * @return Whether there is now a conflict on this property
		 */
		public boolean isConflict() {
			return conflict;
		}
	}

	/** A property was set in a list */
	public static class PutList extends PatchAction {
		private final long index;
		private final AmValue value;
		private final boolean conflict;

		protected PutList(long index, AmValue value, boolean conflict) {
			this.index = index;
			this.value = value;
			this.conflict = conflict;
		}

		/**
		 * The index that was set
		 *
		 * @return The index that was set
		 */
		public long getIndex() {
			return index;
		}

		/**
		 * The new value that was set
		 *
		 * @return The new value that was set
		 */
		public AmValue getValue() {
			return value;
		}

		/**
		 * Whether there is now a conflict at this index
		 *
		 * @return Whether there is now a conflict at this index
		 */
		public boolean isConflict() {
			return conflict;
		}
	}

	/** Some values were inserted into a list */
	public static class Insert extends PatchAction {
		private final long index;
		private final ArrayList<AmValue> values;

		protected Insert(long index, AmValue[] values) {
			this.index = index;
			this.values = new ArrayList<>(Arrays.asList(values));
		}

		/**
		 * The index that was inserted into
		 *
		 * @return The index that was inserted into
		 */
		public long getIndex() {
			return index;
		}

		/**
		 * The new values that were inserted
		 *
		 * @return The new values that were inserted
		 */
		public ArrayList<AmValue> getValues() {
			return values;
		}
	}

	/** Values were spliced into a text object */
	public static class SpliceText extends PatchAction {
		private final long index;
		private final java.lang.String text;

		protected SpliceText(long index, java.lang.String text) {
			this.index = index;
			this.text = text;
		}

		/**
		 * The index that was spliced into
		 *
		 * @return The index that was spliced into
		 */
		public long getIndex() {
			return index;
		}

		/**
		 * The new text that was spliced in
		 *
		 * @return The new text that was spliced in
		 */
		public java.lang.String getText() {
			return text;
		}
	}

	/** A counter was incremented */
	public static class Increment extends PatchAction {
		private Prop property;
		private final long value;

		protected Increment(Prop property, long value) {
			this.property = property;
			this.value = value;
		}

		/**
		 * The property that was incremented
		 *
		 * @return The property that was incremented
		 */
		public Prop getProperty() {
			return property;
		}

		/**
		 * The value that was added to the counter
		 *
		 * @return The value that was added to the counter
		 */
		public long getValue() {
			return value;
		}
	}

	/** A key was deleted from a map */
	public static class DeleteMap extends PatchAction {
		private final String key;

		protected DeleteMap(String key) {
			this.key = key;
		}

		/**
		 * The key that was deleted
		 *
		 * @return The key that was deleted
		 */
		public String getKey() {
			return key;
		}
	}

	/** One or more values were deleted from a list */
	public static class DeleteList extends PatchAction {
		private final long index;
		private final long length;

		protected DeleteList(long index, long length) {
			this.index = index;
			this.length = length;
		}

		/**
		 * The index that was deleted
		 *
		 * @return The index that was deleted
		 */
		public long getIndex() {
			return index;
		}

		/**
		 * The number of values that were deleted
		 *
		 * @return The number of values that were deleted
		 */
		public long getLength() {
			return length;
		}
	}

	/** One or more marks were created in a text object */
	public static class Mark extends PatchAction {
		private final org.automerge.Mark[] marks;

		protected Mark(org.automerge.Mark[] marks) {
			this.marks = marks;
		}

		/**
		 * The marks that were created
		 *
		 * @return The marks that were created
		 */
		public org.automerge.Mark[] getMarks() {
			return marks;
		}
	}

	/** A property which was already in the document is now conflicted */
	public static class FlagConflict {
		private final Prop property;

		protected FlagConflict(Prop property) {
			this.property = property;
		}

		/**
		 * The property that was conflicted
		 *
		 * @return The property that was conflicted
		 */
		public Prop getProperty() {
			return property;
		}
	}
}
