# Updating your system

## Nest upgrade

The upgrade operation is done with the command `nest upgrade`. This will perform a full system upgrade. To upgrade specific packages, enter their names after that last command. Let's get into the details.

First, Nest checks if the package exists in the local cache, and determines if there are new dependencies to install. Then, it checks if the upgrade doesn't enter in conflict with an already installed package. After that, Nest will download the package and its dependencies in a temporary folder, and check if the operation won't overwrite any file belonging to the package in question. At last, Nest deletes the old package, the temporary folder and installs the new one.

## Conclusion of Chapter 4
[//]: # (TODO: add link to the next chapter)
[Next](), we will see how to find informations about a package, whether it is installed or not.