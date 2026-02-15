package org.automerge.kotlin

import kotlinx.coroutines.cancel
import kotlinx.coroutines.CoroutineScope
import kotlinx.coroutines.SupervisorJob
import kotlinx.coroutines.Dispatchers
import kotlinx.coroutines.runBlocking
import org.automerge.ObjectId
import org.automerge.repo.DocHandle
import org.automerge.repo.Repo
import org.automerge.repo.RepoConfig
import org.automerge.repo.storage.InMemoryStorage
import org.junit.jupiter.api.Assertions.*
import org.junit.jupiter.api.Test
import java.util.concurrent.TimeUnit

class DocHandleExtensionsTest {

    private fun withRepo(block: (Repo, DocHandle) -> Unit) {
        val config = RepoConfig.builder().storage(InMemoryStorage()).build()
        Repo.load(config).use { repo ->
            val handle = repo.create().get(5, TimeUnit.SECONDS)
            block(repo, handle)
        }
    }

    @Test
    fun `observe with initialValue returns initial before first change`() {
        withRepo { _, handle ->
            val scope = CoroutineScope(Dispatchers.Default + SupervisorJob())
            try {
                val flow = handle.observe(scope, "default") { doc ->
                    doc.getString(ObjectId.ROOT, "title") ?: "default"
                }
                assertEquals("default", flow.value)
            } finally {
                scope.cancel()
            }
        }
    }

    @Test
    fun `observe with initialValue updates on change`() {
        withRepo { _, handle ->
            val scope = CoroutineScope(Dispatchers.Default + SupervisorJob())
            try {
                val flow = handle.observe(scope, null as String?) { doc ->
                    doc.getString(ObjectId.ROOT, "title")
                }

                // Wait for initial async evaluation
                Thread.sleep(500)

                // Initially null since no data set yet
                assertNull(flow.value)

                // Make a change
                handle.withDocument { doc ->
                    doc.startTransaction().use { tx ->
                        tx.set(ObjectId.ROOT, "title", "Hello")
                        tx.commit()
                    }
                    null
                }.get(5, TimeUnit.SECONDS)

                // Wait for the flow to process the change
                Thread.sleep(1000)

                assertEquals("Hello", flow.value)
            } finally {
                scope.cancel()
            }
        }
    }

    @Test
    fun `suspend observe gets accurate initial value`() {
        withRepo { _, handle ->
            // Set up some data first
            handle.withDocument { doc ->
                doc.startTransaction().use { tx ->
                    tx.set(ObjectId.ROOT, "name", "Alice")
                    tx.commit()
                }
                null
            }.get(5, TimeUnit.SECONDS)

            val scope = CoroutineScope(Dispatchers.Default + SupervisorJob())
            try {
                val flow = runBlocking {
                    handle.observe(scope) { doc ->
                        doc.getString(ObjectId.ROOT, "name")
                    }
                }

                assertEquals("Alice", flow.value)
            } finally {
                scope.cancel()
            }
        }
    }

    @Test
    fun `observeString convenience`() {
        withRepo { _, handle ->
            handle.withDocument { doc ->
                doc.startTransaction().use { tx ->
                    tx.set(ObjectId.ROOT, "greeting", "hi")
                    tx.commit()
                }
                null
            }.get(5, TimeUnit.SECONDS)

            val scope = CoroutineScope(Dispatchers.Default + SupervisorJob())
            try {
                val flow = handle.observeString(scope, ObjectId.ROOT, "greeting")

                // Wait for async initial evaluation
                Thread.sleep(1000)
                assertEquals("hi", flow.value)
            } finally {
                scope.cancel()
            }
        }
    }

    @Test
    fun `observeLong convenience`() {
        withRepo { _, handle ->
            handle.withDocument { doc ->
                doc.startTransaction().use { tx ->
                    tx.set(ObjectId.ROOT, "count", 42)
                    tx.commit()
                }
                null
            }.get(5, TimeUnit.SECONDS)

            val scope = CoroutineScope(Dispatchers.Default + SupervisorJob())
            try {
                val flow = handle.observeLong(scope, ObjectId.ROOT, "count")

                // Wait for async initial evaluation
                Thread.sleep(1000)
                assertEquals(42L, flow.value)
            } finally {
                scope.cancel()
            }
        }
    }

    @Test
    fun `withDocAsync bridge`() {
        withRepo { _, handle ->
            val result = runBlocking {
                handle.withDocAsync { doc ->
                    doc.startTransaction().use { tx ->
                        tx.set(ObjectId.ROOT, "key", "val")
                        tx.commit()
                    }
                    "done"
                }
            }
            assertEquals("done", result)

            // Verify the change
            val value = runBlocking {
                handle.withDocAsync { doc ->
                    doc.getString(ObjectId.ROOT, "key")
                }
            }
            assertEquals("val", value)
        }
    }
}
