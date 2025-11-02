# Hacking

## How does this work?

This repository contains a rust project under `./rust` which builds a shared
library that interfaces with the JNI. This library is then built for all the
major architectures and bundled into a jar by the `./lib` gradle module. The
java code in `./lib` then figures out which shared library to load from based
on the platform it is running on. The developer should never have to think
about native libraries.

The only place where this gets a bit hairy is android. The android build tools
strip out the resources of dependent jars, which means we can't distribute the
shared library as a resource for android. Instead the `./android` gradle
module creates an AAR containing the `jniLibs` which android expects and then
we use some variant aware matching magic in `./lib` to add a dependency on the
android library when building for android. Again, this should mean developers
never have to think about native libraries.

## Codebase Structure

To understand the codebase it's worthwhile to understand a little more detail
about how we integrate with the JVM. This crate uses the JNI interface (which
is old and superceded by the newer FFM API, but FFM is only available in Java 18
and later and this library targets Java 8 up). The JNI interface has a few
ingredients:

* A shared library (the native code) which exports a bunch of symbols called
  `<snake_cased_package_name>_<functionName>`
* A class which has a bunch of methods marked as `native`
* Some code which calls `System.loadLibrary` to load the shared library

Calling a `native` method called `functionName` will then attempt to call a
function called `<snake_cased_package_name>_<functionName>` in any shared
library.

In this codebase this is captured by:

* The `jni_fn` macro, which is used to annotate a rust function in the rust
  crate and prefixes the function name with `org_automerge_`
* The `AutomergeSys` class in the java codebase, which is where all the `native`
  methods which call out to the rust code are
* The `LoadLibrary` class in Java, which takes care of the various ways we try
  and get hold of the shared library to load it in the first place

This means that making changes to this codebase usually means:

* Figure out how the changes you want to make should map to the Java API
* Modify the java codebase to match the new API
* Write the signature to the native methods you need in `AutomergeSys`
* Implement the native methods in the rust crate using the `jni_fn` macro

## Testing

There are basically two kinds of things we want to test:

* The binding logic which uses the `jni` crate to map between Java and Rust
* The library loading logic which locates the correct shared library for the
  platform the library is running on

### Testing the bindings

