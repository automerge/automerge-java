plugins {
    kotlin("jvm")
    id("org.danilopianini.publish-on-central")
    id("org.jetbrains.dokka")
}

// Use in-process compilation to avoid Kotlin daemon filesystem issues
tasks.withType<org.jetbrains.kotlin.gradle.tasks.KotlinCompile> {
    compilerExecutionStrategy.set(org.jetbrains.kotlin.gradle.tasks.KotlinCompilerExecutionStrategy.IN_PROCESS)
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
    implementation(project(":lib"))
    implementation("org.jetbrains.kotlinx:kotlinx-coroutines-core:1.10.2")

    testImplementation(platform("org.junit:junit-bom:5.11.4"))
    testImplementation("org.junit.jupiter:junit-jupiter-api")
    testRuntimeOnly("org.junit.jupiter:junit-jupiter-engine")
    testRuntimeOnly("org.junit.platform:junit-platform-launcher")
    testImplementation("org.slf4j:slf4j-simple:2.0.9")
}

publishOnCentral {
    projectDescription.set("Kotlin extensions for Automerge")
    projectLongName.set("Automerge Kotlin")
}

publishing {
    publications {
        withType<MavenPublication> {
            artifactId = "automerge-kotlin"
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

tasks.test {
    useJUnitPlatform()
}

dokka {
    moduleName.set("Automerge Kotlin")
    // Fail the doc build on unresolved references or other warnings so CI
    // catches broken KDoc before release.
    dokkaPublications.configureEach {
        failOnWarning.set(true)
    }
}
