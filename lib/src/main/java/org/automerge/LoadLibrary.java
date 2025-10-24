package org.automerge;

import java.io.*;
import java.net.URL;
import java.net.URLConnection;
import java.nio.file.*;
import java.util.Locale;
import java.util.Optional;
import java.util.Properties;
import java.util.UUID;
import java.util.stream.Stream;

class LoadLibrary {

	private static class Library {
		public String target;
		public String prefix;
		public String suffix;

		public Library(String target, String prefix, String suffix) {
			this.target = target;
			this.prefix = prefix;
			this.suffix = suffix;
		}

		public String getResourcePath() {
			return String.format("native/%s/%sautomerge_jni.%s", target, prefix, suffix);
		}
	}

	private enum Platform {
		UNKNOWN, WINDOWS_X86_32, WINDOWS_X86_64, LINUX_X86_32, LINUX_X86_64, LINUX_ARM64, SOLARIS_X86_32, SOLARIS_X86_64, SOLARIS_SPARC_32, SOLARIS_SPARC_64, MACOSX_X86_32, MACOSX_X86_64, MACOSX_ARM64, ANDROID_ARM, ANDROID_ARM64, ANDROID_X86_32, ANDROID_X86_64, ANDROID_UNKNOWN;

		Optional<Library> library() {
			switch (this) {
				case WINDOWS_X86_64 :
					return Optional.of(new Library("x86_64-pc-windows-gnu", "", "dll"));
				case WINDOWS_X86_32 :
					return Optional.of(new Library("i686-pc-windows-gnu", "", "dll"));
				case LINUX_X86_64 :
					return Optional.of(new Library("x86_64-unknown-linux-gnu", "lib", "so"));
				case LINUX_ARM64 :
					return Optional.of(new Library("aarch64-unknown-linux-gnu", "lib", "so"));
				case MACOSX_X86_64 :
					return Optional.of(new Library("x86_64-apple-darwin", "lib", "dylib"));
				case MACOSX_ARM64 :
					return Optional.of(new Library("aarch64-apple-darwin", "lib", "dylib"));
				case ANDROID_ARM :
					return Optional.of(new Library("armv7-linux-androideabi", "lib", "so"));
				case ANDROID_ARM64 :
					return Optional.of(new Library("aarch64-linux-android", "lib", "so"));
				case ANDROID_X86_32 :
					return Optional.of(new Library("i686-linux-android", "lib", "so"));
				case ANDROID_X86_64 :
					return Optional.of(new Library("x86_64-linux-android", "lib", "so"));
				default :
					return Optional.empty();
			}
		}

		public boolean isAndroid() {
			switch (this) {
				case ANDROID_ARM :
				case ANDROID_ARM64 :
				case ANDROID_X86_32 :
				case ANDROID_X86_64 :
				case ANDROID_UNKNOWN :
					return true;
				default :
					return false;
			}
		}
	}

	static Platform CURRENT_PLATFORM;

