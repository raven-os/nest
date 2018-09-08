# Searching a package

## Nest search

`nest search <package>` searches the given package in the local cache. You may need to update it first using `nest pull` in order to see an up to date results list. An example output could be :

```
$ nest search gcc
[...]
3 stable::sys-devel/gcc-fortran#8.1.1 (8.0 MiB 26.9 MiB)
2 stable::sys-devel/gcc-aa#8.1.1 (20.4 MiB 117.3MiB)
1 stable::sys-devel/gcc#8.1.1 (32.8MiB 132.0 MiB) (Installed)
```

Between parenthesis are the size of the package, and the installed size of the package respectively.

## Conclusion

[//]: # (TODO: add link to the eleventh chapter)
And that's all for the training-wheels mode command ! Thoses were the most basic and useful operations for Nest. You don't have to keep reading if you're not going to use the advanced mode, which, as its name implies, is for advanced users, as it contains more commands that a newcomer may find too difficult to handle. Whatever you do, do not forget that at the [end]() of this Getting Started there is a cheatsheet with all of Nest's commands. It might come in handy.