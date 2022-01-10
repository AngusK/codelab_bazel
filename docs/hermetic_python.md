# Hermetic Python Tool Chain

Once challenge to build Python run-time envirnonment with Bazel is the Python interpreter.
This repo shows how to use a "hermetic" Python interpreter. The basic idea is to have Bazel
to download the Python interpreter source code and compile it from scratch and use it as
the part of the toolchain.
It's inspired by:
[Hermetic Python with Bazel](https://thethoughtfulkoala.com/posts/2020/05/16/bazel-hermetic-python.html)


## Invoking the hermetic Python toolchain

The hermetic Python interpreter can be invoked by ```--platforms=//platforms:hermetic_python``` flag.
For example,
```
bazel run --platforms=//platforms:hermetic_python //examples/py:bin
```
This argument can be put into ```.bazelrc``` like
[.bazelrc](https://github.com/AngusK/codelab_bazel/blob/master/.bazelrc).
See [platforms](https://github.com/AngusK/codelab_bazel/tree/master/platforms) directory for details.

## Building the Python interpreter

The details can be found in ```WORKSPACE``` file. To compile the interpreter, the following libraries
need to be installed:
 - readline
 - bz2
 - curses
 - dbm.gnu
 - gzip,zip
 - lzma
 - sqlite3
 - ssl
 - tkinter
 - ctypes
On Ubuntu you can try the following command:
```
sudo apt install -y libreadline-gplv2-dev libbz2-dev libncursesw5-dev libgdbm-dev zlib1g-dev liblzma-dev libsqlite3-dev libssl-dev libffi-dev
```



A test was put to ensure the hermetic Python version fixed:
```
bazel test //platforms:version_test
```
See [platforms/version_test.py](https://github.com/AngusK/codelab_bazel/blob/master/platforms/version_test.py).

`.
