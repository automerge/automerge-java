import {
  Repo,
  isValidAutomergeUrl,
  parseAutomergeUrl,
} from "@automerge/automerge-repo";
import { BrowserWebSocketClientAdapter } from "@automerge/automerge-repo-network-websocket";
import { command, run, string, positional, number, subcommands } from "cmd-ts";
import { next as A } from "@automerge/automerge";

const create = command({
  name: "create",
  args: {
    port: positional({
      type: number,
      displayName: "port",
      description: "The port to connect to",
    }),
  },
  handler: ({ port }) => {
    const repo = new Repo({
      network: [new BrowserWebSocketClientAdapter(`ws://localhost:${port}`)],
    });
    const doc = repo.create<{ foo: string }>();
    doc.change((d) => (d.foo = "bar"));
    console.log(doc.url);
    console.log(A.getHeads(doc.doc()).join(","));
  },
});

const fetch = command({
  name: "fetch",
  args: {
    port: positional({
      type: number,
      displayName: "port",
      description: "The port to connect to",
    }),
    docUrl: positional({
      type: string,
      displayName: "docUrl",
      description: "The document url to fetch",
    }),
  },
  handler: ({ port, docUrl }) => {
    const repo = new Repo({
      network: [new BrowserWebSocketClientAdapter(`ws://localhost:${port}`)],
    });
    if (isValidAutomergeUrl(docUrl)) {
    } else {
      throw new Error("Invalid docUrl");
    }
    const doc = repo.find(docUrl);
    repo.find(docUrl).then((d) => console.log(A.getHeads(d.doc()).join(",")));
  },
});

const sendEphemeral = command({
  name: "send-ephemeral",
  args: {
    port: positional({
      type: number,
      displayName: "port",
      description: "The port to connect to",
    }),
    docUrl: positional({
      type: string,
      displayName: "docUrl",
      description: "The document url to fetch",
    }),
    message: positional({
      type: string,
      displayName: "message",
      description: "The message to send",
    }),
  },
  handler: ({ port, docUrl, message }) => {
    const repo = new Repo({
      network: [new BrowserWebSocketClientAdapter(`ws://localhost:${port}`)],
    });
    if (!isValidAutomergeUrl(docUrl)) {
      throw new Error("Invalid docUrl");
    }
    repo
      .find(docUrl)
      .then((doc) => {
        doc.broadcast({ message });
        process.exit(0);
      })
      .catch((e) => {
        console.error(e);
        process.exit(1);
      });
  },
});

const receiveEphemeral = command({
  name: "receive-ephemeral",
  args: {
    port: positional({
      type: number,
      displayName: "port",
      description: "The port to connect to",
    }),
    docUrl: positional({
      type: string,
      displayName: "docUrl",
      description: "The document url to fetch",
    }),
  },
  handler: async ({ port, docUrl }) => {
    const repo = new Repo({
      network: [new BrowserWebSocketClientAdapter(`ws://localhost:${port}`)],
    });
    if (!isValidAutomergeUrl(docUrl)) {
      throw new Error("Invalid docUrl");
    }
    repo.find(docUrl).then((doc) => {
      doc.on("ephemeral-message", ({ message }) => {
        if (typeof message === "object" && "message" in message) {
          console.log(message.message);
        }
      });
      // Signal that we're ready to receive ephemeral messages
      console.log("ready");
    });
  },
});

// This command connects to two servers: first a JS server (which has a storage ID), syncs a
// document with it, then connects to a second server (the Java server). When the second server
// becomes a "generous peer", addGenerousPeer fires and sends a `remote-heads-changed` message
// containing the stored remote heads info with a `Date.now()` timestamp (encoded as f64 by cbor-x).
const createAndRelayHeads = command({
  name: "create-and-relay-heads",
  args: {
    jsServerPort: positional({
      type: number,
      displayName: "jsServerPort",
      description: "The port of the JS server to sync with first",
    }),
    javaServerPort: positional({
      type: number,
      displayName: "javaServerPort",
      description: "The port of the Java server to connect to second",
    }),
  },
  handler: async ({ jsServerPort, javaServerPort }) => {
    const jsAdapter = new BrowserWebSocketClientAdapter(
      `ws://localhost:${jsServerPort}`,
    );
    const repo = new Repo({
      network: [jsAdapter],
      enableRemoteHeadsGossiping: true,
    });

    // Subscribe to a dummy storage ID so that when the Java server connects,
    // addGenerousPeer sends a remote-subscription-change message too.
    repo.subscribeToRemotes(["dummy-storage-id" as any]);

    const doc = repo.create<{ foo: string }>();
    doc.change((d) => (d.foo = "bar"));

    // Wait for sync with JS server to complete. This stores remote heads info
    // (with a Date.now() timestamp) in RemoteHeadsSubscriptions.#syncInfoByDocId.
    await new Promise<void>((resolve) => setTimeout(resolve, 500));

    // Now connect to the Java server. When the peer event fires, sharePolicy
    // returns true (default), so addGenerousPeer is called. This sends a
    // `remote-heads-changed` message with the stored f64 timestamp.
    const javaAdapter = new BrowserWebSocketClientAdapter(
      `ws://localhost:${javaServerPort}`,
    );
    repo.networkSubsystem.addNetworkAdapter(javaAdapter);

    // Wait for the Java server connection to establish and messages to be sent.
    await new Promise<void>((resolve) => setTimeout(resolve, 500));

    console.log(doc.url);
    console.log(A.getHeads(doc.doc()).join(","));
  },
});

const subscribeAndCreate = command({
  name: "subscribe-and-create",
  args: {
    port: positional({
      type: number,
      displayName: "port",
      description: "The port to connect to",
    }),
    storageId: positional({
      type: string,
      displayName: "storageId",
      description: "The storage ID to subscribe to remote heads for",
    }),
  },
  handler: ({ port, storageId }) => {
    const repo = new Repo({
      network: [new BrowserWebSocketClientAdapter(`ws://localhost:${port}`)],
      enableRemoteHeadsGossiping: true,
    });
    repo.subscribeToRemotes([storageId as any]);
    const doc = repo.create<{ foo: string }>();
    doc.change((d) => (d.foo = "bar"));
    console.log(doc.url);
    console.log(A.getHeads(doc.doc()).join(","));
  },
});

const app = subcommands({
  name: "client",
  cmds: {
    create,
    fetch,
    "send-ephemeral": sendEphemeral,
    "receive-ephemeral": receiveEphemeral,
    "subscribe-and-create": subscribeAndCreate,
    "create-and-relay-heads": createAndRelayHeads,
  },
});

run(app, process.argv.slice(2));
