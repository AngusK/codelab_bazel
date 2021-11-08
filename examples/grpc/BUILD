load("@rules_proto_grpc//python:defs.bzl", "python_grpc_library")
load("@rules_proto//proto:defs.bzl", "proto_library")

package(default_visibility = ["//examples:__subpackages__"])

proto_library(
    name = "hello_world_proto",
    srcs = ["hello_world.proto"],
)

python_grpc_library(
    name = "hello_world_py_proto",
    protos = ["hello_world_proto"],
)

py_binary(
    name = "greeter_server",
    srcs = ["greeter_server.py"],
    deps = [
        ":hello_world_py_proto",
        "//third_party/py:absl-py",
    ]
)

py_binary(
    name = "greeter_client",
    srcs = ["greeter_client.py"],
    deps = [
        ":hello_world_py_proto",
    ]
)

