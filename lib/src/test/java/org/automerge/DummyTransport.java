package org.automerge;

import java.util.concurrent.BlockingQueue;
import java.util.concurrent.CompletableFuture;
import java.util.concurrent.Future;
import java.util.concurrent.LinkedBlockingQueue;

class DummyTransport implements Transport {

	private final BlockingQueue<byte[]> incomingMessages = new LinkedBlockingQueue<>();
	private DummyTransport connectedTransport;
	private boolean closed = false;

	public void connect(DummyTransport other) {
		this.connectedTransport = other;
		other.connectedTransport = this;
	}

	public static DummyTransport[] connectedPair() {
		DummyTransport transport1 = new DummyTransport();
		DummyTransport transport2 = new DummyTransport();
		transport1.connect(transport2);
		return new DummyTransport[]{transport1, transport2};
	}

	@Override
	public Future<Void> send(byte[] data) {
		return CompletableFuture.supplyAsync(() -> {
			if (closed || connectedTransport == null || connectedTransport.closed) {
				throw new RuntimeException("Transport is closed or not connected");
			}
			try {
				connectedTransport.incomingMessages.put(data);
				return null;
			} catch (InterruptedException e) {
				Thread.currentThread().interrupt();
				throw new RuntimeException(e);
			}
		});
	}

	@Override
	public Future<byte[]> receive() {
		return CompletableFuture.supplyAsync(() -> {
			if (closed) {
				throw new RuntimeException("Transport is closed");
			}
			try {
				return incomingMessages.take();
			} catch (InterruptedException e) {
				Thread.currentThread().interrupt();
				throw new RuntimeException(e);
			}
		});
	}

	@Override
	public void close() {
		closed = true;
		if (connectedTransport != null) {
			connectedTransport.connectedTransport = null;
		}
		connectedTransport = null;
	}
}
