#!/usr/bin/env python3.7

"""
Incompatible packages should be detected and their installation should fail
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

pkg_a = Package(
    name="a",
    category="sys-libs",
    version="1.0.0",
    kind="virtual",
).add_dependency(some_library_1, "=1.0.0")

pkg_b = Package(
    name="b",
    category="sys-libs",
    version="1.0.0",
    kind="virtual",
).add_dependency(some_library_2, "=2.0.0")

with nest_server(packages=[some_library_1, some_library_2, pkg_a, pkg_b]), create_config() as config_path:
    nest = nest(config=config_path)
    assert nest.pull().returncode == 0
    x = nest.install("a", "b", confirm=True)
    assert x.returncode == 1
    assert "tests::sys-libs/some-library" not in nest.depgraph().installed_packages()
    assert "tests::sys-libs/a" not in nest.depgraph().installed_packages()
    assert "tests::sys-libs/b" not in nest.depgraph().installed_packages()
