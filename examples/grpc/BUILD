load("@rules_proto//proto:defs.bzl", "proto_library")
load("@build_stack_rules_proto//rules:proto_compile.bzl", "proto_compile")

package(default_visibility = ["//examples:__subpackages__"])

proto_library(
    name = "hello_world_proto",
    srcs = ["hello_world.proto"],
)

################
# Proto compile
################
proto_compile(
    name = "hello_world_proto_py_compile",
    outputs = ["hello_world_pb2.py"],
    plugins = ["@build_stack_rules_proto//plugin/builtin:python"],
    proto = ":hello_world_proto",
)

py_library(
    name = "hello_world_py_pb2",
    srcs = ["hello_world_pb2.py"],
    deps = ["//third_party/py:protobuf"],
)

################
# GRPC compile
################
proto_compile(
    name = "hello_world_proto_py_grpc_compile",
    outputs = ["hello_world_pb2_grpc.py"],
    plugins = ["@build_stack_rules_proto//plugin/grpc/grpc:protoc-gen-grpc-python"],
    proto = ":hello_world_proto",
)

py_library(
    name = "hello_world_py_pb2_grpc",
    srcs = ["hello_world_pb2_grpc.py"],
    deps = [
        ":hello_world_py_pb2",
        "//third_party/py:grpcio"
    ],
)