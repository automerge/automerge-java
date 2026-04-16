package org.automerge.repo.websocket.interop;

import java.io.BufferedReader;
import java.io.File;
import java.io.IOException;
import java.io.InputStreamReader;
import java.net.HttpURLConnection;
import java.net.URL;
import java.util.ArrayList;
import java.util.Arrays;
import java.util.List;
import java.util.concurrent.ArrayBlockingQueue;
import java.util.concurrent.BlockingQueue;
import java.util.concurrent.TimeUnit;
import java.util.concurrent.atomic.AtomicReference;

/**
 * Manages the lifecycle of the JavaScript interop test server.
 *
 * <p>
 * On first use, runs {@code npm install} and {@code npm run build} in the
 * {@code websocket/interop-test-server} directory (found relative to the
 * Gradle working directory). Subsequent calls skip the build step.
 *
 * <p>
 * Call {@link #start()} to spawn {@code node server.js 0} and read the port it
 * binds to. Call {@link #close()} (or use try-with-resources) to shut it down.
 */
public class JsServerWrapper implements AutoCloseable {

    /** Cached build result: null = not yet run, non-null = done or failed. */
    private static final AtomicReference<String> buildError = new AtomicReference<>(null);
    private static volatile boolean buildDone = false;

    private final File serverDir;
    private Process serverProcess;
    private int port;

    public JsServerWrapper(File serverDir) {
        this.serverDir = serverDir;
    }

    /**
     * Returns the {@code interop-test-server} directory. Looks for
     * {@code websocket/interop-test-server} relative to the process working
     * directory (the Gradle project root when tests run), or uses the
     * {@code interop.test.server.dir} system property if set.
     *
     * @return the directory, or {@code null} if not found
     */
    public static File findServerDir() {
        String prop = System.getProperty("interop.test.server.dir");
        if (prop != null) {
            File dir = new File(prop);
            if (dir.isDirectory())
                return dir;
        }
        // When Gradle runs :websocket tests, working dir is the websocket/ subproject.
        File[] candidates = {
                new File("interop-test-server"),
                new File("websocket/interop-test-server"),
        };
        for (File c : candidates) {
            if (c.isDirectory() && new File(c, "package.json").exists())
                return c.getAbsoluteFile();
        }
        return null;
    }

    /**
     * Ensures the JS project is built (npm install + npm run build), then spawns
     * {@code node server.js 0} and returns a ready wrapper.
     *
     * @throws Exception
     *             if the build fails or the server does not start
     */
    public static JsServerWrapper start(File serverDir) throws Exception {
        ensureBuilt(serverDir);
        JsServerWrapper w = new JsServerWrapper(serverDir);
        w.startServer();
        return w;
    }

    // -- Build --

    private static synchronized void ensureBuilt(File serverDir) throws Exception {
        if (buildDone)
            return;

        runCommand(serverDir, "npm", "install");
        runCommand(serverDir, "npm", "run", "build");

        buildDone = true;
    }

    private static void runCommand(File dir, String... cmd) throws Exception {
        ProcessBuilder pb = new ProcessBuilder(Arrays.asList(cmd));
        pb.directory(dir);
        pb.redirectErrorStream(true); // merge stderr into stdout
        Process proc = pb.start();

        // Drain output, printing it for debugging
        Thread reader = new Thread(() -> {
            try (BufferedReader r = new BufferedReader(new InputStreamReader(proc.getInputStream()))) {
                String line;
                while ((line = r.readLine()) != null) {
                    System.out.println("[interop-build] " + line);
                }
            } catch (IOException e) {
                // process ended
            }
        }, "interop-build-output");
        reader.setDaemon(true);
        reader.start();

        if (!proc.waitFor(120, TimeUnit.SECONDS)) {
            proc.destroyForcibly();
            throw new RuntimeException(String.format("Command timed out: %s", Arrays.toString(cmd)));
        }
        int exit = proc.exitValue();
        if (exit != 0) {
            throw new RuntimeException(String.format("Command failed (exit %d): %s", exit, Arrays.toString(cmd)));
        }
    }

    // -- Server lifecycle --

    private void startServer() throws Exception {
        ProcessBuilder pb = new ProcessBuilder("node", "server.js", "0");
        pb.directory(serverDir);
        serverProcess = pb.start();

        BlockingQueue<String> stdoutLines = new ArrayBlockingQueue<>(256);

        Thread stdoutThread = new Thread(() -> {
            try (BufferedReader r = new BufferedReader(new InputStreamReader(serverProcess.getInputStream()))) {
                String line;
                while ((line = r.readLine()) != null) {
                    stdoutLines.offer(line);
                }
            } catch (IOException e) {
                // process ended
            }
        }, "js-server-stdout");
        stdoutThread.setDaemon(true);
        stdoutThread.start();

        Thread stderrThread = new Thread(() -> {
            try (BufferedReader r = new BufferedReader(new InputStreamReader(serverProcess.getErrorStream()))) {
                String line;
                while ((line = r.readLine()) != null) {
                    System.err.println("[JS server] " + line);
                }
            } catch (IOException e) {
                // process ended
            }
        }, "js-server-stderr");
        stderrThread.setDaemon(true);
        stderrThread.start();

        long deadline = System.currentTimeMillis() + 10_000;
        while (System.currentTimeMillis() < deadline) {
            String line = stdoutLines.poll(100, TimeUnit.MILLISECONDS);
            if (line == null)
                continue;
            if (line.startsWith("Listening on port ")) {
                port = Integer.parseInt(line.substring("Listening on port ".length()).trim());
                return;
            }
        }
        serverProcess.destroyForcibly();
        throw new RuntimeException("JS server did not print 'Listening on port N' within 10s");
    }

