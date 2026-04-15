package org.automerge.repo;

import org.automerge.Document;
import org.automerge.LoadLibrary;

class RepoSys {

    static {
        LoadLibrary.initialize();
    }

    protected class HubPointer {

        private long pointer;
    }

    protected class DocumentActorPointer {

        private long pointer;
    }

    protected class SamodLoaderPointer {

        private long pointer;
    }

    // AutomergeUrl methods
    public static native AutomergeUrl parseAutomergeUrl(String url);

    public static native String automergeUrlFromDocumentId(DocumentId documentId);

    // DocumentID methods
    public static native DocumentId documentIdFromBytes(byte[] encoded);

    public static native DocumentId generateDocumentId();

    // PeerId methods
    public static native PeerId generatePeerId();

    // SamodLoader methods
    public static native SamodLoaderPointer createSamodLoader(String peerId);

    public static native LoaderStepResult stepSamodLoader(SamodLoaderPointer loader, long timestamp);

    public static native void provideSamodLoaderIoResult(SamodLoaderPointer loader, IoResult<StorageResult> result);

    public static native void freeSamodLoader(SamodLoaderPointer loader);

    // Hub methods — fire-and-forget events → HubResults
    static native HubResults hubHandleEventTick(HubPointer hub, long timestamp);

    static native HubResults hubHandleEventStop(HubPointer hub, long timestamp);

    static native HubResults hubHandleEventActorMessage(HubPointer hub, long timestamp, DocumentActorId actorId,
            DocToHubMsg message);

    static native HubResults hubHandleEventIoComplete(HubPointer hub, long timestamp,
            IoResult<HubIoResult> result);

    static native HubResults hubHandleEventConnectionLost(HubPointer hub, long timestamp,
            ConnectionId connectionId);

    static native HubResults hubHandleEventDialFailed(HubPointer hub, long timestamp, DialerId dialerId,
            String error);

    static native HubResults hubHandleEventRemoveDialer(HubPointer hub, long timestamp, DialerId dialerId);

    static native HubResults hubHandleEventRemoveListener(HubPointer hub, long timestamp,
            ListenerId listenerId);

    // Hub methods — command events → HubCommandResult
    static native HubCommandResult hubHandleEventCreateDocument(HubPointer hub, long timestamp,
            byte[] initialContent);

    static native HubCommandResult hubHandleEventFindDocument(HubPointer hub, long timestamp,
            DocumentId documentId);

    static native HubCommandResult hubHandleEventAddDialer(HubPointer hub, long timestamp, DialerConfig config,
            String url);

    static native HubCommandResult hubHandleEventAddListener(HubPointer hub, long timestamp,
            ListenerConfig config);

    static native HubCommandResult hubHandleEventCreateDialerConnection(HubPointer hub, long timestamp,
            DialerId dialerId);

    static native HubCommandResult hubHandleEventCreateListenerConnection(HubPointer hub, long timestamp,
            ListenerId listenerId);

    static native HubCommandResult hubHandleEventReceive(HubPointer hub, long timestamp,
            ConnectionId connectionId, byte[] message);

    // Hub status
    public static native StorageId hubGetStorageId(HubPointer hub);

    public static native PeerId hubGetPeerId(HubPointer hub);

    public static native boolean hubIsStopped(HubPointer hub);

    public static native void freeHub(HubPointer hub);

    // DocumentActor methods
    public static native DocActorResult documentActorHandleMsg(DocumentActorPointer actor, long timestamp,
            HubToDocMsg msg);

    public static native DocActorResult documentActorHandleIoComplete(DocumentActorPointer actor, long timestamp,
            IoResult<DocumentIoResult> ioResult);

    public static native <T> WithDocResult<T> documentActorWithDocument(DocumentActorPointer actor, long timestamp,
            java.util.function.Function<Document, T> fn);

    public static native boolean documentActorIsStopped(DocumentActorPointer actor);

    public static native void freeDocumentActor(DocumentActorPointer actor);
}
