import json
import os
import shutil
import subprocess
import tarfile
import tempfile
import toml
from typing import Any, Dict, List
from contextlib import contextmanager
from time import sleep


class Package:
    def __init__(
            self,
            name: str,
            category: str,
            version: str,
            kind: str,
            description: str = "A package",
            tags: List[str] = None,
            maintainer: str = "nest-tests@raven-os.org",
            licenses: List[str] = None,
            upstream_url: str = None,
    ):
        self.name = name
        self.category = category
        self.version = version
        self.kind = kind
        self.description = description
        self.tags = tags or []
        self.maintainer = maintainer
        self.licenses = licenses or ["gpl_v3"]
        self.upstream_url = upstream_url or "https://google.com"
        self.dependencies = {}
        self.files = {}

    def full_name(self) -> str:
        return f"tests::{self.category}/{self.name}"

    def package_id(self) -> str:
        return f"tests::{self.category}/{self.name}#{self.version}"

    def add_dependency(self, dependency: 'Package', version_requirement: str) -> 'Package':
        self.dependencies[dependency.full_name()] = version_requirement
        return self

    def add_file(self, path, with_content=None, from_reader=None) -> 'Package':
        if not (with_content ^ from_reader):
            raise ValueError("Invalid arguments: exactly one of 'with_content' and 'from_reader' must be used")
        # self.files[path] =
        return self

    def add_symlink(self, path: str, target: str) -> 'Package':
        return self

    def add_directory(self, path: str) -> 'Package':
        return self

    def _create_in(self, directory: str):
        directory = f"{directory}/{self.category}/{self.name}"
        os.makedirs(directory, exist_ok=True)
        manifest = {
            "name": self.name,
            "category": self.category,
            "version": self.version,
            "kind": self.kind,
            "wrap_date": "2019-05-27T16:34:15Z",
            "metadata": {
                "description": self.description,
                "tags": self.tags,
                "maintainer": self.maintainer,
                "licenses": self.licenses,
                "upstream_url": self.upstream_url
            },
            "dependencies": self.dependencies
        }
        manifest_path = f"{directory}/manifest.toml"
        with open(manifest_path, 'x') as f:
            toml.dump(manifest, f)

        files = [(manifest_path, "manifest.toml")]

        if self.kind == "effective":
            with tarfile.open(f"{directory}/data.tar.gz", "w:gz") as tar:
                pass
            files.append((f"{directory}/data.tar.gz", "data.tar.gz"))

        with tarfile.open(f"{directory}/{self.name}-{self.version}.nest", "x") as tar:
            for name, arcname in files:
                tar.add(name, arcname=arcname)
                os.remove(name)


def _create_packages(packages: List[Package]):
    for package in packages:
        package._create_in("/tmp/nest-server/packages")


def _create_configuration_file():
    configuration = {
        'name': 'tests',
        'pretty_name': 'Tests',
        'package_dir': './packages/',
        'cache_dir': './cache/',
        'auth_token': 'a_very_strong_password', 'links': [
            {'name': 'Tests', 'url': '/', 'active': True},
            {'name': 'Stable',
             'url': 'https://stable.raven-os.org'},
            {'name': 'Beta',
             'url': 'https://beta.raven-os.org'},
            {'name': 'Unstable',
             'url': 'https://unstable.raven-os.org'}
        ]
    }
    with open("/tmp/nest-server/Repository.toml", "w") as f:
        toml.dump(configuration, f)


@contextmanager
def nest_server(packages: List[Package] = None):
    _create_packages(packages or [])
    _create_configuration_file()
    nest_server_path = os.getenv("NEST_SERVER")
    p = subprocess.Popen(["cargo", "run", "-q"], cwd=nest_server_path, stdout=subprocess.DEVNULL)
    sleep(0.5)  # Wait a bit so the server initializes properly
    try:
        yield
    finally:
        p.kill()
        if os.path.exists(f"{nest_server_path}/packages"):
            shutil.rmtree(f"{nest_server_path}/packages")
        if os.path.exists(f"{nest_server_path}/cache"):
            shutil.rmtree(f"{nest_server_path}/cache")


@contextmanager
def create_config(entries: Dict[str, Dict[str, Any]] = None):
    entries = entries or {"repositories": {"tests": {"mirrors": ["http://localhost:8000"]}}}
    path = tempfile.NamedTemporaryFile().name
    with open(path, 'w') as f:
        toml.dump(entries, f)
    try:
        yield path
    finally:
        os.remove(path)


class _Depgraph:
    def __init__(self, path: str):
        if os.path.exists(path):
            self.data = json.load(open(path, 'r'))
        else:
            self.data = {"node_names": {}}

    def installed_packages(self):
        return filter(lambda name: name[0] != '@', self.data["node_names"])

    def groups(self):
        return filter(lambda name: name[0] == '@', self.data["node_names"])


class _Nest:
    def __init__(self, config: str = None, chroot: str = None):
        self.config = config
        self.chroot = chroot

    def _run(self, *args: str, input_str: str = None):
        cmd = ["sudo", f"PATH={os.getenv('PATH')}", "env", "cargo", "run", "-q", "--bin", "nest", "--"]
        if self.config:
            cmd += ("--config", self.config)
        if self.chroot:
            cmd += ("--chroot", self.chroot)
        cmd += args
        return subprocess.run(cmd, capture_output=True, input=input_str and input_str.encode())

    def pull(self, confirm=True):
        return self._run("pull", input_str="yes" if confirm else "no")

    def install(self, *packages: str, confirm=True):
        return self._run("install", *packages, input_str="yes" if confirm else "no")

    def uninstall(self, *packages: str, confirm=True):
        return self._run("uninstall", *packages, input_str="yes" if confirm else "no")

    def list(self):
        pass

    def depgraph(self) -> _Depgraph:
        return _Depgraph(f"{self.chroot}/var/nest/depgraph")

    def help(self):
        return self._run("help")


def nest(config: str = None, chroot: str = None) -> _Nest:
    chroot = chroot or os.getenv("NEST_CHROOT")
    return _Nest(config, chroot)


class _Finest:
    def __init__(self, config: str = None, chroot: str = None):
        self.config = config
        self.chroot = chroot

    def _run(self, *args: str, input_str: str = None):
        cmd = ["sudo", f"PATH={os.getenv('PATH')}", "env", "cargo", "run", "-q", "--bin", "finest", "--"]
        if self.config:
            cmd += ("--config", self.config)
        if self.chroot:
            cmd += ("--chroot", self.chroot)
        cmd += args
        return subprocess.run(cmd, capture_output=True, input=input_str and input_str.encode())

    def pull(self):
        return self._run("pull", input_str="yes")

    def help(self):
        return self._run("help")


def finest(config: str = None, chroot: str = None) -> _Finest:
    chroot = chroot or os.getenv("NEST_CHROOT")
    return _Finest(config, chroot)
