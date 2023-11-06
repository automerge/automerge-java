import java.util.Properties

val isDev = providers.gradleProperty("env").getOrElse("release") == "dev"

plugins {
    `java-library`
    `maven-publish`
    id("com.diffplug.spotless")
    signing
}

java {
    base.archivesBaseName = "automerge"
    withJavadocJar()
    withSourcesJar()
    toolchain {
        languageVersion.set(JavaLanguageVersion.of(11))
    }
}

// Create a configuration which specifies the `TargetJvmEnvironment` as android. We can then attach the 
// androidnative dependency to this configuration. This in turn means that the androidnative dependency
// will only be pulled in by gradle projects building for android. (Maven projects will see it as a
// dependency of type AAR which they just ignore)
val androidRuntime by configurations.creating {
    isCanBeConsumed = true
    isCanBeResolved = false
    attributes {
        attribute(Usage.USAGE_ATTRIBUTE, objects.named(Usage::class.java, Usage.JAVA_RUNTIME))
        attribute(Category.CATEGORY_ATTRIBUTE, objects.named(Category::class.java, Category.LIBRARY))
        attribute(TargetJvmEnvironment.TARGET_JVM_ENVIRONMENT_ATTRIBUTE, objects.named(TargetJvmEnvironment.ANDROID))
        attribute(Bundling.BUNDLING_ATTRIBUTE, objects.named(Bundling::class.java, Bundling.EXTERNAL))
        attribute(LibraryElements.LIBRARY_ELEMENTS_ATTRIBUTE, objects.named(LibraryElements::class.java, LibraryElements.JAR))
    }
    extendsFrom(configurations["implementation"], configurations["runtimeOnly"])
}

// Make sure the androidruntime actually produces something
artifacts {
    add("androidRuntime", tasks.jar)
}

// Add another component for the androidruntime configuration. This ensures that the published
// gradle component metadata contains the androidruntime configuration.
val javaComponent = components.findByName("java") as AdhocComponentWithVariants
javaComponent.addVariantsFromConfiguration(androidRuntime) {
    // dependencies for this variant are considered runtime dependencies
    mapToMavenScope("runtime")
    // and also optional dependencies, because we don't want them to leak
    mapToOptional()
}

// Make the runtimeElements configuration specific to STANDARD_JVM so it doesn't conflict with the androidRuntime configuration
configurations.named("runtimeElements") {
    attributes {
        attribute(TargetJvmEnvironment.TARGET_JVM_ENVIRONMENT_ATTRIBUTE, objects.named(TargetJvmEnvironment.STANDARD_JVM))
    }
}

dependencies {
    androidRuntime(project(":android"))
}

spotless {
    java {
        importOrder()
        targetExclude("src/templates/*", "build/generated/java/BuildInfo.java")
        removeUnusedImports()
        cleanthat() 
        eclipse()
        formatAnnotations()
    }
}

project.version = "0.0.6"

repositories {
    mavenCentral()
}

testing {
    suites {
        // Configure the built-in test suite
        val test by getting(JvmTestSuite::class) {
            // Use JUnit Jupiter test framework
            useJUnitJupiter("5.8.2")
        }
    }
}

// Write the version of the library to a generated java class. This is later
// used when loading the library to generate a versioned name for the temporary
// file which we load using System.load and also to check that the build
// version of the native library matches the version we were expecting.
//
// We can't use a resource for this because android strips all the resources
// out of a jar when building.
val generateVersionFile = tasks.register<Copy>("generateVersionFile") {
    // Read `rust/src/Cargo.toml` and extract the version number
    val cargoToml = file("../rust/Cargo.toml")
    val version = cargoToml.readText().lines().first { it.startsWith("version") }.split("=")[1].trim().trim('"')

    // Generate context and add it to inputs so it's regenerated if version changes
    val templateContext = mapOf("version" to version)
    inputs.properties(templateContext)

    // Actually replace the `$version` property in `BuildInfo.java`
    expand(templateContext)

    from("src/templates/BuildInfo.java")
    into("$buildDir/generated/java")
}

sourceSets {
    getByName("main") {
        java {
            // Add the generated version file from above to the sourceset
            srcDir("$buildDir/generated/java")
        }
    }
}

// Abstraction over the different prefixes and suffixes of native libraries on different platforms
interface RustOutputName {
    abstract val name: String
}
class So(): RustOutputName {
    override val name = "libautomerge_jni.so"
}
class Dll(): RustOutputName {
    override val name = "automerge_jni.dll"
}
class Dylib(): RustOutputName {
    override val name = "libautomerge_jni.dylib"
}

// A native library built by cargo
class NativeTarget(val rustTarget: String, val linkerPath: String, val output: RustOutputName, val linkerArgs: String? = null)


