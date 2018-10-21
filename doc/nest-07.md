# The Advanced mode

If you disable the Training-wheels mode, you'll end up with the Advanced mode. As its name implies, it has a lot more commands and is more complete. All the training-wheels mode commands are still available.

## Key concepts

Before using the Advanced mode, you must understand how Nest works on a deeper level than what was explained in the Training-wheels mode. Nest is based on 3 key concepts :

* Its cache
* Its transactions
* Its dependency graph

### The cache

[//]: # (TODO: add link to the part of the introduction explaining Nest's cache)
Nest's cache was already explained in the [introduction]().

### The transactions

A transaction is an _action of Nest on the filesystem_. A transaction can be, for example, installing, removing or upgrading a package, Each transaction is **reversible**, which means you can always go back if something went wrong, using the `nest reverse` command (which is not implemented yet). It's basically going to reverse each transaction made. For example, the reverse operation of installing a package would be removing it, the reverse operation of upgrading is downgrading, etc...

### The dependency graph (or tree)

[//]: # (TDOO: add link to the part of the introduction explaining dependencies)
[//]: # (TODO: add link to the next chapter)
[//]: # (TODO: add link to the packages' syntax part of the chapter 2)
The dependency graph is a directed graph representing the dependencies of packages. The concept of dependency was detailed in the [introduction](). The way the graph works is simple. Each node is either a group of packages (groups will be explained [here]()), either a single package. Each node has a requirement that is fulfilled by another node. A requirement can be fulfilled by **one** and only one package. Otherwise, Nest will throw an error. A requirement will be noted as `repository::category/package#[version|version_req]`, as explained in this Training-wheels mode [chapter]().

The version system used is [semver](https://semver.org). An important thing to keep in mind, is that versions are **always** chosen by descending order, from the latest one to the oldest one.

[//]: # (TODO: add link to the 10th chapter)
The dependency graph contains a root node named `@root`. Using the command line, you can add or remove requirements for any node you want. This will be explained in [this chapter](). After making all the transactions you want, you can then solve the dependecy graph. This means that Nest will try to solve each requirement of each node of the dependency graph, all recursively.

For example, if you have `stable::sys-devel/gcc#version>=7` as a requirement of `@root`, Nest will choose a version that follows that requirement, for example `7.3.0` and pull a new node in the dependency graph named `stable::sys-devel/gcc#7.3.0`. *Do not mix the package `gcc` and the requirement of `@root` which points to this package!*

Then, Nest will recursively solve gcc's dependencies, just like we did with `@root`, pulling a new node in the graph at the `gcc` requirement from earlier for each dependency.

When Nest is done solving the dependency graph, it can compare it with the one we had prior to all the modifications, to extract a list of *transactions* that lead one graph to the other, and then apply them on the filesystem.

### Example

Let's make this clearer with an example and a visual representation. If you want to install the package `dash` through the command line, without specifying any version, it will be resolved as `stable::sys-devel/dash#*`. Your dependency tree will look like this:

```
@root
├─ stable::sys-devel/dash#*
├─...
└─...
```

Nest will try to find a way to fulfill this requirement. Since no version was specified, it will find the latest version available, let's say *stable::sys-devel/dash#0.5.9*. So you'll find a new node in your graph:

```
@root
├─ stable::sys-devel/dash#*
|  └─ fulfilled by stable::sys-devel/dash#0.5.9
└─...
```

But let's say this version of `dash` requires any version of the package `glibc`, which has 6 as its major version. It's going to add a new requirement to the graph, which will be fulfilled by, for example, *stable::sys-lib/glibc#6.0.1*. Your dependency tree will look like this:

```
@root
├─ stable::sys-devel/dash#*
|  └─ fulfilled by stable::sys-devel/dash#0.5.9
|     └─ stable::sys-lib/glibc#6
|        └─ fulfilled by stable::sys-lib/glibc#6.0.1
|           └─ and so on, until there are no dependencies left
└─...
```

Since a lot of packages have `glibc` as a dependency, Nest has to find a version suiting every package's needs.

Then, let's say you want to update this package `dash` to the latest version available, which is 1.0.1. Let's say this version requires the package `glibc`, but with a version superior or equal to 7.1.0. After the update, your tree will look like this:

```
@root
├─ stable::sys-devel/dash#*
|  └─ fulfilled by stable::sys-devel/dash#1.0.1
|     └─ stable::sys-lib/glibc#>=7.1.0
|        └─ fulfilled by stable::sys-lib/glibc#7.1.4
|           └─ etc...
└─...
```

Keep in mind that Nest always chooses the latest version available fulfilling the requirement.