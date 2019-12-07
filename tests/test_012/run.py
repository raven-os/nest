#!/usr/bin/env python3.7

"""
Dependencies cycles with a single package should be installable
"""

from nesttests import *

some_library = Package(
    name="some-library",
    category="sys-libs",
    version="1.0.0",
    kind="virtual",
)

some_library.add_dependency(some_library, "1.0.0")

with nest_server(packages=[some_library]), create_config() as config_path:
    nest = nest(chroot="chroot", config=config_path)
    assert nest.pull().returncode == 0
    x = nest.install("some-library", confirm=True)
    assert x.returncode == 0
    assert "tests::sys-libs/some-library" in nest.depgraph().installed_packages()
