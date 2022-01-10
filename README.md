# codelab_bazel
A simple example to show how to use Bazel with Python, and put a Python build target into a Docker image to run.

## Examples

All code examples are put under [examples](https://github.com/AngusK/codelab_bazel/tree/master/examples)

## Python

Before you build/run any Python examples, check [Python with Bazel](docs/python_w_bazel.md).

### Python Toolchain

This example shows how to use a "hermetic" Python interpreter. It's inspired by:
[Hermetic Python with Bazel](https://thethoughtfulkoala.com/posts/2020/05/16/bazel-hermetic-python.html)

The hermetic Python interpreter will be invoked by ```--platforms=//platforms:hermetic_python``` flag.
For example,
```
bazel run --platforms=//platforms:hermetic_python //examples/py:bin
```
This argument can be put into ```.bazelrc``` like
[.bazelrc](https://github.com/AngusK/codelab_bazel/blob/master/.bazelrc).
See [platforms](https://github.com/AngusK/codelab_bazel/tree/master/platforms) directory for details.



A test was put to ensure the hermetic Python version fixed:
```
bazel test //platforms:version_test
```
See [platforms/version_test.py](https://github.com/AngusK/codelab_bazel/blob/master/platforms/version_test.py).


## Python package management

### Complete work flow

1. Update ```requirements.in```
1. Run ```bazel run //:requirements.update```
1. Run ```bazel run //third_party:udpate_py_build```

The Python package management has two parts: a) package downloading and b) dependency management.

### Package download

Package download is done by [rules_python](https://github.com/bazelbuild/rules_python). The first-order
dependency is inside ```requirements.in``` and the full list of the packages is in
```requirements_lock.txt``` generated by ```pip-compile``` tool. ```rules_python``` provides a simple rule
to run ```pip-compile``` to update ```requirements_lock.txt```.

### Package dependency

The dependencies between the packages is already stated in ```requirements_lock.txt``` but they are not
automatically pulled in. To solve this issue, the
[third_party/py/BUILD](https://github.com/AngusK/codelab_bazel/blob/angus-add-python-part-docs/third_party/py/BUILD)
is a layer to manage the inter-package dependencies. For example, if one adds ```absl-py``` package directly
as designed by rules_python:
```
deps = [
    requirement("absl-py"),
],
```
, the package ```six``` which is used by ```absl-py``` also needs to be added:
```
deps = [
    requirement("absl-py"),
    requirement("six"),
],
```

To make this automatic and better managed, all Python packages have their own build targets listed in
[third_party/py/BUILD](https://github.com/AngusK/codelab_bazel/blob/angus-add-python-part-docs/third_party/py/BUILD)
so that it can be done by:
```
deps = [
    "//third_party/py:absl-py",
],
```
The target ```//third_party/py:absl-py``` has all its dependency listed as:
```
py_library(
    name = "absl-py",
    deps = [
        ":six",
        requirement("absl-py"),
    ],
)
```


The tool at ```third_party:update_py_build``` updates ```third_party/py/BUILD``` automatically per the content in
```///:requirements_lock.txt```.


## Build the docker image of //examples/py:bin
bazel build //examples/py:bin_image

## Push the image into a local Docker registry
Note: You might need to change the registry from "localhost:5000" to your own at the file
[examples/py/BUILD.bazel](https://github.com/AngusK/codelab_bazel/blob/master/examples/py/BUILD.bazel)

bazel build //examples/py:push_bin_image
