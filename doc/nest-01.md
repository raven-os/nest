# Key concepts

Before using Nest, you must understand how it works. Nest is based on 3 key concepts :

* Its cache
* Its transactions
* Its dependency graph

## The cache

[//]: #(TODO: add link to the next chapter)
Nest's cache is a local copy of the list of all available packages. When you download or update a package, Nest will check if the version of the package you requested exists in your local cache. That's why it is extremely important that you **keep it up to date as often as possible**. You will see how in the [next chapter]().

## The transactions

A transaction is an _action of Nest on the filesystem_. Each transaction is **reversible**, which means you can always go back if something went wrong. You don't need to be scared about installing or removing a package anymore, Nest's got your back. `nest reverse` is not implemented yet, but will be in the near future, hang tight.

## The dependency graph (or tree)

The way the dependency graph works is simple. Each node is either a group of packages, either a single package. A package will be noted as **repository::category/package#version**. The version system used is [semver](https://semver.org).

A node of the graph contains a fulfiller (basically a node identifier in the hashmap), and a list of requirements. A requirement can be for example *stable::sys-devel/gcc#version>=7*, meaning any version posterior or equal to 7 for the package *gcc*, in the category *sys-devel* of the repository *stable*. An important thing to keep in mind, is that versions are **always** chosen by descending order, aka from the latest one to the first one.

The dependency graph has a root node, which is `@root`. When you add a package through the command line, it will add a requirement for that package on `@root`, which will be fulfilled recursively (by fulfilling the requirement's requirement and so on). Then by comparing the two trees, the one you had before that operation and the one with the updated fulfillers, a list of packages will be extracted (or more precisely a list of *transactions* to do). Nest will execute those transactions and the installation will be complete.

By updating your system (or specific packages) with Nest, it's going to update the fulfillers of your dependency tree. Then, as explained before, by comparing the two trees, the list of transactions will be executed by Nest.

Removing a package from your system removes its requirement from the dependency graph (recursively if possible). Then, same as before, the two trees are compared and a list of transactions is extracted and executed by Nest.

### Example

Let's make this clearer with an example. If you want to install the package `dash` through the command line, without specifying any version, it will be resolved as *stable::sys-devel/dash#*\*. Your dependency tree will look like this :

```
@root
    | -> stable::sys-devel/dash#*
```

Nest will try to find a way to fulfill this requirement. Since no version was specified, it will find the latest version available, let's say *stable::sys-devel/dash#0.5.9*. So you'll find a new node in your graph :

```
@root
    | -> stable::sys-devel/dash#*
        |-> fulfilled by stable::sys-devel/dash#0.5.9
```

But let's say this version of `dash` requires any version of the package `glibc`, which has 6 as its major version. It's going to add a new requirement to the graph, which will be fulfilled by, for example, *stable::sts-lib/glibc#6.0.1*. Your dependency tree will look like this :

```
@root
    | -> stable::sys-devel/dash#*
        |-> fulfilled by stable::sys-devel/dash#0.5.9
            | -> stable::sts-lib/glibc#6
                |-> fulfilled by stable::sts-lib/glibc#6.0.1
                    | -> and so on, until there are no dependencies left
```

Since a lot of packages have `glibc` as a dependency, Nest has to find a version suiting every package's needs.

## Conclusion of Chapter 1
[//]: # (TODO: add link to the next chapter)
Alright ! You know everything about how Nest works. It's really important that you understand all that before moving on to the [next chapter](). You need to understand what Nest does to be able to use it at its full potential.