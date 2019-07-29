#!/usr/bin/env python3

"""
Launching nest and finest with a valid configuration file should succeed
"""

from nesttests import *

assert nest().help().returncode == 0
assert finest().help().returncode == 0
