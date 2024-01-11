package org.automerge;

public class Mark {
	private final long start;
	private final long end;
	private final String name;
	private final AmValue value;

	protected Mark(long start, long end, String name, AmValue value) {
		this.start = start;
		this.end = end;
		this.name = name;
		this.value = value;
	}

	public long getStart() {
		return start;
	}

	public long getEnd() {
		return end;
	}

	public String getName() {
		return name;
	}

	public AmValue getValue() {
		return value;
	}

	@Override
	public String toString() {
		return "Mark [start=" + start + ", end=" + end + ", name=" + name + ", value=" + value + "]";
	}
}
