import unittest

from gitnu.strings import sanitize_line, remove_ansi


class TestStringMethods(unittest.TestCase):
    def test_remove_ansi(self):
        x = "[31mmodified:   gitnu/strings.py[m"
        y = "modified:   gitnu/strings.py"
        self.assertEqual(remove_ansi(x), y)

        x = "[32mmodified:   gitnu/strings.py[m"
        y = "modified:   gitnu/strings.py"
        self.assertEqual(remove_ansi(x), y)

    def test_sanitize(self):
        x = "[31mmodified:   gitnu/strings.py[m"
        y = "modified:   gitnu/strings.py"
        self.assertEqual(sanitize_line(x), y)


if __name__ == "__main__":
    unittest.main()
