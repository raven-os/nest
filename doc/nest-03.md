# Uninstalling a package

## Nest uninstall

The operation used to uninstall a package is `nest uninstall <package>`.

This command will remove the given package (if it exists and is currently installed), and check if that package is a dependency of another one, cancelling the operation if so. At last, it deletes the package and its dependencies not used by any other package anymore, and all its associated files.

## Conclusion of Chapter 3
[//]: # (TODO: add link to the next chapter)
[Next](), upgrading your system !