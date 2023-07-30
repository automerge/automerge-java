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
