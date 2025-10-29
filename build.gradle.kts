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
