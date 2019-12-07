#!/usr/bin/env python3

"""
Dependencies cycles should be installable
"""

from nesttests import *

some_dep = Package(
    name="some-dep",
    category="sys-libs",
    version="1.0.0",
    kind="virtual",
)

some_library = Package(
    name="some-library",
    category="sys-libs",
    version="1.0.0",
    kind="virtual",
).add_dependency(some_dep, "1.0.0")

other_dep = Package(
    name="other-dep",
    category="sys-libs",
    version="1.0.0",
    kind="virtual",
)

other_library = Package(
    name="other-package",
    category="sys-libs",
    version="1.0.0",
    kind="virtual",
).add_dependency(other_dep, "1.0.0")

other_library.add_dependency(some_library, "1.0.0")
some_library.add_dependency(other_library, "1.0.0")

with nest_server(packages=[some_library, other_library, some_dep, other_dep]), create_config() as config_path:
    nest = nest(config=config_path)
    assert nest.pull().returncode == 0
    x = nest.install("some-library", confirm=True)
    assert x.returncode == 0
    assert "tests::sys-libs/other-dep" in nest.depgraph().installed_packages()
    assert "tests::sys-libs/some-dep" in nest.depgraph().installed_packages()
    assert "tests::sys-libs/other-package" in nest.depgraph().installed_packages()
    assert "tests::sys-libs/some-library" in nest.depgraph().installed_packages()
