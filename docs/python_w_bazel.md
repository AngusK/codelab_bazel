# Python with Bazel

## Issues

People usually run into the following issues when building/running Python with Bazel:
1. Package management
1. Python interpreter

## Python package management

The package management was partially solved in [rules_python](https://github.com/bazelbuild/rules_python).
In this repo, I try to go one step further to solve the inter-package dependencies. See
[python_pkg.md](python_pkg.md) for details.

## Python interpreter

Another common issue is the inconsistent run-time Python interpreters. This is annoying
issue and crucial to many situations. I adopted a hermetic Python solution for this. See
[hermetic_python.md](hermetic_python.md) for details.
