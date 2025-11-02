plugins {
    id("com.diffplug.spotless") version "6.18.0" apply false
}

fun readCargoVersion(): String {
    val cargoToml = File("$rootDir/rust/Cargo.toml")
    val version = cargoToml.readText().lines().first { it.startsWith("version") }.split("=")[1].trim().trim('"').replace(".", "_")
    return version
}

val cargoVersion = readCargoVersion()

subprojects {
    ext.set("cargoVersion", cargoVersion)
    ext.set("libVersionSuffix", cargoVersion.replace(".", "_"))
}

tasks.register("verifyVersionConsistency") {
    description = "Verifies that lib and android modules have the same version, and optionally matches an expected version"
    group = "verification"

    doLast {
        val libProject = project(":lib")
        val androidProject = project(":android")

        val libVersion = libProject.version.toString()
        val androidVersion = androidProject.version.toString()

        if (libVersion == "unspecified" || androidVersion == "unspecified") {
            throw GradleException("Version not set in one or more modules")
        }

        if (libVersion != androidVersion) {
            throw GradleException(
                """
                Version mismatch between modules!
                  lib/build.gradle.kts: $libVersion
                  android/build.gradle.kts: $androidVersion

                Please ensure both modules have the same version.
                """.trimIndent()
            )
        }

        // Check against expected version if provided via -PexpectedVersion=X.Y.Z
        val expectedVersion = project.findProperty("expectedVersion")?.toString()
        if (expectedVersion != null) {
            if (libVersion != expectedVersion) {
                throw GradleException(
                    """
                    Version mismatch with expected version!
                      lib/build.gradle.kts and android/build.gradle.kts: $libVersion
                      Expected version: $expectedVersion

                    Please update both build.gradle.kts files to version: $expectedVersion
                    """.trimIndent()
                )
            }
            println("✅ Version verified: $libVersion matches expected version $expectedVersion")
        } else {
            println("✅ Version consistency verified: $libVersion")
        }
    }
}
