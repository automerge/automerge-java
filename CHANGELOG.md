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
