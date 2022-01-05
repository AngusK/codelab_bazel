load("@io_bazel_rules_docker//python3:image.bzl", "py3_image")
load("@io_bazel_rules_docker//container:container.bzl", "container_image")

package(default_visibility = ["//examples:__subpackages__"])

container_image(
    name = "grpc_base_image_with_python",
    base = "@python-3.9-slim-bullseye//image",
    symlinks = {
        "/usr/bin/python": "/usr/local/bin/python3",
        "/usr/bin/python3": "/usr/local/bin/python3",
    },
)

# Note: Default distroless Python images can not be used as the
# glibc version is different if built from Ubuntu 20.04 environment.
#
# If this fails to run, try remove 'base' below.
#
# See:
# https://github.com/bazelbuild/rules_docker/issues/1561
# https://github.com/bazelbuild/rules_docker/issues/1694
py3_image(
    name = "greeter_server_py_image",
    srcs = ["//examples/py:greeter_server.py"],
    base = ":grpc_base_image_with_python",
    main = "//examples/py:greeter_server.py",
    deps = [
        "//examples/grpc:hello_world_py_pb2",
        "//examples/grpc:hello_world_py_pb2_grpc",
        "//examples/py:greeter_service",
        "//third_party/py:absl-py",
        "//third_party/py:grpcio",
    ],
)

py3_image(
    name = "greeter_client_py_image",
    srcs = ["//examples/py:greeter_client.py"],
    base = ":grpc_base_image_with_python",
    main = "//examples/py:greeter_client.py",
    deps = [
        "//examples/grpc:hello_world_py_pb2",
        "//examples/grpc:hello_world_py_pb2_grpc",
        "//third_party/py:absl-py",
        "//third_party/py:grpcio",
    ],
)
