# Requirements operations

## See all requirements

`nest graph [package]` shows a user-friendly representation of the dependency graph starting from the specified package. For example, for the package `gcc`, you'll get an output similar to this one :

```
$ nest graph gcc
stable::sys-devel/gcc#1.0.0
├─stable::sys-devel/gcc-libs#1.0.0
│ └─stable::sys-devel/glibc#1.0.0
│   ├─stable::sys-devel/linux-api-headers#1.0.0
│   ├─stable::sys-devel/tzdata#1.0.0
│   └─stable::sys-devel/filesystem#1.0.0
│     └─stable::sys-devel/iana-etc#1.0.0
└─and so on...
```

## Add a requirement

[//]: # (TODO: add link to the first chapter)
The command `nest requirement add [--parent <parent>] <requirement>` adds a requirement to a group. By default, this group is `@root`. With the `--parent` flag you can specify a parent group. As explained in the [first chapter](), this will create a copy of your dependency tree, and add that requirement to it. This command can take several requirements instead of just one, but they will all be added to the same parent.

[//]: # (TODO: add link to the next chapter)
After that, you will need to merge this tree with the original one, or your changes won't be saved. To do so, simply enter `nest merge`. We will detail this command [later](), but what you need to know for now is that it merges your dependencies trees.

If you want to add `gcc` as a requirement of `@root`, the output of that command would be:

```
$ nest requirement add gcc
You are about to add 1 requirement to @root:

    stable::sys-dev/gcc#*

Are you sure you want to add this requirement? [Yes/no]
```

If you read the previous chapters concerning the training-wheels mode, you might remember the command `nest install [package]`, which installs the given package. Well, `nest install [package]` is just a shortcut for `nest requirement add [requirement] && nest merge`!

## Remove requirements

`nest requirement remove <requirement> [--parent <parent>]` removes the given requirement from a group, by default this group is `@root`.

As for `nest requirement add`, this will create a copy of your dependency tree, and update this copy. You'll need to use `nest merge` in order to apply those changes.

`nest uninstall [package]` from the training-wheels mode is just a shortcut for `nest requirement remove [requirement] && nest merge`.

## Update requirements

To update all requirements' filler of the given node recursively, run `nest requirement update [node]`. If no node is given, it will update the `@root` requirements' filler.

As usual, you'll need to merge your trees with `nest merge`.

`nest upgrade` from the training-wheels mode is just a shortcut for `nest requirement update && nest merge`.