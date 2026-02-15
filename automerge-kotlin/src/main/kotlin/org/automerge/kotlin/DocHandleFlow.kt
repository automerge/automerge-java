package org.automerge.kotlin

import kotlinx.coroutines.channels.awaitClose
import kotlinx.coroutines.flow.Flow
import kotlinx.coroutines.flow.callbackFlow
import org.automerge.repo.DocHandle
import org.automerge.repo.DocumentChanged

/**
 * Creates a [Flow] that emits [DocumentChanged] events whenever this document changes.
 *
 * The flow is backed by a change listener registered on this [DocHandle].
 * The listener is automatically removed when the flow collector is cancelled.
 *
 * Events fire for any change reason: local mutation, remote sync, or merge.
 */
fun DocHandle.changeFlow(): Flow<DocumentChanged> = callbackFlow {
    val registration = addChangeListener { event ->
        trySend(event)
    }
    awaitClose { registration.remove() }
}
