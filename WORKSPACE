workspace(name = "bazel_tutorial")

load("@bazel_tools//tools/build_defs/repo:http.bzl", "http_archive")

##################################################################################
#############     Loading rules_python                               #############
##################################################################################

http_archive(
    name = "rules_python",
    sha256 = "cd6730ed53a002c56ce4e2f396ba3b3be262fd7cb68339f0377a45e8227fe332",
    url = "https://github.com/bazelbuild/rules_python/releases/download/0.5.0/rules_python-0.5.0.tar.gz",
)

##################################################################################
#############     Hermetic Python toolchain                          #############
##################################################################################
http_archive(
    name = "python3.8.3_interpreter",
    urls = ["https://www.python.org/ftp/python/3.8.3/Python-3.8.3.tar.xz"],
    sha256 = "dfab5ec723c218082fe3d5d7ae17ecbdebffa9a1aea4d64aa3a2ecdd2e795864",
    strip_prefix = "Python-3.8.3",
    patch_cmds = [
        "mkdir $(pwd)/python3.8.3_install",
				"./configure --prefix=$(pwd)/python3.8.3_install",
        "make",
        "make install",
        "ln -s python3.8.3_install/bin/python3 python_bin",
    ],
    build_file_content = """
#exports_files(["python_bin"])
exports_files(["python3.8.3_install/bin/python3"])
filegroup(
    name = "files",
    srcs = glob(["python3.8.3_install/**"], exclude = ["**/* *"]),
    visibility = ["//visibility:public"],
)
""",
)

register_toolchains("//toolchain:python_interpreter")


load("@rules_python//python:pip.bzl", "pip_parse")

# Create a central repo that knows about the dependencies needed from
# requirements_lock.txt.
pip_parse(
    name = "pip_deps",
    requirements_lock = "//:requirements_lock.txt",
		python_interpreter_target = "@python3.8.3_interpreter//:python3.8.3_install/bin/python3",
)

# Load the starlark macro which will define your dependencies.
load("@pip_deps//:requirements.bzl", "install_deps")

# Call it to define repos for your requirements.
install_deps()

##################################################################################
#############     Loading Go Rules & Gazelle                         #############
##################################################################################
http_archive(
    name = "io_bazel_rules_go",
    sha256 = "69de5c704a05ff37862f7e0f5534d4f479418afc21806c887db544a316f3cb6b",
    urls = [
        "https://mirror.bazel.build/github.com/bazelbuild/rules_go/releases/download/v0.27.0/rules_go-v0.27.0.tar.gz",
        "https://github.com/bazelbuild/rules_go/releases/download/v0.27.0/rules_go-v0.27.0.tar.gz",
    ],
)

load("@io_bazel_rules_go//go:deps.bzl", "go_register_toolchains", "go_rules_dependencies")

go_rules_dependencies()

go_register_toolchains(version = "1.16")

http_archive(
    name = "bazel_gazelle",
    sha256 = "62ca106be173579c0a167deb23358fdfe71ffa1e4cfdddf5582af26520f1c66f",
    urls = [
        "https://mirror.bazel.build/github.com/bazelbuild/bazel-gazelle/releases/download/v0.23.0/bazel-gazelle-v0.23.0.tar.gz",
        "https://github.com/bazelbuild/bazel-gazelle/releases/download/v0.23.0/bazel-gazelle-v0.23.0.tar.gz",
    ],
)

load("@bazel_gazelle//:deps.bzl", "gazelle_dependencies", "go_repository")

gazelle_dependencies()

##################################################################################
#############     Loading rules_grpc_grpc                            #############
##################################################################################

http_archive(
    name = "build_stack_rules_proto",
    strip_prefix = "rules_proto-1.0.0",
    urls = ["https://github.com/stackb/rules_proto/archive/refs/tags/1.0.0.tar.gz"],
)

