package(default_visibility = ["//examples/py:__subpackages__"])

load("@build_stack_rules_proto//rules:proto_compile.bzl", "proto_compile")
load("@io_bazel_rules_docker//container:container.bzl", "container_push")
load("@io_bazel_rules_docker//python3:image.bzl", "py3_image")

py_binary(
    name = "bin",
    srcs = ["bin.py"],
    target_compatible_with = [
        "//platforms/python_interpreter:hermetic",
    ],
    deps = [
        ":lib",
        "//examples/proto:person_py_pb2",
        "//examples/py/fibonacci",
    ],
)

py_library(
    name = "lib",
    srcs = ["lib.py"],
    target_compatible_with = [
        "//platforms/python_interpreter:hermetic",
    ],
)

py_test(
    name = "test",
    srcs = ["test.py"],
    target_compatible_with = [
        "//platforms/python_interpreter:hermetic",
    ],
    deps = [
        ":lib",
        "//examples/py/fibonacci",
    ],
)

py_library(
    name = "greeter_service",
    srcs = ["greeter_service.py"],
    target_compatible_with = [
        "//platforms/python_interpreter:hermetic",
    ],
    deps = [
        "//examples/grpc:hello_world_py_pb2",
        "//examples/grpc:hello_world_py_pb2_grpc",
        "//third_party/py:absl-py",
    ],
)

py_test(
    name = "greeter_service_test",
    srcs = ["greeter_service_test.py"],
    target_compatible_with = [
        "//platforms/python_interpreter:hermetic",
    ],
    deps = [
        ":greeter_service",
        "//examples/grpc:hello_world_py_pb2",
        "//examples/grpc:hello_world_py_pb2_grpc",
        "//third_party/py:grpcio",
        "//third_party/py:grpcio-testing",
    ],
)

py_binary(
    name = "greeter_server",
    srcs = ["greeter_server.py"],
    target_compatible_with = [
        "//platforms/python_interpreter:hermetic",
    ],
    deps = [
        ":greeter_service",
        "//examples/grpc:hello_world_py_pb2_grpc",
        "//third_party/py:absl-py",
        "//third_party/py:grpcio",
    ],
)

py_binary(
    name = "greeter_client",
    srcs = ["greeter_client.py"],
    target_compatible_with = [
        "//platforms/python_interpreter:hermetic",
    ],
    deps = [
        "//examples/grpc:hello_world_py_pb2",
        "//examples/grpc:hello_world_py_pb2_grpc",
        "//third_party/py:absl-py",
        "//third_party/py:grpcio",
    ],
)

py3_image(
    name = "greeter_server_image",
    srcs = ["greeter_server.py"],
    main = "greeter_server.py",
    deps = [
        "//examples/grpc:hello_world_py_pb2",
        "//examples/grpc:hello_world_py_pb2_grpc",
        "//third_party/py:absl-py",
        "//third_party/py:grpcio",
    ],
)

container_push(
    name = "push_greeter_server",
    format = "Docker",
    image = ":greeter_server_image",
    registry = "localhost:5000",
    repository = "codelab_bazel/py_example_image",
    tag = "dev",
)
