# Updating your system

## Nest upgrade

The upgrade operation is done with the command `nest upgrade [package]`. This will perform a full system upgrade, unless you input specific packages. A full system upgrade means that Nest will compare all the installed packages with the ones from your local cache, and if a more recent version is found in the cache, Nest will upgrade your installed packages to that version. In other words, a full system upgrade is basically an upgrade of all your installed pacakges. In general, it's preferable to do a full system upgrade from time to time instead of upgrading each package manually.

The output for that operation is:

```
$ nest upgrade gcc
    install stable::sys-lib/gcc#1.2.3

Would you like to apply these transaction? [Yes/no]
```