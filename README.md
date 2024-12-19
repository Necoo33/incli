# Installation CLI for programming languages

Incli is a Automation Tool For Installing Compilers, Sdk's and Package Managers to different kernels and operating systems.

It has 3 different command types for now:

* install: it is for installing sdk's. 
* help: it is for giving general informations.
* version: it is for writing the version of incli.

## Announcement, Roadmap to v1.0.0

Incli is released almost a year ago, and in that timespan both algorithm and range of installation options has took a long way. In the moment, i'm pretty more confident my program works just well on more user's computers than the moment i was started that project.

The 1.0 release will be a great milestone for Incli, especially for robustness of that project. Up until 1.0, when we install programs on your computer, we've done many assumptions, and 2 of them is:

1 - Your home directory is always either "/root", if you're installing it with root user or "/home/(username)" if you're just another user. Which they are default configurations of linux distros but some of the linux nerds and system admins could want to change that thing.

2 - Your shell is always bash, which is not true always.

Since we were able to successfully implement to get user's home directory and shell informations on my other big liblary, [sys-info-extended](https://crates.io/crates/sys-info-extended), on 1.0 release we'll implement to use that abilities to find your home directory and shell
dynamically, so you can use that program with custom shell and home directory configurations.

My goal is make that program as comprehensive and robust as possible to be able to install any program you could want on your development processes in every linux distro and windows. Since it's very abstract goal, we wouldn't be able to totally achieve that but, we want to implement it on most used programs and systems. We have a long way to achieve that and too many work to do. Our biggest lack is lack of support for Darwin ecosystem and since i don't have any Mac computer, i couldn't provide that support in that moment.

If you like that program, consider to give a like on [github repo](https://github.com/Necoo33/incli)

## Currently Planned to Support Package Managers and Sdk's

* python
* php
* phpmyadmin
* laravel
* ruby
* ocaml
* zig
* elixir
* erlang
* deno

## Current Status

In this situation rust, node.js, yarn, bun, golang, jvm, gradle and maven are supported.

Also we'll make minor updates for supporting new lts and latest releases for Node.js And Golang.

You can look [SUPPORTED-OS](https://github.com/Necoo33/incli/blob/main/SUPPORTED-OS.md) document to learn which distros supported for each program and how to install them. General synthax is:

`(that binary) (action) (program) (version number[if the action is 'install'])`. Also `latest` and `lts` arguments are supported for Node.js(continuously updated), Go(continuously updated) and Java(Java version 21.0.2).

For example:

### Rust

`(that binary) install rust`

### Node.js

`(that binary) install node latest`

### Go

`(that binary) install go lts`

### Bun

`(that binary) install bun`

### Yarn

`(that binary) install yarn`

### Java

``(that binary) install java 11`

### Gradle

`(that binary) install gradle 8.8`

### Maven

`(that binary) install maven 3.9.9`