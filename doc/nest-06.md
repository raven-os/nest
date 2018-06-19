# Fixing your system

[//]: # (TODO: add link to the first chapter)
If you're not careful enough, you might install or remove a package which will damage your system. In the [first chapter]() we talked about how all Nest's transactions are reversible. Let's explain how that's possible, and how to do it.

## Nest log

You can see the list of all the operations made, ordered by chronological order with the command `nest log`. For example, it might look like :
```
16:20:42 - 531 - nest pull
16:20:59 - 532 - nest install gcc
16:21:14 - 533 - nest upgrade dash
```

Each line represents an operation done by Nest. You can see the date at which the operation began, a unique ID associated to each one of them, and finally, the operation itself.

## Nest reverse

To revert to a specific state, run `nest reverse <id>` with the ID of the operation you want to revert to. Nest will then rollback, up to that operation included (meaning it will be the last one from your log). You will lose all the operations you made after that one, as they won't be listed anymore in your log. So use it with extreme caution !

## Conclusion of Chapter 6
[//]: # (TODO: add link to the next chapter)
And that's it ! You know everything about Nest. Isn't this cool ? On the [next chapter]() you'll find a cheatsheet, summarizing everything you just read.