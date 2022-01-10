# codelab_bazel
A simple example to show how to use Bazel with Python, and put a Python build target into a Docker image to run.

## Examples

All code examples are put under [examples](https://github.com/AngusK/codelab_bazel/tree/master/examples)

## Python

To see how to make Python work with Bazel nicely, check [Python with Bazel](docs/python_w_bazel.md).



## Build the docker image of //examples/py:bin
bazel build //examples/py:bin_image

## Push the image into a local Docker registry
Note: You might need to change the registry from "localhost:5000" to your own at the file
[examples/py/BUILD.bazel](https://github.com/AngusK/codelab_bazel/blob/master/examples/py/BUILD.bazel)

bazel build //examples/py:push_bin_image
