package(default_visibility = ["//examples/py:__subpackages__"])

load("@io_bazel_rules_docker//container:container.bzl", "container_push")
load("@io_bazel_rules_docker//python3:image.bzl", "py3_image")

filegroup(
    name = "srcs",
    srcs = glob(["*.py"]) + [
        "BUILD",
    ],
)

py_binary(
    name = "bin",
    srcs = ["bin.py"],
    deps = [
        ":lib",
        "//examples/proto:person_py_pb2",
        "//examples/py/fibonacci",
    ],
)

py_library(
    name = "lib",
    srcs = ["lib.py"],
)

py_test(
    name = "test",
    srcs = ["test.py"],
    deps = [
        ":lib",
        "//examples/py/fibonacci",
    ],
)

py_test(
    name = "fail",
    srcs = ["fail.py"],
    deps = [":lib"],
)

py3_image(
    name = "bin_image",
    srcs = ["bin.py"],
    deps = [":lib",
            "//examples/py/fibonacci"],
    main = "bin.py",
)

container_push(
    name = "push_bin_image",
    image = ":bin_image",
    format = "Docker",
    registry = "localhost:5000",
    repository = "codelab_bazel/py_example_image",
    tag = "dev"
)

py3_image(
    name = "fail_image",
    srcs = ["fail.py"],
    deps = [":lib"],
    main = "fail.py",
)

container_push(
    name = "push_fail_image",
    image = ":fail_image",
    format = "Docker",
    registry = "localhost:5000",
    repository = "codelab_bazel/py_example_fail_image",
    tag = "test"
)
