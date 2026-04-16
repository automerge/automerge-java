plugins {
    `java-library`
    id("org.danilopianini.publish-on-central")
    id("com.diffplug.spotless")
}

java {
    withJavadocJar()
    withSourcesJar()
    toolchain {
        languageVersion.set(JavaLanguageVersion.of(21))
    }
}

repositories {
    mavenCentral()
}

dependencies {
    api(project(":lib"))
    implementation("org.java-websocket:Java-WebSocket:1.5.7")
    implementation("org.slf4j:slf4j-api:2.0.9")

    testImplementation(platform("org.junit:junit-bom:5.11.4"))
    testImplementation("org.junit.jupiter:junit-jupiter-api")
    testRuntimeOnly("org.junit.jupiter:junit-jupiter-engine")
    testRuntimeOnly("org.junit.platform:junit-platform-launcher")
    testImplementation("org.slf4j:slf4j-simple:2.0.9")
}

spotless {
    java {
        importOrder()
        targetExclude("src/templates/*")
        removeUnusedImports()
        cleanthat()
        eclipse().configFile("${project.rootDir}/spotless.eclipseformat.xml")
        formatAnnotations()
    }
}

publishOnCentral {
    projectDescription.set("WebSocket network transport for Automerge repos")
    projectLongName.set("Automerge WebSocket")
}

publishing {
    publications {
        withType<MavenPublication> {
            artifactId = "automerge-websocket"
        }
    }
}

val env = providers.gradleProperty("env").getOrElse("release")
val isDev = env == "dev"

if (isDev) {
    tasks.register<Exec>("compileRustForTest") {
        workingDir = File("../rust")
        commandLine = listOf("cargo", "build")
    }

    val version = (project.extra.get("libVersionSuffix") as String)

    tasks.register("createVersionedLibForTest") {
        dependsOn("compileRustForTest")
        val debugDir = file("../rust/target/debug")
        doLast {
            listOf("libautomerge_jni" to "so", "libautomerge_jni" to "dylib", "automerge_jni" to "dll").forEach { (base, ext) ->
                val src = debugDir.resolve("$base.$ext")
                if (src.exists()) {
                    src.copyTo(debugDir.resolve("${base}_$version.$ext"), overwrite = true)
                }
            }
        }
    }

    tasks.withType<Test> {
        dependsOn("createVersionedLibForTest")
        systemProperty("java.library.path", file("../rust/target/debug").absolutePath)
    }
}

tasks.compileJava {
    options.release = 8
}

tasks.test {
    useJUnitPlatform()
    // Run tests with the websocket module dir as working dir so that
    // JsServerWrapper can find interop-test-server/ via a relative path.
    workingDir = projectDir
    testLogging {
        showStandardStreams = true
    }
}
