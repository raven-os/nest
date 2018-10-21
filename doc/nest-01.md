# Introduction

If you already have some experience with a Linux package manager, you might want to skip this chapter. Here, we will basically explain what a package manager is, as well as what packages and dependencies are.

## Package manager

A package is a compressed file archive containing all of the files coming with a particular software or application. If you're familiar with Windows, a package is basically a .zip or .rar file of a specific software, containing all the files needed for it to function properly.

Nest is Raven's package manager. A package manager allows the user to install, upgrade or remove software packages on their system. It is similar to an app store for any smartphone, or the Windows Store for example. One of the main differences between those ones and Nest (as well as most other package managers), is that the upgrading must be manually commanded. On a smartphone, the applications are usually automatically upgraded overnight when you're charging your phone, if an update is available. Here, it's up to the user to upgrade their system whenever they feel the need to.

To use Nest, the only interface available for now is the Command Line Interface (CLI). It might be a bit nerve racking for someone who has never seen a terminal or typed any command before, but don't worry, this guide is here to help you feel comfortable!

## Packages and repositories

Packages are stored in repositories. Nest has 3 official repositories: `stable`, `beta` and `nightly`. Think of a repository as a big folder containing all the packages.

When a package is first uploaded, it goes in the `nightly` repository. The package is still under development at this point, and some parts of it may be missing or not working at all. When the package seems stable enough, it's moved to the `beta` repository. This is where the Raven's team is trying to find any bugs in the package. Whenever the found bugs are fixed, and no one has reported any bugs anymore, the package is considered fully stable, and is then moved to the `stable` repository.

## Packages and dependencies

Often, a package requires another one to work properly. Say, for example, someone develops a messenging application, which uses encrypted messages. They use a package written by someone else to encrypt the messages. We say that such messenging application `depends` on that encryption package, because without that package, the application would not work at all. Now, if you want to install that application, you'll also need to install the package they used to encrypt messages.

If a package is required by another one to work properly, that package is called a `dependency`. Whenever you install a package via a package manager (and Nest as well), it will check if the package in question has any dependencies. And if it does, it will install only the dependencies that are not already installed on your system. So, when you install a package, don't be surprised if you see several other packages getting installed too, they are just dependencies needed for that package to work properly.

## The cache

Nest is using a cache (a local copy) to store a copy of the list of all available packages for Nest on your system. Nest can only install packages present in your cache. That's why you need to **keep it up-to-date as often as possible**, otherwise Nest won't be able to install new packages that came out on Nest's server, since your local cache isn't up-to-date.

[//]: # (TODO: add link to the next chapter)
In the [next chapter](), we'll start to go into details on how to use the Training-wheels mode of Nest.