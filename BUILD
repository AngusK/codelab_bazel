load("@rules_python//python:pip.bzl", "compile_pip_requirements")

# This enables to update the requirements_lock.txt by running
# > bazel run //:requirements.update
compile_pip_requirements(
    name = "requirements",
    extra_args = ["--allow-unsafe"],
    requirements_in = "requirements.in",
    requirements_txt = "requirements_lock.txt",
)

# Export this file so the tool under third_party can read as input.
exports_files(["requirements_lock.txt"])
