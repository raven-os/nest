# The advanced mode

If you disable the training-wheels mode, you'll end up with the advanced mode. As its name implies, it has a lot more commands and is more complete. All the training-wheels mode commands are still available, since most of them are basically shortcuts for other commands. We'll explain this in a bit.

## Principles of the advanced mode

[//]: # (TODO: add link to the first chapter)
We talked about the requirements in the dependency tree during the [first chapter](). For the training-wheels mode, you didn't need to know more about that. But now, we need to clarify some things.

There are two types of requirements in Nest:

* automatic requirements
* static requirements

The automatic requirements are handled by their parent, and so, will be deleted or modified when the parent is updated. This is the case for classical dependencies.

The static requirements are dependencies that will stay, even if the parent if updated (but not if it's deleted of course). Those are usually the dependencies added by the user.

This allows Nest to know which requirement to delete when a package is updated : removing all the automatic ones, and leaving the static ones. By default, all the commands of Nest are used to handle static requirements.

The advanced mode allows you to :

* See all the requirements you currently have
* Create and manage a group of requirements
* Add requirements
* Remove requirements
* Update requirements
* Merge your dependency trees

[//]: # (TODO: add link to the next chapter)
Starting from the [next chapter](), we'll first go over all those sub-commands.