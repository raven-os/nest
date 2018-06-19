# Installing a package

Starting from this chapter, you're going to learn how to use Nest. We'll begin with the most important operations: updating your local cache, and installing a package.

## Nest pull

[//]: # (TODO: add link to the last chapter)
As said in the [last chapter](), `nest pull` synchronizes the local packages' cache with the one on Raven's server. You need to **keep it up to date as often as possible**, or you won't be able to install the packages or versions that came out after your last cache update. This is very important. Do not forget that.

The local cache's default location is `/var/nest/cache`.

## Nest install

To install packages, run `nest install <package>`. You can even put a prerequisite instead of just the package's name. Nest does a lot of things when installing a package, let's go over each one of them.

First, Nest checks if the package exists in the local cache. If it does, great ! Nest continues its work. If it doesn't, you need to run `nest pull` to refresh your cache first, or maybe the package you requested does not exist at all. Nest determines after that if there are any dependencies to install by going through the dependency graph.

The next part is to check if the package getting installed doesn't enter in conflict with an already installed package. This can happen, and if that's the case, it will halt the operation, waiting for your approval to continue.

Once it's done, Nest will download the package and its dependencies (if there are any) in a temporary folder and check if the installation would overwrite an existing file. Then, Nest will install the package, and delete the temporary folder at the end of the transaction.

## Conclusion of Chapter 2
[//]: # (TODO: add link to the next chapter)
Let's continue with the [next operation](), uninstalling a package.