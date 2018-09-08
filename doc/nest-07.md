# Manage groups

With Nest you can create groups of packages. Groups are a great way to manage very easily packages used for the same purpose. For example, if you're working on a school project, or just at work working on your personnal laptop, you can create a group containing all the packages you need for that. Once your work is done, you might not need those packages anymore, so you can safely delete that group from your dependency graph, which will remove all the packages from that group.

## Create a group

Creating a group is possible via the command `nest group create <name> [--parent group]`. A group name always start with `@`. By default, this will create the given group as a child of `@root`, if no parent group is provided.

## Delete a group

`nest group delete <group>` deletes the given group if it's empty. If it's not, use the _-f_ or _--force_ flag to remove its content recursively.

## List groups

To list all your groups, use `nest group list`. The output will be similar to this :

```
$ nest group list
@root
@schoolProject1
@superCoolAndSecretProject
@schoolProject2
```