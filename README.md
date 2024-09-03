# Installation CLI for programming languages

Incli is a Automation Tool For Installing Compilers, Sdk's and Package Managers to different kernels and operating systems.

It has 3 different command types for now:

* install: it is for installing sdk's. 
* help: it is for giving general informations.
* version: it is for writing the version of incli.

If you like that program, consider to give a like on [github repo](https://github.com/Necoo33/incli)

## Currently Planned to Support Package Managers and Sdk's

* Gradle
* Maven
* python
* php
* phpmyadmin
* laravel
* ruby
* ocaml
* zig

## Current Status

In this situation rust, node.js, yarn, bun and golang are supported.

Also we'll make minor updates for supporting new lts and latest releases for Node.js And Golang.

# Rust:

### Windows

I tried that setup on my another computer and it worked.

Be sure you have the latest version of Visual C++ Redistributable for Visual Studio on computer, or you can't run the package.

* Windows 10 Home Basic - &#10003;

I don't have any idea if another windows versions will work or not if latest Redistributable is downloaded, it should be tested. 

### Linux

In linux, that distributions are tested:

* Ubuntu 22.04 LTS - &#10003;
* Ubuntu 22 wsl - &#129300;
* Debian 12 wsl - &#10003;
* Arch wsl - &#10003;
* Kali Linux wsl - &#10003;
* Alma Linux 9 wsl - &#10003;
* CentOS 9 Stream - &#10003;
* Pardus 23 XFCE - &#10003;
* Fedora 39 Workstation - &#10003;
* Rocky Linux 9.4 - &#10003;
* Alpine Linux wsl - &#2049;

You can view more detailed information on "SUPPORTED-OS.md" file about the supports.

### Mac OS

There is no implementation yet

# Node.js

When installing Node.js, you can also specialize a third argument. If you want to install latest long term support version, you can run a command like that: `(that binary) install node lts`, or if you want to download latest version, you can run that command: `(that binary) install node latest`. If you don't specialize third argument, that'll install lts version.

### Windows:

* Windows 10 Home Basic - &#10003;

### Linux:

* Ubuntu 22.04 LTS - &#10003;
* Ubuntu 22 wsl - &#129300;
* Debian 12 wsl - &#10003;
* Arch wsl - &#10003;
* Kali Linux wsl - &#10003;
* Alma Linux 9 wsl - &#10003;
* CentOS 9 Stream - &#10003;
* Pardus 23 XFCE - &#10003;
* Fedora 39 Workstation - &#10003;
* Rocky Linux 9.4 - &#10003;
* Alpine Linux wsl - &#2049;

### Mac Os

There is no implementation yet.

# Bun:

### Windows

Because bun has not windows version, we cannot implemented that yet.

### Linux

In linux, that distributions are tested:

* Ubuntu 22.04 LTS - &#10003;
* Ubuntu 22 wsl - &#10003;
* Debian 12 wsl - &#10003;
* Arch wsl - &#10003;
* Kali Linux wsl - &#10003;
* Alma Linux 9 wsl - &#10003;
* CentOS 9 Stream - &#10003;
* Pardus 23 XFCE - &#10003;
* Fedora 39 Workstation - &#10003;
* Rocky Linux 9.4 - &#10003;
* Alpine Linux wsl - &#2049;

You can view more detailed information on "SUPPORTED-OS.md" file about the supports.

### Mac OS

There is no implementation yet

# Yarn:

### Windows

* Windows 11 - &#10003;
* Windows 10 Home - &#10003;

### Linux

In linux, that distributions are tested:

* Ubuntu 22.04 LTS - &#10003;
* Ubuntu 22 wsl - &#10003;
* Debian 12 wsl - &#10003;
* Arch wsl - &#10003;
* Kali Linux wsl - &#10003;
* Alma Linux 9 wsl - &#10003;
* CentOS 9 Stream - &#10003;
* Pardus 23 XFCE - &#10003;
* Fedora 39 Workstation - &#10003;
* Rocky Linux 9.4 - &#10003;
* Alpine Linux wsl - &#2049;

You can view more detailed information on "SUPPORTED-OS.md" file about the supports.

### Mac OS

There is no implementation yet

# Go

When installing Go, you can also specialize a third argument. If you want to install latest long term support version, you can run a command like that: `(that binary) install go lts`, or if you want to download latest version, you can run that command: `(that binary) install go latest`. If you don't specialize third argument, that'll install lts version.

### Windows:

* Windows 10 Home Basic - &#10003;

### Linux:

* Ubuntu 22.04 LTS - &#10003;
* Ubuntu 22 wsl - &#10003;
* Debian 12 wsl - &#10003;
* Arch wsl - &#10003;
* Kali Linux wsl - &#10003;
* Alma Linux 9 wsl - &#10003;
* CentOS 9 Stream - &#10003;
* Pardus 23 XFCE - &#10003;
* Fedora 39 Workstation - &#10003;
* Rocky Linux 9.4 - &#10003;
* Alpine Linux wsl - &#2049;

### Mac Os

There is no implementation yet.

# Java

You can install pretty much every open jdk versions that released from 8 to 24, which in early access. If you want to download Java 8 on windows, you also have to specialize a third argument, like so: `(that binary) install java 8 ('jre' or 'jdk')` this is because in Java 8 the Java Development Kit doesn't include Java Runtime Environment and you also have to download it seperately, which ultimately means you have to make 2 installation, and you can install it on every linux system without fourth command if it's installable. In newer versions you just use this synthax: `(that binary) install java (version number)`. If you don't specify third argument, it'll install the latest lts version, which is 21.

### Windows:

* Windows 10 Home Basic - &#10003;

### Linux:

* Ubuntu 24.04 LTS - &#10003;
* Ubuntu 22 wsl - &#10003;
* Debian 12 wsl - &#10003;
* Arch wsl - &#10003;
* Kali Linux wsl - &#10003;
* Alma Linux 9 wsl - &#10003;
* CentOS 9 Stream - &#10003;
* Pardus 23 XFCE - &#10003;
* Fedora 39 Workstation - &#10003;
* Rocky Linux 9.4 - 
* Alpine Linux wsl - &#2049;

### Mac Os

There is no implementation yet.

## Guide

You have to compile that app for specific platform that you'll use.

Than You can run that app on your terminal(in windows, powershell works well) like that synthax:

help command:

`(That Binary) help rust` or `(That Binary) help`

install command:

`(That Binary) install rust`

version command:

`(That Binary) version rust` or `(That Binary) version`

## Our Roadmap to Next Releases

The roadmap of this project is this:

* Up until 1.0 release, implement all currently planned sdk's for at least 1 release and distribution.

* Adding support for installing specific versions of sdk's.

* Support for option for adding that program directly to env's when it executed first time

* adding "uninstall" command support and deleting downloaded tools.

I'm also open for contributions about implementing other sdk's and package managers with the test results of that codes. If you want to support that project and contribute, please check out the "CONTRIBUTION_GUIDE.md". If you like that project anyway, please check out the github repo and give a star to that project.

## Our Roadmap to Individual Releases

### v0.7.0

* Jdk support

### v0.8.0

* Gradle Support

### v0.9.0

* Maven Support

### v1.0.0

* Adding Uninstall Support

### v1.1.0

* Python Support

### v1.2.0

* Adding support for downloading everything on demanded path.

### v1.3.0

* Support for adding individual sdk's to system's PATH variable.

### v1.4.0

* Adding Support For Downloading Specific Versions.

### v1.5.0

* Adding Composer Support
* Adding PhpMyAdmin Support

### v1.6.0

* Adding Laravel Support

### v1.7.0

* Adding Ruby Support
* Adding rbenv Support

### v1.8.0

* Adding Zig Support

### v1.9.0

* Adding Ocaml Support

