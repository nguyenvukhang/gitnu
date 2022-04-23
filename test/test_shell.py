import unittest

from gitnu.shell import system, system_std, systemlist


class TestShellMethods(unittest.TestCase):
    def test_system(self):
        self.assertEqual(system(['echo', 'hello']), 'hello')

if __name__ == "__main__":
    unittest.main()
