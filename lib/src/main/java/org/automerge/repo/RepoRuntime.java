package org.automerge.repo;

import java.util.Map;
import java.util.Optional;
import java.util.concurrent.CompletableFuture;
import java.util.concurrent.ConcurrentHashMap;
import java.util.concurrent.CopyOnWriteArrayList;
import java.util.concurrent.ExecutorService;
import java.util.concurrent.Executors;
import java.util.concurrent.ScheduledExecutorService;
import java.util.concurrent.ScheduledFuture;
import java.util.concurrent.TimeUnit;
import java.util.concurrent.atomic.AtomicBoolean;
import org.automerge.LoadLibrary;
import org.slf4j.Logger;
import org.slf4j.LoggerFactory;

/**
 * The RepoRuntime is the execution engine for a Repo instance.
 *
 * It manages the lifecycle of the Hub and DocumentActors, routes messages
 * between them, executes IO operations, and tracks pending commands.
 *
 * Thread Model: - Hub events are processed sequentially on a single-threaded
 * hubExecutor - DocumentActor operations can run in parallel on the
 * multi-threaded documentExecutor - IO operations (storage, network) run on the
 * multi-threaded ioExecutor
 */
class RepoRuntime {
    static {
        LoadLibrary.initialize();
    }

    // Core components
    private final RepoSys.HubPointer hubPointer;
    private final RepoConfig config;

    // Executors - created and owned by RepoRuntime
    private final ExecutorService hubExecutor;
    private final ExecutorService documentExecutor;
    private final SerializingExecutor<DocumentActorId> serializingDocumentExecutor;
    private final ExecutorService ioExecutor;
    private final ScheduledExecutorService tickScheduler;

    private static final Logger log = LoggerFactory.getLogger(RepoRuntime.class);

    private final AtomicBoolean stopped;
    private final AtomicBoolean closing;

    // Actor registry - only modified by hub thread, can be read from any thread
    private final ConcurrentHashMap<DocumentActorId, DocumentActor> documentActors;

    // Command tracking - only accessed by hub thread
    private final ConcurrentHashMap<CommandId, CompletableFuture<?>> pendingCommands;

    // Transport storage
    private final ConcurrentHashMap<ConnectionId, Transport> transports;

    // Dialer registry: maps DialerId -> user's Dialer implementation
    private final ConcurrentHashMap<DialerId, Dialer> dialers;

    // Dialer handles: maps DialerId -> DialerHandle (for notifying lifecycle
    // events)
    private final ConcurrentHashMap<DialerId, DialerHandle> dialerHandles;

    // Acceptor handles: maps ListenerId -> AcceptorHandle
    private final ConcurrentHashMap<ListenerId, AcceptorHandle> acceptorHandles;

    // Change listeners - keyed by document actor ID
    private final ConcurrentHashMap<DocumentActorId, CopyOnWriteArrayList<ChangeListener>> changeListeners;

    // Tick event scheduling
    private ScheduledFuture<?> tickFuture;

    /**
     * Creates a RepoRuntime with the given hub pointer and configuration.
     *
     * @param hubPointer
     *                   The native hub pointer
     * @param config
     *                   The repository configuration
     */
    RepoRuntime(RepoSys.HubPointer hubPointer, RepoConfig config) {
        this.hubPointer = hubPointer;
        this.config = config;

        // Create executors
        this.hubExecutor = Executors.newSingleThreadExecutor(r -> {
            Thread t = new Thread(r, "automerge-hub");
            t.setDaemon(true);
            return t;
        });
        // Use work-stealing pool for document operations (CPU-bound)
        // Wrapped in SerializingExecutor to ensure tasks for the same document don't
        // run concurrently
        this.documentExecutor = Executors.newWorkStealingPool();
        this.serializingDocumentExecutor = new SerializingExecutor<>(documentExecutor);
        this.ioExecutor = Executors.newCachedThreadPool(r -> {
            Thread t = new Thread(r, "automerge-io");
            t.setDaemon(true);
            return t;
        });
        this.tickScheduler = Executors.newSingleThreadScheduledExecutor(r -> {
            Thread t = new Thread(r, "automerge-tick");
            t.setDaemon(true);
            return t;
        });

        this.stopped = new AtomicBoolean(false);
        this.closing = new AtomicBoolean(false);

        // Initialize registries
        this.documentActors = new ConcurrentHashMap<>();
        this.pendingCommands = new ConcurrentHashMap<>();
        this.transports = new ConcurrentHashMap<>();
        this.dialers = new ConcurrentHashMap<>();
        this.dialerHandles = new ConcurrentHashMap<>();
        this.acceptorHandles = new ConcurrentHashMap<>();
        this.changeListeners = new ConcurrentHashMap<>();
        this.tickFuture = null;
    }