http_archive(
    name = "com_github_grpc_grpc",
    #sha256 = 
    strip_prefix = "grpc-1.41.1",
    urls = ["https://github.com/grpc/grpc/archive/refs/tags/v1.41.1.tar.gz"],
)


load("@com_github_grpc_grpc//bazel:grpc_deps.bzl", "grpc_deps")

grpc_deps()

load("@com_github_grpc_grpc//bazel:grpc_extra_deps.bzl", "grpc_extra_deps")

grpc_extra_deps()


##################################################################################
#############     Loading JVM Rules - Maven                          #############
##################################################################################

RULES_JVM_EXTERNAL_TAG = "2.8"

RULES_JVM_EXTERNAL_SHA = "79c9850690d7614ecdb72d68394f994fef7534b292c4867ce5e7dec0aa7bdfad"

http_archive(
    name = "rules_jvm_external",
    sha256 = RULES_JVM_EXTERNAL_SHA,
    strip_prefix = "rules_jvm_external-%s" % RULES_JVM_EXTERNAL_TAG,
    url = "https://github.com/bazelbuild/rules_jvm_external/archive/%s.zip" % RULES_JVM_EXTERNAL_TAG,
)

load("@rules_jvm_external//:defs.bzl", "maven_install")

##################################################################################
#############     Loading junit5 test rules                          #############
##################################################################################

# This is copied from the junit team's example since currently Bazel still does not
# support junit5.
# Ref: (r5.8.0-RC1 branch)
# https://github.com/junit-team/junit5-samples/tree/r5.8.0-RC1
load(":junit5.bzl", "junit_jupiter_java_repositories", "junit_platform_java_repositories")

JUNIT_JUPITER_VERSION = "5.8.0-RC1"

JUNIT_PLATFORM_VERSION = "1.8.0-RC1"

junit_jupiter_java_repositories(
    version = JUNIT_JUPITER_VERSION,
)

junit_platform_java_repositories(
    version = JUNIT_PLATFORM_VERSION,
)

##################################################################################
#############     Loading Rules Rust                                 #############
##################################################################################

http_archive(
    name = "rules_rust",
    sha256 = "531bdd470728b61ce41cf7604dc4f9a115983e455d46ac1d0c1632f613ab9fc3",
    strip_prefix = "rules_rust-d8238877c0e552639d3e057aadd6bfcf37592408",
    urls = [
        # `main` branch as of 2021-08-23
        "https://github.com/bazelbuild/rules_rust/archive/d8238877c0e552639d3e057aadd6bfcf37592408.tar.gz",
    ],
)

load("@rules_rust//rust:repositories.bzl", "rust_repositories")

rust_repositories()

##################################################################################
#############     Loading Docker Rules                               #############
##################################################################################

http_archive(
    name = "io_bazel_rules_docker",
    sha256 = "1f4e59843b61981a96835dc4ac377ad4da9f8c334ebe5e0bb3f58f80c09735f4",
    strip_prefix = "rules_docker-0.19.0",
    urls = ["https://github.com/bazelbuild/rules_docker/releases/download/v0.19.0/rules_docker-v0.19.0.tar.gz"],
)

##################################################################################
#############     Initializing Docker Rules                          #############
##################################################################################

load(
    "@io_bazel_rules_docker//repositories:repositories.bzl",
    container_repositories = "repositories",
)

container_repositories()

load("@io_bazel_rules_docker//repositories:deps.bzl", container_deps = "deps")

container_deps()

# For py_image
load(
    "@io_bazel_rules_docker//python3:image.bzl",
    _py3_image_repos = "repositories",
)

_py3_image_repos()

# For java_image
load(
    "@io_bazel_rules_docker//java:image.bzl",
    _java_image_repos = "repositories",
)

_java_image_repos()

load("@io_bazel_rules_docker//container:container.bzl", "container_pull")

container_pull(
    name = "python-3.9-slim-bullseye",
    registry = "index.docker.io",
    repository = "library/python",
    tag = "3.9-slim-bullseye",
)
