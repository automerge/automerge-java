plugins {
    id("com.android.application") version "8.13.0"
}

var projectVersion = "0.0.7"

android {
    namespace = "org.automerge.testapp"
    compileSdkVersion = "android-26"

    defaultConfig {
        applicationId = "org.automerge.testapp"
        minSdk = 26
        targetSdk = 33
        versionCode = 1
        versionName = "1.0"

        testInstrumentationRunner = "androidx.test.runner.AndroidJUnitRunner"
    }

    compileOptions {
        sourceCompatibility = JavaVersion.VERSION_1_8
        targetCompatibility = JavaVersion.VERSION_1_8
    }

    buildTypes {
        getByName("release") {
            isMinifyEnabled = false
        }
    }
}

repositories {
    mavenCentral()
    google()
}

// Support two modes:
// 1. Local dev: use project dependencies (fast iteration)
// 2. CI: use built artifacts (test actual published files)
val useBuiltArtifacts = project.hasProperty("useBuiltArtifacts")
                        && project.property("useBuiltArtifacts") == "true"

dependencies {
    if (useBuiltArtifacts) {
        // CI mode: Use built JAR and AAR as file dependencies
        // This tests the actual artifacts that would be published
        implementation(files("../lib/build/libs/automerge-${projectVersion}.jar"))
        implementation(files("../android/build/outputs/aar/android-release.aar"))
    } else {
        // Local dev mode: Use project dependencies
        // Faster iteration, automatically rebuilds on changes
        implementation(project(":lib"))
        implementation(project(":android"))
    }

    // Android test dependencies
    androidTestImplementation("androidx.test:runner:1.5.2")
    androidTestImplementation("androidx.test:rules:1.5.0")
    androidTestImplementation("androidx.test.ext:junit:1.1.5")
    androidTestImplementation("junit:junit:4.13.2")
}
