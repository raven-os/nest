# Updating your system

## Update requirements

To update all requirements' fulfiller of the given node recursively, run `nest requirement update [node]`. If no node is given, it will update the `@root` requirements' fulfiller.

Nest makes a copy of your dependency graph and updates all its requirements' recursively. Then Nest merges that graph with the current one.

Its output looks like this:

```
$ nest requirement update
You are about to update 1 requirement from the node @root to this:

    stable::sys-devel/dash#0.6.0
Some of its dependencies must be updated to this version:

    stable::sys-lib/glibc#6
```

## Merging the dependency graphs

`nest merge` is merging the copy of your dependency graph on which you applied the transactions to your current dependency graph.

Those two graphs are compared to extract a list of transactions that Nest will do, in order to go from one graph to the other. And so, your old dependency graph is replaced by the one you altered with Nest's commands.

As you might have guessed, it's really important to use this command when you're done modifying your dependency graph. Otherwise the transactions you did won't apply.

Its output is, for example :

```
$ nest merge
    Install (2) stable::sys-devel/gcc#8.2.1
                stable::sys-devel/gcc-libs#8.2.1
    Upgrade (1) stable::shell/dash#0.5.10
You are going to apply those 3 transactions to your current dependency graph, continue? [Yes/no]
```

## Training-wheels mode shortcuts

If you've read the chapters concerning the Training-wheels mode, you might notice that some of its commands look similar to some of the Advanced mode commands. This is simply because the Training-wheels commands are shortcuts for the advanced mode commands!

`nest install [package]` is a shortcut for `nest requirement add [requirement] && nest merge`.

`nest uninstall [package]` is a shortcut for `nest requirement remove [requirement] && nest merge`.

`nest upgrade` is a shortcut for `nest requirement update && nest merge`.