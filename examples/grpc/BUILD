load("@rules_proto_grpc//python:defs.bzl", "python_grpc_library")
load("@rules_proto//proto:defs.bzl", "proto_library")
load("@io_bazel_rules_docker//python3:image.bzl", "py3_image")
load("@io_bazel_rules_docker//container:container.bzl", "container_push")

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
    ]
)

py_binary(
    name = "greeter_client",
    srcs = ["greeter_client.py"],
    deps = [
        ":hello_world_py_proto",
    ]
)

# FIXME: this will fail when 'bazel run'. See:
# https://github.com/bazelbuild/rules_docker/issues/1561
# https://github.com/bazelbuild/rules_docker/issues/1694
py3_image(
    name = "greeter_server_image",
    srcs = ["greeter_server.py"],
    deps = [
        ":hello_world_py_proto",
    ],
    main = "greeter_server.py",
)
