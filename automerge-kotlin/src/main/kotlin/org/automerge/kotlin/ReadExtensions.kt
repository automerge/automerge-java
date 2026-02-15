package org.automerge.kotlin

import org.automerge.AmValue
import org.automerge.ObjectId
import org.automerge.Read
import java.util.Date

// ── Scalar reads from maps ──────────────────────────────────────────

/** Get a String value from a map, or null if absent or wrong type. */
fun Read.getString(obj: ObjectId, key: String): String? =
    get(obj, key).orElse(null)?.let { (it as? AmValue.Str)?.value }

/** Get a Long value from a map (Int or UInt), or null if absent or wrong type. */
fun Read.getLong(obj: ObjectId, key: String): Long? =
    get(obj, key).orElse(null)?.let {
        when (it) {
            is AmValue.Int -> it.value
            is AmValue.UInt -> it.value
            else -> null
        }
    }

/** Get a Double value from a map, or null if absent or wrong type. */
fun Read.getDouble(obj: ObjectId, key: String): Double? =
    get(obj, key).orElse(null)?.let { (it as? AmValue.F64)?.value }

/** Get a Boolean value from a map, or null if absent or wrong type. */
fun Read.getBoolean(obj: ObjectId, key: String): Boolean? =
    get(obj, key).orElse(null)?.let { (it as? AmValue.Bool)?.value }

/** Get a ByteArray value from a map, or null if absent or wrong type. */
fun Read.getBytes(obj: ObjectId, key: String): ByteArray? =
    get(obj, key).orElse(null)?.let { (it as? AmValue.Bytes)?.value }

/** Get a Counter value from a map as Long, or null if absent or wrong type. */
fun Read.getCounter(obj: ObjectId, key: String): Long? =
    get(obj, key).orElse(null)?.let { (it as? AmValue.Counter)?.value }

/** Get a Timestamp value from a map, or null if absent or wrong type. */
fun Read.getTimestamp(obj: ObjectId, key: String): Date? =
    get(obj, key).orElse(null)?.let { (it as? AmValue.Timestamp)?.value }

// ── Scalar reads from lists ─────────────────────────────────────────

/** Get a String value from a list by index, or null if absent or wrong type. */
fun Read.getString(obj: ObjectId, idx: Long): String? =
    get(obj, idx).orElse(null)?.let { (it as? AmValue.Str)?.value }

/** Get a Long value from a list by index (Int or UInt), or null if absent or wrong type. */
fun Read.getLong(obj: ObjectId, idx: Long): Long? =
    get(obj, idx).orElse(null)?.let {
        when (it) {
            is AmValue.Int -> it.value
            is AmValue.UInt -> it.value
            else -> null
        }
    }

/** Get a Double value from a list by index, or null if absent or wrong type. */
fun Read.getDouble(obj: ObjectId, idx: Long): Double? =
    get(obj, idx).orElse(null)?.let { (it as? AmValue.F64)?.value }

/** Get a Boolean value from a list by index, or null if absent or wrong type. */
fun Read.getBoolean(obj: ObjectId, idx: Long): Boolean? =
    get(obj, idx).orElse(null)?.let { (it as? AmValue.Bool)?.value }

/** Get a ByteArray value from a list by index, or null if absent or wrong type. */
fun Read.getBytes(obj: ObjectId, idx: Long): ByteArray? =
    get(obj, idx).orElse(null)?.let { (it as? AmValue.Bytes)?.value }

// ── Object reads (return ObjectId of nested map/list/text) ──────────

/** Get the ObjectId of a nested Map from a map key, or null if absent or wrong type. */
fun Read.getMapId(obj: ObjectId, key: String): ObjectId? =
    get(obj, key).orElse(null)?.let { (it as? AmValue.Map)?.id }

/** Get the ObjectId of a nested List from a map key, or null if absent or wrong type. */
fun Read.getListId(obj: ObjectId, key: String): ObjectId? =
    get(obj, key).orElse(null)?.let { (it as? AmValue.List)?.id }

/** Get the ObjectId of a nested Text from a map key, or null if absent or wrong type. */
fun Read.getTextId(obj: ObjectId, key: String): ObjectId? =
    get(obj, key).orElse(null)?.let { (it as? AmValue.Text)?.id }
