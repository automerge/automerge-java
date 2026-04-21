## 0.0.9

### Added

* `org.automerge.repo` - an implementation of the `automerge-repo` patterns from
  the JavaScript world. This makes it easy to wire up networking and storage to
  a reactive view of a document.
* `automerge-websocket` - a package which implements a websocket transport for
  `org.automerge.repo`. This is wire compatible with the JavaScript implementation
  and so can be used to interoperate with web clients talking to JS sync servers
  or run Java sync servers talking to web clients
* `automerge-kotlin` - a small library of convenience functions for use when
  working with `org.automerge.repo` from kotlin
* SLF4j integration

### Changed

* Updated to the `0.0.8` version of the Rust library

## 0.0.8

* Update the rust `automerge` library to `0.7.1`. This dramatically improves
  memory use as well as containing many bug fixes.
* Add support for windows ARM
* Fix bug where ARM libraries for Linux were not loaded correctly
* Fix a bug where some methods would return nonsensical results on windows
* Support Java 8
* Fix a bug where generating patches which contained conflicts could crash

## 0.0.7

* Update the rust `automerge` library to 0.5.7. This introduces some performance
  improvements to the sync protocol. No user facing or network-incompatible
  changes
* Add `Read.getMarksAtIndex` to lookup the marks for a particular index in a
  text sequence
* Add `Read.getObjectType` which returns the type of object an object ID refers
  to
* Add support for cursors, which are stable references to a position in a
  sequence
* Make `PatchLog` a public type
* Fix a bug where passing a null object ID to some methods would crash the JVM

## 0.0.6

A bump purely to make some packaging machinery happy. No changes

## 0.0.5

* * Make AmValue.Counter.getValue() return long

## 0.0.4

* Upgrade to automerge rust 0.5.1
* `Transaction` no longer has a generic parameter and the `HashAndPatches`
  version is gone. Instead call `Document.startTransaction(PatchLog)` to get
  patches created during the transaction.
* Added `Document.startTransactionAt` to start a transaction at a given set of
  heads rather than the current heads of the document
* All the `*ForPatches` methods have been removed and replaced with overloads
  which take a `PatchLog` as an argument. To obtain patches first call these
  various methods, passing in a `PatchLog` and then use `Document.makePatches`
  to turn the patch log into a list of patches.
* Added `Document.diff` to obtain a list of patches representing the difference
  between two different sets of heads of the document.
* Make `MapEntry` public

## 0.0.3

* Added SyncState.isInSync
