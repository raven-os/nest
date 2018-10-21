# The training-wheels mode

## Introduction

[//]: # (TODO: add link to the chapter about the advanced mode)
As Nest is a dependency-based package manager, it may be difficult for a newcomer to understand how to use it. That's why the training-wheels mode exists. It allows newcomers to get a grip on Nest very easily, making it closer to other package managers and easier to understand. This mode is enabled by default. There is another mode available, recommended for more experienced users, the Advanced mode, which is detailed [here]().

## Package's syntax

The packages are represented in the following way by Nest: `repository::category/package#[version|version_req]` (*version_req* means *version requirement*). For example, `stable::sys-devel/gcc#8.1.1` means the package named `gcc` with a version equal to 8.1.1 from the category `sys-devel` in the repository `stable`. This syntax is used because several packages might share the same name, but have completely opposite purpose. The packages are divided between the three repositories `stable`, `beta` and `nightly`. Each repository disposes of their own categories.

When you install a package, you can input a version requirement as well, for example `beta::shell/dash#>=0.4.0` means the package named `dash` with a version superior or equal to 0.4.0 from the category `shell` in the repository `beta`. Whenever Nest installs a package, it will always install the latest version available matching the version requirement. If you don't put any version requirement, it will install the latest version available.

## Training-wheels mode commands

This mode is emphased on 5 sub-commands. We will detail them in the following chapters:

* pull
* install
* remove
* upgrade
* search

[//]: # (TODO: add link to the next chapter)
In the [next chapter](), we will explain how a package can be installed.