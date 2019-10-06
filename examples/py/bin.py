# pylint: disable=superfluous-parens
"""A tiny example binary for the native Python rules of Bazel."""
from bazel_tutorial.examples.py.lib import GetNumber
from bazel_tutorial.examples.py.fibonacci.fib import Fib

print("The number is %d" % GetNumber())
print("Fib(5) == %d" % Fib(5))
