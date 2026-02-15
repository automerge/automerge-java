package org.automerge.kotlin

import kotlinx.coroutines.CoroutineScope
import kotlinx.coroutines.launch
import kotlinx.coroutines.flow.SharingStarted
import kotlinx.coroutines.flow.StateFlow
import kotlinx.coroutines.flow.distinctUntilChanged
import kotlinx.coroutines.flow.map
import kotlinx.coroutines.flow.stateIn
import kotlinx.coroutines.future.await
import org.automerge.AmValue
import org.automerge.Document
import org.automerge.MapEntry
import org.automerge.ObjectId
import org.automerge.repo.DocHandle

// ── Core observe ────────────────────────────────────────────────────

/**
 * Observe a document query as a [StateFlow].
 *
 * This suspend function:
 * 1. Evaluates [query] once via `handle.withDocument(query)` to get the initial value.
 * 2. Collects [changeFlow] and re-evaluates [query] on each change.
 * 3. Uses [distinctUntilChanged] to suppress duplicate emissions.
 * 4. Returns a [StateFlow] scoped to [scope].
 *
 * **Error handling:** If a query re-evaluation throws, the StateFlow retains its
 * previous value (the error is swallowed). This prevents a single bad read from
 * terminating the observation.
 *
 * @param scope The [CoroutineScope] that controls the lifetime of the observation.
 * @param query A function that reads from a [Document] and returns a value of type [T].
 * @return A [StateFlow] of the query result.
 */
suspend fun <T> DocHandle.observe(
    scope: CoroutineScope,
    query: (Document) -> T,
): StateFlow<T> {
    val initial = withDocument(query).await()
    @Suppress("UNCHECKED_CAST")
    return changeFlow()
        .map {
            try {
                withDocument(query).await()
            } catch (_: Exception) {
                initial // On error, keep previous/initial value
            }
        }
        .distinctUntilChanged()
        .stateIn(scope, SharingStarted.Eagerly, initial)
}

/**
 * Observe a document query as a [StateFlow] with an explicit initial value.
 *
 * This non-suspend variant returns immediately. The first real query evaluation
 * is launched asynchronously in [scope]. Useful in `ViewModel.init {}` where
 * you want to assign a `val` without launching a coroutine.
 *
 * @param scope The [CoroutineScope] that controls the lifetime of the observation.
 * @param initialValue The value the StateFlow starts with before the first query.
 * @param query A function that reads from a [Document] and returns a value of type [T].
 * @return A [StateFlow] of the query result.
 */
fun <T> DocHandle.observe(
    scope: CoroutineScope,
    initialValue: T,
    query: (Document) -> T,
): StateFlow<T> {
    val state = kotlinx.coroutines.flow.MutableStateFlow(initialValue)

    // Launch a coroutine that:
    // 1. Evaluates the query immediately to get the current state
    // 2. Then collects changeFlow and re-evaluates on each change
    scope.launch {
        // Initial evaluation
        try {
            state.value = withDocument(query).await()
        } catch (_: Exception) {
            // Keep initialValue on error
        }
        // Ongoing evaluation on changes
        changeFlow().collect {
            try {
                val newValue = withDocument(query).await()
                state.value = newValue
            } catch (_: Exception) {
                // Keep previous value on error
            }
        }
    }

    return state
}

// ── Convenience extensions ──────────────────────────────────────────

/** Observe a String property at [key] in map [obj]. */
fun DocHandle.observeString(
    scope: CoroutineScope, obj: ObjectId, key: String,
): StateFlow<String?> = observe(scope, null) { doc -> doc.getString(obj, key) }

/** Observe a Long property at [key] in map [obj]. */
fun DocHandle.observeLong(
    scope: CoroutineScope, obj: ObjectId, key: String,
): StateFlow<Long?> = observe(scope, null) { doc -> doc.getLong(obj, key) }

/** Observe a Double property at [key] in map [obj]. */
fun DocHandle.observeDouble(
    scope: CoroutineScope, obj: ObjectId, key: String,
): StateFlow<Double?> = observe(scope, null) { doc -> doc.getDouble(obj, key) }

/** Observe a Boolean property at [key] in map [obj]. */
fun DocHandle.observeBoolean(
    scope: CoroutineScope, obj: ObjectId, key: String,
): StateFlow<Boolean?> = observe(scope, null) { doc -> doc.getBoolean(obj, key) }

/** Observe the text content of a Text object. */
fun DocHandle.observeText(
    scope: CoroutineScope, obj: ObjectId,
): StateFlow<String?> = observe(scope, null) { doc -> doc.text(obj).orElse(null) }

/** Observe the entries of a Map object. */
fun DocHandle.observeMapEntries(
    scope: CoroutineScope, obj: ObjectId,
): StateFlow<List<MapEntry>> = observe(scope, emptyList()) { doc ->
    doc.mapEntries(obj).orElse(null)?.toList() ?: emptyList()
}

/** Observe the items of a List object. */
fun DocHandle.observeListItems(
    scope: CoroutineScope, obj: ObjectId,
): StateFlow<List<AmValue>> = observe(scope, emptyList()) { doc ->
    doc.listItems(obj).orElse(null)?.toList() ?: emptyList()
}

/** Suspend-friendly wrapper around [DocHandle.withDocument]. */
suspend fun <T> DocHandle.withDocAsync(mutation: (Document) -> T): T =
    withDocument(mutation).await()
