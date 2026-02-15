package org.automerge.kotlin

import kotlinx.coroutines.future.await
import org.automerge.repo.AutomergeUrl
import org.automerge.repo.DocHandle
import org.automerge.repo.DocumentId
import org.automerge.repo.Repo

/** Suspend-friendly [Repo.find] that returns `null` instead of [java.util.Optional.empty]. */
suspend fun Repo.findDocument(documentId: DocumentId): DocHandle? =
    find(documentId).await().orElse(null)

/** Suspend-friendly [Repo.find] that returns `null` instead of [java.util.Optional.empty]. */
suspend fun Repo.findDocument(url: AutomergeUrl): DocHandle? =
    find(url).await().orElse(null)
