"""Run gitn as a Python module.
Usage: python -m gitn
"""
from __future__ import absolute_import, division, print_function, unicode_literals

from gitn import main


def run():
    """Start the command-line interface."""
    main.main()


if __name__ == "__main__":
    run()
