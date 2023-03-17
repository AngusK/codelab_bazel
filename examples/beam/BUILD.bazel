package(default_visibility = ["//build_examples:__subpackages__"])

py_binary(
    name = "word_count",
    srcs = ["word_count.py"],
    args = [
        "--input",
        "requirements_lock.txt",
        "--output",
        "/tmp/wordcount_tmp.txt",
    ],
    data = ["//:requirements_lock.txt"],
    deps = [
        "//third_party/py:apache-beam",
    ],
)
