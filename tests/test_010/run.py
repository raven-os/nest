#!/usr/bin/env python3.7

"""
The newest version of a package should be picked if possible
"""

from nesttests import *

some_library_1 = Package(
    name="some-library",
    category="sys-libs",
    version="1.0.0",
    kind="virtual",
)

some_library_2 = Package(
    name="some-library",
    category="sys-libs",
    version="2.0.0",
    kind="virtual",
)

with nest_server(packages=[some_library_1, some_library_2]), create_config() as config_path:
    nest = nest(config=config_path)
    assert nest.pull().returncode == 0
    x = nest.install("some-library", confirm=True)
    assert x.returncode == 0
    assert "tests::sys-libs/some-library#2.0.0" in nest.depgraph().installed_packages_with_versions()
