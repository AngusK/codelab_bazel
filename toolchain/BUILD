load("@rules_python//python:defs.bzl", "py_runtime", "py_runtime_pair")

py_runtime(
    name = "python3.8.3_runtime",
    files = ["@python3.8.3_interpreter//:files"],
    interpreter = "@python3.8.3_interpreter//:python_bin",
    python_version = "PY3",
    visibility = ["//visibility:public"],
)

py_runtime_pair(
    name = "py_runtime_pair_py3.8.3",
    py2_runtime = None,
    py3_runtime = ":python3.8.3_runtime",
)

toolchain(
    name = "py",
    toolchain = ":py_runtime_pair_py3.8.3",
    toolchain_type = "@bazel_tools//tools/python:toolchain_type",
)
