# Hermetic Python Tool Chain

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

`.
