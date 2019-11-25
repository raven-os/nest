#!/usr/bin/env python3.7

"""
Installation of an unavailable packages should trigger an error
"""

from nesttests import *

with nest_server(), create_config() as config_path:
    nest = nest(config=config_path)
    assert nest.pull().returncode == 0
    assert nest.install("unavailable-package").returncode == 1