    /**
     * Starts the runtime, beginning event processing.
     */
    void start() {
        // Schedule periodic tick events (every 100ms)
        tickFuture = tickScheduler.scheduleAtFixedRate(() -> {
            long now = System.currentTimeMillis();
            submitToHub(() -> RepoSys.hubHandleEventTick(hubPointer, now));
        }, 100, 100, TimeUnit.MILLISECONDS);
    }

    /**
     * Submits a fire-and-forget hub event to the hub executor. The supplier is
     * called on the hub thread; its HubResults are processed automatically.
     */
    private void submitToHub(java.util.function.Supplier<HubResults> eventOp) {
        if (stopped.get()) {
            log.trace("Ignoring hub task submitted after shutdown");
            return;
        }
        try {
            hubExecutor.submit(() -> {
                try {
                    processHubResults(eventOp.get());
                    cleanupStoppedActors();
                } catch (Exception e) {
                    log.error("Error in hub task", e);
                }
            });
        } catch (java.util.concurrent.RejectedExecutionException e) {
            if (!stopped.get())
                throw e;
        }
    }

    /**
     * Dispatches a command to the hub and returns a future for tracking completion.
     * The commandOp supplier is called on the hub executor thread; it must return a
     * HubCommandResult so the CommandId and HubResults are atomically available.
     */
    private <T extends CommandResult> CompletableFuture<T> dispatchHubCommand(
            java.util.function.Supplier<HubCommandResult> commandOp) {
        return dispatchHubCommand(commandOp, null);
    }

    /**
     * Dispatches a command to the hub, registering a hook that runs on the hub
     * thread inside processCommandCompletions — after the command result is known
     * but BEFORE the returned future completes and BEFORE later processors in the
     * same HubResults batch run. Use this when the command returns an id that
     * must be installed into a lookup map (dialers, acceptorHandles, transports)
     * so that in-batch events referencing the new id can find it.
     */
    @SuppressWarnings("unchecked")
    private <T extends CommandResult> CompletableFuture<T> dispatchHubCommand(
            java.util.function.Supplier<HubCommandResult> commandOp,
            java.util.function.Consumer<T> onHubThreadCompletion) {
        if (stopped.get()) {
            throw new IllegalStateException("Runtime is stopped");
        }
        CompletableFuture<T> future = new CompletableFuture<>();
        try {
            hubExecutor.submit(() -> {
                try {
                    HubCommandResult result = commandOp.get();
                    CommandId commandId = result.getCommandId();
                    pendingCommands.put(commandId, future);
                    java.util.function.Consumer<CommandResult> hook = onHubThreadCompletion == null ? null
                            : (java.util.function.Consumer<CommandResult>) (java.util.function.Consumer<?>) onHubThreadCompletion;
                    processHubResults(result.getResults(), commandId, hook);
                    cleanupStoppedActors();
                } catch (Exception e) {
                    log.error("Error in hub command", e);
                    future.completeExceptionally(e);
                }
            });
        } catch (java.util.concurrent.RejectedExecutionException e) {
            if (!stopped.get())
                throw e;
            future.completeExceptionally(new IllegalStateException("Runtime stopped"));
        }
        return future;
    }

    /**
     * Submits a task to the IO executor. During shutdown, rejections are
     * silently ignored. Unexpected rejections are propagated.
     */
    private void submitToIo(Runnable task) {
        try {
            ioExecutor.submit(task);
        } catch (java.util.concurrent.RejectedExecutionException e) {
            if (!stopped.get()) {
                throw e;
            }
        }
    }

    /**
     * Submits a task to the serializing document executor. During shutdown,
     * rejections are silently ignored. Unexpected rejections are propagated.
     */
    private void submitToDocExecutor(DocumentActorId actorId, Runnable task) {
        try {
            serializingDocumentExecutor.execute(actorId, task);
        } catch (java.util.concurrent.RejectedExecutionException e) {
            if (!stopped.get()) {
                throw e;
            }
        }
    }

