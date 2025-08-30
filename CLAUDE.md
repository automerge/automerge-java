# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

This is automerge-jni, a Java implementation of Automerge (a CRDT library) that wraps the Rust automerge implementation via JNI. The project builds native libraries for multiple platforms and packages them into a JAR for distribution.

## Architecture

The project consists of three main components:

- **`/rust`**: Rust crate that builds a shared library (`libautomerge_jni.so/.dylib/.dll`) interfacing with JNI
- **`/lib`**: Main Gradle module containing Java code that loads the appropriate native library based on platform
- **`/android`**: Gradle module that creates an AAR with `jniLibs` for Android builds

The Java code automatically detects the platform and loads the correct native library. For Android, variant-aware matching adds a dependency on the Android AAR when building for Android.

## Development Commands

### Quick Development Testing
```bash
./gradlew lib:test -Penv=dev
```
This builds a debug version of the Rust library for the current architecture only, making it available on the library path for Java tests. Much faster than a full cross-platform build.

### Full Build (All Platforms)
```bash
./gradlew build
```
Builds release binaries for all supported architectures. Requires extensive cross-compilation toolchain setup (see HACKING.md).

### Code Formatting
```bash
./gradlew spotlessApply
```
Formats Java code using Spotless with Eclipse formatter.

### Running Tests
```bash
./gradlew test
```
Runs the Java test suite.

## Required Configuration

For full builds, you need a `local.properties` file in the root with linker paths for cross-compilation:
```
aarchLinkerPath=<path to aarch64-linux-gnu-gcc>
win64LinkerPath=<path to x86_64-w64-mingw32-gcc>
win32LinkerPath=<path to i686-w64-mingw32-gcc>
x86_64LinuxLinkerPath=<path to gcc>
aarchDarwinLinkerPath=<osxcross path>/target/bin/arm64-apple-darwin20.4-clang
aarchDarwinLdPath=<osxcross path>/target/bin/x86_64-apple-darwin20.4-ld
x86_64DarwinLinkerPath=<osxcross path>/target/bin/x86_64-apple-darwin20.4-clang
x86_64DarwinLdPath=<osxcross path>/target/bin/x86_64-apple-darwin20.4-ld
```

For Android builds, also add:
```
sdk.dir=<android SDK dir>
ndkPath=<path to NDK>
```

## Key Java Classes

- `Document`: Main automerge document class
- `Transaction`: For making changes to documents
- `DocHandle`: Repository document handle with URL-based addressing  
- `Repo`: Repository for managing multiple documents with networking
- `AutomergeSys`: Low-level JNI interface to Rust code
- `LoadLibrary`: Handles platform detection and native library loading

## Platform Support

Supported target platforms:
- x86_64-pc-windows-gnu / i686-pc-windows-gnu
- x86_64-unknown-linux-gnu / aarch64-unknown-linux-gnu  
- x86_64-apple-darwin / aarch64-apple-darwin
- Android: armv7-linux-androideabi, aarch64-linux-android, i686-linux-android, x86_64-linux-android

## Version Management

Library version is read from `rust/Cargo.toml` and used to generate versioned native library names and Java build metadata.