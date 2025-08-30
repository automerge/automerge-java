package org.automerge;

import java.util.concurrent.Future;

public interface Transport {
	Future<Void> send(byte[] data);
	Future<byte[]> receive();
	void close();
}
