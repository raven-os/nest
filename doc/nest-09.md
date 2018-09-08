# Managing Nest's repositories

Several repositories are available for Nest. The official ones are `stable`, `beta` and `nightly`.

When a package is uploaded, it first goes in the `nightly` repository. We (the Raven team and people we trust) go through all the packages and test them, to see if the installation/removal work as expected, and if we can't find any bugs. If everything's good, the package is moved to `beta`. We're not looking for bugs anymore at this point, but if you find any, don't hesitate to report them to the package owner. After a while, if no bug is being reported, the package is moved to `stable`.

All the packages in `nightly` have experimental, new features, so any update could break your system if you installed one of them. In other words, in `nightly`, the final package is still in development, you should not install any package from this repository, unless you have a good reason. Whereas in `stable`, an update won't cause any problem. Only `stable` is enabled by default. To add the other ones, just uncomment them from the `config.toml`. For unofficial repositories, you should add them by yourself, as they are not handled by Nest.

```toml
#
# ...
#
[[repositories]]
name = "stable"
mirrors = [ "https://stable.raven-os.org" ]

[[repositories]]
name = "nightly"
mirrors = [ "https://nightly.raven-os.org" ]
mirrors = [ "https://mirror.raven-os.org/nightly" ]

[[repositories]]
name = "beta"
mirror = [ "https://beta.raven-os.org" ]

[[repositories]]
name = "scary-unofficial-repository"
mirrors = [ "http://coolwebsite.io" ]
```

[//]: # (TODO: add link to the mirror list)
To see a full list of the mirrors for official repositories, click [here]().

But be careful, the unofficial repositories are not being maintained or tested by the Raven's team or someone we fully trust. You must decide whether to trust their maintainers and take full responsibility for any consequences of using any unofficial repository.

## List all repositories

`nest repository list` lists all the repositories and their mirrors that were added to the `config.toml` of Nest. Using the same config as above, its output would be :

```
$ nest repository list
    - stable
        https://stable.raven-os.org
    - nightly
        https://nightly.raven-os.org
        https://mirror.raven-os.org/nightly
    - beta
        https://beta.raven-os.org
    - scary-unofficial-repository
        http://coolwebsite.io
```

## Pull all the repositories

The command `nest pull` from the training-wheels mode is just a shortcut for `nest repository pull`. That latter is no different from `nest pull`, there are no additional flags or things to know about it. You still have to pull frequently in order to keep your cache up to date.