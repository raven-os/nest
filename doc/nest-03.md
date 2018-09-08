# Installing and uninstalling a package

## Nest pull

`nest pull` synchronizes the local packages' cache with the one on Raven's server. You need to **keep this local cache up to date as often as possible**, or you won't be able to install the packages or versions that came out after your last cache update. This is very important. Do not forget that.

## Nest install

To install packages, run `nest install <package>`. You can even put a requirement instead of the package's name. Nest does a lot of things when installing a package, let's go over each one of them.

[//]: # (TODO: add link to the list of packages of Nest)
[//]: # (TODO: add link to the first chapter)
First, Nest checks if the package exists in the local cache. If it does, great ! Nest continues its work. If it doesn't, you might need to run `nest pull` to refresh your cache first, or maybe the package you requested doesn't exist at all. You might want to check your syntax, and you can see a list of all available packages for Nest [here]() in the official repositories. Nest determines after that if there are any dependencies to install by going through the dependency graph, as explained in the [first chapter](), and will install them as well.

The next part is to check if the package getting installed doesn't enter in conflict with an already installed package. This can happen, and if that's the case, it will halt the operation, waiting for your approval to continue.

Once it's done, Nest will download the package and its dependencies (if there are any) in a temporary folder and check if the installation would overwrite an existing file. To finish, Nest will install the package, and delete the temporary folder at the end of the transaction. The local cache's default location is `/var/nest/cache`.

Easy, right ? The output of that command looks like this :

```
$ nest install dash
    install stable::sys-lib/gcc-base#1.0.0
    install stable::sys-lib/libgcc#1.0.0
    install stable::sys-lib/libc#1.0.0
    install stable::shell/dash#1.0.0

Would you like to apply these transactions? [Yes/no]
```

## Nest uninstall

The operation used to uninstall a package is `nest uninstall [package]`.

This command will remove the given package. Nest basically removes the requirement for that package in the dependency graph and all its requirements if they're not needed by another package.