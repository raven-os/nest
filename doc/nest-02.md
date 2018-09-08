# The training-wheels mode

As Nest is a dependency-based package manager, it can be difficult for a newcomer to understand how to use it. That's why the training-wheels mode exists. It allows newcomers to get a grip on Nest very easily, making it closer to other package managers. This mode is enabled by default. If you wish to disable or re-enable it, you can do so in Nest's `config.toml`, which is located by default in `/etc/nest/config.toml`. Just set the `training_wheels` variable to false or true.

Only those 5 sub-commands are available in the training-wheels mode :

* pull
* install
* remove
* upgrade
* search

[//]: # (TODO: add link to the next chapter)
Let's start with the installation of a package in the [next chapter]().