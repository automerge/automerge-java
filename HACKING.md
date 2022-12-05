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

## Testing without a full build

The complete build requires building release binaries for every architecture.
This can be quite time consuming and also requries a lot of toolchain setup.
If you just want to make some changes to the java/rust code and test it you
can run `./gradlew lib:test -Penv=dev`. The `env=dev` part will build a debug
build of the Rust project for the current architecture and make it available
on the library path of the java tests. This is much, much faster.

## Building

In general you will need a toolchain set up for all the following platforms:

* x86_64-pc-windows-gnu
* i686-pc-windows-gnu
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






