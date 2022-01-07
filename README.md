# codelab_bazel
A simple example to show how to use Bazel with Python, and put a Python build target into a Docker image to run.

## Python

### Python Toolchain

This example shows how to use a "hermetic" Python interpreter. It's inspired by:
[Hermetic Python with Bazel](https://thethoughtfulkoala.com/posts/2020/05/16/bazel-hermetic-python.html)

Please look into [platforms](https://github.com/AngusK/codelab_bazel/tree/master/platforms).

A test was put to ensure the hermetic Python version fixed:
```
bazel test //platforms:version_test
```
[platforms/version_test.py](https://github.com/AngusK/codelab_bazel/blob/master/platforms/version_test.py)


### Running the examples

bazel run //examples/py:bin
bazel build //examples/py:bin

##

## Build the docker image of //examples/py:bin
bazel build //examples/py:bin_image

## Push the image into a local Docker registry
Note: You might need to change the registry from "localhost:5000" to your own at the file
[examples/py/BUILD.bazel](https://github.com/AngusK/codelab_bazel/blob/master/examples/py/BUILD.bazel)

bazel build //examples/py:push_bin_image
