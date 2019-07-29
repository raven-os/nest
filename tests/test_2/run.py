#!/usr/bin/env python3

"""
Pulling available repositories with a valid configuration file should succeed
"""

from nesttests import *

with nest_server(), create_config() as config_path:
    assert nest(chroot="/tmp/chroot", config=config_path).pull().returncode == 0
