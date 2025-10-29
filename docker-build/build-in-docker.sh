#!/bin/bash
# Wrapper script to build automerge-jni in the Docker container
# This script passes all cross-compilation properties via Gradle command-line

set -e

echo "=== Building automerge-jni in Docker container ==="
echo ""
echo "Working directory: $(pwd)"
echo ""

# Auto-detect osxcross Darwin linker paths
# The binaries are named like: arm64-apple-darwin20.4-clang, x86_64-apple-darwin20.4-ld
# We need to find the actual version suffix that osxcross created
OSXCROSS_BIN="/opt/osxcross/target/bin"

if [ -d "$OSXCROSS_BIN" ]; then
    echo "=== Detecting osxcross Darwin toolchain versions ==="

    # Find the arm64 clang binary (matches arm64-apple-darwin*-clang)
    AARCH_DARWIN_CLANG=$(find "$OSXCROSS_BIN" -name "arm64-apple-darwin*-clang" -o -name "aarch64-apple-darwin*-clang" | head -n1)
    if [ -z "$AARCH_DARWIN_CLANG" ]; then
        echo "ERROR: Could not find arm64 Darwin clang in $OSXCROSS_BIN"
        exit 1
    fi

    # Extract the version prefix (e.g., arm64-apple-darwin20.4)
    AARCH_DARWIN_PREFIX=$(basename "$AARCH_DARWIN_CLANG" | sed 's/-clang$//')
    AARCH_DARWIN_LD="$OSXCROSS_BIN/${AARCH_DARWIN_PREFIX}-ld"

    # Find the x86_64 clang binary
    X86_64_DARWIN_CLANG=$(find "$OSXCROSS_BIN" -name "x86_64-apple-darwin*-clang" | head -n1)
    if [ -z "$X86_64_DARWIN_CLANG" ]; then
        echo "ERROR: Could not find x86_64 Darwin clang in $OSXCROSS_BIN"
        exit 1
    fi

    # Extract the version prefix (e.g., x86_64-apple-darwin20.4)
    X86_64_DARWIN_PREFIX=$(basename "$X86_64_DARWIN_CLANG" | sed 's/-clang$//')
    X86_64_DARWIN_LD="$OSXCROSS_BIN/${X86_64_DARWIN_PREFIX}-ld"

    echo "Detected ARM64 Darwin toolchain:"
    echo "  Clang: $AARCH_DARWIN_CLANG"
    echo "  LD:    $AARCH_DARWIN_LD"
    echo "Detected x86_64 Darwin toolchain:"
    echo "  Clang: $X86_64_DARWIN_CLANG"
    echo "  LD:    $X86_64_DARWIN_LD"
    echo ""
else
    echo "WARNING: osxcross not found at $OSXCROSS_BIN, Darwin builds will fail"
    AARCH_DARWIN_CLANG="/fake"
    AARCH_DARWIN_LD="/fake"
    X86_64_DARWIN_CLANG="/fake"
    X86_64_DARWIN_LD="/fake"
fi

echo "=== Building JAR with all native libraries ==="
echo "Using Gradle -P properties to configure cross-compilation toolchains..."
echo ""

# Build with all cross-compilation properties passed via command-line
# These override any values in local.properties without modifying files
./gradlew clean lib:jar --no-daemon \
  -Psdk.dir=/fake \
  -PndkPath=/fake \
  -PaarchLinkerPath=/usr/bin/aarch64-linux-gnu-gcc \
  -Pwin64LinkerPath=/usr/bin/x86_64-w64-mingw32-gcc \
  -Pwin32LinkerPath=/usr/bin/i686-w64-mingw32-gcc \
  -Px86_64LinuxLinkerPath=/usr/bin/gcc \
  -PaarchDarwinLinkerPath="$AARCH_DARWIN_CLANG" \
  -PaarchDarwinLdPath="$AARCH_DARWIN_LD" \
  -Px86_64DarwinLinkerPath="$X86_64_DARWIN_CLANG" \
  -Px86_64DarwinLdPath="$X86_64_DARWIN_LD" \
  "$@"

echo ""
echo "=== Build complete! ==="
echo "JAR location: lib/build/libs/"
ls -lh lib/build/libs/automerge-*.jar

echo ""
echo "=== Verifying JAR contents ==="
jar -tf lib/build/libs/automerge-*.jar | grep "native/" | sort

echo ""
echo "=== Native libraries included ==="
jar -tf lib/build/libs/automerge-*.jar | grep -E '\.(so|dll|dylib)$' | wc -l | xargs echo "Count:"