if  (isDev) {
    /// If we're in dev mode we don't crossbuild.

    tasks.register<Exec>("compileRustForTest") {
        workingDir = File("../rust")
        commandLine = mutableListOf("cargo", "build")
    }

    tasks.register<Copy>("createVersionedLibForTest") {
        dependsOn("compileRustForTest")
        // Read `rust/src/Cargo.toml` and extract the version number
        val cargoToml = file("../rust/Cargo.toml")

        // set in the top level build.gradle.kts
        val version = (project.extra.get("libVersionSuffix") as String)

        include("libautomerge_jni.so")
        include("libautomerge_jni.dylib")
        include("automerge_jni.dll")
        from("../rust/target/debug")
        into("../rust/target/debug")
        rename("(lib)?automerge_jni.(so|dylib|dll)$", "$1automerge_jni_$version.$2")
        //commandLine = mutableListOf("cp", "libautomerge_jni.so", "libautomerge_jni_$version.so")
    }

    tasks.withType<Test> {
        dependsOn("createVersionedLibForTest")
        systemProperty("java.library.path", "../rust/target/debug/")
    }
} else {
    val localProps = Properties()
    localProps.load(rootProject.file("local.properties").inputStream())
    val aarchLinkerPath: String by localProps
    val win64LinkerPath: String by localProps
    val win32LinkerPath: String by localProps
    val x86_64LinuxLinkerPath: String by localProps
    val aarchDarwinLinkerPath: String by localProps
    val aarchDarwinLdPath: String by localProps
    val x86_64DarwinLinkerPath: String by localProps
    val x86_64DarwinLdPath: String by localProps

    // The list of all the targets we build from rust. 
    //
    // Note that this does not include android targets because we distribute them
    // separately via an AAR.
    val nativeTargets = arrayOf(
        NativeTarget("x86_64-unknown-linux-gnu",  x86_64LinuxLinkerPath, So()),
        NativeTarget("aarch64-unknown-linux-gnu", aarchLinkerPath, So()),
        NativeTarget("x86_64-pc-windows-gnu", win64LinkerPath, Dll()),
        NativeTarget("i686-pc-windows-gnu", win32LinkerPath, Dll()),
        // For Darwin we use the -fuse-ld=ld flag to force the ld clang uses to
        // be the linker for the particular architecture we're building for.
        NativeTarget(
            "aarch64-apple-darwin",
            aarchDarwinLinkerPath,
            Dylib(),
            "-fuse-ld=${aarchDarwinLdPath}"
        ),
        NativeTarget(
            "x86_64-apple-darwin",
            x86_64DarwinLinkerPath,
            Dylib(),
            "-fuse-ld=${x86_64DarwinLdPath}"
        )
    )

    // The tasks which compile the native libraries. The above list of targets is
    // mapped to this list. For each target we register a task which performs the
    // compilation and a CopySpec which will place the built library in the resources
    // folder which `LoadLibrary` expects for the given architecture
    data class BuildNativeTask(val copy: CopySpec, val compileTask: TaskProvider<Exec>)
    val nativeTasks: MutableList<BuildNativeTask> = mutableListOf()

    for (target in nativeTargets) {
        val output = "../rust/target/${target.rustTarget}/release/${target.output.name}"
        val env: MutableMap<String, String> = mutableMapOf(
            "CARGO_TARGET_${target.rustTarget.uppercase().replace("-", "_")}_LINKER" to target.linkerPath,
        )
        if (target.linkerArgs != null) {
            env["RUSTFLAGS"] = "-C link-arg=${target.linkerArgs}"
        }
        val task: TaskProvider<Exec> = tasks.register("compile-native-${target.rustTarget}", Exec::class) {
            workingDir = File("../rust")
            environment(env)
            commandLine = mutableListOf("cargo", "build", "--profile", "release", "--target", target.rustTarget)
            outputs.file(output)
            outputs.upToDateWhen { false }
        }
        val spec = copySpec {
            from("../rust/target/${target.rustTarget}/release/") {
                include("${target.output.name}")
                into("native/${target.rustTarget}/")
            }
        }
        nativeTasks.add(BuildNativeTask(spec, task))
    }
    tasks.processResources {
        for (nativeTask in nativeTasks) {
            dependsOn(nativeTask.compileTask)
            with(nativeTask.copy)
        }
    }
}

tasks.withType<Test> {
    this.testLogging {
        this.showStandardStreams = true
    }
}

tasks.compileJava {
    dependsOn(generateVersionFile)
}

tasks.named("sourcesJar") {
    mustRunAfter(generateVersionFile)
}

publishing {
    publications {
        create<MavenPublication>("mavenJava") {
            groupId = "org.automerge"
            artifactId = "automerge"
            version = project.version.toString()
            from(components["java"])
            pom {
                name.set("automerge")
                description.set("Automerge is a JSON-like data structure that can be modified concurrently by different users, and merged again automatically.")
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
                // By default the "androidnative" dependency is rendered to the POM as an optional dependency. 
                // The dependency is an AAR though, not a JAR and this is not added to the POM. This means
                // that maven will attempt to download the jar and then get upset when it can't find it.
                // This hack adds the "<type>aar</type>" element to the dependency in the POM so that maven
                // doesn't try and download it (unless building an android project, when maybe it works?)
                withXml {
                    // The names of nodes in the groovy object model use QNames, not raw
                    // strings. This helper creates a qname in the maven POM namespace
                    fun xmlName(name: String): groovy.namespace.QName {
                        return groovy.namespace.QName("http://maven.apache.org/POM/4.0.0", name)
                    }
                    val dependenciesNode = asNode().children().find {
                        it is groovy.util.Node && it.name() == xmlName("dependencies")
                    }
                    fun artifactId(depNode: groovy.util.Node): String? {
                        val children: List<groovy.util.Node> =  depNode.children().mapNotNull { 
                            when (it) {
                                is groovy.util.Node -> it
                                else -> null
                            }
                        }
                        children.first{
                            it.name() == xmlName("artifactId")
                        }?.let {
                            return (it.value() as groovy.util.NodeList).text()
                        }
                        return null
                    }
                    if (dependenciesNode != null && dependenciesNode is groovy.util.Node) {
                        val depNode = dependenciesNode.children().first {
                            it is groovy.util.Node && artifactId(it) == "androidnative"
                        } as groovy.util.Node
                        depNode.appendNode(xmlName("type"), "aar")
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
}

signing {
    sign(publishing.publications["mavenJava"])
    useGpgCmd()
}
