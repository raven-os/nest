# Searching a package

## Nest search

`nest search <package>` searches the given package in the local cache. You may need to update it first by using `nest pull` in order to see an up to date results list. The output is for example:

```
$ nest search gcc
[...]
2 stable::sys-lib/gcc#7.2.3 (43.5MiB)
1 stable::sys-devel/gcc#8.1.1 (32.8MiB) (Installed)
```

Between parenthesis is the size of the package once installed.