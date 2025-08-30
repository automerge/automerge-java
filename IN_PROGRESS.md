# In Progress

We're currently in the last stage of phase 1, mapping the Rust actor types to Java actor types. The three types are `Hub`, `DocumentActor`, and `SamodLoader`. They are all already represented by `*Pointer` types in the `AutomergeSys.java` file, the task now is to define matching java classes which wrap the underlying pointers and expose the relevant methods from the Rust interface.

Classes to implement:

* ✅ `Hub` (basic structure implemented, remaining: complete JNI conversion modules)
* ⏳ `DocumentActor`
* ⏳ `SamodLoader`

## Hub Implementation Status

### ✅ Completed:
- Basic Java `Hub` class structure with core methods
- JNI method signatures in `AutomergeSys.java`
- Rust JNI implementation skeleton in `samod_hub_jni.rs`
- Supporting classes: `EstablishedPeer` for simple peer info
- Module structure for conversion functions

### 🔧 Remaining Work:
- Complete Rust->Java conversion modules:
  - `samod_connection_info.rs` (fix lifetime issues)
  - `samod_peer_id.rs` conversion functions
  - `samod_storage_id.rs` conversion functions  
  - Missing utility functions in `interop.rs`
- Fix compilation errors in existing conversion modules
- Complete `getConnections()` and `getEstablishedPeers()` implementations

### 📋 Hub Interface:
The Hub class provides these key methods:
- `handleEvent(timestamp, event)` - Main event processing
- `getStorageId()` - Get storage identifier
- `getPeerId()` - Get peer identifier  
- `isConnectedTo(peerId)` - Check peer connection status
- `isStopped()` - Check if hub is stopped
- `free()` - Manual memory management

> [!IMPORTANT]
> The version of `samod-core` we are working with is a local fork. Look in `Cargo.toml` for the exact location. This means you should look at the types and documentation in that build rather than the build on crates.io

----------------------------------

Currently this library provides a wrapper around the raw automerge rust library which is exposed via the `org.automerge.Document` class. Automerge is mostly used on the web where in addition to the core `@automerge/automerge` javascript package (which also wraps the rust library) there is the `@automerge/automerge-repo` package. This package provides networking and storage to complement the pure in-memory data structure which is exposed via the core automerge library. In rust I have built a crate called `samod` which implements the same network and storage protocol as `automerge-repo` and I want to use this crate to expose the same functionality in java.

## The `samod` and `samod-core` crates

`samod` is a high level wrapper around the `samod-core` crate. For this project we should use the `samod-core` crate directly. `samod-core` is a sans-IO implementation of the networking and storage interfaces which is intended to be used via FFI. The documentation for the `samod-core` crate can be found at [https://docs.rs/samod-core](https://docs.rs/samod-core) and has a good introduction to the core concepts, here's the TLDR:

* `samod-core` is a sans-IO actor model based implementation of a network and storage protocol
* There are two kinds of actors: a single "hub" actor, which all network messages are routed through, and a "document actor" per automerge document, which the hub actor controls the spawning of and routes messages to
* Each actor has an interface where you twiddle the state of the actor and handle a returned data structure describing what IO to perform and what commands are completed
* Interaction with the overall system is achieved by dispatching "commands" to the hub actor and waiting for the command to be completed in a future result
* Interaction with a particular automerge document is achieved by finding the document actor corresponding to the document ID (retrieved from either a create or find command) and then using the `with_document` method on the document actor, passing it a closure.
* The runtime (what we are writing) is responsible for routing messages between actors and handling IO operations
* Interaction with the network is achieved by creating "connections", which are effectively an ID used to tag incoming and outgoing `Vec<u8>` messages with

## Desired Java Interface

The interface I want to wrap this in is much higher level. Rather than thinking about actors and messages I want the user to create a `Repo` instance, which takes an implementation of a `Storage` interface and an `AnnouncePolicy` interface. Network communication is handled by creating an instance of a `Transport` interface, which might look like this:

```java
public interface Transport {
	Future<Void> send(byte[] data);
	Future<byte[]> receive();
	void close();
}
```

and passing this to something like a `Repo.connect` method which takes a `Transport` instance and returns a `Future<ConnFinishedReason>` describing why the connection completed.

The basic approach to achieve this is:

* Define Java <-> Rust bindings for all the data transfer types (*Id, *Result,
  etc.) in `samod-core`. This is now complete
* Define Java implementations of the Hub and DocumentActor classes which wrap
  the underlying actors in a low level Java version of the same core interface
* Write the higher level java interface which achieves the desired interface


## Type Mapping Strategy

To avoid memory sharing issues between Java and Rust, wherever possible we use a strategy of transferring the value across the JNI boundary, rather than retaining pointers to the Rust side. The main places where this is not possible are the actor objects (Hub, DocumentActor, SamodLoader).

## Implementation Strategy

### Phase 1: Foundation - Rust JNI Bindings for samod-core (First Major Chunk)

All that remains of this phase is the actor type mappings.

### Phase 2: Java Abstraction

Once the JNI foundation is solid, build the higher-level Java interfaces:

- Implement the `Repo` class that manages the hub actor lifecycle
- Create `Storage` and `AnnouncePolicy` interfaces with default implementations
- Build the `Transport` interface and connection management
- Implement proper async/Future patterns for command dispatching