    /**
     * Stops the runtime gracefully.
     */
    void stop() {
        if (!stopped.getAndSet(true)) {
            // Cancel tick events
            if (tickFuture != null) {
                tickFuture.cancel(false);
                tickFuture = null;
            }

            long now = System.currentTimeMillis();
            try {
                hubExecutor.submit(() -> {
                    try {
                        HubResults results = RepoSys.hubHandleEventStop(hubPointer, now);
                        processHubResults(results);
                    } catch (Exception e) {
                        log.error("Error submitting stop event", e);
                    }
                });
            } catch (java.util.concurrent.RejectedExecutionException e) {
                // already shut down
            }
        }
    }

    /**
     * Closes the runtime and frees all resources.
     */
    void close() {
        // Prevent re-entrant close calls
        if (!closing.compareAndSet(false, true)) {
            log.debug("Close already in progress, skipping");
            return;
        }

        // Step 0: Close all dialer and acceptor handles
        for (DialerHandle handle : dialerHandles.values()) {
            handle.close();
        }
        for (AcceptorHandle handle : acceptorHandles.values()) {
            handle.close();
        }

        // Step 1: Submit stop event to trigger samod-core shutdown
        stop();

        // Shutdown hub executor and wait for event loop to complete
        hubExecutor.shutdown();
        try {
            if (!hubExecutor.awaitTermination(10, TimeUnit.SECONDS)) {
                log.error("Hub executor did not terminate gracefully, forcing shutdown");
                hubExecutor.shutdownNow();
            }
        } catch (InterruptedException e) {
            Thread.currentThread().interrupt();
            hubExecutor.shutdownNow();
        }

        // Step 2: Now that hub has finished, shutdown other executors
        documentExecutor.shutdown();
        ioExecutor.shutdown();
        tickScheduler.shutdown();

        try {
            boolean docDone = documentExecutor.awaitTermination(10, TimeUnit.SECONDS);
            boolean ioDone = ioExecutor.awaitTermination(10, TimeUnit.SECONDS);
            boolean tickDone = tickScheduler.awaitTermination(10, TimeUnit.SECONDS);

            if (!docDone || !ioDone || !tickDone) {
                log.error("Some executors did not terminate gracefully, forcing shutdown");
                documentExecutor.shutdownNow();
                ioExecutor.shutdownNow();
                tickScheduler.shutdownNow();
            }
        } catch (InterruptedException e) {
            Thread.currentThread().interrupt();
            log.error("Interrupted while waiting for executors to terminate");
        }

        // Step 3: Free native resources
        for (Map.Entry<DocumentActorId, DocumentActor> entry : documentActors.entrySet()) {
            entry.getValue().free();
            serializingDocumentExecutor.cleanup(entry.getKey());
        }
        documentActors.clear();

        // Close all transports
        for (Map.Entry<ConnectionId, Transport> entry : transports.entrySet()) {
            entry.getValue().close();
        }
        transports.clear();

        // Free hub
        RepoSys.freeHub(hubPointer);

        // Complete any pending commands with error
        for (CompletableFuture<?> future : pendingCommands.values()) {
            future.completeExceptionally(new IllegalStateException("Runtime closed"));
        }
        pendingCommands.clear();
    }

    /**
     * Get the peer ID for this repo runtime.
     */
    public PeerId getPeerId() {
        return config.getPeerId();
    }

    /**
     * Processes the results from hub.handleEvent*().
     */
    private void processHubResults(HubResults results) {
        processHubResults(results, null, null);
    }

    /**
     * Processes the results from a hub command, running an optional hub-thread
     * hook against the command's own result before anything else. The hook is
     * the mechanism for installing handle-returning command ids (DialerId,
     * ListenerId, ConnectionId) into lookup maps so later processors in the
     * same batch — which may reference those ids — see them.
     */
    private void processHubResults(HubResults results, CommandId hookCommandId,
            java.util.function.Consumer<CommandResult> hook) {
        processCommandCompletions(results.getCompletedCommands(), hookCommandId, hook);
        processActorSpawns(results.getSpawnActors());
        processActorMessages(results.getActorMessages());
        processNetworkTasks(results.getNewTasks());
        processConnectionEvents(results.getConnectionEvents());
        processDialRequests(results.getDialRequests());
        processDialerEvents(results.getDialerEvents());
    }

