# Installation CLI for programming languages

Incli is a Automation Tool For Installing Compilers, Sdk's and Package Managers to different kernels and operating systems.

It has 3 different command types for now:

* install: it is for installing sdk's. 
* help: it is for giving general informations.
* version: it is for writing the version of incli.

## Currently Planned to Support Package Managers and Sdk's

* Rust (Cargo and Rustc)
* Golang
* Java - Gradle
* Java - Maven
* python
* Node.js (npm)
* Bun
* Yarn

## Current Status

In this situation, cli only supports rust for only windows and some linux distributions.

It is tested on this machines:

### Windows

I tried that setup on my another computer and it worked.

Be sure you have the latest version of Visual C++ Redistributable for Visual Studio on computer, or you can't run the package.

* Windows 10 Home Basic

I don't have any idea if another windows versions will work or not if latest Redistributable is downloaded, it should be tested. 

### Linux

In linux, that distributions are tested:

* Debian 12 Wsl
* Arch Wsl
* Kali Linux Wsl

And since ubuntu 22 is a Debian based Linux Distribution, it's very likely that will also work on that.

### Mac OS

There is no implementation yet

## Guide

You have to compile that app for specific platform that you'll use.

Than You can run that app on your terminal(in windows, powershell works well) like that synthax:

help command:

`(That Binary) help rust` or `(That Binary) help`

install command:

`(That Binary) install rust`

version command:

`(That Binary) version rust` or `(That Binary) help`

## Our Roadmap to First Stable Release

The roadmap of this project is this:

* Up until 1.0 release, implement all currently planned sdk's for at least 1 release and distribution.

* Adding support for installing specific versions of sdk's.

* Support for option for adding that program directly to env's when it executed first time

I'm also open for contributions about implementing other sdk's and package managers with the test results of that codes. If you want to 