# Installing and removing packages using requirements

## Add a requirement

The command `nest requirement add <requirement> [--parent <parent>]` adds a requirement to a group. By default, this group is `@root`. With the `--parent` flag you can specify a parent group.

[//]: # (TODO: add link to the merge chapter)
When adding a requirement, Nest makes a copy of your current dependency graph. It adds that requirement to the parent you specified (or `@root`) to that copy. It then fulfills the dependencies of that package recursively. When it's all done, Nest substracts that modified graph with the current one (the one that was not altered, the one your system is currently using), and extracts a list of transactions to apply. This is called "merging". You'll find more information about this [later]().

Its output looks like this:

```
$ nest requirement add gcc stable::shell/dash#0.5.2
You are about to add 2 requirements to @root:

    stable::sys-dev/gcc#*
    stable::shell/dash#0.5.2

Are you sure you want to add those requirements? [Yes/no]
```

## Remove a requirement

`nest requirement remove <requirement> [--parent <parent>]` removes the given requirement from a group, by default this group is `@root`.

It works the same way as `nest requirement add`: Nest makes a copy of your dependency graph, removes the requirement from that copy and its dependencies recursively if possible. Then Nest merges that tree with the current one.