#!/usr/bin/env python3

import pytest


def inc(x):
    return x + 1


def test_answer():
    assert inc(3) == 5
