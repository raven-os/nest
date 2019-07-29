#!/usr/bin/env python3

"""
Pulling available repositories with an invalid configuration file should fail
"""

from nesttests import *
import os

assert nest(config="/non_existent/not_existing_either.toml").pull().returncode == 1
assert finest(config="/non_existent/not_existing_either.toml").pull().returncode == 1

with create_config(entries={}) as config_path:
    os.chmod(path=config_path, mode=0o222)  # Make the configuration file write-only
    assert nest(config=config_path).pull().returncode == 1
    assert finest(config=config_path).pull().returncode == 1

with create_config(entries={}) as config_path:
    with open(config_path, 'w+') as f:
        f.write("<(^v^)>")  # Write invalid data
    assert nest(config=config_path).pull().returncode == 1
    assert finest(config=config_path).pull().returncode == 1
