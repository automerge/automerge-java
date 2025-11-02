import java.util.Properties

plugins {
    id("com.android.library") version "8.13.0"
    id("org.danilopianini.publish-on-central") version "9.1.7"
}

// Load properties with priority: -P flags > local.properties
// This allows CI/Docker to override via command-line without modifying local.properties
val ndkPath = requirePropertyWithFallback("ndkPath")
val ndkApiLevel = 26
val rustDir = "${projectDir}/../rust"

data class AndroidLib(val rustTarget: String, val abiName: String, val jniFolderName: String)
val androids = arrayOf(
    AndroidLib("armv7-linux-androideabi", "armv7a-linux-androideabi", "armeabi-v7a"),
    AndroidLib("aarch64-linux-android", "aarch64-linux-android", "arm64-v8a"),
    AndroidLib("i686-linux-android", "i686-linux-android", "x86"),
    AndroidLib("x86_64-linux-android", "x86_64-linux-android", "x86_64"),
)

data class AndroidRustLib(val android: AndroidLib, val compileTask: TaskProvider<Exec>) {
    val rustOutputPath: String
        get() = "$rustDir/target/${android.rustTarget}/release/libautomerge_jni.so"
    val jniOutputPath: String
        get() = "${android.jniFolderName}/"
}
val androidLibs: MutableList<AndroidRustLib> = mutableListOf()

for (android in androids) {
    val task: TaskProvider<Exec> = tasks.register("compileAndroid${android.abiName}", Exec::class) {
        var toolChainLocation = ndkPath + "/toolchains/llvm/prebuilt/linux-x86_64/bin"
        // The LINKER environment variable we pass to cargo is the target
        // triple uppercased with dashes turned to underscores
        val linker = android.rustTarget.uppercase().replace("-", "_")
        workingDir = File("../rust")
        environment("CC", "${toolChainLocation}/clang")
        environment("CXX", "${toolChainLocation}/clang++")
        environment("CARGO_TARGET_${linker}_LINKER", "${toolChainLocation}/${android.abiName}${ndkApiLevel}-clang")
        environment("CARGO_BUILD_BUILD_DIR", "../rust/crossbuild/${android.rustTarget}")
        commandLine = mutableListOf("cargo", "build", "--profile", "release", "--target", android.rustTarget)
        outputs.file("../rust/target/$android/release/libautomerge_jni.so")
        outputs.upToDateWhen { false }
    }
    androidLibs.add(AndroidRustLib(android, task))
}

repositories {
    mavenCentral()
    google()
}

group = "org.automerge"
version = "0.0.7"

publishOnCentral {
    projectDescription.set("Shared libraries for automerge on android")
    projectLongName.set("Automerge Android Native Libraries")
    projectUrl.set("https://automerge.org")
    licenseName.set("MIT")
    licenseUrl.set("https://opensource.org/licenses/MIT")
    scmConnection.set("scm:git:git://github.com/automerge/automerge-java.git")
}

android {
    publishing {
        singleVariant("release")
    }
    namespace ="org.automerge.androidnative"
    compileSdkVersion = "android-26"
    defaultConfig {
        minSdk = 26
    }

    compileOptions {
        sourceCompatibility = JavaVersion.VERSION_1_8
        targetCompatibility = JavaVersion.VERSION_1_8
    }
    sourceSets {
        getByName("main") {
            java {
                srcDir("./src/main/java")
            }
            jniLibs {
                srcDir("build/jniLibs")
            }
            manifest.srcFile("./AndroidManifest.xml")
        }
    }
}

tasks.register<Copy>("copyJniLibs") {
    destinationDir = File("${layout.buildDirectory.get().asFile}/jniLibs")
    // set in the top level build.gradle.kts
    val version = (project.extra.get("libVersionSuffix") as String)
    for (androidLib in androidLibs) {
        dependsOn(androidLib.compileTask)
        from(androidLib.rustOutputPath) {
            into(androidLib.jniOutputPath)
        }
    }
    rename("libautomerge_jni.so$", "libautomerge_jni_$version.so")
}

tasks.configureEach(Action<Task> {
    if (this.name.startsWith("mergeReleaseJniLibFolders") || this.name.startsWith("mergeDebugJniLibFolders")) {
        this.dependsOn("copyJniLibs")
    }
})

publishing {
    publications {
        create<MavenPublication>("release") {
            artifactId = "androidnative"
            afterEvaluate {
                from(components["release"])
            }
            pom {
                developers {
                    developer {
                        id.set("alex")
                        name.set("Alex Good")
                        email.set("alex@memoryandthought.me")
                    }
                }
            }
        }
    }
}

signing {
    // For CI: use in-memory keys from environment
    val signingKey: String? = System.getenv("SIGNING_KEY")
    val signingPassword: String? = System.getenv("SIGNING_PASSWORD")

    if (signingKey != null && signingPassword != null) {
        useInMemoryPgpKeys(signingKey, signingPassword)
    } else {
        // For local: use GPG agent
        useGpgCmd()
    }
}
