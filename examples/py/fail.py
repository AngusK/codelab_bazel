"""A tiny example binary for the native Python rules of Bazel."""
import unittest
from bazel_tutorial.examples.py.lib import GetNumber


class TestGetNumber(unittest.TestCase):

  def test_fail(self):
    self.assertEquals(GetNumber(), 0)


if __name__ == '__main__':
  unittest.main()
