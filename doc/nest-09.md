# The repositories and the dependency graph

## List all repositories

`nest repository list` lists all the repositories that were added to the `config.toml` of Nest. Its output could be:

```
$ nest repository list
    - stable
    - nightly
    - beta
    - unofficial-repository
```

## Pull all the repositories

[//]: # (TODO: add link to the `nest pull` part of the 3rd chapter)
The command `nest pull` from the Training-wheels mode is just a shortcut for `nest repository pull`. That latter is no different from `nest pull` from the Training-wheels mode, explained [there]().

## See all requirements

`nest graph` shows a user-friendly representation of the dependency graph starting from `@root`. Its output can be for example:

```
$ nest graph
@root
├─a52dec
| ├─glibc
| | └─...
| └─...
├─aalib
| └─...
└─...
```

Don't hesitate to use this command during the next chapters in order to have a visual representation of a dependency graph. It might help you understand.