load("@rules_proto//proto:defs.bzl", "proto_library")
load("@build_stack_rules_proto//rules:proto_compile.bzl", "proto_compile")

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

#java_proto_library(
#    name = "address_java_proto",
#    protos = [":address_proto"],
#)
#
#java_proto_library(
#    name = "person_java_proto",
#    protos = [
#        ":person_proto",
#        ":address_proto",
#    ],
#)

################################
# Python examples below
################################
proto_compile(
    name = "address_proto_py_compile",
    outputs = ["address_pb2.py"],
    plugins = ["@build_stack_rules_proto//plugin/builtin:python"],
    proto = ":address_proto",
)

py_library(
    name = "address_py_pb2",
    srcs = ["address_pb2.py"],
    deps = [
        "//third_party/py:protobuf",
    ]
)

proto_compile(
    name = "person_proto_py_compile",
    outputs = ["person_pb2.py"],
    plugins = ["@build_stack_rules_proto//plugin/builtin:python"],
    proto = ":person_proto",
)

py_library(
    name = "person_py_pb2",
    srcs = ["person_pb2.py"],
    deps = [
        ":address_py_pb2",
        "//third_party/py:protobuf",
    ]
)