    /**
     * Processes messages to document actors.
     */
    private void processActorMessages(java.util.List<ActorMessage> messages) {
        long now = System.currentTimeMillis();

        for (ActorMessage message : messages) {
            DocumentActorId actorId = message.getActorId();
            HubToDocMsg msg = message.getMessage();

            DocumentActor actor = documentActors.get(actorId);

            if (actor == null) {
                log.error("Received message for unknown actor: " + actorId);
                continue;
            }

            submitToDocExecutor(actorId, () -> {
                try {
                    DocActorResult result = RepoSys.documentActorHandleMsg(actor.getPointer(), now, msg);
                    processDocActorResult(actorId, result);
                } catch (Exception e) {
                    log.error("Error processing actor message for {}: {}", actorId, e.getMessage(), e);
                }
            });
        }
    }

    /**
     * Processes the result from a DocumentActor operation.
     */
    private void processDocActorResult(DocumentActorId actorId, DocActorResult result) {
        // Route outgoing messages back to hub
        for (DocToHubMsg msg : result.getOutgoingMessages()) {
            final DocToHubMsg capturedMsg = msg;
            final long now = System.currentTimeMillis();
            submitToHub(() -> RepoSys.hubHandleEventActorMessage(hubPointer, now, actorId, capturedMsg));
        }

        // Execute IO tasks
        for (IoTask<DocumentIoTask> ioTask : result.getIoTasks()) {
            DocumentIoTask task = ioTask.getAction();

            if (task instanceof DocumentIoTask.Storage) {
                DocumentIoTask.Storage storageTaskWrapper = (DocumentIoTask.Storage) task;
                StorageTask storageTask = storageTaskWrapper.getStorageTask();

                submitToIo(() -> {
                    try {
                        StorageResult storageResult = executeStorageOperation(storageTask);
                        IoResult<DocumentIoResult> ioResult = new IoResult<>(ioTask.getTaskId(),
                                new DocumentIoResult.Storage(storageResult));
                        routeIoResultToActor(actorId, ioResult);
                    } catch (Exception e) {
                        log.error("Storage task failed: ", e);
                    }
                });
            } else if (task instanceof DocumentIoTask.CheckAnnouncePolicy) {
                DocumentIoTask.CheckAnnouncePolicy checkPolicy = (DocumentIoTask.CheckAnnouncePolicy) task;
                PeerId peerId = checkPolicy.getPeerId();

                DocumentActor actor = this.documentActors.get(actorId);
                if (actor == null) {
                    throw new IllegalStateException("Document actor not found");
                }
                DocumentId docId = actor.getDocumentId();

                IoTaskId taskId = ioTask.getTaskId();
                config.getAnnouncePolicy().shouldAnnounce(docId, peerId).whenComplete((shouldAnnounce, error) -> {
                    if (error != null) {
                        log.error("Announce policy check failed: {}", error.getMessage(), error);
                        return;
                    }

                    IoResult<DocumentIoResult> ioResult = new IoResult<>(taskId,
                            new DocumentIoResult.CheckAnnouncePolicy(shouldAnnounce));
                    routeIoResultToActor(actorId, ioResult);
                });
            }
        }

        // Notify change listeners
        for (DocumentChanged changeEvent : result.getChangeEvents()) {
            notifyChangeListeners(actorId, changeEvent);
        }
    }

    /**
     * Routes an IO result back to a document actor.
     */
    private void routeIoResultToActor(DocumentActorId actorId, IoResult<DocumentIoResult> ioResult) {
        DocumentActor actor = documentActors.get(actorId);

        if (actor == null) {
            log.error("Cannot route IO result to unknown actor: " + actorId);
            return;
        }

        submitToDocExecutor(actorId, () -> {
            try {
                long now = System.currentTimeMillis();
                DocActorResult result = RepoSys.documentActorHandleIoComplete(actor.getPointer(), now, ioResult);
                processDocActorResult(actorId, result);
            } catch (Exception e) {
                log.error("Error handling IO result for actor {}: {}", actorId, e.getMessage(), e);
            }
        });
    }