The first of these is independent of how the library is loaded and so we
achieve it by just compiling a version of the library for the current
architecture see [testing without a full build](#testing-without-a-full-build)
to see how to do this.

### Testing library loading

Testing the library loading is more involved. In order to test library loading
we need to build a complete JAR containing all the shared libraries, then run
the tests against this library on the target architecture.

#### Non-android targets

On non-android targets the shared library is bundled in the resources of the
JAR. Testing then means building the JAR as in [building for every
architecture](#building-for-every-architecture), then running the `lib:testJar`
task. This task loads and compiles the test classes without compiling the source
classes and then runs the tests against the JAR we just built.

#### Android Targets

On android targets the android build system strips the resources out of the jar.
This means that to distribute the android library we add an optional dependency
on an AAR which contains the native libraries. To test this we actually build a
test android app (in `./android-test-app`) and run the
`android-test-app:connectedAndroidTest` to run the unit tests for this
application on a connected android device. This ensures that we actually run the
android build tools in question and that the native libraries are in the right
place in the AAR to be picked up by the library loading code.

## Building the thing

Building this project requires compiling the rust shared library for every
architecture we support (which is quite a lot). This in turn means that you need
a toolchain for each of these environments set up locally. This is extremely
fiddly so there's a lot of CI infrastructure which handles this. You can read
more about it in the following sections

* [quickly running tests against the current architecture](#testing-without-a-full-build)
* [building and testing for every architecture](#building-for-every-architecture)
* [the CI setup](#how-ci-works)

### Testing without a full build

The complete build requires building release binaries for every architecture.
This can be quite time consuming and also requries a lot of toolchain setup.
If you just want to make some changes to the java/rust code and test it you
can run `./gradlew lib:test -Penv=dev`. The `env=dev` part will build a debug
build of the Rust project for the current architecture and make it available
on the library path of the java tests. This is much, much faster.

### Building for every architecture

In general you will need a toolchain set up for all the following platforms:

* x86_64-pc-windows-gnu
* i686-pc-windows-gnu
* aarch64-pc-windows-gnullvm
* x86_64-unknown-linux-gnu
* aarch64-unknown-linux-gnu
* x86_64-apple-darwin
* aarch64-apple-darwin
* armv7-linux-androideabi
* aarch64-linux-android
* i686-linux-android
* x86_64-linux-android

To build the android libs you will also need the android sdk and ndk installed
with an entry in local.properties pointing at them.

Concretely what that means is that for each of the above targets you run

    `rustup add <target>`

Then, you'll need to create a `local.properties` file in the root of this
repository with the following contents:

```
sdk.dir=<android SDK dir>
ndkPath=<path to NDK>
aarchLinkerPath=<path>
win64LinkerPath=<path>
win32LinkerPath=<path>
aarchWinLinkerPath=<path>
x86_64LinuxLinkerPath=<path>
aarchDarwinLinkerPath=<path>
aarchDarwinLdPath=<path>
x86_64DarwinLinkerPath=<path>
x86_64DarwinLdPath=<path>
```

How to find these linkers for your system is your problem I'm afraid. On x64-64
arch linux you need to install the following packages:

* `aarch64-linux-gnu-gcc`
* `mingw-w64-gcc`
* `i686-pc-windows-gnu`

Then setup [`osxcross`](https://github.com/tpoechtrager/osxcross). Then the
contents of `local.properties` might look like this:

```
aarchLinkerPath=/usr/bin/aarch64-linux-gnu-gcc
win64LinkerPath=/usr/bin/x86_64-w64-mingw32-gcc
win32LinkerPath=/usr/bin/i686-w64-mingw32-gcc
x86_64LinuxLinkerPath=/usr/bin/gcc
aarchDarwinLinkerPath=<osxcross path>/target/bin/arm64-apple-darwin20.4-clang
aarchDarwinLdPath=<osxcross path>/target/bin/x86_64-apple-darwin20.4-ld
x86_64DarwinLinkerPath=<osxcross path>/target/bin/x86_64-apple-darwin20.4-clang
x86_64DarwinLdPath=<osxcross path>/target/bin/x86_64-apple-darwin20.4-ld
```

### How CI Works

There are two CI workflows, `ci.yaml` - which runs tests, and `release.yaml`
which publishes the library to Maven Central. Both of these tasks need
built artifacts (the JAR and AAR), so there are also two reusable workflows
`build-jar.yaml` and `build-aar.yaml` which achieve this and are invoked
from the top level workflows.

#### `build-jar.yaml`

This is probably the most interesting workflow. It makes use of a docker
image built by the `Dockerfile` in `./docker-build` which contains all
the toolchains required for crossbuilding for the various architectures
we support.

#### `build-aar.yaml`

This task builds the shared library for the various android architectures
and bundles them all into an AAR using the `android:assembleRelease` task.

#### `ci.yaml`

This workflow first runs the dev-tests (current architecture only) to
ensure the JNI bindings are valid. Then it runs the build-jar and build-aar
workflows and runs the `lib:testJar` and `android-test-app:connectedAndroidTest`
tasks against these built artifacts for as many architectures as are
supported by Github Actions runners.

#### `release.yaml`

This workflow builds runs the reusable workflows to build the JAR and aar
and then publishe them to Maven Central. This logic is explained in more
detail in the [release process](#release-process) section.

## Release Process

Releases are triggered by pushing version tags (e.g., `v0.0.8`) and require
manual approval before publishing to Maven Central. The release process
does the following:

1. Validate that the version in the tag matches the versions in the `build.gradle` files
   using the `verifyVersionConsistency` task
2. Check that the version is not already published to Maven Central
3. Builds the cross-platform JAR and Android AAR using the reusable workflows
4. Publishes both artifacts to Maven Central Portal using the
   [`publish-on-central`](https://github.com/DanySK/publish-on-central) gradle
   plugin
5. Creates a GitHub release with downloadable artifacts

### Architecture

```
┌─────────────────────────────────────────────────────────────────┐
│              Git Tag Push (e.g., v0.0.8)                        │
└────────────────────┬────────────────────────────────────────────┘
                     │
                     ▼
┌─────────────────────────────────────────────────────────────────┐
│           Release Workflow (.github/workflows/release.yaml)      │
└────────────────────┬────────────────────────────────────────────┘
                     │
                     ├──────────────────┬───────────────────┐
                     ▼                  ▼                   ▼
          ┌──────────────────┐   ┌─────────────┐   ┌──────────────┐
          │   Validation     │   │ Build JAR   │   │ Build AAR    │
          │   - Version      │   │ (reusable   │   │ (reusable    │
          │     consistency  │   │  workflow)  │   │  workflow)   │
          │   - Duplicate    │   └──────┬──────┘   └──────┬───────┘
          │     check        │          │                 │
          └──────────────────┘          │                 │
                                        ▼                 ▼
                              ┌──────────────────────────────────┐
                              │  Publish Job (requires approval) │
                              │  1. Download artifacts           │
                              │  2. Sign with in-memory GPG keys │
                              │  3. Upload bundle to Portal API  │
                              │  4. Validate and release         │
                              │  5. Create GitHub release        │
                              └──────────────────────────────────┘
```

### publish-on-central Plugin

Both `lib/build.gradle.kts` and `android/build.gradle.kts` use the `publish-on-central` plugin, which:

- Publishes directly to Maven Central Portal API
- Automatically generates POM metadata from `publishOnCentral` configuration
- Handles bundle creation, validation, and release
- Supports in-memory GPG signing for CI environments

**Key tasks provided by the plugin:**
- `publishAllPublicationsToProjectLocalRepository` - Stages artifacts locally
- `zipMavenCentralPortalPublication` - Creates deployment bundle
- `releaseMavenCentralPortalPublication` - Uploads, validates, and releases to Maven Central

### Required GitHub Configuration

#### Secrets

1. **`MAVEN_CENTRAL_USERNAME`**: Maven Central Portal username
   - Obtain from: https://central.sonatype.com/
   - Ensure you have permissions for `org.automerge` namespace

2. **`MAVEN_CENTRAL_PASSWORD`**: Maven Central Portal password or user token
   - Recommended: Generate a user token instead of using password
   - Generate at: https://central.sonatype.com/account

3. **`SIGNING_KEY`**: Base64-encoded GPG private key
   ```bash
   # Export your GPG key
   gpg --export-secret-keys --armor YOUR_KEY_ID | base64 | pbcopy
   # Linux: | base64 -w0 | xclip -selection clipboard
   ```

4. **`SIGNING_PASSWORD`**: Passphrase for the GPG key

### Performing a Release

#### 1. Prepare the Release

Update version in both Gradle build files:

```bash
# lib/build.gradle.kts
version = "0.0.8"

# android/build.gradle.kts
version = "0.0.8"
```

Note: The Rust `Cargo.toml` version is only used for the native library filename suffix and doesn't need to match the Maven version.

Update `CHANGELOG.md` with release notes.

#### 2. Create and Push Tag

```bash
git add lib/build.gradle.kts android/build.gradle.kts CHANGELOG.md
git commit -m "Release v0.0.8"
git tag v0.0.8
git push origin main
git push origin v0.0.8
```

#### 3. Approve Deployment

1. Go to Actions → "Release to Maven Central" workflow
2. Wait for validation and build jobs to complete
3. Click "Review deployments" when the publish job is waiting
4. Approve the deployment

#### 4. Verify Publication

Wait 10-30 minutes for Maven Central sync, then verify:
- JAR: https://repo1.maven.org/maven2/org/automerge/automerge/
- AAR: https://repo1.maven.org/maven2/org/automerge/androidnative/

### Local Testing

Test the publishing process locally without actually publishing:

```bash
# Verify version consistency (without expected version)
./gradlew verifyVersionConsistency

# Verify version matches a specific release version
./gradlew verifyVersionConsistency -PexpectedVersion=0.0.8

# Generate POMs to verify metadata
./gradlew lib:generatePomFileForOSSRHPublication
./gradlew android:generatePomFileForReleasePublication

# Stage publications locally
./gradlew publishAllPublicationsToProjectLocalRepository

# Create bundle (doesn't upload)
./gradlew zipMavenCentralPortalPublication

# Check the bundle contents
ls -lh build/maven-central-portal/
```

The bundle will be in `build/maven-central-portal/bundle.zip` and can be inspected without publishing.
