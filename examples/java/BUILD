load("//:junit5.bzl", "java_junit5_test")

package(default_visibility = ["//examples:__subpackages__"])

java_junit5_test(
    name = "person_java_proto_test",
    srcs = ["PersonProtoTest.java"],
    test_package = "examples.java",
    deps = [
        "//examples/proto:person_java_proto",
        "//third_party/java:junit5",
    ],
)
