# Updating your system

## Nest upgrade

The upgrade operation is done with the command `nest upgrade [package]`. This will perform a full system upgrade, unless there are specified packages. Let's go into the details.

First, Nest checks if the package exists in the local cache, and determines if there are new dependencies to install. Then, it checks if the upgrade doesn't enter in conflict with an already installed package. After that, Nest will download the package and its dependencies in a temporary folder, and check if the operation won't overwrite any file belonging to the package in question. At last, Nest deletes the old package, the temporary folder and installs the new one.

The output for that operation is:

```
$ nest upgrade gcc
    install stable::sys-lib/gcc-libs#1.2.3
    install stable::sys-lib/gcc#1.2.3

Would you like to apply these transaction? [Yes/no]
```