    /**
     * Processes actor spawn requests from the hub. Actors are already fully
     * constructed by the Rust layer; no additional JNI spawning is needed.
     */
    private void processActorSpawns(java.util.List<SpawnedActor> spawnedActors) {
        for (SpawnedActor spawned : spawnedActors) {
            try {
                DocumentActor actor = spawned.getActor();
                DocumentActorId actorId = actor.getActorId();
                DocActorResult initialResult = spawned.getInitialResult();

                DocumentActor existing = documentActors.putIfAbsent(actorId, actor);

                if (existing != null) {
                    actor.free();
                    throw new IllegalStateException("Attempted to spawn actor with duplicate ID: " + actorId);
                }

                processDocActorResult(actorId, initialResult);
            } catch (Exception e) {
                log.error("Failed to spawn document actor: ", e);
            } catch (Throwable t) {
                log.error("Throwable in spawn document actor: {}", t.getMessage(), t);
            }
        }
    }

    /**
     * Processes network IO tasks from the hub.
     */
    private void processNetworkTasks(java.util.List<IoTask<HubIoAction>> tasks) {
        for (IoTask<HubIoAction> task : tasks) {
            HubIoAction action = task.getAction();
            IoTaskId taskId = task.getTaskId();

            if (action instanceof HubIoAction.Send) {
                HubIoAction.Send send = (HubIoAction.Send) action;
                ConnectionId connId = send.getConnectionId();
                byte[] message = send.getMessage();

                submitToIo(() -> {
                    Transport transport = transports.get(connId);
                    if (transport == null) {
                        log.error("Cannot send on unknown connection: {}", connId);
                        return;
                    }

                    transport.send(message).thenAccept(v -> {
                        IoResult<HubIoResult> ioResult = new IoResult<>(taskId, HubIoResult.SEND);
                        long now = System.currentTimeMillis();
                        submitToHub(() -> RepoSys.hubHandleEventIoComplete(hubPointer, now, ioResult));
                    }).exceptionally(err -> {
                        log.error("Send failed on connection {}: {}", connId, err.getMessage());

                        long now = System.currentTimeMillis();
                        submitToHub(() -> RepoSys.hubHandleEventConnectionLost(hubPointer, now, connId));

                        IoResult<HubIoResult> ioResult = new IoResult<>(taskId, HubIoResult.SEND);
                        submitToHub(() -> RepoSys.hubHandleEventIoComplete(hubPointer, now, ioResult));

                        Transport failedTransport = transports.remove(connId);
                        if (failedTransport != null) {
                            failedTransport.close();
                        }

                        return null;
                    });
                });
            } else if (action instanceof HubIoAction.Disconnect) {
                HubIoAction.Disconnect disconnect = (HubIoAction.Disconnect) action;
                ConnectionId connId = disconnect.getConnectionId();

                submitToIo(() -> {
                    Transport transport = transports.remove(connId);
                    if (transport != null) {
                        transport.close();
                    }

                    IoResult<HubIoResult> ioResult = new IoResult<>(taskId, HubIoResult.DISCONNECT);
                    long now = System.currentTimeMillis();
                    submitToHub(() -> RepoSys.hubHandleEventIoComplete(hubPointer, now, ioResult));
                });
            }
        }
    }

    /**
     * Removes and frees any stopped actors from the registry.
     */
    private void cleanupStoppedActors() {
        documentActors.entrySet().removeIf(entry -> {
            DocumentActorId actorId = entry.getKey();
            DocumentActor actor = entry.getValue();
            if (actor.isStopped()) {
                actor.free();
                serializingDocumentExecutor.cleanup(actorId);
                return true;
            }
            return false;
        });
    }

    /**
     * Processes completed commands and completes their futures. If the hook is
     * non-null, it runs against the matching command's result BEFORE any future
     * is completed and before any later processor in this HubResults batch runs
     * — this is how handle-returning command ids are installed into lookup maps
     * atomically with respect to in-batch events that reference them.
     */
    @SuppressWarnings("unchecked")
    private void processCommandCompletions(Map<CommandId, CommandResult> completedCommands, CommandId hookCommandId,
            java.util.function.Consumer<CommandResult> hook) {
        if (hook != null) {
            CommandResult hookResult = completedCommands.get(hookCommandId);
            if (hookResult == null) {
                throw new IllegalStateException("Hub-thread completion hook registered for " + hookCommandId
                        + " but that command did not complete in its issuing batch");
            }
            hook.accept(hookResult);
        }

        for (Map.Entry<CommandId, CommandResult> entry : completedCommands.entrySet()) {
            CommandId commandId = entry.getKey();
            CommandResult result = entry.getValue();

            CompletableFuture<?> future = pendingCommands.remove(commandId);

            if (future != null) {
                ((CompletableFuture<Object>) future).complete(result);
            } else {
                if (!stopped.get()) {
                    log.warn("No pending future for command", "commandId", commandId);
                } else {
                    log.trace("Command completed during shutdown (future already cancelled): {}", commandId);
                }
            }
        }
    }

