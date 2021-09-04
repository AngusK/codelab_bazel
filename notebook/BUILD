load("@io_bazel_rules_docker//container:container.bzl", "container_push")
load("@io_bazel_rules_docker//python3:image.bzl", "py3_image")
load("@rules_python//python:defs.bzl", "py_binary", "py_library")

py_library(
    name = "all_deps_library",
    deps = [
        "//third_party/py:tensorflow",
    ],
)

# To run this notebook with our own dependency:
# > bazel build //notebook:main
# > bazel-bin/notebook/main
py_binary(
    name = "main",
    srcs = ["main.py"],
    deps = [
        ":all_deps_library",
        "//third_party/py:notebook",
    ],
)

py3_image(
    name = "image",
    srcs = ["main.py"],
    base = "@my_py3_base//image",
    main = "main.py",
    deps = [
        ":all_deps_library",
        "//third_party/py:notebook",
    ],
)

# To run this image:
# > docker run -p 8888:8888 \
# >     <repository> \
# >     --ip 0.0.0.0 --allow-root
container_push(
    name = "push_image",
    format = "Docker",
    image = ":image",
    registry = "gcr.io",
    repository = "<repository_name>",
    tag = "latest",
)