    public int getPort() {
        return port;
    }

    // -- Client --

    /**
     * Runs {@code node client.js <args>}, collects stdout lines, then kills the
     * process.
     *
     * <p>
     * The JS client scripts (create, fetch) do not call {@code process.exit()} —
     * they keep the WebSocket connection open after printing their output. We
     * therefore read lines until there has been no new output for
     * {@code quietMs} milliseconds, then destroy the process.
     */
    public List<String> runClient(String... args) throws Exception {
        return runClient(2000, args);
    }

    public List<String> runClient(long quietMs, String... args) throws Exception {
        List<String> cmd = new ArrayList<>();
        cmd.add("node");
        cmd.add("client.js");
        for (String arg : args) {
            cmd.add(arg);
        }

        ProcessBuilder pb = new ProcessBuilder(cmd);
        pb.directory(serverDir);
        Process proc = pb.start();

        BlockingQueue<String> queue = new ArrayBlockingQueue<>(256);

        Thread stderrThread = new Thread(() -> {
            try (BufferedReader r = new BufferedReader(new InputStreamReader(proc.getErrorStream()))) {
                String line;
                while ((line = r.readLine()) != null) {
                    System.err.println("[JS client] " + line);
                }
            } catch (IOException e) {
                // process ended
            }
        }, "js-client-stderr");
        stderrThread.setDaemon(true);
        stderrThread.start();

        Thread stdoutThread = new Thread(() -> {
            try (BufferedReader r = new BufferedReader(new InputStreamReader(proc.getInputStream()))) {
                String line;
                while ((line = r.readLine()) != null) {
                    queue.offer(line);
                }
            } catch (IOException e) {
                // process ended
            }
        }, "js-client-stdout");
        stdoutThread.setDaemon(true);
        stdoutThread.start();

        // Collect lines until quietMs have passed with no new output (indicating
        // the client has printed everything it will print) or total 15s timeout.
        List<String> output = new ArrayList<>();
        long totalDeadline = System.currentTimeMillis() + 15_000;
        long lastOutputTime = System.currentTimeMillis();
        while (System.currentTimeMillis() < totalDeadline) {
            String line = queue.poll(100, TimeUnit.MILLISECONDS);
            if (line != null) {
                output.add(line);
                lastOutputTime = System.currentTimeMillis();
            } else if (System.currentTimeMillis() - lastOutputTime >= quietMs) {
                break; // quiet period elapsed — client has printed everything
            }
        }

        proc.destroyForcibly();
        proc.waitFor(5, TimeUnit.SECONDS);

        if (output.isEmpty()) {
            throw new RuntimeException("JS client produced no output. Args: " + Arrays.toString(args));
        }
        return output;
    }

    // -- Storage keys --

    /**
     * Fetches {@code /storage-keys} and returns each key as a list of path
     * segments (e.g. {@code [["docId","snapshot","hash"], ...]}).
     */
    public List<List<String>> storageKeys() throws Exception {
        URL url = new URL("http://localhost:" + port + "/storage-keys");
        HttpURLConnection conn = (HttpURLConnection) url.openConnection();
        conn.setRequestMethod("GET");
        conn.setConnectTimeout(5000);
        conn.setReadTimeout(5000);

        StringBuilder sb = new StringBuilder();
        try (BufferedReader r = new BufferedReader(new InputStreamReader(conn.getInputStream()))) {
            String line;
            while ((line = r.readLine()) != null) {
                sb.append(line);
            }
        }
        return parseJsonArrayOfArrays(sb.toString().trim());
    }

    // -- minimal JSON parser for array of string arrays --

    private static List<List<String>> parseJsonArrayOfArrays(String json) {
        List<List<String>> result = new ArrayList<>();
        if (json.equals("[]"))
            return result;
        json = json.substring(1, json.length() - 1).trim();
        int i = 0;
        while (i < json.length()) {
            if (json.charAt(i) == '[') {
                int end = findClosingBracket(json, i);
                result.add(parseStringArray(json.substring(i + 1, end)));
                i = end + 1;
                while (i < json.length() && (json.charAt(i) == ',' || json.charAt(i) == ' '))
                    i++;
            } else {
                i++;
            }
        }
        return result;
    }

    private static int findClosingBracket(String s, int open) {
        int depth = 0;
        for (int i = open; i < s.length(); i++) {
            if (s.charAt(i) == '[')
                depth++;
            else if (s.charAt(i) == ']') {
                depth--;
                if (depth == 0)
                    return i;
            }
        }
        throw new IllegalArgumentException("No closing bracket in: " + s.substring(open));
    }

    private static List<String> parseStringArray(String inner) {
        List<String> result = new ArrayList<>();
        boolean inString = false;
        int start = -1;
        for (int i = 0; i < inner.length(); i++) {
            char c = inner.charAt(i);
            if (c == '"' && !inString) {
                inString = true;
                start = i + 1;
            } else if (c == '"' && inString) {
                result.add(inner.substring(start, i));
                inString = false;
            }
        }
        return result;
    }

    @Override
    public void close() {
        if (serverProcess != null && serverProcess.isAlive()) {
            serverProcess.destroyForcibly();
        }
    }
}
