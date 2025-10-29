# Cross-Compilation Docker Image

This directory contains the Dockerfile for building a Docker image with all the cross-compilation toolchains needed to build automerge-jni for all supported platforms.

## Prerequisites

You need a macOS SDK to build this image. Due to Apple's licensing restrictions, the SDK cannot be included in this repository or distributed in a public Docker image.

## Building the Image

### Step 1: Obtain the macOS SDK

You have two options:

**Option A: Extract from Xcode (requires a Mac)**

```bash
# Ensure Xcode Command Line Tools are installed
xcode-select --install

# Navigate to the SDK directory
cd $(xcode-select -p)/Platforms/MacOSX.platform/Developer/SDKs

# Create a tarball
tar -czf ~/MacOSX$(sw_vers -productVersion | cut -d. -f1).sdk.tar.xz MacOSX.sdk
```

**Option B: Use osxcross's packaging script**

```bash
# Clone osxcross
git clone https://github.com/tpoechtrager/osxcross
cd osxcross

# Run the SDK packaging script (requires Xcode)
./tools/gen_sdk_package.sh

# This creates MacOSX*.sdk.tar.xz in the current directory
```

### Step 2: Place the SDK in the build context

```bash
# From the repository root
cd /path/to/automerge-jni
mkdir -p docker-build/sdk
cp ~/MacOSX*.sdk.tar.xz docker-build/sdk/
```

The `sdk/` directory is git-ignored, so the SDK will not be committed to the repository.

### Step 3: Build the Docker image

```bash
cd docker-build
docker build -t automerge-cross:latest .
```

This will take approximately 15-30 minutes depending on your machine. The build process:
- Installs all cross-compilation toolchains (Linux ARM64, Windows x86/x64, macOS x86/ARM64)
- Installs Rust with all target platforms
- Builds osxcross with the provided macOS SDK

### Step 4: Push to GitHub Container Registry

```bash
# Create a Personal Access Token if you haven't already:
# 1. Go to GitHub Settings → Developer settings → Personal access tokens → Tokens (classic)
# 2. Click "Generate new token (classic)"
# 3. Select scopes: write:packages, read:packages, delete:packages
# 4. Generate and copy the token

# Login to GitHub Container Registry
echo $GITHUB_TOKEN | docker login ghcr.io -u YOUR_USERNAME --password-stdin

# Tag the image (use lowercase for org/repo names)
docker tag automerge-cross:latest ghcr.io/YOUR_ORG/automerge-cross:latest

# Push the image
docker push ghcr.io/YOUR_ORG/automerge-cross:latest
```

### Step 5: Make the image private (required due to Apples licensing terms)

If this is for a private repository:

1. Go to https://github.com/YOUR_ORG?tab=packages
2. Click on the "automerge-cross" package
3. Go to "Package settings"
4. Change visibility to "Private"

### Step 6: Configure GitHub Secrets

For GitHub Actions to use the image, add these secrets to your repository:

1. Go to your repository on GitHub
2. Navigate to Settings → Secrets and variables → Actions
3. Add the following secrets:
   - `GHCR_USERNAME`: Your GitHub username
   - `GHCR_TOKEN`: Your Personal Access Token (needs `read:packages` scope)

## Using the Image in CI

The image is referenced in `.github/workflows/full-build.yaml`:

```yaml
container:
  image: ghcr.io/YOUR_ORG/automerge-cross:latest
  credentials:
    username: ${{ secrets.GHCR_USERNAME }}
    password: ${{ secrets.GHCR_TOKEN }}
```

Replace `YOUR_ORG` with your GitHub organization or username.

## Updating the Image

If you need to update the toolchains or dependencies:

1. Make changes to the `Dockerfile`
2. Rebuild the image: `docker build -t automerge-cross:latest .`
3. Tag with a version: `docker tag automerge-cross:latest ghcr.io/YOUR_ORG/automerge-cross:v2`
4. Push both tags:
   ```bash
   docker push ghcr.io/YOUR_ORG/automerge-cross:latest
   docker push ghcr.io/YOUR_ORG/automerge-cross:v2
   ```

## Testing the Image Locally

Once you've built the image, you can test it locally to verify that cross-compilation works:

### Quick Test (Recommended)

```bash
cd /path/to/automerge-jni

docker run --rm \
  -v "$(pwd):/workspace" \
  -w /workspace \
  automerge-cross:latest \
  ./docker-build/build-in-docker.sh
```

This will:
1. Mount your source directory into the container
2. Pass all cross-compilation toolchain paths via Gradle `-P` properties
3. Build all native libraries for all 6 platforms
4. Create the JAR with all libraries bundled
5. Verify the JAR contains all native libraries

**Your local files remain untouched** - the build script uses Gradle command-line properties which override `local.properties` without modifying any files. Build artifacts are created in `lib/build/` as usual.

### Build with Custom Gradle Arguments

You can pass additional Gradle arguments to the build script:

```bash
docker run --rm \
  -v "$(pwd):/workspace" \
  -w /workspace \
  automerge-cross:latest \
  ./docker-build/build-in-docker.sh --info --stacktrace
```

### Interactive Testing

For manual testing and debugging:

```bash
docker run --rm -it \
  -v "$(pwd):/workspace" \
  -w /workspace \
  automerge-cross:latest \
  bash
```

Then inside the container:
```bash
# Use the build script (recommended)
./docker-build/build-in-docker.sh

# Or manually run Gradle with properties
./gradlew lib:jar \
  -Psdk.dir=/fake \
  -PndkPath=/fake \
  -PaarchLinkerPath=/usr/bin/aarch64-linux-gnu-gcc \
  -Pwin64LinkerPath=/usr/bin/x86_64-w64-mingw32-gcc \
  # ... (see build-in-docker.sh for full list)

# Or manually build specific Rust targets
cd rust
cargo build --release --target x86_64-pc-windows-gnu
```

## Image Contents

The built image includes:

- **Base:** Ubuntu 22.04
- **Java:** OpenJDK 21
- **Rust:** Latest stable with targets for:
  - x86_64-unknown-linux-gnu
  - aarch64-unknown-linux-gnu
  - x86_64-pc-windows-gnu
  - i686-pc-windows-gnu
  - x86_64-apple-darwin
  - aarch64-apple-darwin
- **Cross-compilation toolchains:**
  - GCC for Linux ARM64 (`aarch64-linux-gnu-gcc`)
  - MinGW-w64 for Windows x86 and x64
  - osxcross for macOS x86_64 and ARM64
