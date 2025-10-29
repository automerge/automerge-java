import java.util.Properties

plugins {
    id("com.android.library") version "8.13.0"
    `maven-publish`
    signing
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
        register<MavenPublication>("automerge") {
            groupId = "org.automerge"
            artifactId = "androidnative"
            version = "0.0.7"
            afterEvaluate {
                from(components["release"])
            }
            pom {
                name.set("automerge-android-native")
                description.set("Shared libraries for automerge on android")
                url.set("automerge.org")
                licenses {
                    license {
                        name.set("MIT")
                        url.set("https://opensource.org/licenses/MIT")
                    }
                }
                developers {
                    developer {
                        id.set("alex")
                        name.set("Alex Good")
                        email.set("alex@memoryandthought.me")
                    }
                }
                scm {
                    connection.set("scm:git:git://github.com/automerge/automerge-java.git")
                    url.set("https://github.com/automerge/automerge-java")
                }
            }
        }
    }
    repositories {
        maven {
            url = uri("https://s01.oss.sonatype.org/service/local/staging/deploy/maven2/")
            credentials {
                username = project.findProperty("ossrhUsername")?.toString() ?: System.getenv("OSSRH_USERNAME")
                password = project.findProperty("ossrhPassword")?.toString() ?: System.getenv("OSSRH_PASSWORD")
            }
        }
    }
}

signing {
    sign(publishing.publications["automerge"])
    useGpgCmd()
}