    /**
     * Processes dial requests from the hub. When the hub emits DialRequests, the
     * runtime calls the registered Dialer.connect() which returns a future.
     */
    private void processDialRequests(java.util.List<DialRequest> requests) {
        for (DialRequest request : requests) {
            DialerId dialerId = request.getDialerId();
            Dialer dialer = dialers.get(dialerId);

            if (dialer == null) {
                // Dialer removed before request was processed
                long now = System.currentTimeMillis();
                submitToHub(() -> RepoSys.hubHandleEventDialFailed(hubPointer, now, dialerId, "No dialer registered"));
                continue;
            }

            submitToIo(() -> {
                dialer.connect().whenComplete((transport, connectError) -> {
                    if (connectError != null) {
                        if (!stopped.get()) {
                            Throwable cause = unwrapCompletionException(connectError);
                            String msg = cause.getMessage() != null ? cause.getMessage() : "Connection failed";
                            long now = System.currentTimeMillis();
                            submitToHub(() -> RepoSys.hubHandleEventDialFailed(hubPointer, now, dialerId, msg));
                        }
                        return;
                    }

                    try {
                        CompletableFuture<CommandResult.CreateConnection> future = dispatchHubCommand(
                                () -> RepoSys.hubHandleEventCreateDialerConnection(hubPointer,
                                        System.currentTimeMillis(), dialerId),
                                (CommandResult.CreateConnection result) -> driveConnection(result.getConnectionId(),
                                        transport));

                        future.exceptionally(e -> {
                            // dispatchHubCommand future failed (e.g. runtime stopped)
                            transport.close();
                            return null;
                        });
                    } catch (Exception e) {
                        // dispatchHubCommand threw (e.g. runtime stopped)
                        transport.close();
                    }
                });
            });
        }
    }

    private static Throwable unwrapCompletionException(Throwable e) {
        return (e instanceof java.util.concurrent.CompletionException && e.getCause() != null)
                ? e.getCause()
                : e;
    }

    /**
     * Processes dialer lifecycle events from the hub.
     */
    private void processDialerEvents(java.util.List<DialerEvent> events) {
        for (DialerEvent event : events) {
            if (event instanceof DialerEvent.MaxRetriesReached) {
                DialerEvent.MaxRetriesReached maxRetries = (DialerEvent.MaxRetriesReached) event;
                DialerHandle handle = dialerHandles.get(maxRetries.getDialerId());
                if (handle != null) {
                    handle.notifyMaxRetriesReached();
                }
            }
        }
    }

    /**
     * Processes connection lifecycle events from the hub. Routes events to the
     * appropriate DialerHandle or AcceptorHandle based on ConnectionOwner.
     */
    private void processConnectionEvents(java.util.List<ConnectionEvent> events) {
        for (ConnectionEvent event : events) {
            ConnectionOwner owner = event.getOwner();

            if (event instanceof ConnectionEvent.HandshakeCompleted) {
                PeerId peerId = ((ConnectionEvent.HandshakeCompleted) event).getPeerInfo().getPeerId();

                if (owner instanceof ConnectionOwner.DialerOwner) {
                    DialerId did = ((ConnectionOwner.DialerOwner) owner).getDialerId();
                    DialerHandle handle = dialerHandles.get(did);
                    if (handle != null) {
                        handle.notifyEstablished(peerId, event.getConnectionId());
                    }
                } else if (owner instanceof ConnectionOwner.ListenerOwner) {
                    // Listener connections don't have handles to notify for now
                }
            } else if (event instanceof ConnectionEvent.ConnectionFailed) {
                if (owner instanceof ConnectionOwner.DialerOwner) {
                    DialerId did = ((ConnectionOwner.DialerOwner) owner).getDialerId();
                    DialerHandle handle = dialerHandles.get(did);
                    if (handle != null) {
                        handle.notifyDisconnected();
                    }
                }

                // Clean up transport
                ConnectionId connId = event.getConnectionId();
                Transport transport = transports.remove(connId);
                if (transport != null) {
                    transport.close();
                }
            }
        }
    }

