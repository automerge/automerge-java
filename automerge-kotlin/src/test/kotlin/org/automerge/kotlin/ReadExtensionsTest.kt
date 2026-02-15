package org.automerge.kotlin

import org.automerge.Document
import org.automerge.ObjectId
import org.automerge.ObjectType
import org.junit.jupiter.api.Assertions.*
import org.junit.jupiter.api.Test

class ReadExtensionsTest {

    @Test
    fun `getString returns string value from map`() {
        val doc = Document()
        doc.startTransaction().use { tx ->
            tx.set(ObjectId.ROOT, "title", "Hello World")
            tx.commit()
        }
        assertEquals("Hello World", doc.getString(ObjectId.ROOT, "title"))
    }

    @Test
    fun `getString returns null for missing key`() {
        val doc = Document()
        assertNull(doc.getString(ObjectId.ROOT, "missing"))
    }

    @Test
    fun `getString returns null for wrong type`() {
        val doc = Document()
        doc.startTransaction().use { tx ->
            tx.set(ObjectId.ROOT, "count", 42)
            tx.commit()
        }
        assertNull(doc.getString(ObjectId.ROOT, "count"))
    }

    @Test
    fun `getLong returns long value from map`() {
        val doc = Document()
        doc.startTransaction().use { tx ->
            tx.set(ObjectId.ROOT, "count", 42)
            tx.commit()
        }
        assertEquals(42L, doc.getLong(ObjectId.ROOT, "count"))
    }

    @Test
    fun `getLong returns null for missing key`() {
        val doc = Document()
        assertNull(doc.getLong(ObjectId.ROOT, "missing"))
    }

    @Test
    fun `getDouble returns double value from map`() {
        val doc = Document()
        doc.startTransaction().use { tx ->
            tx.set(ObjectId.ROOT, "pi", 3.14)
            tx.commit()
        }
        assertEquals(3.14, doc.getDouble(ObjectId.ROOT, "pi"))
    }

    @Test
    fun `getBoolean returns boolean value from map`() {
        val doc = Document()
        doc.startTransaction().use { tx ->
            tx.set(ObjectId.ROOT, "flag", true)
            tx.commit()
        }
        assertEquals(true, doc.getBoolean(ObjectId.ROOT, "flag"))
    }

    @Test
    fun `getBytes returns byte array from map`() {
        val doc = Document()
        val data = byteArrayOf(1, 2, 3, 4)
        doc.startTransaction().use { tx ->
            tx.set(ObjectId.ROOT, "data", data)
            tx.commit()
        }
        val result = doc.getBytes(ObjectId.ROOT, "data")
        assertNotNull(result)
        assertArrayEquals(data, result)
    }

    @Test
    fun `getMapId returns ObjectId for nested map`() {
        val doc = Document()
        var nestedId: ObjectId? = null
        doc.startTransaction().use { tx ->
            nestedId = tx.set(ObjectId.ROOT, "nested", ObjectType.MAP)
            tx.commit()
        }
        val result = doc.getMapId(ObjectId.ROOT, "nested")
        assertNotNull(result)
        assertEquals(nestedId, result)
    }

    @Test
    fun `getListId returns ObjectId for nested list`() {
        val doc = Document()
        var listId: ObjectId? = null
        doc.startTransaction().use { tx ->
            listId = tx.set(ObjectId.ROOT, "items", ObjectType.LIST)
            tx.commit()
        }
        val result = doc.getListId(ObjectId.ROOT, "items")
        assertNotNull(result)
        assertEquals(listId, result)
    }

    @Test
    fun `getTextId returns ObjectId for nested text`() {
        val doc = Document()
        var textId: ObjectId? = null
        doc.startTransaction().use { tx ->
            textId = tx.set(ObjectId.ROOT, "content", ObjectType.TEXT)
            tx.commit()
        }
        val result = doc.getTextId(ObjectId.ROOT, "content")
        assertNotNull(result)
        assertEquals(textId, result)
    }

    @Test
    fun `getString from list by index`() {
        val doc = Document()
        doc.startTransaction().use { tx ->
            val listId = tx.set(ObjectId.ROOT, "items", ObjectType.LIST)
            tx.insert(listId, 0, "first")
            tx.insert(listId, 1, "second")
            tx.commit()
        }
        val listId = doc.getListId(ObjectId.ROOT, "items")!!
        assertEquals("first", doc.getString(listId, 0L))
        assertEquals("second", doc.getString(listId, 1L))
    }

    @Test
    fun `getLong from list by index`() {
        val doc = Document()
        doc.startTransaction().use { tx ->
            val listId = tx.set(ObjectId.ROOT, "numbers", ObjectType.LIST)
            tx.insert(listId, 0, 10)
            tx.insert(listId, 1, 20)
            tx.commit()
        }
        val listId = doc.getListId(ObjectId.ROOT, "numbers")!!
        assertEquals(10L, doc.getLong(listId, 0L))
        assertEquals(20L, doc.getLong(listId, 1L))
    }

    @Test
    fun `getMapId returns null for non-map value`() {
        val doc = Document()
        doc.startTransaction().use { tx ->
            tx.set(ObjectId.ROOT, "name", "Alice")
            tx.commit()
        }
        assertNull(doc.getMapId(ObjectId.ROOT, "name"))
    }
}
