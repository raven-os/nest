# Cheatsheet

|Task                                   |Command                   |Notes                                                               |
|---------------------------------------|--------------------------|--------------------------------------------------------------------|
|Update the local cache                 |`nest pull`               |The local cache must be kept up to date as often as possible        |
|Install a package                      |`nest install <package>`  |                                                                    |
|Uninstall a package                    |`nest uninstall <package>`|                                                                    |
|Update and upgrade all                 |`nest upgrade`            |A full upgrade of the system.                                       |
|Search a package in the local cache    |`nest search <package>`   |                                                                    |
|Get information of an installed package|`nest list <package>`     |                                                                    |
|See the log of all Nest's operations   |`nest log`                |Chronologically lists all the operations that were performed by Nest|
|Revert to a certain state              |`nest reverse <id>`       |Revert the system up to the operation (included) of the given ID    |