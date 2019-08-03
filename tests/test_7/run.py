#!/usr/bin/env python3.7

"""
Packages with dependencies should be installable, and their dependencies should be installed with them
"""

from nesttests import *

some_library = Package(
    name="some-library",
    category="sys-libs",
    version="1.0.0",
    kind="virtual",
)

some_package = Package(
    name="some-package",
    category="sys-apps",
    version="1.0.0",
    kind="virtual",
).add_dependency(some_library, "1.0.0")

with nest_server(packages=[some_library, some_package]), create_config() as config_path:
    nest = nest(config=config_path)
    assert nest.pull().returncode == 0
    assert nest.install("some-package", confirm=True).returncode == 0
    assert "tests::sys-apps/some-package" in nest.depgraph().installed_packages()
    assert "tests::sys-libs/some-library" in nest.depgraph().installed_packages()
