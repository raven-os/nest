#!/usr/bin/env python3.7

"""
Packages should be made available after a pull operation
"""

from nesttests import *

available_package = Package(
    name="available-package",
    category="sys-apps",
    version="1.0.0",
    kind="virtual",
    description="A package",
    tags=["test"]
)

with nest_server(packages=[available_package]), create_config() as config_path:
    nest = nest(config=config_path)
    assert nest.install("available-package").returncode == 1
    assert nest.pull().returncode == 0
    assert nest.install("available-package", confirm=False).returncode == 0
    assert "tests::sys-apps/available-package" not in nest.depgraph().installed_packages()
