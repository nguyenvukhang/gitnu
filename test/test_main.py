#!/usr/bin/env python3

import unittest
from ..src.strings import sanitize

class TestStringMethods(unittest.TestCase):

    def test_upper(self):
        self.assertEqual('foo'.upper(), 'FOO')

    def test_isupper(self):
        self.assertTrue('FOO'.isupper())
        self.assertFalse('Foo'.isupper())

# if __name__ == '__main__':
#     unittest.main()