	static {
		String name = System.getProperty("os.name").toLowerCase(Locale.ENGLISH);
		String arch = System.getProperty("os.arch").toLowerCase(Locale.ENGLISH);
		String vm = System.getProperty("java.vm.name").toLowerCase(Locale.ENGLISH);
		if (name.startsWith("windows") && "x86".equals(arch)) {
			CURRENT_PLATFORM = Platform.WINDOWS_X86_32;
		} else if (name.startsWith("windows") && ("x86_64".equals(arch) || "amd64".equals(arch))) {
			CURRENT_PLATFORM = Platform.WINDOWS_X86_64;
		} else if ("dalvik".equals(vm) && "armeabi-v7a".equals(arch)) {
			CURRENT_PLATFORM = Platform.ANDROID_ARM;
		} else if ("dalvik".equals(vm) && "aarch64".equals(arch)) {
			CURRENT_PLATFORM = Platform.ANDROID_ARM64;
		} else if ("dalvik".equals(vm) && "x64".equals(arch)) {
			CURRENT_PLATFORM = Platform.ANDROID_X86_32;
		} else if ("dalvik".equals(vm) && "x64_64".equals(arch)) {
			CURRENT_PLATFORM = Platform.ANDROID_X86_64;
		} else if ("dalvik".equals(vm)) {
			CURRENT_PLATFORM = Platform.ANDROID_UNKNOWN;
		} else if ("linux".equals(name) && "i386".equals(arch)) {
			CURRENT_PLATFORM = Platform.LINUX_X86_32;
		} else if ("linux".equals(name) && "amd64".equals(arch)) {
			CURRENT_PLATFORM = Platform.LINUX_X86_64;
		} else if ("linux".equals(name) && "aarch64".equals(arch)) {
			CURRENT_PLATFORM = Platform.LINUX_ARM64;
		} else if ("sunos".equals(name) && "x86".equals(arch)) {
			CURRENT_PLATFORM = Platform.SOLARIS_X86_32;
		} else if ("sunos".equals(name) && "amd64".equals(arch)) {
			CURRENT_PLATFORM = Platform.SOLARIS_X86_64;
		} else if ("sunos".equals(name) && "sparc".equals(arch)) {
			CURRENT_PLATFORM = Platform.SOLARIS_SPARC_32;
		} else if ("sunos".equals(name) && "sparcv9".equals(arch)) {
			CURRENT_PLATFORM = Platform.SOLARIS_SPARC_64;
		} else if ("mac os x".equals(name) && "x86".equals(arch)) {
			CURRENT_PLATFORM = Platform.MACOSX_X86_32;
		} else if ("mac os x".equals(name) && ("x86_64".equals(arch) || "amd64".equals(arch))) {
			CURRENT_PLATFORM = Platform.MACOSX_X86_64;
		} else if ("mac os x".equals(name) && "aarch64".equals(arch)) {
			CURRENT_PLATFORM = Platform.MACOSX_ARM64;
		} else {
			CURRENT_PLATFORM = Platform.UNKNOWN;
		}
	}

	// The extension added to the library name to create a lockfile for it
	private static final String LIBRARY_LOCK_EXT = ".lck";

	static boolean loaded = false;

	public static synchronized void initialize() {
		// Only do this on first load and _dont_ do it on android, where libraries
		// are provided by jniLibs and the version.properties file is removed from the
		// jar by the build process
		if (!loaded && !CURRENT_PLATFORM.isAndroid()) {
			cleanup();
		}
		loadAutomergeJniLib();
	}

	private static File getTempDir() {
		return new File(System.getProperty("java.io.tmpdir"));
	}

	/**
	 * Delete unused library files
	 *
	 * <p>
	 * On windows the library files are locked by the JVM and will not be deleted on
	 * close. To prevent accumulating many versions of the lockfile we create a file
	 * with the `LIBRARY_LOCK_EXT` extension which is deleted on close and then we
	 * delete every library file that does not have a lockfile the next time we
	 * load. This should mean we never have more library files than the number of
	 * instances of the JVM which have loaded the library.
	 */
	static void cleanup() {
		String searchPattern = "automerge-jni-" + getVersion();

		try (Stream<Path> dirList = Files.list(getTempDir().toPath())) {
			dirList.filter(path -> !path.getFileName().toString().endsWith(LIBRARY_LOCK_EXT)
					&& path.getFileName().toString().startsWith(searchPattern)).forEach(nativeLib -> {
						Path lckFile = Paths.get(nativeLib + LIBRARY_LOCK_EXT);
						if (Files.notExists(lckFile)) {
							try {
								Files.delete(nativeLib);
							} catch (Exception e) {
								System.err.println("Failed to delete old native lib: " + e.getMessage());
							}
						}
					});
		} catch (IOException e) {
			System.err.println("Failed to open directory: " + e.getMessage());
		}
	}

