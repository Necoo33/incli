# Installation CLI for programming languages

Incli is a Automation Tool For Installing Compilers, Sdk's and Package Managers to different kernels and operating systems.

## Current Status In Project

We support 3 different command types:

* install: it is for installing sdk's. 
* help: it is for giving general informations.
* version: it is for writing the version of incli.

It supports that shells:

* Bash
* Zsh
* Fish

And it supports that languages/tools:

* Rust
* Node.js
* Yarn
* Bun
* Go
* Java
* Gradle
* Maven

If you want to see os supports, check out either [SUPPORTED-OS.md](https://github.com/Necoo33/incli/blob/main/SUPPORTED-OS.md) or look down below.

## Currently Planned to Support Package Managers and Sdk's

* python
* php
* phpmyadmin
* ruby
* ocaml
* zig
* elixir
* erlang
* deno

## Operating System Support

You can look [SUPPORTED-OS](https://github.com/Necoo33/incli/blob/main/SUPPORTED-OS.md) document to learn which distros supported for each program and how to install them.

## How To Use It

General synthax is:

`(that binary) (action) (program) (version number[if the action is 'install'])`. Also `latest` and `lts` arguments are supported for Node.js(continuously updated), Go(continuously updated) and Java(Java version 21.0.2).

For example:

### Rust

`(that binary) install rust`

### Node.js

`(that binary) install node (lts / latest)`

### Go

`(that binary) install go (lts / latest)`

### Bun

`(that binary) install bun`

### Yarn

`(that binary) install yarn`

### Java

`(that binary) install java (11 / lts / latest)`

### Gradle

`(that binary) install gradle 8.8`

### Maven

`(that binary) install maven 3.9.9`
