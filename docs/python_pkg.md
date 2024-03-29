# Python Package Management


## The complete work flow

1. Update ```requirements.in``` to add the packages you are using.
1. Run ```bazel run //:requirements.update``` to get other packages from higher order of dependencies.
1. Run ```bazel run //third_party:update_py_build``` to reflect the inter-package dependenciesin a BUILD file.

The Python package management has two parts: a) package downloading and b) dependency management.

## a) Package download

Package download is done by [rules_python](https://github.com/bazelbuild/rules_python). The first-order
dependences, which means the packages your Python script is immediately using, is inside ```requirements.in```.
The full list of the packages is in ```requirements_lock.txt``` generated by ```pip-compile``` tool.
```rules_python``` provides a simple rule to run ```pip-compile``` to update ```requirements_lock.txt```.

## b) Package dependencies

The dependencies between the packages are already stated in ```requirements_lock.txt``` but they are not
automatically pulled in to your build rules. For example, if your build target depends on ```absl-py```:
```
deps = [
    requirement("absl-py"),
],
```
, you also need add package ```six``` which is used by ```absl-py```:
```
deps = [
    requirement("absl-py"),
    requirement("six"),
],
```
This is not always easy to discover and you might run into run-time errors to find this out.

To solve this issue, a layer to manage the inter-package dependencies is introduced at:
[third_party/py/BUILD](https://github.com/AngusK/codelab_bazel/blob/master/third_party/py/BUILD).
By changing
```
deps = [
    requirement("absl-py"),
],
```
to
```
deps = [
    "//third_party/py:absl-py"
],
```
, which depends on package ```six``` in the ```third_party/py/BUILD``` file:
```
py_library(
    name = "absl-py",
    deps = [
        ":six",
        requirement("absl-py"),
    ],
)
```
, you automatically pull in package ```six``` to your build target. The tool at ```//third_party:update_py_build```
updates ```third_party/py/BUILD``` automatically per the content in ```///:requirements_lock.txt```.
