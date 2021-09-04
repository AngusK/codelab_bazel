load("@rules_proto_grpc//python:defs.bzl", "python_proto_library")
load("@rules_proto_grpc//java:defs.bzl", "java_proto_library")
load("@rules_proto//proto:defs.bzl", "proto_library")

package(default_visibility = ["//examples:__subpackages__"])

proto_library(
    name = "address_proto",
    srcs = ["address.proto"],
)

proto_library(
    name = "person_proto",
    srcs = ["person.proto"],
    deps = [
        ":address_proto",
        # Well known protos should be included as deps in the
        # proto_library rules of the source files importing them.
        # A list of all @com_google_protobuf well known protos can
        # seen with:
        # `bazel query 'kind(proto_library, @com_google_protobuf//:all)'`
        "@com_google_protobuf//:any_proto",
    ],
)

java_proto_library(
    name = "address_java_proto",
    protos = [":address_proto"],
)

java_proto_library(
    name = "person_java_proto",
    protos = [
        ":person_proto",
        ":address_proto",
    ],
)

python_proto_library(
    name = "py_protos",
    protos = [
        ":address_proto",
        ":person_proto",
    ],
)
