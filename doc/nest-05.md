# Searching a package

## Nest search

`nest search <package>` search the given package in the local cache. You may need to update it first using `nest pull` in order to see an up to date results list. An example output could be

```
[...]
3 core/gcc-fortran 8.1.1 (8.0 MiB 26.9 MiB)
2 core/gcc-aa 8.1.1 (20.4 MiB 117.3MiB)
1 core/gcc 8.1.1 (32.8MiB 132.0 MiB) (Installed)
```

Between parenthesis are the size of the package, and the installed size of the package respectively.

## Nest list

`nest list` gives you informations about all your installed packages, like their metadatas, their versions, etc... It also uses your local cache, don't forget to keep it up to date ! For example, an excerpt of the output could be :

```
stable::shell/dash 0.5.9.1-1 (72.5 KiB 132 KiB)
stable::sys-devel/gcc 8.1.1 (32.8 MiB 132.0 MiB) (Out of date)
```

The `Out of date` annotation tells you that the version installed of that package is not the latest one available from your local cache.

## Conclusion of Chapter 5
[//]: # (TODO: add link to the next chapter)
If you install or remove a package that breaks your system, you can fix it very easily with Nest ! Let's jump into the [next chapter]() to see how.