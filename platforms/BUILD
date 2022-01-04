package(default_visibility = ["//visibility:public"])

platform(
    name = "hermetic_python",
    constraint_values = [
        "//platforms/python_interpreter:hermetic",
        "@platforms//os:linux",
        "@platforms//cpu:x86_64",
    ],
)

py_test(
    name = "version_test",
    srcs = ["version_test.py"],
    target_compatible_with = [
        "//platforms/python_interpreter:hermetic",
    ],
)
