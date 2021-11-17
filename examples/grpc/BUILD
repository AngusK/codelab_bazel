load("@rules_proto//proto:defs.bzl", "proto_library")
load("@com_github_grpc_grpc//bazel:python_rules.bzl", "py_grpc_library", "py_proto_library",
)

package(default_visibility = ["//examples:__subpackages__"])

proto_library(
    name = "hello_world_proto",
    srcs = ["hello_world.proto"],
)

py_proto_library(
    name = "hello_world_py_pb2",
    deps = [":hello_world_proto"],
)

py_grpc_library(
    name = "hello_world_py_pb2_grpc",
    srcs = ["hello_world_proto"],
    deps = ["hello_world_py_pb2"],
)

py_binary(
    name = "greeter_server",
    srcs = ["greeter_server.py"],
    deps = [
        ":hello_world_py_pb2",
        ":hello_world_py_pb2_grpc",
        "//third_party/py:absl-py",
    ]
)

py_binary(
    name = "greeter_client",
    srcs = ["greeter_client.py"],
    deps = [
        ":hello_world_py_proto",
        "//third_party/py:absl-py",
    ]
)

