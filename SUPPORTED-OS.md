# Supported Operating Systems for Various Sdk's

Meaning Of Symbols:

&#10003; - Tested And Works
&#129300; - Likely
&#2049; - it can be or not
&#10060; - not implemented or not able to implement.

### Disclaimers

* In windows, you need to have the latest version of Visual C++ Redistributable for Visual Studio on computer, or you can't run the binary.

* In linux sections if any dist is mentioned with "wsl" key is meaning that tested on that distribution's windows subsystem linux version. If there is no mention about "wsl", than that means it tested on actual versions of that distribution.

And also don't forget to run "sudo (your package manager) update" and "sudo (your package manager) upgrade" command, otherwise application can give error.

## Rust

### Synthax

`(that binary) install rust` and it'll install the latest version.

##### Windows

* Windows 11 Pro - &#129300;
* Windows 10 Home - &#10003;
* Older Windows Releases - &#2049;

#### Linux

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
* Other Linux Distributions - &#10060;

##### MacOs

Not Implemented

## Node.js

### Synthax

When installing Node.js, you can also specialize a third argument. If you want to install latest long term support version, you can run a command like that: `(that binary) install node lts`, or if you want to download latest version, you can run that command: `(that binary) install node latest`. If you don't specialize third argument, that'll install lts version.

#### Windows

* Windows 11 Pro - &#129300;
* Windows 10 Home - &#10003;
* Older Windows Releases - &#2049;

#### Linux

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
* Other Linux Distributions - &#10060;

##### MacOs

Not Implemented

## Bun

### Synthax

`(that binary) install bun` and it'll install the latest version.

#### Windows

* Bun does not support windows yet.

#### Linux

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
* Alpine Linux wsl - &#10060;
* Other Linux Distributions - &#10060;

##### MacOs

Not Implemented

## Yarn

### Synthax

`(that binary) install yarn` and it'll install the latest version.

#### Windows

* Windows 11 Pro - &#10003;
* Windows 10 Home - &#10003;

#### Linux

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
* Alpine Linux wsl - &#10060;
* Other Linux Distributions - &#10060;

##### MacOs

Not Implemented

## Go

### Synthax

When installing Go, you can also specialize a third argument. If you want to install latest long term support version, you can run a command like that: `(that binary) install go lts`, or if you want to download latest version, you can run that command: `(that binary) install go latest`. If you don't specialize third argument, that'll install lts version.

#### Windows

* Windows 10 Home Basic - &#10003;

#### Linux

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

## Java

### Synthax

You can install pretty much every open jdk versions that released from 8 to 24, which in early access. If you want to download Java 8 on windows, you also have to specialize a third argument, like so: `(that binary) install java 8 ('jre' or 'jdk')` this is because in Java 8 the Java Development Kit doesn't include Java Runtime Environment and you also have to download it seperately, which ultimately means you have to make 2 installation, and you can install it on every linux system without fourth command if it's installable. In newer versions you just use this synthax: `(that binary) install java (version number)`. If you don't specify third argument, it'll install the latest lts version, which is 21.

#### Windows

* Windows 10 Home Basic - &#10003;

#### Linux

* Ubuntu 24.04 LTS - &#10003;
* Ubuntu 22.04 LTS - &#129300;
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

#### Mac Os

There is no implementation yet.

## Gradle

### Synthax

You can install every gradle version that available on gradle's official website. Incli downloads the binary version of gradle. You can download it via typing `(that binary) install gradle (version number)`. You can install every version if it's file name ends with "bin" before .zip extension. For example that command installs the gradle 8.10.2: `(that binary) install gradle 8.10.2`. If you want, you can install milestone versions too, like that: `(that binary) install gradle 8.10.2-milestone-1`.

#### Windows

* Windows 11 Pro - &#10003;

#### Linux

* Ubuntu 24.04 LTS - &#129300;
* Ubuntu 22.04 LTS - &#129300;
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

## Maven

You can install pretty much every maven versions via that synthax: `(that binary) install maven (version number[for instance: "3.9.9"])`, and even install beta, alpha, m1 and rc versions via that synthax: `(that binary) install maven 4.0.0-rc-1`. 

#### Windows

* Windows 11 Pro - &#10003;

#### Linux

* Ubuntu 24.04 LTS - &#10003;
* Ubuntu 22.04 LTS - &#129300;
* Ubuntu 22 wsl - &#129300;
* Debian 12 wsl - &#10003;
* Arch wsl - &#10003;
* Kali Linux wsl - &#10003;
* Alma Linux 9 wsl - &#10003;
* CentOS 9 Stream - &#10003;
* Pardus 23 XFCE - &#10003;
* Fedora 39 Workstation - &#129300;
* Fedora 40 wsl - &#10003;
* Rocky Linux 9.4 - &#10003;
* Alpine Linux wsl - &#2049;

### Mac Os

There is no implementation yet.