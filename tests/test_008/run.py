#!/usr/bin/env python3.7

"""
Removal of unknown packages should fail
"""

from nesttests import *

with nest_server(packages=[]), create_config() as config_path:
    nest = nest(config=config_path)
    assert nest.uninstall("some-package", confirm=True).returncode == 1
