plugins {
    id("com.diffplug.spotless") version "6.18.0" apply false
    id("org.danilopianini.publish-on-central") version "9.1.7" apply false
    id("org.jetbrains.dokka") version "2.0.0" apply false
}

fun readCargoVersion(): String {
    val cargoToml = File("$rootDir/rust/Cargo.toml")
    val version = cargoToml.readText().lines().first { it.startsWith("version") }.split("=")[1].trim().trim('"').replace(".", "_")
    return version
}

val cargoVersion = readCargoVersion()

tasks.register("verifyVersionConsistency") {
    description = "Verifies the project version matches -PexpectedVersion=X.Y.Z (used by the release workflow)."
    group = "verification"

    doLast {
        val actual = subprojects.first().version.toString()
        if (actual == "unspecified") {
            throw GradleException("Version not set in subprojects { } block")
        }
        val expected = project.findProperty("expectedVersion")?.toString()
        if (expected != null && expected != actual) {
            throw GradleException(
                """
                Version mismatch!
                  build.gradle.kts: $actual
                  Expected (from tag): $expected

                Update `version` in the root build.gradle.kts `subprojects { }` block to $expected.
                """.trimIndent()
            )
        }
        println("Version verified: $actual" + if (expected != null) " matches expected $expected" else "")
    }
}

subprojects {
    group = "org.automerge"
    version = "0.0.9"

    ext.set("cargoVersion", cargoVersion)
    ext.set("libVersionSuffix", cargoVersion.replace(".", "_"))

    // Apply shared publishing and signing config to any subproject that uses publish-on-central
    pluginManager.withPlugin("org.danilopianini.publish-on-central") {
        extensions.configure<org.danilopianini.gradle.mavencentral.PublishOnCentralExtension> {
            projectUrl.set("https://automerge.org")
            licenseName.set("MIT")
            licenseUrl.set("https://opensource.org/licenses/MIT")
            scmConnection.set("scm:git:git://github.com/automerge/automerge-java.git")
        }

        extensions.configure<PublishingExtension> {
            publications.withType<MavenPublication> {
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

        extensions.configure<SigningExtension> {
            val signingKeyId: String? = System.getenv("SIGNING_KEY_ID")
            val signingKey: String? = System.getenv("SIGNING_KEY")
            val signingPassword: String? = System.getenv("SIGNING_PASSWORD")

            if (signingKey != null && signingPassword != null) {
                if (signingKeyId != null) {
                    useInMemoryPgpKeys(signingKeyId, signingKey, signingPassword)
                } else {
                    useInMemoryPgpKeys(signingKey, signingPassword)
                }
            } else {
                useGpgCmd()
            }
        }
    }
}
