#!/usr/bin/env python3

"""
Launching nest and finest with the --help option should succeed, even with an invalid configuration file
"""

from nesttests import *
import os

assert nest(config="/non_existent/not_existing_either.toml").help().returncode == 0
assert finest(config="/non_existent/not_existing_either.toml").help().returncode == 0

with create_config(entries={}) as config_path:
    os.chmod(path=config_path, mode=0o222)  # Make the configuration file write-only
    assert nest(config=config_path).help().returncode == 0
    assert finest(config=config_path).help().returncode == 0

with create_config(entries={}) as config_path:
    with open(config_path, 'w+') as f:
        f.write("<(^v^)>")  # Write invalid data
    assert nest(config=config_path).help().returncode == 0
    assert finest(config=config_path).help().returncode == 0
