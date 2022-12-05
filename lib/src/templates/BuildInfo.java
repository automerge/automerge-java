package org.automerge;

class BuildInfo {
    public static String getExpectedRustLibVersion() {
        // Gradle replaces this with the version of the rust library this java library was built against
        return "${version}";
    }
}