    /**
     * Central method that activates a transport. Used by both dialer (after
     * successful Dialer.connect()) and acceptor (after accept()). Calls the
     * package-private Transport.connect() to flush any buffered messages and start
     * direct delivery.
     */
    void driveConnection(ConnectionId connId, Transport transport) {
        transports.put(connId, transport);

        transport.connect(
                // onMessage
                msg -> {
                    if (stopped.get()) {
                        return;
                    }
                    try {
                        CompletableFuture<?> future = dispatchHubCommand(
                                () -> RepoSys.hubHandleEventReceive(hubPointer, System.currentTimeMillis(), connId,
                                        msg));
                        // Prevent unhandled exception warnings
                        future.exceptionally(e -> null);
                    } catch (IllegalStateException e) {
                        // Runtime stopped between check and dispatch - ignore
                    }
                },
                // onClose
                () -> {
                    transports.remove(connId);
                    long now = System.currentTimeMillis();
                    submitToHub(() -> RepoSys.hubHandleEventConnectionLost(hubPointer, now, connId));
                });
    }

    /**
     * Executes a storage operation and returns the result.
     */
    private StorageResult executeStorageOperation(StorageTask task) {
        Storage storage = config.getStorage();

        if (task instanceof StorageTask.Load) {
            StorageTask.Load load = (StorageTask.Load) task;
            try {
                Optional<byte[]> value = storage.load(load.getKey()).get();
                return new StorageResult.Load(value.orElse(null));
            } catch (Exception e) {
                throw new RuntimeException("Load failed", e);
            }
        } else if (task instanceof StorageTask.LoadRange) {
            StorageTask.LoadRange loadRange = (StorageTask.LoadRange) task;
            try {
                Map<StorageKey, byte[]> values = storage.loadRange(loadRange.getPrefix()).get();
                return new StorageResult.LoadRange(values);
            } catch (Exception e) {
                throw new RuntimeException("LoadRange failed", e);
            }
        } else if (task instanceof StorageTask.Put) {
            StorageTask.Put put = (StorageTask.Put) task;
            try {
                storage.put(put.getKey(), put.getValue()).get();
                return new StorageResult.Put();
            } catch (Exception e) {
                throw new RuntimeException("Put failed", e);
            }
        } else if (task instanceof StorageTask.Delete) {
            StorageTask.Delete delete = (StorageTask.Delete) task;
            try {
                storage.delete(delete.getKey()).get();
                return new StorageResult.Delete();
            } catch (Exception e) {
                throw new RuntimeException("Delete failed", e);
            }
        } else {
            throw new IllegalArgumentException("Unknown storage task type: " + task.getClass());
        }
    }

    /**
     * Gets the configuration.
     */
    RepoConfig getConfig() {
        return config;
    }

    /**
     * Checks if the runtime is stopped.
     */
    boolean isStopped() {
        return stopped.get();
    }

    /**
     * Notifies all registered change listeners for the given document actor.
     */
    private void notifyChangeListeners(DocumentActorId actorId, DocumentChanged event) {
        CopyOnWriteArrayList<ChangeListener> listeners = changeListeners.get(actorId);
        if (listeners != null) {
            for (ChangeListener listener : listeners) {
                try {
                    listener.onDocumentChanged(event);
                } catch (Exception e) {
                    log.error("Change listener threw exception for actor {}: {}", actorId, e.toString());
                }
            }
        }
    }

    /**
     * Registers a change listener for the given document actor.
     */
    ListenerRegistration addChangeListener(DocumentActorId actorId, ChangeListener listener) {
        CopyOnWriteArrayList<ChangeListener> listeners = changeListeners.computeIfAbsent(actorId,
                k -> new CopyOnWriteArrayList<>());
        listeners.add(listener);
        return new ListenerRegistration(listener, listeners);
    }

    /**
     * Executes a function with access to a document.
     */
    <T> CompletableFuture<T> withDocument(DocumentActorId actorId,
            java.util.function.Function<org.automerge.Document, T> fn) {
        CompletableFuture<T> future = new CompletableFuture<>();

        DocumentActor actor = documentActors.get(actorId);

        if (actor == null) {
            future.completeExceptionally(new IllegalStateException("Document actor not found: " + actorId));
            return future;
        }

        if (actor.isStopped()) {
            future.completeExceptionally(new IllegalStateException("Document actor is stopped: " + actorId));
            return future;
        }

        try {
            submitToDocExecutor(actorId, () -> {
                try {
                    long now = System.currentTimeMillis();
                    WithDocResult<T> result = RepoSys.documentActorWithDocument(actor.getPointer(), now, fn);
                    processDocActorResult(actorId, result.getActorResult());
                    future.complete(result.getValue());
                } catch (Exception e) {
                    future.completeExceptionally(e);
                }
            });
        } catch (java.util.concurrent.RejectedExecutionException e) {
            // submitToDocExecutor re-throws unexpected rejections
            future.completeExceptionally(new IllegalStateException("Runtime is shutting down", e));
        }

        return future;
    }

