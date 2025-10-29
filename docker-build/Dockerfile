FROM ubuntu:22.04

# Prevent interactive prompts
ENV DEBIAN_FRONTEND=noninteractive

# Install base dependencies
RUN apt-get update && apt-get install -y \
    build-essential \
    curl \
    git \
    openjdk-21-jdk \
    gcc-aarch64-linux-gnu \
    g++-aarch64-linux-gnu \
    gcc-mingw-w64-x86-64 \
    g++-mingw-w64-x86-64 \
    gcc-mingw-w64-i686 \
    g++-mingw-w64-i686 \
    libxml2-dev \
    libssl-dev \
    zlib1g-dev \
    llvm-dev \
    clang \
    cmake \
    python3 \
    && rm -rf /var/lib/apt/lists/*

# Install Rust
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
ENV PATH="/root/.cargo/bin:${PATH}"

# Add Rust targets
RUN rustup target add x86_64-unknown-linux-gnu && \
    rustup target add aarch64-unknown-linux-gnu && \
    rustup target add x86_64-pc-windows-gnu && \
    rustup target add i686-pc-windows-gnu && \
    rustup target add aarch64-pc-windows-gnullvm && \
    rustup target add x86_64-apple-darwin && \
    rustup target add aarch64-apple-darwin

# Build osxcross
RUN git clone https://github.com/tpoechtrager/osxcross /opt/osxcross

# Copy the macOS SDK from the build context
# You must place MacOSX*.sdk.tar.xz in docker-build/sdk/ before building
COPY sdk/*.sdk.tar.xz /opt/osxcross/tarballs/

# Build osxcross with the SDK
WORKDIR /opt/osxcross
RUN UNATTENDED=1 ./build.sh

ENV PATH="/opt/osxcross/target/bin:${PATH}"

# Add the llvm-mingw libraries
RUN curl -L curl -L https://github.com/mstorsjo/llvm-mingw/releases/download/20251021/llvm-mingw-20251021-ucrt-ubuntu-22.04-x86_64.tar.xz > llvm-mingw.tar.xz
RUN mkdir -p /opt/llvm-mingw
RUN tar -xf llvm-mingw.tar.xz -C /opt/llvm-mingw

ENV AARCH64_MINGW_PATH=/opt/llvm-mingw/llvm-mingw-20251021-ucrt-ubuntu-22.04-x86_64/bin/aarch64-w64-mingw32-gcc

WORKDIR /workspace
