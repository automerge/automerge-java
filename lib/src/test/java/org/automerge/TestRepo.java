package org.automerge;

import java.util.concurrent.Future;
import org.junit.jupiter.api.Assertions;
import org.junit.jupiter.api.Test;

class TestRepo {

	@Test
	public void testCreate() {
		Repo repo = new Repo();
		DocHandle doc = repo.create().join();
	}

	@Test
	public void testLoad() {
		Repo repo = new Repo();
	}

	@Test
	public void testFind() {
		Repo alice = new Repo();
		Repo bob = new Repo();

		// Create two connected DummyTransport instances
		DummyTransport[] transports = DummyTransport.connectedPair();

		alice.connect(transports[0]);
		bob.connect(transports[1]);

		DocHandle bobHandle = bob.create().join();
        DocHandle aliceHandle = alice.find(bobHandle.getUrl()).join();
        Assertions.assertEquals(bobHandle, aliceHandle);
	}

	@Test
	public void testConnectedTransports() throws Exception {
		// Create two connected DummyTransport instances
		DummyTransport[] transports = DummyTransport.connectedPair();
		DummyTransport transport1 = transports[0];
		DummyTransport transport2 = transports[1];

		// Test message passing between transports
		byte[] message = "Hello, World!".getBytes();

		// Send message from transport1 to transport2
		Future<Void> sendFuture = transport1.send(message);
		sendFuture.get(); // Wait for send to complete

		// Receive message on transport2
		Future<byte[]> receiveFuture = transport2.receive();
		byte[] receivedMessage = receiveFuture.get();

		// Verify the message was received correctly
		Assertions.assertArrayEquals(message, receivedMessage);

		// Test bidirectional communication
		byte[] response = "Hello back!".getBytes();
		transport2.send(response).get();
		byte[] receivedResponse = transport1.receive().get();

		Assertions.assertArrayEquals(response, receivedResponse);

		// Test cleanup
		transport1.close();
		transport2.close();
	}
}
