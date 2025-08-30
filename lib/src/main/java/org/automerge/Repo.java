package org.automerge;

import java.util.concurrent.CompletableFuture;
import java.util.concurrent.Future;

public class Repo {

	public CompletableFuture<DocHandle> create() {
		return CompletableFuture.supplyAsync(() -> new DocHandle());
	}

    public CompletableFuture<DocHandle> find(AutomergeUrl url) {
        return CompletableFuture.supplyAsync(() -> new DocHandle());
    }

	public CompletableFuture<Void> connect(Transport transport) {
		return CompletableFuture.runAsync(() -> {
		});
	}
}
