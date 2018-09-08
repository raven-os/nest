# Cheatsheet

To conclude this Getting Started with Nest, we present to you the Nest's cheatsheet. Don't hesitate to come back to it if you have trouble remembering something about one of Nest's command. We do hope you'll have fun using Nest!

| Task | Command | Notes |
|-|-|-|
|**TRAINING-WHEELS MODE COMMANDS**
| Update the local cache | `nest pull` | The local cache must be kept up to date as often as possible |
| Install a package | `nest install <package>` ||
| Uninstall a package | `nest uninstall <package>` ||
| Update and upgrade all | `nest upgrade <package>` | A full upgrade of the system if no packages are provided |
| Search a package in the local cache | `nest search <package>` ||
|**ADVANCED MODE COMMANDS**
| Add a requirement | `nest requirement add <req> [--parent <parent>]` | Adds a requirement to the given parent (default: `@root`) |
| Remove a requirement | `nest requirement remove <req> [--parent <parent>]` | Removes a requirement from the given parent (default: `@root`) |
| Update a requirement | `nest requirement update [node]` | Updates all requirement's filler of the given node recursively (default: `@root`), fulfilling empty requirements |
| Create a group | `nest group create <group> [--parent <parent>]` | Group names always start with `@`. Creates a group to the given parent (default: `@root`) |
| Delete a group | `nest group delete <group>` | If the group is not empty, use `-f` or `--force` flag to force the removal |
| List all existing groups | `nest group list` ||
| Pull repositories | `nest repository pull <repo>`|Pulls all of the repositories if none are given |
| List all available repositories and their mirrors | `nest repository list` ||
| Show the current dependency graph of a package | `nest graph <package>` ||
| Merge the dependencies graphs | `nest merge` | This command must be executed after a command altered the dependency graph in any way |

Just a reminder that all the training-wheels commands are still available in the advanced mode.