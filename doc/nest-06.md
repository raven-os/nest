# The repositories

[//]: # (TODO: add link to the part about repositories from the introduction)
The concept of repositories was explained in the [introduction](). We exposed that only the `stable` repository was enabled by default. Now, we will explain here how to add the other ones.

## The official repositories

The repositories' configuration in the `config.toml` of Nest looks like this:

```toml
#
# ...
#
[[repositories]]
name = "stable"
mirrors = [ "...", "...", "..." ]

# [[repositories]]
# name = "beta"
# mirrors = [ "...", "..." ]

# [[repositories]]
# name = "nightly"
# mirrors = [ "...", "...", "..." ]
```

A mirror is basically a copy of all datas from the main server. They exist for redundancy and speed. Mirrors are spread all over the globe in strategic locations. If you live in Sweden and use a mirror from Sweden, the download speed should be faster than if you used a mirror from Australia for example. The point of mirrors is to choose the nearest one from your location, so you'll download packages as fast as possible. It's advised to have several mirrors for each repository you use in your `config.toml` file, because a mirror can be down or broken for a small period of time, or just be down forever. If Nest can't reach a mirror, it will try with the next one in the list. Nest always goes through the list of mirors in the order they are listed.

If you followed the instructions during Raven's installation, no configuration is necessary regarding the mirrors. Raven should have asked you in which country you live, and has selected mirrors accordingly.

Only `stable` is enabled by default, as it is the most stable repository available for Nest. You'll have to uncomment (remove the `#` characters at the beginning of the lines) on the other repositories in order to use them. It is recommended to only install packages from the `stable` repository only, since the stable packages (hence the name) only are uploaded there, meaning that any update won't break your system.

## The unofficial repositories

There also exist unofficial repositories for Nest. They are not being maintained by the Raven's team or someone we can fully trust. You must decide whether to trust their maintainers and take full responsibility for any consequences of using any unofficial repository. To add them, it's the same as the official repositories, just add them to your `config.toml` file, with their list of mirrors.

It's recommended to keep `stable` as the only repository for Nest. This is the default behaviour, so you don't need to worry about anything.

## Conclusion

[//]: # (TODO: add link to the last chapter)
This concludes the guide for the Training-wheels mode for Nest.

To be followed is the Advanced mode. It concerns experienced users, as it contains more commands that a newcomer may find too difficult to handle. Whatever you do, do not forget that at the [end]() of this Getting Started is a cheatsheet with all of Nest's commands. It might come in handy.