package(default_visibility = ["//third_party:__subpackages__"])

load("@rules_rust//rust:rust.bzl", "rust_binary")

rust_binary(
    name = "update_py_build",
    srcs = ["gen_third_party_py_build.rs"],
    args = [
        "requirements_lock.txt",
        "third_party/py/BUILD",
    ],
    data = [
        "//:requirements_lock.txt",
        "//third_party/py:BUILD",
    ],
)
