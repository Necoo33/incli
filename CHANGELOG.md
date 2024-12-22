# Incli Changelog

## v0.10.1

Upgraded Node.js's latest version to 23.5.0 .

## v0.10.0

Added Maven Support.
Added `lts` and `latest` support for java installations. `lts` is set to version 21.0.2 and `latest` set to version 24-ea.

## v0.9.1

A potential bug were fixed on various installations of java on linux.

## v0.9.0

All Java installation functions updated. Now, all java installations add `JAVA_HOME` environment variable to the system on windows and user on linux. Since our next program to support, maven needs to work with that env, it'd be a better behavior to adding that env on the moment of installation of java. But we'll check if `JAVA_HOME` env exist in the moment of installation of maven, so that'll be backward compatible. So, this release is kind of a preparation release to support maven better.
sys-info-extended version upgraded to 0.9.1 .
Upgraded Node.js's latest version to 23.4.0 .
README.md file modified.
Support for maven installations postponed to v0.10.0 version.

## v0.8.6

Upgraded Node.js's lts version to 22.12.0 .
Upgraded Go's lts version to 1.23.4 .

## v0.8.5

Upgraded Node.js's latest version to 23.3.0 .
Upgraded zip's version to 2.2.1 .

## v0.8.4

Upgraded Node.js's latest version to 23.2.0 .
Upgraded Go's lts version to 1.23.3 .

## v0.8.3

Upgraded Node.js's lts version to 22.11.0 .

## v0.8.2

Upgraded Node.js's latest version to 23.1.0 .

## v0.8.1

Upgraded Node.js's latest version to 23.0.0 .
sys-info-extended version upgraded to 0.8.1

## v0.8.0

Added Gradle Support.
Added zip liblary on dependencies for extracting .zip files.
Some bugs about configuration of incli_envs.sh file debugged.
Upgraded Node.js's lts version to 20.18.0 .

## v0.7.5

Upgraded Go's lts version to 1.23.2 .
sys-info-extended liblary version upgraded to 0.7.0 .

## v0.7.4

Upgraded Node.js's latest version to 22.9.0 .
Fixed Some Documentation Typos.

## v0.7.3

Upgraded Go's lts version to 1.23.1 .
sys-info-extended liblary version upgraded to 0.6.0 .

## v0.7.2

Fixed some documentation typos.

## v0.7.1

README.md and SUPPORTED-OS.md files updated.
Upgraded Node.js's latest version to 22.8.0 .

## v0.7.0

Added Java Virtual Machine Support.
Upgraded Go's lts version to 1.23.0 .
sys-info-extended liblary version upgraded to 0.5.0 .

## v0.6.1

README.md file updated as well.

## v0.6.0

Added Rocky Linux To Supported Distros .
Upgraded Node.js's latest version to 22.7.0 .
Upgraded Node.js's lts version to 20.17.0 .

## v0.5.16

Upgraded Node.js's latest version to 22.6.0 .
Upgraded Go's lts version to 1.22.6 .

## v0.5.15

Upgraded Node.js's lts version to 20.16.0 .
Upgraded Node.js's latest version to 22.5.1 .

## v0.5.14

Upgraded Node.js's latest version to 22.5.0 .
Upgraded Go's latest version to 1.23rc2 .

## v0.5.13

Upgraded Node.js's lts version to 20.15.1 .
Upgraded Node.js's latest version to 22.4.1 .

## v0.5.12

Upgraded Go's lts version to 1.22.5 .
Upgraded Node.js's latest version to 22.4.0 .

## v0.5.11

Upgraded Go's latest version to v1.23rc1 .

## v0.5.10

Upgraded Node.js's latest version to 20.15.0 .

## v0.5.9

Upgraded Node.js's latest version to 22.3.0 .

## v0.5.8

Upgraded Go's lts version to 1.22.4 .

## v0.5.7

Upgraded Node.js's lts version to 20.14.0 .
Upgraded Node.js's latest version to 22.2.0

## v0.5.6

Upgraded Go's lts version to 1.22.3.
Upgraded Node.js's lts version to 20.13.1 .
Upgraded Node.js's latest version to 22.1.0


## v0.5.5

Upgraded Node.js's latest version to 22.0.0 .

## v0.5.4

Upgraded Node.js's lts version to 20.12.2 .
Upgraded Node.js's latest version to 21.7.3 .
sys-info-extended liblary version upgraded to 0.4.0

## v0.5.3

Upgraded Go's lts version to 1.22.2 .
Upgraded Node.js's lts version to 20.12.1 .
Upgraded Node.js's latest version to 21.7.2 .

## v0.5.2

Upgraded Go's lts version to 1.22.1 .
Upgraded Node.js's lts version to 20.12.0 .
Upgraded Node.js's latest version to 21.7.1 .

## v0.5.1

Upgraded Go's lts version to 1.22.0 .
Planned features section updated.
sys-info-extended liblary version upgraded to 0.3.0

## v0.5.0

Added Golang Support.
Project structure changed, from now there is no need to be in user folder for adding node.js to env's. It both downloads it truly for root user and other users.
Also there is no need for being in user folder for installing golang and adding env's of it.
From now on, there is no need to be in a certain directory to make installation correctly.

## v0.4.0

Added yarn support.
Fixed Some Documentation Typos.
Fixed a bug about node.js's installation of lts version on some linux distros.
Supported Node.js lts version upgraded to v20.11.0 .

## v0.3.1

Node.js latest version support updated to v21.6.1 . 
Some other documentation changes occured.
newest version of [sys-info-extended](https://crates.io/crates/sys-info-extended) package will be used now.

## v0.3.0

Switched sys-info liblary to sys-info-extended liblary.
Project Restructured.
Added Bun Support For Linux Distros.

## v0.2.1

Some Documentation Fixes, Nothing Other Changed.

## v0.2.0

Added Node.js support for Windows and Linux And Some Documentation Fixes.

## v0.1.1

Some Documentation Fixes, Nothing Changes

## v0.1.0

Incli Released with full support for rust on windows and most known linux distros.