	private static void extractAndLoadLibraryFile(Library library, String targetFolder) {
		String uuid = UUID.randomUUID().toString();
		String extractedLibFileName = String.format("automerge-jni-%s-%s-%s.%s", getVersion(), uuid, library.target,
				library.suffix);
		String extractedLckFileName = extractedLibFileName + LIBRARY_LOCK_EXT;

		Path extractedLibFile = Paths.get(targetFolder, extractedLibFileName);
		Path extractedLckFile = Paths.get(targetFolder, extractedLckFileName);

		try {
			// Extract a native library file into the target directory
			try (InputStream reader = getResourceAsStream(library.getResourcePath())) {
				if (Files.notExists(extractedLckFile)) {
					Files.createFile(extractedLckFile);
				}

				Files.copy(reader, extractedLibFile, StandardCopyOption.REPLACE_EXISTING);
			} finally {
				// Delete the extracted lib file on JVM exit.
				extractedLibFile.toFile().deleteOnExit();
				extractedLckFile.toFile().deleteOnExit();
			}

			// Set executable (x) flag to enable Java to load the native library
			extractedLibFile.toFile().setReadable(true);
			extractedLibFile.toFile().setWritable(true, true);
			extractedLibFile.toFile().setExecutable(true);

			System.load(extractedLibFile.toString());
		} catch (IOException e) {
			throw new RuntimeException("unable to load automerge-jni", e);
		}
	}

	// Replacement of java.lang.Class#getResourceAsStream(String) to disable sharing
	// the resource
	// stream in multiple class loaders and specifically to avoid
	// https://bugs.openjdk.java.net/browse/JDK-8205976
	private static InputStream getResourceAsStream(String name) {
		ClassLoader cl = LoadLibrary.class.getClassLoader();
		URL url = cl.getResource(name);
		if (url == null) {
			throw new RuntimeException("Resource not found: " + name);
		}
		try {
			URLConnection connection = url.openConnection();
			connection.setUseCaches(false);
			return connection.getInputStream();
		} catch (IOException e) {
			throw new RuntimeException("unable to get resource reader", e);
		}
	}

	/**
	 * Loads the native library
	 *
	 * <p>
	 * We first try the system library path, then if that fails we try and load one
	 * of the bundled libraries from this jar.
	 */
	private static void loadAutomergeJniLib() {
		if (loaded) {
			return;
		}

		String libName = "automerge_jni_" + BuildInfo.getExpectedRustLibVersion().replace('.', '_');
		// Try System.loadLibrary first
		try {
			System.loadLibrary(libName);
			loaded = true;
			return;
		} catch (UnsatisfiedLinkError e) {
			if (CURRENT_PLATFORM.isAndroid()) {
				// We can't bundle the libs in android except via jniLibs, if
				// we were unable to load using loadLibrary then there's no hope
				throw e;
			}
		}
		// Alright, it's not on the library path, lets find it in the jar

		// temporary library folder
		String tempFolder = getTempDir().getAbsolutePath();
		if (CURRENT_PLATFORM.library().isEmpty()) {
			throw new UnsupportedPlatformException("no native automerge library found for " + CURRENT_PLATFORM.name());
		}
		Library lib = CURRENT_PLATFORM.library().get();
		// Try extracting the library from jar
		extractAndLoadLibraryFile(lib, tempFolder);

		// Check that we have the correct version of the library (BuildInfo is
		// generated by gradle and contains the version of the rust library we
		// were built against)
		String expectedLibVersion = BuildInfo.getExpectedRustLibVersion();
		String actualLibVersion = AutomergeSys.rustLibVersion();
		if (!expectedLibVersion.equals(actualLibVersion)) {
			throw new RuntimeException("Automerge native library version mismatch. Expected " + expectedLibVersion
					+ " but got " + actualLibVersion);
		}

		loaded = true;
	}

	public static String getVersion() {
		InputStream input = getResourceAsStream("version.properties");

		String version = "unknown";
		try {
			Properties versionData = new Properties();
			versionData.load(input);
			version = versionData.getProperty("version", version);
			return version;
		} catch (IOException e) {
			throw new RuntimeException("unable to load version properties", e);
		}
	}
}
