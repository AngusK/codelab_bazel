"""A tiny example binary for the native Python rules of Bazel."""

import unittest
from bazel_tutorial.examples.py.lib import GetNumber
from bazel_tutorial.examples.py.fibonacci.fib import Fib


class TestGetNumber(unittest.TestCase):

  def test_ok(self):
    self.assertEqual(GetNumber(), 42)

  def test_fib(self):
    self.assertEqual(Fib(5), 8)

if __name__ == '__main__':
  unittest.main()
