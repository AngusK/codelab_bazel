import sys
import unittest


class TestPythonPlatform(unittest.TestCase):
    def test_python_version(self):
        self.assertEqual(sys.version_info[:3], (3, 8, 3))


if __name__ == '__main__':
    unittest.main()