    /**
     * Creates a new document with initial content.
     */
    CompletableFuture<CommandResult.CreateDocument> createDocument(byte[] initialContent) {
        return dispatchHubCommand(
                () -> RepoSys.hubHandleEventCreateDocument(hubPointer, System.currentTimeMillis(), initialContent));
    }

    /**
     * Finds an existing document by ID.
     */
    CompletableFuture<CommandResult.FindDocument> findDocument(DocumentId documentId) {
        return dispatchHubCommand(
                () -> RepoSys.hubHandleEventFindDocument(hubPointer, System.currentTimeMillis(), documentId));
    }

    // ===== Dialer/Listener methods =====

    /**
     * Registers a dialer with the hub and stores its handle.
     *
     * @param config
     *               The dialer configuration
     * @param dialer
     *               The user's Dialer implementation
     * @return A DialerHandle for the registered dialer
     */
    DialerHandle addDialer(Dialer dialer, DialerConfig config) {
        CompletableFuture<CommandResult.AddDialer> future = dispatchHubCommand(
                () -> RepoSys.hubHandleEventAddDialer(hubPointer, System.currentTimeMillis(), config, dialer.getUrl()),
                (CommandResult.AddDialer result) -> {
                    DialerId id = result.getDialerId();
                    dialers.put(id, dialer);
                    dialerHandles.put(id, new DialerHandle(id, this));
                });

        try {
            CommandResult.AddDialer result = future.get(5, TimeUnit.SECONDS);
            return dialerHandles.get(result.getDialerId());
        } catch (Exception e) {
            throw new RuntimeException("Failed to add dialer", e);
        }
    }

    /**
     * Registers a listener with the hub and returns an acceptor handle.
     *
     * @param url
     *            The URL identifying this listener endpoint
     * @return An AcceptorHandle for the registered listener
     */
    AcceptorHandle makeAcceptor(String url) {
        ListenerConfig listenerConfig = new ListenerConfig(url);
        CompletableFuture<CommandResult.AddListener> future = dispatchHubCommand(
                () -> RepoSys.hubHandleEventAddListener(hubPointer, System.currentTimeMillis(), listenerConfig),
                (CommandResult.AddListener result) -> {
                    ListenerId id = result.getListenerId();
                    acceptorHandles.put(id, new AcceptorHandle(id, this));
                });

        try {
            CommandResult.AddListener result = future.get(5, TimeUnit.SECONDS);
            return acceptorHandles.get(result.getListenerId());
        } catch (Exception e) {
            throw new RuntimeException("Failed to create acceptor", e);
        }
    }

    /**
     * Removes a dialer from the hub and cleans up.
     */
    void removeDialer(DialerId dialerId) {
        dialers.remove(dialerId);
        dialerHandles.remove(dialerId);
        submitToHub(() -> RepoSys.hubHandleEventRemoveDialer(hubPointer, System.currentTimeMillis(), dialerId));
    }

    /**
     * Removes a listener from the hub and cleans up.
     */
    void removeListener(ListenerId listenerId) {
        acceptorHandles.remove(listenerId);
        submitToHub(() -> RepoSys.hubHandleEventRemoveListener(hubPointer, System.currentTimeMillis(), listenerId));
    }

    /**
     * Accepts a connection on a listener. Called by AcceptorHandle.accept().
     */
    CompletableFuture<ConnectionId> acceptListenerConnection(ListenerId listenerId, Transport transport) {
        CompletableFuture<CommandResult.CreateConnection> future = dispatchHubCommand(
                () -> RepoSys.hubHandleEventCreateListenerConnection(hubPointer, System.currentTimeMillis(),
                        listenerId),
                (CommandResult.CreateConnection result) -> driveConnection(result.getConnectionId(), transport));

        return future.thenApply(CommandResult.CreateConnection::getConnectionId);
    }
}
