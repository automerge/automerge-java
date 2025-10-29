import org.gradle.api.Project
import org.gradle.api.GradleException
import java.util.Properties
import java.io.File

/**
 * Load a property with priority:
 * 1. Command-line -P flags (highest priority, for CI/Docker)
 * 2. local.properties file (fallback, for local development)
 */
fun Project.getPropertyWithFallback(key: String): String? {
    // Check for project property first (includes -P flags and gradle.properties)
    val projectProp = findProperty(key) as String?
    if (projectProp != null) {
        return projectProp
    }

    // Fall back to local.properties
    val localPropsFile = rootProject.file("local.properties")
    if (localPropsFile.exists()) {
        val localProps = Properties()
        localProps.load(localPropsFile.inputStream())
        return localProps.getProperty(key)
    }

    return null
}

/**
 * Load a required property with fallback to local.properties.
 * Throws an exception if the property is not found.
 */
fun Project.requirePropertyWithFallback(key: String): String {
    return getPropertyWithFallback(key)
        ?: throw GradleException("Required property '$key' not found in project properties or local.properties")
}
