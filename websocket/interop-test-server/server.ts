import express from "express";
import { WebSocketServer } from "ws";
import {
  Chunk,
  Repo,
  RepoConfig,
  StorageAdapterInterface,
  StorageKey,
} from "@automerge/automerge-repo";
import { NodeWSServerAdapter } from "@automerge/automerge-repo-network-websocket";

class Server {
  #socket: WebSocketServer;

  #server: ReturnType<import("express").Express["listen"]>;
  #storage: InMemoryStorageAdapter;

  #repo: Repo;

  constructor(port: number) {
    this.#socket = new WebSocketServer({ noServer: true });

    const PORT = port;
    const app = express();
    app.use(express.static("public"));
    this.#storage = new InMemoryStorageAdapter();

    const config: RepoConfig = {
      // network: [new NodeWSServerAdapter(this.#socket) as any],
      network: [new NodeWSServerAdapter(this.#socket as any)],
      storage: this.#storage,
      /** @ts-ignore @type {(import("automerge-repo").PeerId)}  */
      peerId: `storage-server` as PeerId,
      // Since this is a server, we don't share generously — meaning we only sync documents they already
      // know about and can ask for by ID.
      sharePolicy: async () => false,
    };
    const serverRepo = new Repo(config);
    this.#repo = serverRepo;

    app.get("/", (req, res) => {
      res.send(`👍 @automerge/automerge-repo-sync-server is running`);
    });

    app.get("/storage-keys", (req, res) => {
      const keys = this.#storage.keys();
      res.json(keys);
    });

    this.#server = app.listen(PORT, () => {
      console.log(`Listening on port ${this.#server.address().port}`);
    });

    this.#server.on("upgrade", (request, socket, head) => {
      console.log("upgrading to websocket");
      this.#socket.handleUpgrade(request, socket, head, (socket) => {
        this.#socket.emit("connection", socket, request);
      });
    });
  }

  close() {
    this.#storage.log();
    this.#socket.close();
    this.#server.close();
  }
}

class InMemoryStorageAdapter implements StorageAdapterInterface {
  #data: Map<StorageKey, Uint8Array> = new Map();

  async load(key: StorageKey): Promise<Uint8Array | undefined> {
    return this.#data.get(key);
  }
  async save(key: StorageKey, data: Uint8Array): Promise<void> {
    this.#data.set(key, data);
  }
  async remove(key: StorageKey): Promise<void> {
    this.#data.delete(key);
  }
  async loadRange(keyPrefix: StorageKey): Promise<Chunk[]> {
    let result: Chunk[] = [];
    for (const [key, value] of this.#data.entries()) {
      if (isPrefixOf(keyPrefix, key)) {
        result.push({
          key,
          data: value,
        });
      }
    }
    return result;
  }

  removeRange(keyPrefix: StorageKey): Promise<void> {
    for (const [key] of this.#data.entries()) {
      if (isPrefixOf(keyPrefix, key)) {
        this.#data.delete(key);
      }
    }
    return Promise.resolve();
  }

  keys(): string[][] {
    return Array.from(this.#data.keys());
  }

  log() {
    console.log(`InMemoryStorageAdapter has ${this.#data.size} items:`);
    for (const [key, value] of this.#data.entries()) {
      console.log(`  ${key.join("/")}: ${value.length} bytes`);
    }
  }
}

function isPrefixOf(prefix: StorageKey, candidate: StorageKey): boolean {
  return (
    prefix.length <= candidate.length &&
    prefix.every((segment, index) => segment === candidate[index])
  );
}

const port = process.argv[2] ? parseInt(process.argv[2]) : 8080;
const server = new Server(port);

process.on("SIGINT", () => {
  server.close();
  process.exit(0);
});
