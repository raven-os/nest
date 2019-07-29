#!/usr/bin/env python3

"""
Standalone (without any dependencies) packages should be installable
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
    assert nest.pull().returncode == 0
    assert nest.install("available-package", confirm=True).returncode == 0
    assert "tests::sys-apps/available-package" in nest.depgraph().installed_packages()
