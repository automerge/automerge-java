package org.automerge.kotlin

import kotlinx.coroutines.Dispatchers
import kotlinx.coroutines.flow.first
import kotlinx.coroutines.flow.take
import kotlinx.coroutines.flow.toList
import kotlinx.coroutines.launch
import kotlinx.coroutines.runBlocking
import org.automerge.ObjectId
import org.automerge.repo.DocHandle
import org.automerge.repo.Repo
import org.automerge.repo.RepoConfig
import org.automerge.repo.storage.InMemoryStorage
import org.junit.jupiter.api.Assertions.*
import org.junit.jupiter.api.Test
import java.util.concurrent.TimeUnit

class DocHandleFlowTest {

    private fun withRepo(block: (Repo, DocHandle) -> Unit) {
        val config = RepoConfig.builder().storage(InMemoryStorage()).build()
        Repo.load(config).use { repo ->
            val handle = repo.create().get(5, TimeUnit.SECONDS)
            block(repo, handle)
        }
    }

    @Test
    fun `changeFlow emits event on document change`() {
        withRepo { _, handle ->
            runBlocking {
                val job = launch(Dispatchers.Default) {
                    val event = handle.changeFlow().first()
                    assertNotNull(event)
                    assertNotNull(event.newHeads)
                    assertTrue(event.newHeads.isNotEmpty())
                }

                // Small delay to ensure the flow collector starts and registers the listener
                Thread.sleep(100)

                // Make a change to trigger the flow
                handle.withDocument { doc ->
                    doc.startTransaction().use { tx ->
                        tx.set(ObjectId.ROOT, "key", "value")
                        tx.commit()
                    }
                    null
                }.get(5, TimeUnit.SECONDS)

                job.join()
            }
        }
    }

    @Test
    fun `changeFlow emits multiple events`() {
        withRepo { _, handle ->
            runBlocking {
                val job = launch(Dispatchers.Default) {
                    val events = handle.changeFlow().take(3).toList()
                    assertEquals(3, events.size)
                }

                // Small delay to ensure the flow collector starts
                Thread.sleep(100)

                repeat(3) { i ->
                    handle.withDocument { doc ->
                        doc.startTransaction().use { tx ->
                            tx.set(ObjectId.ROOT, "key$i", "value$i")
                            tx.commit()
                        }
                        null
                    }.get(5, TimeUnit.SECONDS)
                }

                job.join()
            }
        }
    }
}
