package(default_visibility = ["//examples/py:__subpackages__"])

py_binary(
    name = "bin",
    srcs = ["bin.py"],
    deps = [
        ":lib",
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
