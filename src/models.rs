use crate::utils;
use crate::rust;
use crate::node;
use crate::bun;
use crate::yarn;
use crate::go;
use crate::java;
use crate::gradle;
use crate::maven;

#[derive(PartialEq, Clone)]
pub enum CommandType {
    Version, Install, Help
}

#[derive(Clone)]
pub enum OsType {
    Linux, Darwin, Windows
}

#[derive(Clone)]
pub struct UserSession {
    pub os_type: String,
    pub os_release: String,
    pub current_user: String
}

#[derive(Clone)]
pub struct UserAction {
    pub os_type: OsType,
    pub os_release: String,
    pub current_user: String,
    pub first_arg: CommandType,
    pub second_arg: String,
    pub third_arg: String,
    pub fourth_arg: String,
}

pub trait CreateAction {
    fn action(&self, cmd_type: String, second_arg: String, third_arg: String, fourth_arg: String) -> UserAction;
}

pub trait Execution {
    fn execute_windows(&self) -> Self;
    fn execute_linux(&self) -> Self;
    fn execute_darwin(&self) -> Self;
    fn install_rust(&self) -> Self;
    fn install_jdk(&self) -> Self;
    fn install_gradle(&self) -> Self;
    fn install_maven(&self) -> Self;
    fn install_nodejs(&self) -> Self;
    fn install_bun(&self) -> Self;
    fn install_yarn(&self) -> Self;
    fn install_go(&self) -> Self;
    fn install_python(&self) -> Self;
}

pub trait Help {
    fn log_help(&self) -> Self;
    fn log_help_error(&self) -> Self;
    fn rust_help(&self) -> Self;
    fn go_help(&self) -> Self;
    fn nodejs_help(&self) -> Self;
    fn bun_help(&self) -> Self;
    fn yarn_help(&self) -> Self;
    fn jdk_help(&self) -> Self;
    fn gradle_help(&self) -> Self;
    fn maven_help(&self) -> Self;
    fn python_help(&self) -> Self;
}

pub trait Version {
    fn log_version(&self) -> Self;
    fn log_version_error(&self) -> Self;
    fn rust_version(&self) -> Self;
    fn go_version(&self) -> Self;
    fn nodejs_version(&self) -> Self;
    fn bun_version(&self) -> Self;
    fn yarn_version(&self) -> Self;
    fn jdk_version(&self) -> Self;
    fn gradle_version(&self) -> Self;
    fn maven_version(&self) -> Self;
    fn python_version(&self) -> Self;
}

impl CreateAction for UserSession {
    fn action(&self, cmd_type: String, second_arg: String, third_arg: String, fourth_arg: String) -> UserAction {
        let operating_system_type;
        let command_type;

        match self.os_type.as_str() {
            "Windows" => operating_system_type = OsType::Windows,
            "Linux" => operating_system_type = OsType::Linux,
            "Darwin" => operating_system_type = OsType::Darwin,
            &_ => panic!("unsupported operating system")
        }

        match cmd_type.as_str() {
            "install" => command_type = CommandType::Install,
            "version" => command_type = CommandType::Version,
            "help" => command_type = CommandType::Help,
            &_ => panic!("unrecognized second argument. if you don't know how to use this tool, please type 'incli help' to learn it")
        }

        let s_arg = match second_arg.as_str() {
            "rust" => "rust".to_string(),
            "java" => "java".to_string(),
            "go" => "go".to_string(),
            "gradle" => "gradle".to_string(),
            "maven" => "maven".to_string(),
            "zig" => "zig".to_string(),
            "composer" => "composer".to_string(),
            "android" => "android".to_string(),
            "node" => "node".to_string(),
            "bun" => "bun".to_string(),
            "yarn" => "yarn".to_string(),
            "python" => "python".to_string(),
            &_ => "".to_string()
        };

        return UserAction {
            os_release: self.os_release.clone(),
            os_type: operating_system_type,
            current_user: self.current_user.clone(),
            first_arg: command_type.clone(),
            second_arg: s_arg,
            third_arg: third_arg.to_string(),
            fourth_arg: fourth_arg.to_string()
        }
    }
}

impl Execution for UserAction {
    fn execute_windows(&self) -> Self {
        match self.os_type {
            OsType::Windows => {
                self.install_rust()
                    .install_go()
                    .install_jdk()
                    .install_gradle()
                    .install_maven()
                    .install_nodejs()
                    .install_bun()
                    .install_yarn()
                    .install_python();

                return Self {
                    os_type: self.os_type.clone(),
                    os_release: self.os_release.clone(),
                    current_user: self.current_user.clone(),
                    first_arg: self.first_arg.clone(),
                    second_arg: self.second_arg.clone(),
                    third_arg: self.third_arg.clone(),
                    fourth_arg: self.fourth_arg.clone()
                };
            }

            _ => {
                return Self {
                    os_type: self.os_type.clone(),
                    os_release: self.os_release.clone(),
                    current_user: self.current_user.clone(),
                    first_arg: self.first_arg.clone(),
                    second_arg: self.second_arg.clone(),
                    third_arg: self.third_arg.clone(),
                    fourth_arg: self.fourth_arg.clone()
                };
            }
        }

    }

    fn execute_linux(&self) -> Self {
        match self.os_type {
            OsType::Linux => {
                self.install_rust()
                    .install_go()
                    .install_jdk()
                    .install_gradle()
                    .install_maven()
                    .install_nodejs()
                    .install_bun()
                    .install_yarn()
                    .install_python();

                return Self {
                    os_type: self.os_type.clone(),
                    os_release: self.os_release.clone(),
                    current_user: self.current_user.clone(),
                    first_arg: self.first_arg.clone(),
                    second_arg: self.second_arg.clone(),
                    third_arg: self.third_arg.clone(),
                    fourth_arg: self.fourth_arg.clone()
                };
            },
            _ => {
                return Self {
                    os_type: self.os_type.clone(),
                    os_release: self.os_release.clone(),
                    current_user: self.current_user.clone(),
                    first_arg: self.first_arg.clone(),
                    second_arg: self.second_arg.clone(),
                    third_arg: self.third_arg.clone(),
                    fourth_arg: self.fourth_arg.clone()
                };
            }
        }
    }

    fn execute_darwin(&self) -> Self {
        match self.os_type {
            OsType::Darwin => {
                self.install_rust()
                    .install_go()
                    .install_jdk()
                    .install_gradle()
                    .install_maven()
                    .install_nodejs()
                    .install_bun()
                    .install_yarn()
                    .install_python();

                return Self {
                    os_type: self.os_type.clone(),
                    os_release: self.os_release.clone(),
                    current_user: self.current_user.clone(),
                    first_arg: self.first_arg.clone(),
                    second_arg: self.second_arg.clone(),
                    third_arg: self.third_arg.clone(),
                    fourth_arg: self.fourth_arg.clone()
                };
            },
            _ => {
                return Self {
                    os_type: self.os_type.clone(),
                    os_release: self.os_release.clone(),
                    current_user: self.current_user.clone(),
                    first_arg: self.first_arg.clone(),
                    second_arg: self.second_arg.clone(),
                    third_arg: self.third_arg.clone(),
                    fourth_arg: self.fourth_arg.clone()
                };
            }
        }
    }

    fn install_rust(&self) -> Self {
        if self.first_arg == CommandType::Install && self.second_arg == "rust".to_string() {
            match self.os_type {
                OsType::Windows => rust::install_rust_on_windows(),
                OsType::Linux => {
                    let linux_dist = utils::return_linux_dist_etc_os_release();
                    let mut null_var = "".to_string();

                    match linux_dist {
                        "ubuntu" => rust::install_rust_on_debian_based_distros(),
                        "debian" => rust::install_rust_on_debian_based_distros(),
                        "pardus" => rust::install_rust_on_debian_based_distros(),
                        "arch wsl" => rust::install_rust_on_arch_wsl_and_kali(),
                        "kali linux" => rust::install_rust_on_arch_wsl_and_kali(),
                        "fedora" => rust::install_rust_on_fedora_and_centos(),
                        "centos" => rust::install_rust_on_fedora_and_centos(),
                        "alma linux" => rust::install_rust_on_fedora_and_centos(),
                        "rocky" => rust::install_rust_on_fedora_and_centos(),
                        "alpine" => rust::install_rust_on_alpine_linux(),
                        &_ => null_var = "none".to_string()
                    }

                    if null_var == "none".to_string() {
                        if utils::check_if_linux_dist_is_arch_linux() {
                            rust::install_rust_on_arch_wsl_and_kali()
                        }
                    }
                },
                OsType::Darwin => println!("Unfortunately, we don't have Mac Os Support Yet."),
                
            }
        }

        return Self {
            os_type: self.os_type.clone(),
            os_release: self.os_release.clone(),
            current_user: self.current_user.clone(),
            first_arg: self.first_arg.clone(),
            second_arg: self.second_arg.clone(),
            third_arg: self.third_arg.clone(),
            fourth_arg: self.fourth_arg.clone()
        };
    }

    fn install_go(&self) -> Self {
        if self.first_arg == CommandType::Install && self.second_arg == "go".to_string() {
            match self.os_type {
                OsType::Windows => {
                    match self.third_arg.as_str() {
                        "lts" => go::install_go_on_windows("https://go.dev/dl/go1.24.3.windows-amd64.msi", "go1.24.3.windows-amd64.msi"),
                        "" => go::install_go_on_windows("https://go.dev/dl/go1.24.3.windows-amd64.msi", "go1.24.3.windows-amd64.msi"),
                        "latest" => go::install_go_on_windows("https://go.dev/dl/go1.24rc3.windows-amd64.msi", "go1.24rc3.windows-amd64.msi"),
                        &_ => go::install_go_error()
                    };
                },
                OsType::Linux => {
                    let linux_dist = utils::return_linux_dist_etc_os_release();
                    let mut null_var = "".to_string();

                    match linux_dist {
                        "ubuntu" => {
                            match self.third_arg.as_str() {
                                "lts" => go::install_go_on_debian_based_distros("https://go.dev/dl/go1.24.3.linux-amd64.tar.gz", "go1.24.3.linux-amd64.tar.gz"),
                                "" => go::install_go_on_debian_based_distros("https://go.dev/dl/go1.24.3.linux-amd64.tar.gz", "go1.24.3.linux-amd64.tar.gz"),
                                "latest" => go::install_go_on_debian_based_distros("https://go.dev/dl/go1.24rc3.linux-amd64.tar.gz", "go1.24rc3.linux-amd64.tar.gz"),
                                &_ => go::install_go_error()
                            }
                        },
                        "debian" => {
                            match self.third_arg.as_str() {
                                "lts" => go::install_go_on_debian_based_distros("https://go.dev/dl/go1.24.3.linux-amd64.tar.gz", "go1.24.3.linux-amd64.tar.gz"),
                                "" => go::install_go_on_debian_based_distros("https://go.dev/dl/go1.24.3.linux-amd64.tar.gz", "go1.24.3.linux-amd64.tar.gz"),
                                "latest" => go::install_go_on_debian_based_distros("https://go.dev/dl/go1.24rc3.linux-amd64.tar.gz", "go1.24rc3.linux-amd64.tar.gz"),
                                &_ => go::install_go_error()
                            }
                        },
                        "pardus" => {
                            match self.third_arg.as_str() {
                                "lts" => go::install_go_on_debian_based_distros("https://go.dev/dl/go1.24.3.linux-amd64.tar.gz", "go1.24.3.linux-amd64.tar.gz"),
                                "" => go::install_go_on_debian_based_distros("https://go.dev/dl/go1.24.3.linux-amd64.tar.gz", "go1.24.3.linux-amd64.tar.gz"),
                                "latest" => go::install_go_on_debian_based_distros("https://go.dev/dl/go1.24rc3.linux-amd64.tar.gz", "go1.24rc3.linux-amd64.tar.gz"),
                                &_ => go::install_go_error()
                            }
                        },
                        "arch wsl" => {
                            match self.third_arg.as_str() {
                                "lts" => go::install_go_on_arch_linux("https://go.dev/dl/go1.24.3.linux-amd64.tar.gz", "go1.24.3.linux-amd64.tar.gz"),
                                "" => go::install_go_on_arch_linux("https://go.dev/dl/go1.24.3.linux-amd64.tar.gz", "go1.24.3.linux-amd64.tar.gz"),
                                "latest" => go::install_go_on_arch_linux("https://go.dev/dl/go1.24rc3.linux-amd64.tar.gz", "go1.24rc3.linux-amd64.tar.gz"),
                                &_ => go::install_go_error()
                            }
                        }
                        "kali linux" => {
                            match self.third_arg.as_str() {
                                "lts" => go::install_go_on_debian_based_distros("https://go.dev/dl/go1.24.3.linux-amd64.tar.gz", "go1.24.3.linux-amd64.tar.gz"),
                                "" => go::install_go_on_debian_based_distros("https://go.dev/dl/go1.24.3.linux-amd64.tar.gz", "go1.24.3.linux-amd64.tar.gz"),
                                "latest" => go::install_go_on_debian_based_distros("https://go.dev/dl/go1.24rc3.linux-amd64.tar.gz", "go1.24rc3.linux-amd64.tar.gz"),
                                &_ => go::install_go_error()
                            }
                        },
                        "fedora" => {
                            match self.third_arg.as_str() {
                                "lts" => go::install_go_on_centos_and_fedora("https://go.dev/dl/go1.24.3.linux-amd64.tar.gz", "go1.24.3.linux-amd64.tar.gz"),
                                "" => go::install_go_on_centos_and_fedora("https://go.dev/dl/go1.24.3.linux-amd64.tar.gz", "go1.24.3.linux-amd64.tar.gz"),
                                "latest" => go::install_go_on_centos_and_fedora("https://go.dev/dl/go1.24rc3.linux-amd64.tar.gz", "go1.24rc3.linux-amd64.tar.gz"),
                                &_ => go::install_go_error()
                            }
                        },
                        "centos" => {
                            match self.third_arg.as_str() {
                                "lts" => go::install_go_on_centos_and_fedora("https://go.dev/dl/go1.24.3.linux-amd64.tar.gz", "go1.24.3.linux-amd64.tar.gz"),
                                "" => go::install_go_on_centos_and_fedora("https://go.dev/dl/go1.24.3.linux-amd64.tar.gz", "go1.24.3.linux-amd64.tar.gz"),
                                "latest" => go::install_go_on_centos_and_fedora("https://go.dev/dl/go1.24rc3.linux-amd64.tar.gz", "go1.24rc3.linux-amd64.tar.gz"),
                                &_ => go::install_go_error()
                            }
                        },
                        "rocky" => {
                            match self.third_arg.as_str() {
                                "lts" => go::install_go_on_rocky_linux("https://go.dev/dl/go1.24.3.linux-amd64.tar.gz", "go1.24.3.linux-amd64.tar.gz"),
                                "" => go::install_go_on_rocky_linux("https://go.dev/dl/go1.24.3.linux-amd64.tar.gz", "go1.24.3.linux-amd64.tar.gz"),
                                "latest" => go::install_go_on_rocky_linux("https://go.dev/dl/go1.24rc3.linux-amd64.tar.gz", "go1.24rc3.linux-amd64.tar.gz"),
                                &_ => go::install_go_error()
                            }
                        }
                        "alma linux" => {
                            match self.third_arg.as_str() {
                                "lts" => go::install_go_on_alma_linux("https://go.dev/dl/go1.24.3.linux-amd64.tar.gz", "go1.24.3.linux-amd64.tar.gz"),
                                "" => go::install_go_on_alma_linux("https://go.dev/dl/go1.24.3.linux-amd64.tar.gz", "go1.24.3.linux-amd64.tar.gz"),
                                "latest" => go::install_go_on_alma_linux("https://go.dev/dl/go1.24rc3.linux-amd64.tar.gz", "go1.24rc3.linux-amd64.tar.gz"),
                                &_ => go::install_go_error()
                            }
                        },
                        "alpine" => go::install_go_on_alpine_linux("https://go.dev/dl/go1.24.3.linux-amd64.tar.gz", "go1.24.3.linux-amd64.tar.gz"),
                        &_ => null_var = "none".to_string()
                    }

                    if null_var == "none".to_string() {
                        if utils::check_if_linux_dist_is_arch_linux() {
                            match self.third_arg.as_str() {
                                "lts" => go::install_go_on_arch_linux("https://go.dev/dl/go1.24.3.linux-amd64.tar.gz", "go1.24.3.linux-amd64.tar.gz"),
                                "" => go::install_go_on_arch_linux("https://go.dev/dl/go1.24.3.linux-amd64.tar.gz", "go1.24.3.linux-amd64.tar.gz"),
                                "latest" => go::install_go_on_arch_linux("https://go.dev/dl/go1.24rc3.linux-amd64.tar.gz", "go1.24rc3.linux-amd64.tar.gz"),
                                &_ => go::install_go_error()
                            }
                        }
                    }
                },
                OsType::Darwin => println!("That's not implemented yet.")
            }
        }

        return Self {
            os_type: self.os_type.clone(),
            os_release: self.os_release.clone(),
            current_user: self.current_user.clone(),
            first_arg: self.first_arg.clone(),
            second_arg: self.second_arg.clone(),
            third_arg: self.third_arg.clone(),
            fourth_arg: self.fourth_arg.clone()
        };
    }

    fn install_jdk(&self) -> Self {
        if self.first_arg == CommandType::Install && self.second_arg == "java".to_string() {
            match self.os_type {
                OsType::Windows => match self.third_arg.as_str() {
                    "8" => match self.fourth_arg.as_str() {
                        "jdk" => java::install_java_on_windows("https://github.com/adoptium/temurin8-binaries/releases/download/jdk8u422-b05/OpenJDK8U-jdk_x64_windows_hotspot_8u422b05.msi", "jdk-8.msi", "8"),
                        "jre" => java::install_java_on_windows("https://github.com/adoptium/temurin8-binaries/releases/download/jdk8u422-b05/OpenJDK8U-jre_x64_windows_hotspot_8u422b05.msi", "jre-8.msi", "8"),
                        &_ => ()
                    },
                    "9" => println!("Unfortunately, There Is no installation of Java 9 for Windows"),
                    "10" => println!("Unfortunately, There Is no installation of Java 10 for Windows"),
                    "11" => java::install_java_on_windows("https://download.java.net/java/GA/jdk11/9/GPL/openjdk-11.0.2_windows-x64_bin.zip", "jdk-11.zip", "11"),
                    "12" => java::install_java_on_windows("https://download.java.net/java/GA/jdk12.0.2/e482c34c86bd4bf8b56c0b35558996b9/10/GPL/openjdk-12.0.2_windows-x64_bin.zip", "jdk-12.zip", "12"),
                    "13" => java::install_java_on_windows("https://download.java.net/java/GA/jdk13.0.2/d4173c853231432d94f001e99d882ca7/8/GPL/openjdk-13.0.2_windows-x64_bin.zip", "jdk-13.zip", "13"),
                    "14" => java::install_java_on_windows("https://download.java.net/java/GA/jdk14.0.2/205943a0976c4ed48cb16f1043c5c647/12/GPL/openjdk-14.0.2_windows-x64_bin.zip", "jdk-14.zip", "14"),
                    "15" => java::install_java_on_windows("https://download.java.net/java/GA/jdk15.0.2/0d1cfde4252546c6931946de8db48ee2/7/GPL/openjdk-15.0.2_windows-x64_bin.zip", "jdk-15.zip", "15"),
                    "16" => java::install_java_on_windows("https://download.java.net/java/GA/jdk16.0.2/d4a915d82b4c4fbb9bde534da945d746/7/GPL/openjdk-16.0.2_windows-x64_bin.zip", "jdk-16.zip", "16"),
                    "17" => java::install_java_on_windows("https://download.java.net/java/GA/jdk17.0.2/dfd4a8d0985749f896bed50d7138ee7f/8/GPL/openjdk-17.0.2_windows-x64_bin.zip", "jdk-17.zip", "17"),
                    "18" => java::install_java_on_windows("https://download.java.net/java/GA/jdk18.0.2/f6ad4b4450fd4d298113270ec84f30ee/9/GPL/openjdk-18.0.2_windows-x64_bin.zip", "jdk-18.zip", "18"),
                    "19" => java::install_java_on_windows("https://download.java.net/java/GA/jdk19.0.1/afdd2e245b014143b62ccb916125e3ce/10/GPL/openjdk-19.0.1_windows-x64_bin.zip", "jdk-19.zip", "19"),
                    "20" => java::install_java_on_windows("https://download.java.net/java/GA/jdk20.0.2/6e380f22cbe7469fa75fb448bd903d8e/9/GPL/openjdk-20.0.2_windows-x64_bin.zip", "jdk-20.zip", "20"),
                    "21" => java::install_java_on_windows("https://download.java.net/java/GA/jdk21.0.2/f2283984656d49d69e91c558476027ac/13/GPL/openjdk-21.0.2_windows-x64_bin.zip", "jdk-21.zip", "21"),
                    "" => java::install_java_on_windows("https://download.java.net/java/GA/jdk21.0.2/f2283984656d49d69e91c558476027ac/13/GPL/openjdk-21.0.2_windows-x64_bin.zip", "jdk-21.zip", "21"),
                    "22" => java::install_java_on_windows("https://download.java.net/java/GA/jdk22.0.1/c7ec1332f7bb44aeba2eb341ae18aca4/8/GPL/openjdk-22.0.1_windows-x64_bin.zip", "jdk-22.zip", "22"),
                    "23" => java::install_java_on_windows("https://download.java.net/java/early_access/jdk24/13/GPL/openjdk-24-ea+13_windows-x64_bin.zip", "jdk-23.zip", "23"),
                    "24" => todo!(),
                    _ => ()
                },
                OsType::Linux => {
                    let linux_dist = utils::return_linux_dist_etc_os_release();
                    let mut null_var = "".to_string();

                    match linux_dist {
                        "ubuntu" => {
                            match self.third_arg.as_str() {
                                "8" => java::install_java_on_debian_based_distros("", "", "8"),
                                "9" => java::install_java_on_debian_based_distros("https://download.java.net/java/GA/jdk9/9.0.4/binaries/openjdk-9.0.4_linux-x64_bin.tar.gz", "jdk-9.tar.gz", "9"),
                                "10" => java::install_java_on_debian_based_distros("https://download.java.net/java/GA/jdk10/10.0.2/19aef61b38124481863b1413dce1855f/13/openjdk-10.0.2_linux-x64_bin.tar.gz", "jdk-10.tar.gz", "10"),
                                "11" => java::install_java_on_debian_based_distros("https://download.java.net/java/GA/jdk11/9/GPL/openjdk-11.0.2_linux-x64_bin.tar.gz", "jdk-11.tar.gz", "11"),
                                "12" => java::install_java_on_debian_based_distros("https://download.java.net/java/GA/jdk12.0.2/e482c34c86bd4bf8b56c0b35558996b9/10/GPL/openjdk-12.0.2_linux-x64_bin.tar.gz", "jdk-12.tar.gz", "12"),
                                "13" => java::install_java_on_debian_based_distros("https://download.java.net/java/GA/jdk13.0.2/d4173c853231432d94f001e99d882ca7/8/GPL/openjdk-13.0.2_linux-x64_bin.tar.gz", "jdk-13.tar.gz", "13"),
                                "14" => java::install_java_on_debian_based_distros("https://download.java.net/java/GA/jdk14.0.2/205943a0976c4ed48cb16f1043c5c647/12/GPL/openjdk-14.0.2_linux-x64_bin.tar.gz", "jdk-14.tar.gz", "14"),
                                "15" => java::install_java_on_debian_based_distros("https://download.java.net/java/GA/jdk15.0.2/0d1cfde4252546c6931946de8db48ee2/7/GPL/openjdk-15.0.2_linux-x64_bin.tar.gz", "jdk-15.tar.gz", "15"),
                                "16" => java::install_java_on_debian_based_distros("https://download.java.net/java/GA/jdk16.0.2/d4a915d82b4c4fbb9bde534da945d746/7/GPL/openjdk-16.0.2_linux-x64_bin.tar.gz", "jdk-16.tar.gz", "16"),
                                "17" => java::install_java_on_debian_based_distros("https://download.java.net/java/GA/jdk17.0.2/dfd4a8d0985749f896bed50d7138ee7f/8/GPL/openjdk-17.0.2_linux-x64_bin.tar.gz", "jdk-17.tar.gz", "17"),
                                "18" => java::install_java_on_debian_based_distros("https://download.java.net/java/GA/jdk18.0.2/f6ad4b4450fd4d298113270ec84f30ee/9/GPL/openjdk-18.0.2_linux-x64_bin.tar.gz", "jdk-18.tar.gz", "18"),
                                "19" => java::install_java_on_debian_based_distros("https://download.java.net/java/GA/jdk19.0.1/afdd2e245b014143b62ccb916125e3ce/10/GPL/openjdk-19.0.1_linux-x64_bin.tar.gz", "jdk-19.tar.gz", "19"),
                                "20" => java::install_java_on_debian_based_distros("https://download.java.net/java/GA/jdk20.0.2/6e380f22cbe7469fa75fb448bd903d8e/9/GPL/openjdk-20.0.2_linux-x64_bin.tar.gz", "jdk-20.tar.gz", "20"),
                                "21" => java::install_java_on_debian_based_distros("https://download.java.net/java/GA/jdk21.0.2/f2283984656d49d69e91c558476027ac/13/GPL/openjdk-21.0.2_linux-x64_bin.tar.gz", "jdk-21.tar.gz", "21"),
                                "" => java::install_java_on_debian_based_distros("https://download.java.net/java/GA/jdk21.0.2/f2283984656d49d69e91c558476027ac/13/GPL/openjdk-21.0.2_linux-x64_bin.tar.gz", "jdk-21.tar.gz", "21"),
                                "lts" => java::install_java_on_debian_based_distros("https://download.java.net/java/GA/jdk21.0.2/f2283984656d49d69e91c558476027ac/13/GPL/openjdk-21.0.2_linux-x64_bin.tar.gz", "jdk-21.tar.gz", "21"),
                                "22" => java::install_java_on_debian_based_distros("https://download.java.net/java/GA/jdk22.0.1/c7ec1332f7bb44aeba2eb341ae18aca4/8/GPL/openjdk-22.0.1_linux-x64_bin.tar.gz", "jdk-22.tar.gz", "22"),
                                "23" => java::install_java_on_debian_based_distros("https://download.java.net/java/GA/jdk23/3c5b90190c68498b986a97f276efd28a/37/GPL/openjdk-23_linux-x64_bin.tar.gz", "jdk-23.tar.gz", "23"),
                                "24" => java::install_java_on_debian_based_distros("https://download.java.net/java/early_access/jdk24/13/GPL/openjdk-24-ea+13_linux-x64_bin.tar.gz", "java-24-ea.tar.gz", "24"),
                                "latest" => java::install_java_on_debian_based_distros("https://download.java.net/java/GA/jdk21.0.2/f2283984656d49d69e91c558476027ac/13/GPL/openjdk-21.0.2_linux-x64_bin.tar.gz", "jdk-21.tar.gz", "21"),
                                &_ => ()
                            }
                        },
                        "debian" => {
                            match self.third_arg.as_str() {
                                "8" => java::install_java_on_debian_based_distros("", "", "8"),
                                "9" => java::install_java_on_debian_based_distros("https://download.java.net/java/GA/jdk9/9.0.4/binaries/openjdk-9.0.4_linux-x64_bin.tar.gz", "jdk-9.tar.gz", "9"),
                                "10" => java::install_java_on_debian_based_distros("https://download.java.net/java/GA/jdk10/10.0.2/19aef61b38124481863b1413dce1855f/13/openjdk-10.0.2_linux-x64_bin.tar.gz", "jdk-10.tar.gz", "10"),
                                "11" => java::install_java_on_debian_based_distros("https://download.java.net/java/GA/jdk11/9/GPL/openjdk-11.0.2_linux-x64_bin.tar.gz", "jdk-11.tar.gz", "11"),
                                "12" => java::install_java_on_debian_based_distros("https://download.java.net/java/GA/jdk12.0.2/e482c34c86bd4bf8b56c0b35558996b9/10/GPL/openjdk-12.0.2_linux-x64_bin.tar.gz", "jdk-12.tar.gz", "12"),
                                "13" => java::install_java_on_debian_based_distros("https://download.java.net/java/GA/jdk13.0.2/d4173c853231432d94f001e99d882ca7/8/GPL/openjdk-13.0.2_linux-x64_bin.tar.gz", "jdk-13.tar.gz", "13"),
                                "14" => java::install_java_on_debian_based_distros("https://download.java.net/java/GA/jdk14.0.2/205943a0976c4ed48cb16f1043c5c647/12/GPL/openjdk-14.0.2_linux-x64_bin.tar.gz", "jdk-14.tar.gz", "14"),
                                "15" => java::install_java_on_debian_based_distros("https://download.java.net/java/GA/jdk15.0.2/0d1cfde4252546c6931946de8db48ee2/7/GPL/openjdk-15.0.2_linux-x64_bin.tar.gz", "jdk-15.tar.gz", "15"),
                                "16" => java::install_java_on_debian_based_distros("https://download.java.net/java/GA/jdk16.0.2/d4a915d82b4c4fbb9bde534da945d746/7/GPL/openjdk-16.0.2_linux-x64_bin.tar.gz", "jdk-16.tar.gz", "16"),
                                "17" => java::install_java_on_debian_based_distros("https://download.java.net/java/GA/jdk17.0.2/dfd4a8d0985749f896bed50d7138ee7f/8/GPL/openjdk-17.0.2_linux-x64_bin.tar.gz", "jdk-17.tar.gz", "17"),
                                "18" => java::install_java_on_debian_based_distros("https://download.java.net/java/GA/jdk18.0.2/f6ad4b4450fd4d298113270ec84f30ee/9/GPL/openjdk-18.0.2_linux-x64_bin.tar.gz", "jdk-18.tar.gz", "18"),
                                "19" => java::install_java_on_debian_based_distros("https://download.java.net/java/GA/jdk19.0.1/afdd2e245b014143b62ccb916125e3ce/10/GPL/openjdk-19.0.1_linux-x64_bin.tar.gz", "jdk-19.tar.gz", "19"),
                                "20" => java::install_java_on_debian_based_distros("https://download.java.net/java/GA/jdk20.0.2/6e380f22cbe7469fa75fb448bd903d8e/9/GPL/openjdk-20.0.2_linux-x64_bin.tar.gz", "jdk-20.tar.gz", "20"),
                                "21" => java::install_java_on_debian_based_distros("https://download.java.net/java/GA/jdk21.0.2/f2283984656d49d69e91c558476027ac/13/GPL/openjdk-21.0.2_linux-x64_bin.tar.gz", "jdk-21.tar.gz", "21"),
                                "" => java::install_java_on_debian_based_distros("https://download.java.net/java/GA/jdk21.0.2/f2283984656d49d69e91c558476027ac/13/GPL/openjdk-21.0.2_linux-x64_bin.tar.gz", "jdk-21.tar.gz", "21"),
                                "lts" => java::install_java_on_debian_based_distros("https://download.java.net/java/GA/jdk21.0.2/f2283984656d49d69e91c558476027ac/13/GPL/openjdk-21.0.2_linux-x64_bin.tar.gz", "jdk-21.tar.gz", "21"),
                                "22" => java::install_java_on_debian_based_distros("https://download.java.net/java/GA/jdk22.0.1/c7ec1332f7bb44aeba2eb341ae18aca4/8/GPL/openjdk-22.0.1_linux-x64_bin.tar.gz", "jdk-22.tar.gz", "22"),
                                "23" => java::install_java_on_debian_based_distros("https://download.java.net/java/GA/jdk23/3c5b90190c68498b986a97f276efd28a/37/GPL/openjdk-23_linux-x64_bin.tar.gz", "jdk-23.tar.gz", "23"),
                                "24" => java::install_java_on_debian_based_distros("https://download.java.net/java/early_access/jdk24/13/GPL/openjdk-24-ea+13_linux-x64_bin.tar.gz", "java-24-ea.tar.gz", "24"),
                                "latest" => java::install_java_on_debian_based_distros("https://download.java.net/java/GA/jdk21.0.2/f2283984656d49d69e91c558476027ac/13/GPL/openjdk-21.0.2_linux-x64_bin.tar.gz", "jdk-21.tar.gz", "21"),
                                &_ => ()
                            }
                        },
                        "pardus" => {
                            match self.third_arg.as_str() {
                                "8" => java::install_java_on_debian_based_distros("", "", "8"),
                                "9" => java::install_java_on_debian_based_distros("https://download.java.net/java/GA/jdk9/9.0.4/binaries/openjdk-9.0.4_linux-x64_bin.tar.gz", "jdk-9.tar.gz", "9"),
                                "10" => java::install_java_on_debian_based_distros("https://download.java.net/java/GA/jdk10/10.0.2/19aef61b38124481863b1413dce1855f/13/openjdk-10.0.2_linux-x64_bin.tar.gz", "jdk-10.tar.gz", "10"),
                                "11" => java::install_java_on_debian_based_distros("https://download.java.net/java/GA/jdk11/9/GPL/openjdk-11.0.2_linux-x64_bin.tar.gz", "jdk-11.tar.gz", "11"),
                                "12" => java::install_java_on_debian_based_distros("https://download.java.net/java/GA/jdk12.0.2/e482c34c86bd4bf8b56c0b35558996b9/10/GPL/openjdk-12.0.2_linux-x64_bin.tar.gz", "jdk-12.tar.gz", "12"),
                                "13" => java::install_java_on_debian_based_distros("https://download.java.net/java/GA/jdk13.0.2/d4173c853231432d94f001e99d882ca7/8/GPL/openjdk-13.0.2_linux-x64_bin.tar.gz", "jdk-13.tar.gz", "13"),
                                "14" => java::install_java_on_debian_based_distros("https://download.java.net/java/GA/jdk14.0.2/205943a0976c4ed48cb16f1043c5c647/12/GPL/openjdk-14.0.2_linux-x64_bin.tar.gz", "jdk-14.tar.gz", "14"),
                                "15" => java::install_java_on_debian_based_distros("https://download.java.net/java/GA/jdk15.0.2/0d1cfde4252546c6931946de8db48ee2/7/GPL/openjdk-15.0.2_linux-x64_bin.tar.gz", "jdk-15.tar.gz", "15"),
                                "16" => java::install_java_on_debian_based_distros("https://download.java.net/java/GA/jdk16.0.2/d4a915d82b4c4fbb9bde534da945d746/7/GPL/openjdk-16.0.2_linux-x64_bin.tar.gz", "jdk-16.tar.gz", "16"),
                                "17" => java::install_java_on_debian_based_distros("https://download.java.net/java/GA/jdk17.0.2/dfd4a8d0985749f896bed50d7138ee7f/8/GPL/openjdk-17.0.2_linux-x64_bin.tar.gz", "jdk-17.tar.gz", "17"),
                                "18" => java::install_java_on_debian_based_distros("https://download.java.net/java/GA/jdk18.0.2/f6ad4b4450fd4d298113270ec84f30ee/9/GPL/openjdk-18.0.2_linux-x64_bin.tar.gz", "jdk-18.tar.gz", "18"),
                                "19" => java::install_java_on_debian_based_distros("https://download.java.net/java/GA/jdk19.0.1/afdd2e245b014143b62ccb916125e3ce/10/GPL/openjdk-19.0.1_linux-x64_bin.tar.gz", "jdk-19.tar.gz", "19"),
                                "20" => java::install_java_on_debian_based_distros("https://download.java.net/java/GA/jdk20.0.2/6e380f22cbe7469fa75fb448bd903d8e/9/GPL/openjdk-20.0.2_linux-x64_bin.tar.gz", "jdk-20.tar.gz", "20"),
                                "21" => java::install_java_on_debian_based_distros("https://download.java.net/java/GA/jdk21.0.2/f2283984656d49d69e91c558476027ac/13/GPL/openjdk-21.0.2_linux-x64_bin.tar.gz", "jdk-21.tar.gz", "21"),
                                "" => java::install_java_on_debian_based_distros("https://download.java.net/java/GA/jdk21.0.2/f2283984656d49d69e91c558476027ac/13/GPL/openjdk-21.0.2_linux-x64_bin.tar.gz", "jdk-21.tar.gz", "21"),
                                "lts" => java::install_java_on_debian_based_distros("https://download.java.net/java/GA/jdk21.0.2/f2283984656d49d69e91c558476027ac/13/GPL/openjdk-21.0.2_linux-x64_bin.tar.gz", "jdk-21.tar.gz", "21"),
                                "22" => java::install_java_on_debian_based_distros("https://download.java.net/java/GA/jdk22.0.1/c7ec1332f7bb44aeba2eb341ae18aca4/8/GPL/openjdk-22.0.1_linux-x64_bin.tar.gz", "jdk-22.tar.gz", "22"),
                                "23" => java::install_java_on_debian_based_distros("https://download.java.net/java/GA/jdk23/3c5b90190c68498b986a97f276efd28a/37/GPL/openjdk-23_linux-x64_bin.tar.gz", "jdk-23.tar.gz", "23"),
                                "24" => java::install_java_on_debian_based_distros("https://download.java.net/java/early_access/jdk24/13/GPL/openjdk-24-ea+13_linux-x64_bin.tar.gz", "java-24-ea.tar.gz", "24"),
                                "latest" => java::install_java_on_debian_based_distros("https://download.java.net/java/GA/jdk21.0.2/f2283984656d49d69e91c558476027ac/13/GPL/openjdk-21.0.2_linux-x64_bin.tar.gz", "jdk-21.tar.gz", "21"),
                                &_ => ()
                            }
                        },
                        "kali linux" => {
                            match self.third_arg.as_str() {
                                "8" => java::install_java_on_debian_based_distros("", "", "8"),
                                "9" => java::install_java_on_debian_based_distros("https://download.java.net/java/GA/jdk9/9.0.4/binaries/openjdk-9.0.4_linux-x64_bin.tar.gz", "jdk-9.tar.gz", "9"),
                                "10" => java::install_java_on_debian_based_distros("https://download.java.net/java/GA/jdk10/10.0.2/19aef61b38124481863b1413dce1855f/13/openjdk-10.0.2_linux-x64_bin.tar.gz", "jdk-10.tar.gz", "10"),
                                "11" => java::install_java_on_debian_based_distros("https://download.java.net/java/GA/jdk11/9/GPL/openjdk-11.0.2_linux-x64_bin.tar.gz", "jdk-11.tar.gz", "11"),
                                "12" => java::install_java_on_debian_based_distros("https://download.java.net/java/GA/jdk12.0.2/e482c34c86bd4bf8b56c0b35558996b9/10/GPL/openjdk-12.0.2_linux-x64_bin.tar.gz", "jdk-12.tar.gz", "12"),
                                "13" => java::install_java_on_debian_based_distros("https://download.java.net/java/GA/jdk13.0.2/d4173c853231432d94f001e99d882ca7/8/GPL/openjdk-13.0.2_linux-x64_bin.tar.gz", "jdk-13.tar.gz", "13"),
                                "14" => java::install_java_on_debian_based_distros("https://download.java.net/java/GA/jdk14.0.2/205943a0976c4ed48cb16f1043c5c647/12/GPL/openjdk-14.0.2_linux-x64_bin.tar.gz", "jdk-14.tar.gz", "14"),
                                "15" => java::install_java_on_debian_based_distros("https://download.java.net/java/GA/jdk15.0.2/0d1cfde4252546c6931946de8db48ee2/7/GPL/openjdk-15.0.2_linux-x64_bin.tar.gz", "jdk-15.tar.gz", "15"),
                                "16" => java::install_java_on_debian_based_distros("https://download.java.net/java/GA/jdk16.0.2/d4a915d82b4c4fbb9bde534da945d746/7/GPL/openjdk-16.0.2_linux-x64_bin.tar.gz", "jdk-16.tar.gz", "16"),
                                "17" => java::install_java_on_debian_based_distros("https://download.java.net/java/GA/jdk17.0.2/dfd4a8d0985749f896bed50d7138ee7f/8/GPL/openjdk-17.0.2_linux-x64_bin.tar.gz", "jdk-17.tar.gz", "17"),
                                "18" => java::install_java_on_debian_based_distros("https://download.java.net/java/GA/jdk18.0.2/f6ad4b4450fd4d298113270ec84f30ee/9/GPL/openjdk-18.0.2_linux-x64_bin.tar.gz", "jdk-18.tar.gz", "18"),
                                "19" => java::install_java_on_debian_based_distros("https://download.java.net/java/GA/jdk19.0.1/afdd2e245b014143b62ccb916125e3ce/10/GPL/openjdk-19.0.1_linux-x64_bin.tar.gz", "jdk-19.tar.gz", "19"),
                                "20" => java::install_java_on_debian_based_distros("https://download.java.net/java/GA/jdk20.0.2/6e380f22cbe7469fa75fb448bd903d8e/9/GPL/openjdk-20.0.2_linux-x64_bin.tar.gz", "jdk-20.tar.gz", "20"),
                                "21" => java::install_java_on_debian_based_distros("https://download.java.net/java/GA/jdk21.0.2/f2283984656d49d69e91c558476027ac/13/GPL/openjdk-21.0.2_linux-x64_bin.tar.gz", "jdk-21.tar.gz", "21"),
                                "" => java::install_java_on_debian_based_distros("https://download.java.net/java/GA/jdk21.0.2/f2283984656d49d69e91c558476027ac/13/GPL/openjdk-21.0.2_linux-x64_bin.tar.gz", "jdk-21.tar.gz", "21"),
                                "lts" => java::install_java_on_debian_based_distros("https://download.java.net/java/GA/jdk21.0.2/f2283984656d49d69e91c558476027ac/13/GPL/openjdk-21.0.2_linux-x64_bin.tar.gz", "jdk-21.tar.gz", "21"),
                                "22" => java::install_java_on_debian_based_distros("https://download.java.net/java/GA/jdk22.0.1/c7ec1332f7bb44aeba2eb341ae18aca4/8/GPL/openjdk-22.0.1_linux-x64_bin.tar.gz", "jdk-22.tar.gz", "22"),
                                "23" => java::install_java_on_debian_based_distros("https://download.java.net/java/GA/jdk23/3c5b90190c68498b986a97f276efd28a/37/GPL/openjdk-23_linux-x64_bin.tar.gz", "jdk-23.tar.gz", "23"),
                                "24" => java::install_java_on_debian_based_distros("https://download.java.net/java/early_access/jdk24/13/GPL/openjdk-24-ea+13_linux-x64_bin.tar.gz", "java-24-ea.tar.gz", "24"),
                                "latest" => java::install_java_on_debian_based_distros("https://download.java.net/java/GA/jdk21.0.2/f2283984656d49d69e91c558476027ac/13/GPL/openjdk-21.0.2_linux-x64_bin.tar.gz", "jdk-21.tar.gz", "21"),
                                &_ => ()
                            }
                        },
                        "arch wsl" => {
                            match self.third_arg.as_str() {
                                "8" => println!("Unfortunately, there is no installation option for Java 8 on Arch Wsl."),
                                "9" => java::install_java_on_arch_linux("https://download.java.net/java/GA/jdk9/9.0.4/binaries/openjdk-9.0.4_linux-x64_bin.tar.gz", "jdk-9.tar.gz", "9"),
                                "10" => java::install_java_on_arch_linux("https://download.java.net/java/GA/jdk10/10.0.2/19aef61b38124481863b1413dce1855f/13/openjdk-10.0.2_linux-x64_bin.tar.gz", "jdk-10.tar.gz", "10"),
                                "11" => java::install_java_on_arch_linux("https://download.java.net/java/GA/jdk11/9/GPL/openjdk-11.0.2_linux-x64_bin.tar.gz", "jdk-11.tar.gz", "11"),
                                "12" => java::install_java_on_arch_linux("https://download.java.net/java/GA/jdk12.0.2/e482c34c86bd4bf8b56c0b35558996b9/10/GPL/openjdk-12.0.2_linux-x64_bin.tar.gz", "jdk-12.tar.gz", "12"),
                                "13" => java::install_java_on_arch_linux("https://download.java.net/java/GA/jdk13.0.2/d4173c853231432d94f001e99d882ca7/8/GPL/openjdk-13.0.2_linux-x64_bin.tar.gz", "jdk-13.tar.gz", "13"),
                                "14" => java::install_java_on_arch_linux("https://download.java.net/java/GA/jdk14.0.2/205943a0976c4ed48cb16f1043c5c647/12/GPL/openjdk-14.0.2_linux-x64_bin.tar.gz", "jdk-14.tar.gz", "14"),
                                "15" => java::install_java_on_arch_linux("https://download.java.net/java/GA/jdk15.0.2/0d1cfde4252546c6931946de8db48ee2/7/GPL/openjdk-15.0.2_linux-x64_bin.tar.gz", "jdk-15.tar.gz", "15"),
                                "16" => java::install_java_on_arch_linux("https://download.java.net/java/GA/jdk16.0.2/d4a915d82b4c4fbb9bde534da945d746/7/GPL/openjdk-16.0.2_linux-x64_bin.tar.gz", "jdk-16.tar.gz", "16"),
                                "17" => java::install_java_on_arch_linux("https://download.java.net/java/GA/jdk17.0.2/dfd4a8d0985749f896bed50d7138ee7f/8/GPL/openjdk-17.0.2_linux-x64_bin.tar.gz", "jdk-17.tar.gz", "17"),
                                "18" => java::install_java_on_arch_linux("https://download.java.net/java/GA/jdk18.0.2/f6ad4b4450fd4d298113270ec84f30ee/9/GPL/openjdk-18.0.2_linux-x64_bin.tar.gz", "jdk-18.tar.gz", "18"),
                                "19" => java::install_java_on_arch_linux("https://download.java.net/java/GA/jdk19.0.1/afdd2e245b014143b62ccb916125e3ce/10/GPL/openjdk-19.0.1_linux-x64_bin.tar.gz", "jdk-19.tar.gz", "19"),
                                "20" => java::install_java_on_arch_linux("https://download.java.net/java/GA/jdk20.0.2/6e380f22cbe7469fa75fb448bd903d8e/9/GPL/openjdk-20.0.2_linux-x64_bin.tar.gz", "jdk-20.tar.gz", "20"),
                                "21" => java::install_java_on_arch_linux("https://download.java.net/java/GA/jdk21.0.2/f2283984656d49d69e91c558476027ac/13/GPL/openjdk-21.0.2_linux-x64_bin.tar.gz", "jdk-21.tar.gz", "21"),
                                "" => java::install_java_on_arch_linux("https://download.java.net/java/GA/jdk21.0.2/f2283984656d49d69e91c558476027ac/13/GPL/openjdk-21.0.2_linux-x64_bin.tar.gz", "jdk-21.tar.gz", "21"),
                                "lts" => java::install_java_on_arch_linux("https://download.java.net/java/GA/jdk21.0.2/f2283984656d49d69e91c558476027ac/13/GPL/openjdk-21.0.2_linux-x64_bin.tar.gz", "jdk-21.tar.gz", "21"),
                                "22" => java::install_java_on_arch_linux("https://download.java.net/java/GA/jdk22.0.1/c7ec1332f7bb44aeba2eb341ae18aca4/8/GPL/openjdk-22.0.1_linux-x64_bin.tar.gz", "jdk-22.tar.gz", "22"),
                                "23" => java::install_java_on_arch_linux("https://download.java.net/java/GA/jdk23/3c5b90190c68498b986a97f276efd28a/37/GPL/openjdk-23_linux-x64_bin.tar.gz", "jdk-23.tar.gz", "23"),
                                "24" => java::install_java_on_arch_linux("https://download.java.net/java/early_access/jdk24/13/GPL/openjdk-24-ea+13_linux-x64_bin.tar.gz", "java-24-ea.tar.gz", "24"),
                                "latest" => java::install_java_on_arch_linux("https://download.java.net/java/early_access/jdk24/13/GPL/openjdk-24-ea+13_linux-x64_bin.tar.gz", "java-24-ea.tar.gz", "24"),
                                &_ => ()
                            }
                        },
                        "centos" => {
                            match self.third_arg.as_str() {
                                "8" => java::install_java_on_centos_and_fedora("", "", "8"),
                                "9" => java::install_java_on_centos_and_fedora("https://download.java.net/java/GA/jdk9/9.0.4/binaries/openjdk-9.0.4_linux-x64_bin.tar.gz", "jdk-9.tar.gz", "9"),
                                "10" => java::install_java_on_centos_and_fedora("https://download.java.net/java/GA/jdk10/10.0.2/19aef61b38124481863b1413dce1855f/13/openjdk-10.0.2_linux-x64_bin.tar.gz", "jdk-10.tar.gz", "10"),
                                "11" => java::install_java_on_centos_and_fedora("https://download.java.net/java/GA/jdk11/9/GPL/openjdk-11.0.2_linux-x64_bin.tar.gz", "jdk-11.tar.gz", "11"),
                                "12" => java::install_java_on_centos_and_fedora("https://download.java.net/java/GA/jdk12.0.2/e482c34c86bd4bf8b56c0b35558996b9/10/GPL/openjdk-12.0.2_linux-x64_bin.tar.gz", "jdk-12.tar.gz", "12"),
                                "13" => java::install_java_on_centos_and_fedora("https://download.java.net/java/GA/jdk13.0.2/d4173c853231432d94f001e99d882ca7/8/GPL/openjdk-13.0.2_linux-x64_bin.tar.gz", "jdk-13.tar.gz", "13"),
                                "14" => java::install_java_on_centos_and_fedora("https://download.java.net/java/GA/jdk14.0.2/205943a0976c4ed48cb16f1043c5c647/12/GPL/openjdk-14.0.2_linux-x64_bin.tar.gz", "jdk-14.tar.gz", "14"),
                                "15" => java::install_java_on_centos_and_fedora("https://download.java.net/java/GA/jdk15.0.2/0d1cfde4252546c6931946de8db48ee2/7/GPL/openjdk-15.0.2_linux-x64_bin.tar.gz", "jdk-15.tar.gz", "15"),
                                "16" => java::install_java_on_centos_and_fedora("https://download.java.net/java/GA/jdk16.0.2/d4a915d82b4c4fbb9bde534da945d746/7/GPL/openjdk-16.0.2_linux-x64_bin.tar.gz", "jdk-16.tar.gz", "16"),
                                "17" => java::install_java_on_centos_and_fedora("https://download.java.net/java/GA/jdk17.0.2/dfd4a8d0985749f896bed50d7138ee7f/8/GPL/openjdk-17.0.2_linux-x64_bin.tar.gz", "jdk-17.tar.gz", "17"),
                                "18" => java::install_java_on_centos_and_fedora("https://download.java.net/java/GA/jdk18.0.2/f6ad4b4450fd4d298113270ec84f30ee/9/GPL/openjdk-18.0.2_linux-x64_bin.tar.gz", "jdk-18.tar.gz", "18"),
                                "19" => java::install_java_on_centos_and_fedora("https://download.java.net/java/GA/jdk19.0.1/afdd2e245b014143b62ccb916125e3ce/10/GPL/openjdk-19.0.1_linux-x64_bin.tar.gz", "jdk-19.tar.gz", "19"),
                                "20" => java::install_java_on_centos_and_fedora("https://download.java.net/java/GA/jdk20.0.2/6e380f22cbe7469fa75fb448bd903d8e/9/GPL/openjdk-20.0.2_linux-x64_bin.tar.gz", "jdk-20.tar.gz", "20"),
                                "21" => java::install_java_on_centos_and_fedora("https://download.java.net/java/GA/jdk21.0.2/f2283984656d49d69e91c558476027ac/13/GPL/openjdk-21.0.2_linux-x64_bin.tar.gz", "jdk-21.tar.gz", "21"),
                                "" => java::install_java_on_centos_and_fedora("https://download.java.net/java/GA/jdk21.0.2/f2283984656d49d69e91c558476027ac/13/GPL/openjdk-21.0.2_linux-x64_bin.tar.gz", "jdk-21.tar.gz", "21"),
                                "lts" => java::install_java_on_centos_and_fedora("https://download.java.net/java/GA/jdk21.0.2/f2283984656d49d69e91c558476027ac/13/GPL/openjdk-21.0.2_linux-x64_bin.tar.gz", "jdk-21.tar.gz", "21"),
                                "22" => java::install_java_on_centos_and_fedora("https://download.java.net/java/GA/jdk22.0.1/c7ec1332f7bb44aeba2eb341ae18aca4/8/GPL/openjdk-22.0.1_linux-x64_bin.tar.gz", "jdk-22.tar.gz", "22"),
                                "23" => java::install_java_on_centos_and_fedora("https://download.java.net/java/GA/jdk23/3c5b90190c68498b986a97f276efd28a/37/GPL/openjdk-23_linux-x64_bin.tar.gz", "jdk-23.tar.gz", "23"),
                                "24" => java::install_java_on_centos_and_fedora("https://download.java.net/java/early_access/jdk24/13/GPL/openjdk-24-ea+13_linux-x64_bin.tar.gz", "java-24-ea.tar.gz", "24"),
                                "latest" => java::install_java_on_centos_and_fedora("https://download.java.net/java/early_access/jdk24/13/GPL/openjdk-24-ea+13_linux-x64_bin.tar.gz", "java-24-ea.tar.gz", "24"),
                                &_ => ()
                            }
                        },
                        "fedora" => {
                            match self.third_arg.as_str() {
                                "8" => java::install_java_on_centos_and_fedora("", "", "8"),
                                "9" => java::install_java_on_centos_and_fedora("https://download.java.net/java/GA/jdk9/9.0.4/binaries/openjdk-9.0.4_linux-x64_bin.tar.gz", "jdk-9.tar.gz", "9"),
                                "10" => java::install_java_on_centos_and_fedora("https://download.java.net/java/GA/jdk10/10.0.2/19aef61b38124481863b1413dce1855f/13/openjdk-10.0.2_linux-x64_bin.tar.gz", "jdk-10.tar.gz", "10"),
                                "11" => java::install_java_on_centos_and_fedora("https://download.java.net/java/GA/jdk11/9/GPL/openjdk-11.0.2_linux-x64_bin.tar.gz", "jdk-11.tar.gz", "11"),
                                "12" => java::install_java_on_centos_and_fedora("https://download.java.net/java/GA/jdk12.0.2/e482c34c86bd4bf8b56c0b35558996b9/10/GPL/openjdk-12.0.2_linux-x64_bin.tar.gz", "jdk-12.tar.gz", "12"),
                                "13" => java::install_java_on_centos_and_fedora("https://download.java.net/java/GA/jdk13.0.2/d4173c853231432d94f001e99d882ca7/8/GPL/openjdk-13.0.2_linux-x64_bin.tar.gz", "jdk-13.tar.gz", "13"),
                                "14" => java::install_java_on_centos_and_fedora("https://download.java.net/java/GA/jdk14.0.2/205943a0976c4ed48cb16f1043c5c647/12/GPL/openjdk-14.0.2_linux-x64_bin.tar.gz", "jdk-14.tar.gz", "14"),
                                "15" => java::install_java_on_centos_and_fedora("https://download.java.net/java/GA/jdk15.0.2/0d1cfde4252546c6931946de8db48ee2/7/GPL/openjdk-15.0.2_linux-x64_bin.tar.gz", "jdk-15.tar.gz", "15"),
                                "16" => java::install_java_on_centos_and_fedora("https://download.java.net/java/GA/jdk16.0.2/d4a915d82b4c4fbb9bde534da945d746/7/GPL/openjdk-16.0.2_linux-x64_bin.tar.gz", "jdk-16.tar.gz", "16"),
                                "17" => java::install_java_on_centos_and_fedora("https://download.java.net/java/GA/jdk17.0.2/dfd4a8d0985749f896bed50d7138ee7f/8/GPL/openjdk-17.0.2_linux-x64_bin.tar.gz", "jdk-17.tar.gz", "17"),
                                "18" => java::install_java_on_centos_and_fedora("https://download.java.net/java/GA/jdk18.0.2/f6ad4b4450fd4d298113270ec84f30ee/9/GPL/openjdk-18.0.2_linux-x64_bin.tar.gz", "jdk-18.tar.gz", "18"),
                                "19" => java::install_java_on_centos_and_fedora("https://download.java.net/java/GA/jdk19.0.1/afdd2e245b014143b62ccb916125e3ce/10/GPL/openjdk-19.0.1_linux-x64_bin.tar.gz", "jdk-19.tar.gz", "19"),
                                "20" => java::install_java_on_centos_and_fedora("https://download.java.net/java/GA/jdk20.0.2/6e380f22cbe7469fa75fb448bd903d8e/9/GPL/openjdk-20.0.2_linux-x64_bin.tar.gz", "jdk-20.tar.gz", "20"),
                                "21" => java::install_java_on_centos_and_fedora("https://download.java.net/java/GA/jdk21.0.2/f2283984656d49d69e91c558476027ac/13/GPL/openjdk-21.0.2_linux-x64_bin.tar.gz", "jdk-21.tar.gz", "21"),
                                "" => java::install_java_on_centos_and_fedora("https://download.java.net/java/GA/jdk21.0.2/f2283984656d49d69e91c558476027ac/13/GPL/openjdk-21.0.2_linux-x64_bin.tar.gz", "jdk-21.tar.gz", "21"),
                                "lts" => java::install_java_on_centos_and_fedora("https://download.java.net/java/GA/jdk21.0.2/f2283984656d49d69e91c558476027ac/13/GPL/openjdk-21.0.2_linux-x64_bin.tar.gz", "jdk-21.tar.gz", "21"),
                                "22" => java::install_java_on_centos_and_fedora("https://download.java.net/java/GA/jdk22.0.1/c7ec1332f7bb44aeba2eb341ae18aca4/8/GPL/openjdk-22.0.1_linux-x64_bin.tar.gz", "jdk-22.tar.gz", "22"),
                                "23" => java::install_java_on_centos_and_fedora("https://download.java.net/java/GA/jdk23/3c5b90190c68498b986a97f276efd28a/37/GPL/openjdk-23_linux-x64_bin.tar.gz", "jdk-23.tar.gz", "23"),
                                "24" => java::install_java_on_centos_and_fedora("https://download.java.net/java/early_access/jdk24/13/GPL/openjdk-24-ea+13_linux-x64_bin.tar.gz", "java-24-ea.tar.gz", "24"),
                                "latest" => java::install_java_on_centos_and_fedora("https://download.java.net/java/early_access/jdk24/13/GPL/openjdk-24-ea+13_linux-x64_bin.tar.gz", "java-24-ea.tar.gz", "24"),
                                &_ => ()
                            }
                        },
                        "rocky" => {
                            match self.third_arg.as_str() {
                                "8" => java::install_java_on_rocky_linux("", "", "8"),
                                "9" => java::install_java_on_rocky_linux("https://download.java.net/java/GA/jdk9/9.0.4/binaries/openjdk-9.0.4_linux-x64_bin.tar.gz", "jdk-9.tar.gz", "9"),
                                "10" => java::install_java_on_rocky_linux("https://download.java.net/java/GA/jdk10/10.0.2/19aef61b38124481863b1413dce1855f/13/openjdk-10.0.2_linux-x64_bin.tar.gz", "jdk-10.tar.gz", "10"),
                                "11" => java::install_java_on_rocky_linux("https://download.java.net/java/GA/jdk11/9/GPL/openjdk-11.0.2_linux-x64_bin.tar.gz", "jdk-11.tar.gz", "11"),
                                "12" => java::install_java_on_rocky_linux("https://download.java.net/java/GA/jdk12.0.2/e482c34c86bd4bf8b56c0b35558996b9/10/GPL/openjdk-12.0.2_linux-x64_bin.tar.gz", "jdk-12.tar.gz", "12"),
                                "13" => java::install_java_on_rocky_linux("https://download.java.net/java/GA/jdk13.0.2/d4173c853231432d94f001e99d882ca7/8/GPL/openjdk-13.0.2_linux-x64_bin.tar.gz", "jdk-13.tar.gz", "13"),
                                "14" => java::install_java_on_rocky_linux("https://download.java.net/java/GA/jdk14.0.2/205943a0976c4ed48cb16f1043c5c647/12/GPL/openjdk-14.0.2_linux-x64_bin.tar.gz", "jdk-14.tar.gz", "14"),
                                "15" => java::install_java_on_rocky_linux("https://download.java.net/java/GA/jdk15.0.2/0d1cfde4252546c6931946de8db48ee2/7/GPL/openjdk-15.0.2_linux-x64_bin.tar.gz", "jdk-15.tar.gz", "15"),
                                "16" => java::install_java_on_rocky_linux("https://download.java.net/java/GA/jdk16.0.2/d4a915d82b4c4fbb9bde534da945d746/7/GPL/openjdk-16.0.2_linux-x64_bin.tar.gz", "jdk-16.tar.gz", "16"),
                                "17" => java::install_java_on_rocky_linux("https://download.java.net/java/GA/jdk17.0.2/dfd4a8d0985749f896bed50d7138ee7f/8/GPL/openjdk-17.0.2_linux-x64_bin.tar.gz", "jdk-17.tar.gz", "17"),
                                "18" => java::install_java_on_rocky_linux("https://download.java.net/java/GA/jdk18.0.2/f6ad4b4450fd4d298113270ec84f30ee/9/GPL/openjdk-18.0.2_linux-x64_bin.tar.gz", "jdk-18.tar.gz", "18"),
                                "19" => java::install_java_on_rocky_linux("https://download.java.net/java/GA/jdk19.0.1/afdd2e245b014143b62ccb916125e3ce/10/GPL/openjdk-19.0.1_linux-x64_bin.tar.gz", "jdk-19.tar.gz", "19"),
                                "20" => java::install_java_on_rocky_linux("https://download.java.net/java/GA/jdk20.0.2/6e380f22cbe7469fa75fb448bd903d8e/9/GPL/openjdk-20.0.2_linux-x64_bin.tar.gz", "jdk-20.tar.gz", "20"),
                                "21" => java::install_java_on_rocky_linux("https://download.java.net/java/GA/jdk21.0.2/f2283984656d49d69e91c558476027ac/13/GPL/openjdk-21.0.2_linux-x64_bin.tar.gz", "jdk-21.tar.gz", "21"),
                                "" => java::install_java_on_rocky_linux("https://download.java.net/java/GA/jdk21.0.2/f2283984656d49d69e91c558476027ac/13/GPL/openjdk-21.0.2_linux-x64_bin.tar.gz", "jdk-21.tar.gz", "21"),
                                "lts" => java::install_java_on_rocky_linux("https://download.java.net/java/GA/jdk21.0.2/f2283984656d49d69e91c558476027ac/13/GPL/openjdk-21.0.2_linux-x64_bin.tar.gz", "jdk-21.tar.gz", "21"),
                                "22" => java::install_java_on_rocky_linux("https://download.java.net/java/GA/jdk22.0.1/c7ec1332f7bb44aeba2eb341ae18aca4/8/GPL/openjdk-22.0.1_linux-x64_bin.tar.gz", "jdk-22.tar.gz", "22"),
                                "23" => java::install_java_on_rocky_linux("https://download.java.net/java/GA/jdk23/3c5b90190c68498b986a97f276efd28a/37/GPL/openjdk-23_linux-x64_bin.tar.gz", "jdk-23.tar.gz", "23"),
                                "24" => java::install_java_on_rocky_linux("https://download.java.net/java/early_access/jdk24/13/GPL/openjdk-24-ea+13_linux-x64_bin.tar.gz", "java-24-ea.tar.gz", "24"),
                                "latest" => java::install_java_on_rocky_linux("https://download.java.net/java/early_access/jdk24/13/GPL/openjdk-24-ea+13_linux-x64_bin.tar.gz", "java-24-ea.tar.gz", "24"),
                                &_ => ()
                            }
                        },
                        "alma linux" => {
                            match self.third_arg.as_str() {
                                "8" => java::install_java_on_alma_linux("", "", "8"),
                                "9" => java::install_java_on_alma_linux("https://download.java.net/java/GA/jdk9/9.0.4/binaries/openjdk-9.0.4_linux-x64_bin.tar.gz", "jdk-9.tar.gz", "9"),
                                "10" => java::install_java_on_alma_linux("https://download.java.net/java/GA/jdk10/10.0.2/19aef61b38124481863b1413dce1855f/13/openjdk-10.0.2_linux-x64_bin.tar.gz", "jdk-10.tar.gz", "10"),
                                "11" => java::install_java_on_alma_linux("https://download.java.net/java/GA/jdk11/9/GPL/openjdk-11.0.2_linux-x64_bin.tar.gz", "jdk-11.tar.gz", "11"),
                                "12" => java::install_java_on_alma_linux("https://download.java.net/java/GA/jdk12.0.2/e482c34c86bd4bf8b56c0b35558996b9/10/GPL/openjdk-12.0.2_linux-x64_bin.tar.gz", "jdk-12.tar.gz", "12"),
                                "13" => java::install_java_on_alma_linux("https://download.java.net/java/GA/jdk13.0.2/d4173c853231432d94f001e99d882ca7/8/GPL/openjdk-13.0.2_linux-x64_bin.tar.gz", "jdk-13.tar.gz", "13"),
                                "14" => java::install_java_on_alma_linux("https://download.java.net/java/GA/jdk14.0.2/205943a0976c4ed48cb16f1043c5c647/12/GPL/openjdk-14.0.2_linux-x64_bin.tar.gz", "jdk-14.tar.gz", "14"),
                                "15" => java::install_java_on_alma_linux("https://download.java.net/java/GA/jdk15.0.2/0d1cfde4252546c6931946de8db48ee2/7/GPL/openjdk-15.0.2_linux-x64_bin.tar.gz", "jdk-15.tar.gz", "15"),
                                "16" => java::install_java_on_alma_linux("https://download.java.net/java/GA/jdk16.0.2/d4a915d82b4c4fbb9bde534da945d746/7/GPL/openjdk-16.0.2_linux-x64_bin.tar.gz", "jdk-16.tar.gz", "16"),
                                "17" => java::install_java_on_alma_linux("https://download.java.net/java/GA/jdk17.0.2/dfd4a8d0985749f896bed50d7138ee7f/8/GPL/openjdk-17.0.2_linux-x64_bin.tar.gz", "jdk-17.tar.gz", "17"),
                                "18" => java::install_java_on_alma_linux("https://download.java.net/java/GA/jdk18.0.2/f6ad4b4450fd4d298113270ec84f30ee/9/GPL/openjdk-18.0.2_linux-x64_bin.tar.gz", "jdk-18.tar.gz", "18"),
                                "19" => java::install_java_on_alma_linux("https://download.java.net/java/GA/jdk19.0.1/afdd2e245b014143b62ccb916125e3ce/10/GPL/openjdk-19.0.1_linux-x64_bin.tar.gz", "jdk-19.tar.gz", "19"),
                                "20" => java::install_java_on_alma_linux("https://download.java.net/java/GA/jdk20.0.2/6e380f22cbe7469fa75fb448bd903d8e/9/GPL/openjdk-20.0.2_linux-x64_bin.tar.gz", "jdk-20.tar.gz", "20"),
                                "21" => java::install_java_on_alma_linux("https://download.java.net/java/GA/jdk21.0.2/f2283984656d49d69e91c558476027ac/13/GPL/openjdk-21.0.2_linux-x64_bin.tar.gz", "jdk-21.tar.gz", "21"),
                                "" => java::install_java_on_alma_linux("https://download.java.net/java/GA/jdk21.0.2/f2283984656d49d69e91c558476027ac/13/GPL/openjdk-21.0.2_linux-x64_bin.tar.gz", "jdk-21.tar.gz", "21"),
                                "lts" => java::install_java_on_alma_linux("https://download.java.net/java/GA/jdk21.0.2/f2283984656d49d69e91c558476027ac/13/GPL/openjdk-21.0.2_linux-x64_bin.tar.gz", "jdk-21.tar.gz", "21"),
                                "22" => java::install_java_on_alma_linux("https://download.java.net/java/GA/jdk22.0.1/c7ec1332f7bb44aeba2eb341ae18aca4/8/GPL/openjdk-22.0.1_linux-x64_bin.tar.gz", "jdk-22.tar.gz", "22"),
                                "23" => java::install_java_on_alma_linux("https://download.java.net/java/GA/jdk23/3c5b90190c68498b986a97f276efd28a/37/GPL/openjdk-23_linux-x64_bin.tar.gz", "jdk-23.tar.gz", "23"),
                                "24" => java::install_java_on_alma_linux("https://download.java.net/java/early_access/jdk24/13/GPL/openjdk-24-ea+13_linux-x64_bin.tar.gz", "java-24-ea.tar.gz", "24"),
                                "latest" => java::install_java_on_alma_linux("https://download.java.net/java/early_access/jdk24/13/GPL/openjdk-24-ea+13_linux-x64_bin.tar.gz", "java-24-ea.tar.gz", "24"),
                                &_ => ()
                            }
                        },
                        &_ => null_var = "none".to_string()
                    }

                    if null_var == "none".to_string() {
                        if utils::check_if_linux_dist_is_arch_linux() {
                            match self.third_arg.as_str() {
                                "8" => println!("Unfortunately, there is no installation option for Java 8 on Arch Wsl."),
                                "9" => java::install_java_on_arch_linux("https://download.java.net/java/GA/jdk9/9.0.4/binaries/openjdk-9.0.4_linux-x64_bin.tar.gz", "jdk-9.tar.gz", "9"),
                                "10" => java::install_java_on_arch_linux("https://download.java.net/java/GA/jdk10/10.0.2/19aef61b38124481863b1413dce1855f/13/openjdk-10.0.2_linux-x64_bin.tar.gz", "jdk-10.tar.gz", "10"),
                                "11" => java::install_java_on_arch_linux("https://download.java.net/java/GA/jdk11/9/GPL/openjdk-11.0.2_linux-x64_bin.tar.gz", "jdk-11.tar.gz", "11"),
                                "12" => java::install_java_on_arch_linux("https://download.java.net/java/GA/jdk12.0.2/e482c34c86bd4bf8b56c0b35558996b9/10/GPL/openjdk-12.0.2_linux-x64_bin.tar.gz", "jdk-12.tar.gz", "12"),
                                "13" => java::install_java_on_arch_linux("https://download.java.net/java/GA/jdk13.0.2/d4173c853231432d94f001e99d882ca7/8/GPL/openjdk-13.0.2_linux-x64_bin.tar.gz", "jdk-13.tar.gz", "13"),
                                "14" => java::install_java_on_arch_linux("https://download.java.net/java/GA/jdk14.0.2/205943a0976c4ed48cb16f1043c5c647/12/GPL/openjdk-14.0.2_linux-x64_bin.tar.gz", "jdk-14.tar.gz", "14"),
                                "15" => java::install_java_on_arch_linux("https://download.java.net/java/GA/jdk15.0.2/0d1cfde4252546c6931946de8db48ee2/7/GPL/openjdk-15.0.2_linux-x64_bin.tar.gz", "jdk-15.tar.gz", "15"),
                                "16" => java::install_java_on_arch_linux("https://download.java.net/java/GA/jdk16.0.2/d4a915d82b4c4fbb9bde534da945d746/7/GPL/openjdk-16.0.2_linux-x64_bin.tar.gz", "jdk-16.tar.gz", "16"),
                                "17" => java::install_java_on_arch_linux("https://download.java.net/java/GA/jdk17.0.2/dfd4a8d0985749f896bed50d7138ee7f/8/GPL/openjdk-17.0.2_linux-x64_bin.tar.gz", "jdk-17.tar.gz", "17"),
                                "18" => java::install_java_on_arch_linux("https://download.java.net/java/GA/jdk18.0.2/f6ad4b4450fd4d298113270ec84f30ee/9/GPL/openjdk-18.0.2_linux-x64_bin.tar.gz", "jdk-18.tar.gz", "18"),
                                "19" => java::install_java_on_arch_linux("https://download.java.net/java/GA/jdk19.0.1/afdd2e245b014143b62ccb916125e3ce/10/GPL/openjdk-19.0.1_linux-x64_bin.tar.gz", "jdk-19.tar.gz", "19"),
                                "20" => java::install_java_on_arch_linux("https://download.java.net/java/GA/jdk20.0.2/6e380f22cbe7469fa75fb448bd903d8e/9/GPL/openjdk-20.0.2_linux-x64_bin.tar.gz", "jdk-20.tar.gz", "20"),
                                "21" => java::install_java_on_arch_linux("https://download.java.net/java/GA/jdk21.0.2/f2283984656d49d69e91c558476027ac/13/GPL/openjdk-21.0.2_linux-x64_bin.tar.gz", "jdk-21.tar.gz", "21"),
                                "" => java::install_java_on_arch_linux("https://download.java.net/java/GA/jdk21.0.2/f2283984656d49d69e91c558476027ac/13/GPL/openjdk-21.0.2_linux-x64_bin.tar.gz", "jdk-21.tar.gz", "21"),
                                "lts" => java::install_java_on_arch_linux("https://download.java.net/java/GA/jdk21.0.2/f2283984656d49d69e91c558476027ac/13/GPL/openjdk-21.0.2_linux-x64_bin.tar.gz", "jdk-21.tar.gz", "21"),
                                "22" => java::install_java_on_arch_linux("https://download.java.net/java/GA/jdk22.0.1/c7ec1332f7bb44aeba2eb341ae18aca4/8/GPL/openjdk-22.0.1_linux-x64_bin.tar.gz", "jdk-22.tar.gz", "22"),
                                "23" => java::install_java_on_arch_linux("https://download.java.net/java/GA/jdk23/3c5b90190c68498b986a97f276efd28a/37/GPL/openjdk-23_linux-x64_bin.tar.gz", "jdk-23.tar.gz", "23"),
                                "24" => java::install_java_on_arch_linux("https://download.java.net/java/early_access/jdk24/13/GPL/openjdk-24-ea+13_linux-x64_bin.tar.gz", "java-24-ea.tar.gz", "24"),
                                "latest" => java::install_java_on_arch_linux("https://download.java.net/java/early_access/jdk24/13/GPL/openjdk-24-ea+13_linux-x64_bin.tar.gz", "java-24-ea.tar.gz", "24"),
                                &_ => ()
                            }
                        }
                    }
                },
                OsType::Darwin => println!("That's not implemented yet.")
            }
        }

        return Self {
            os_type: self.os_type.clone(),
            os_release: self.os_release.clone(),
            current_user: self.current_user.clone(),
            first_arg: self.first_arg.clone(),
            second_arg: self.second_arg.clone(),
            third_arg: self.third_arg.clone(),
            fourth_arg: self.fourth_arg.clone()
        };
    }

    fn install_gradle(&self) -> Self {
        if self.first_arg == CommandType::Install && self.second_arg == "gradle".to_string() {
            match self.os_type {
                OsType::Windows => gradle::install_gradle_on_windows(&self.third_arg),
                OsType::Linux => {
                    let linux_dist = utils::return_linux_dist_etc_os_release();
                    let mut null_var = "".to_string();

                    match linux_dist {
                        "ubuntu" => gradle::install_gradle_on_debian_based_distros(&self.third_arg),
                        "debian" => gradle::install_gradle_on_debian_based_distros(&self.third_arg),
                        "kali linux" => gradle::install_gradle_on_debian_based_distros(&self.third_arg),
                        "pardus" => gradle::install_gradle_on_debian_based_distros(&self.third_arg),
                        "arch wsl" => gradle::install_gradle_on_arch_linux(&self.third_arg),
                        "alma linux" => gradle::install_gradle_on_alma_linux(&self.third_arg),
                        "centos" => gradle::install_gradle_on_centos_and_fedora(&self.third_arg),
                        "fedora" => gradle::install_gradle_on_centos_and_fedora(&self.third_arg),
                        "rocky" => gradle::install_gradle_on_rocky_linux(&self.third_arg),
                        &_ => null_var = "none".to_string()
                    }

                    if null_var == "none".to_string() {
                        if utils::check_if_linux_dist_is_arch_linux() {
                            gradle::install_gradle_on_arch_linux(&self.third_arg)
                        }
                    }
                },
                OsType::Darwin => println!("That's not implemented yet.")
            }
        }

        return Self {
            os_type: self.os_type.clone(),
            os_release: self.os_release.clone(),
            current_user: self.current_user.clone(),
            first_arg: self.first_arg.clone(),
            second_arg: self.second_arg.clone(),
            third_arg: self.third_arg.clone(),
            fourth_arg: self.fourth_arg.clone()
        };
    }

    fn install_maven(&self) -> Self {
        if self.first_arg == CommandType::Install && self.second_arg == "maven".to_string() {
            // Because the source files of maven's way of maintenances differentiates widely, we have to
            // make that preparations for creating links, file names and versions, otherwise the
            // actual installations of maven could be way more unmaintenable.
            let generate_version = match self.third_arg.as_str() {
                "1" => "1.0".to_string(),
                "1.1.0" => "1.1".to_string(),
                "1.0.0" => "1.0".to_string(),
                "2" => "2.0".to_string(),
                "2.0.0" => "2.0".to_string(),
                "2.1" => "2.1.0".to_string(),
                "2.2" => "2.2.0".to_string(),
                "3" => "3.0".to_string(),
                "3.0.0" => "3.0".to_string(),
                "3.1" => "3.1.0".to_string(),
                "3.2" => "3.2.0".to_string(),
                _ => self.third_arg.trim().to_string().clone()
            };

            println!("generated version: {}", generate_version);

            let generate_file_name = match self.os_type {
                OsType::Windows => match self.third_arg.starts_with("1") {
                    true => format!("maven-{}.zip", generate_version),
                    false => match self.third_arg.starts_with("2") {
                        true => match generate_version.as_str() {
                            "2.0" => "maven-2.0.zip".to_string(),
                            "2.0.1" => "maven-2.0.1-bin.zip".to_string(),
                            "2.0.2" => "maven-2.0.2-bin.zip".to_string(),
                            "2.0.3" => "maven-2.0.3-bin.zip".to_string(),
                            "2.0.4" => "maven-2.0.4-bin.zip".to_string(),
                            "2.0.5" => "maven-2.0.5-bin.zip".to_string(),
                            "2.0.6" => "maven-2.0.6-bin.zip".to_string(),
                            "2.0.7" => "maven-2.0.7-bin.zip".to_string(),
                            _ => format!("apache-maven-{}-bin.zip", generate_version)
                        },
                        false => format!("apache-maven-{}-bin.zip", generate_version)
                    }
                },
                OsType::Linux => match self.third_arg.starts_with("1") {
                    true => format!("maven-{}.tar.gz", generate_version),
                    false => match self.third_arg.starts_with("2") {
                        true => match generate_version.as_str() {
                            "2.0" => "maven-2.0.tar.gz".to_string(),
                            "2.0.1" => "maven-2.0.1-bin.tar.gz".to_string(),
                            "2.0.2" => "maven-2.0.2-bin.tar.gz".to_string(),
                            "2.0.3" => "maven-2.0.3-bin.tar.gz".to_string(),
                            "2.0.4" => "maven-2.0.4-bin.tar.gz".to_string(),
                            "2.0.5" => "maven-2.0.5-bin.tar.gz".to_string(),
                            "2.0.6" => "maven-2.0.6-bin.tar.gz".to_string(),
                            "2.0.7" => "maven-2.0.7-bin.tar.gz".to_string(),
                            _ => format!("apache-maven-{}-bin.tar.gz", generate_version)
                        },
                        false => format!("apache-maven-{}-bin.tar.gz", generate_version)
                    }
                },
                OsType::Darwin => "not implemented for darwin!".to_string()
            };

            println!("generated file name: {}", generate_file_name);

            let generate_download_link = match &self.third_arg.starts_with("1") {
                true => format!("https://archive.apache.org/dist/maven/binaries/{}", generate_file_name),
                false => match &self.third_arg.starts_with("2") {
                    true => format!("https://archive.apache.org/dist/maven/binaries/{}", generate_file_name),
                    false => match &self.third_arg.starts_with("3") {
                        true => match self.third_arg.starts_with("3.2") || self.third_arg.starts_with("3.1") || self.third_arg.starts_with("3.0") {
                            true => format!("https://archive.apache.org/dist/maven/binaries/{}", generate_file_name),
                            false => format!("https://dlcdn.apache.org/maven/maven-3/{}/binaries/{}", generate_version, generate_file_name)
                        },
                        false => format!("https://dlcdn.apache.org/maven/maven-4/{}/binaries/{}", generate_version, generate_file_name)
                    }
                }
            };

            println!("generated download link: {}", generate_download_link);

            match self.os_type {
                OsType::Windows => maven::install_maven_on_windows(&generate_download_link, &generate_file_name, &generate_version),
                OsType::Linux => {
                    let linux_dist = utils::return_linux_dist_etc_os_release();
                    let mut null_var = "".to_string();

                    match linux_dist {
                        "ubuntu" => maven::install_maven_on_debian_based_distros(&generate_download_link, &generate_file_name, &generate_version),
                        "debian" => maven::install_maven_on_debian_based_distros(&generate_download_link, &generate_file_name, &generate_version),
                        "kali linux" => maven::install_maven_on_debian_based_distros(&generate_download_link, &generate_file_name, &generate_version),
                        "pardus" => maven::install_maven_on_debian_based_distros(&generate_download_link, &generate_file_name, &generate_version),
                        "arch wsl" => maven::install_maven_on_arch_linux(&generate_download_link, &generate_file_name, &generate_version),
                        "alma linux" => maven::install_maven_on_alma_linux(&generate_download_link, &generate_file_name, &generate_version),
                        "centos" => maven::install_maven_on_centos_and_fedora(&generate_download_link, &generate_file_name, &generate_version),
                        "fedora" => maven::install_maven_on_centos_and_fedora(&generate_download_link, &generate_file_name, &generate_version),
                        "rocky" => maven::install_maven_on_rocky_linux(&generate_download_link, &generate_file_name, &generate_version),
                        &_ => null_var = "none".to_string()
                    }

                    if null_var == "none".to_string() {
                        if utils::check_if_linux_dist_is_arch_linux() {
                            maven::install_maven_on_arch_linux(&generate_download_link, &generate_file_name, &generate_version)
                        }
                    }
                },
                OsType::Darwin => ()
            }
        }

        return Self {
            os_type: self.os_type.clone(),
            os_release: self.os_release.clone(),
            current_user: self.current_user.clone(),
            first_arg: self.first_arg.clone(),
            second_arg: self.second_arg.clone(),
            third_arg: self.third_arg.clone(),
            fourth_arg: self.fourth_arg.clone()
        };
    }

    fn install_nodejs(&self) -> Self {
        if self.first_arg == CommandType::Install && self.second_arg == "node".to_string() {
            match self.os_type {
                OsType::Windows => {
                    match self.third_arg.as_str() {
                        "lts" => node::install_nodejs_on_windows("https://nodejs.org/dist/v22.15.1/node-v22.15.1-x64.msi", "node-v22.15.1-x64.msi"),
                        "" => node::install_nodejs_on_windows("https://nodejs.org/dist/v22.15.1/node-v22.15.1-x64.msi", "node-v22.15.1-x64.msi"),
                        "latest" => node::install_nodejs_on_windows("https://nodejs.org/dist/v24.0.2/node-v24.0.2-x64.msi", "node-v24.0.2-x64.msi"),
                        &_ => node::install_nodejs_error()
                    };
                },
                OsType::Linux => {
                    let linux_dist = utils::return_linux_dist_etc_os_release();
                    let mut null_var = "".to_string();

                    match linux_dist {
                        "ubuntu" => {
                            match self.third_arg.as_str() {
                                "lts" => node::install_nodejs_on_debian_based_distros("https://nodejs.org/dist/v22.15.1/node-v22.15.1-linux-x64.tar.xz", "node-v22.15.1-linux-x64.tar.xz"),
                                "" => node::install_nodejs_on_debian_based_distros("https://nodejs.org/dist/v22.15.1/node-v22.15.1-linux-x64.tar.xz", "node-v22.15.1-linux-x64.tar.xz"),
                                "latest" => node::install_nodejs_on_debian_based_distros("https://nodejs.org/dist/v24.0.2/node-v24.0.2-linux-x64.tar.xz", "node-v24.0.2-linux-x64.tar.xz"),
                                &_ => node::install_nodejs_error()
                            }
                        },
                        "debian" => {
                            match self.third_arg.as_str() {
                                "lts" => node::install_nodejs_on_debian_based_distros("https://nodejs.org/dist/v22.15.1/node-v22.15.1-linux-x64.tar.xz", "node-v22.15.1-linux-x64.tar.xz"),
                                "" => node::install_nodejs_on_debian_based_distros("https://nodejs.org/dist/v22.15.1/node-v22.15.1-linux-x64.tar.xz", "node-v22.15.1-linux-x64.tar.xz"),
                                "latest" => node::install_nodejs_on_debian_based_distros("https://nodejs.org/dist/v24.0.2/node-v24.0.2-linux-x64.tar.xz", "node-v24.0.2-linux-x64.tar.xz"),
                                &_ => node::install_nodejs_error()
                            }
                        },
                        "pardus" => {
                            match self.third_arg.as_str() {
                                "lts" => node::install_nodejs_on_debian_based_distros("https://nodejs.org/dist/v22.15.1/node-v22.15.1-linux-x64.tar.xz", "node-v22.15.1-linux-x64.tar.xz"),
                                "" => node::install_nodejs_on_debian_based_distros("https://nodejs.org/dist/v22.15.1/node-v22.15.1-linux-x64.tar.xz", "node-v22.15.1-linux-x64.tar.xz"),
                                "latest" => node::install_nodejs_on_debian_based_distros("https://nodejs.org/dist/v24.0.2/node-v24.0.2-linux-x64.tar.xz", "node-v24.0.2-linux-x64.tar.xz"),
                                &_ => node::install_nodejs_error()
                            }
                        },
                        "arch wsl" => {
                            match self.third_arg.as_str() {
                                "lts" => node::install_nodejs_on_arch_linux("https://nodejs.org/dist/v22.15.1/node-v22.15.1-linux-x64.tar.xz", "node-v22.15.1-linux-x64.tar.xz"),
                                "" => node::install_nodejs_on_arch_linux("https://nodejs.org/dist/v22.15.1/node-v22.15.1-linux-x64.tar.xz", "node-v22.15.1-linux-x64.tar.xz"),
                                "latest" => node::install_nodejs_on_arch_linux("https://nodejs.org/dist/v24.0.2/node-v24.0.2-linux-x64.tar.xz", "node-v24.0.2-linux-x64.tar.xz"),
                                &_ => node::install_nodejs_error()
                            }
                        }
                        "kali linux" => {
                            match self.third_arg.as_str() {
                                "lts" => node::install_nodejs_on_debian_based_distros("https://nodejs.org/dist/v22.15.1/node-v22.15.1-linux-x64.tar.xz", "node-v22.15.1-linux-x64.tar.xz"),
                                "" => node::install_nodejs_on_debian_based_distros("https://nodejs.org/dist/v22.15.1/node-v22.15.1-linux-x64.tar.xz", "node-v22.15.1-linux-x64.tar.xz"),
                                "latest" => node::install_nodejs_on_debian_based_distros("https://nodejs.org/dist/v24.0.2/node-v24.0.2-linux-x64.tar.xz", "node-v24.0.2-linux-x64.tar.xz"),
                                &_ => node::install_nodejs_error()
                            }
                        },
                        "fedora" => {
                            match self.third_arg.as_str() {
                                "lts" => node::install_nodejs_on_centos_and_fedora("https://nodejs.org/dist/v22.15.1/node-v22.15.1-linux-x64.tar.xz", "node-v22.15.1-linux-x64.tar.xz"),
                                "" => node::install_nodejs_on_centos_and_fedora("https://nodejs.org/dist/v22.15.1/node-v22.15.1-linux-x64.tar.xz", "node-v22.15.1-linux-x64.tar.xz"),
                                "latest" => node::install_nodejs_on_centos_and_fedora("https://nodejs.org/dist/v24.0.2/node-v24.0.2-linux-x64.tar.xz", "node-v24.0.2-linux-x64.tar.xz"),
                                &_ => node::install_nodejs_error()
                            }
                        },
                        "centos" => {
                            match self.third_arg.as_str() {
                                "lts" => node::install_nodejs_on_centos_and_fedora("https://nodejs.org/dist/v22.15.1/node-v22.15.1-linux-x64.tar.xz", "node-v22.15.1-linux-x64.tar.xz"),
                                "" => node::install_nodejs_on_centos_and_fedora("https://nodejs.org/dist/v22.15.1/node-v22.15.1-linux-x64.tar.xz", "node-v22.15.1-linux-x64.tar.xz"),
                                "latest" => node::install_nodejs_on_centos_and_fedora("https://nodejs.org/dist/v24.0.2/node-v24.0.2-linux-x64.tar.xz", "node-v24.0.2-linux-x64.tar.xz"),
                                &_ => node::install_nodejs_error()
                            }
                        },
                        "rocky" => {
                            match self.third_arg.as_str() {
                                "lts" => node::install_nodejs_on_rocky_linux("https://nodejs.org/dist/v22.15.1/node-v22.15.1-linux-x64.tar.xz", "node-v22.15.1-linux-x64.tar.xz"),
                                "" => node::install_nodejs_on_rocky_linux("https://nodejs.org/dist/v22.15.1/node-v22.15.1-linux-x64.tar.xz", "node-v22.15.1-linux-x64.tar.xz"),
                                "latest" => node::install_nodejs_on_rocky_linux("https://nodejs.org/dist/v24.0.2/node-v24.0.2-linux-x64.tar.xz", "node-v24.0.2-linux-x64.tar.xz"),
                                &_ => node::install_nodejs_error()
                            }
                        },
                        "alma linux" => {
                            match self.third_arg.as_str() {
                                "lts" => node::install_nodejs_on_alma_linux("https://nodejs.org/dist/v22.15.1/node-v22.15.1-linux-x64.tar.xz", "node-v22.15.1-linux-x64.tar.xz"),
                                "" => node::install_nodejs_on_alma_linux("https://nodejs.org/dist/v22.15.1/node-v22.15.1-linux-x64.tar.xz", "node-v22.15.1-linux-x64.tar.xz"),
                                "latest" => node::install_nodejs_on_alma_linux("https://nodejs.org/dist/v24.0.2/node-v24.0.2-linux-x64.tar.xz", "node-v24.0.2-linux-x64.tar.xz"),
                                &_ => node::install_nodejs_error()
                            }
                        },
                        "alpine" => node::install_node_on_alpine_linux(),
                        &_ => null_var = "none".to_string()
                    }

                    if null_var == "none".to_string() {
                        if utils::check_if_linux_dist_is_arch_linux() {
                            match self.third_arg.as_str() {
                                "lts" => node::install_nodejs_on_arch_linux("https://nodejs.org/dist/v22.15.1/node-v22.15.1-linux-x64.tar.xz", "node-v22.15.1-linux-x64.tar.xz"),
                                "" => node::install_nodejs_on_arch_linux("https://nodejs.org/dist/v22.15.1/node-v22.15.1-linux-x64.tar.xz", "node-v22.15.1-linux-x64.tar.xz"),
                                "latest" => node::install_nodejs_on_arch_linux("https://nodejs.org/dist/v24.0.2/node-v24.0.2-linux-x64.tar.xz", "node-v24.0.2-linux-x64.tar.xz"),
                                &_ => node::install_nodejs_error()
                            }
                        }
                    }
                },
                OsType::Darwin => println!("That's not implemented yet.")
            }
        }

        return Self {
            os_type: self.os_type.clone(),
            os_release: self.os_release.clone(),
            current_user: self.current_user.clone(),
            first_arg: self.first_arg.clone(),
            second_arg: self.second_arg.clone(),
            third_arg: self.third_arg.clone(),
            fourth_arg: self.fourth_arg.clone()
        };
    }

    fn install_bun(&self) -> Self {
        if self.first_arg == CommandType::Install && self.second_arg == "bun".to_string() {
            match self.os_type {
                OsType::Windows => println!("Unfortunately, in that moment bun hasn't any windows support."),
                OsType::Linux => {
                    let linux_dist = utils::return_linux_dist_etc_os_release();
                    let mut null_var = "".to_string();

                    match linux_dist {
                        "ubuntu" => bun::install_bun_on_linux(),
                        "debian" => bun::install_bun_on_linux(),
                        "pardus" => bun::install_bun_on_linux(),
                        "arch wsl" => bun::install_bun_on_linux(),
                        "kali linux" => bun::install_bun_on_linux(),
                        "fedora" => bun::install_bun_on_linux(),
                        "centos" => bun::install_bun_on_linux(),
                        "rocky" => bun::install_bun_on_rocky_linux(),
                        "alma linux" => bun::install_bun_on_linux(),
                        "alpine linux" => bun::install_bun_on_linux(),
                        &_ => null_var = "none".to_string() 
                    }

                    if null_var == "none".to_string() {
                        if utils::check_if_linux_dist_is_arch_linux() {
                            bun::install_bun_on_linux()
                        }
                    }
                },
                OsType::Darwin => println!("That's not implemented yet.")
            }
        }

        return Self {
            os_type: self.os_type.clone(),
            os_release: self.os_release.clone(),
            current_user: self.current_user.clone(),
            first_arg: self.first_arg.clone(),
            second_arg: self.second_arg.clone(),
            third_arg: self.third_arg.clone(),
            fourth_arg: self.fourth_arg.clone()
        };
    }

    fn install_yarn(&self) -> Self {
        if self.first_arg == CommandType::Install && self.second_arg == "yarn".to_string() {
            match self.os_type {
                OsType::Windows => yarn::install_yarn_on_windows(),
                OsType::Linux => yarn::install_yarn_on_linux(),
                OsType::Darwin => println!("Not Implemented Yet.")
            }
        }

        return Self {
            os_type: self.os_type.clone(),
            os_release: self.os_release.clone(),
            current_user: self.current_user.clone(),
            first_arg: self.first_arg.clone(),
            second_arg: self.second_arg.clone(),
            third_arg: self.third_arg.clone(),
            fourth_arg: self.fourth_arg.clone()
        };
    }

    fn install_python(&self) -> Self {
        if self.first_arg == CommandType::Install && self.second_arg == "python".to_string() {
            todo!()
        }

        return Self {
            os_type: self.os_type.clone(),
            os_release: self.os_release.clone(),
            current_user: self.current_user.clone(),
            first_arg: self.first_arg.clone(),
            second_arg: self.second_arg.clone(),
            third_arg: self.third_arg.clone(),
            fourth_arg: self.fourth_arg.clone()
        };
    }
}

// aşşağıdaki ufulelerde nermefraz lisanları ve paket idarecileri hakkında bilgi ver:

impl Help for UserAction {
    fn log_help(&self) -> Self {
        if self.first_arg == CommandType::Help {
            println!("Hello to the incli! With incli, you can download rust, go, java, python and node.js sdk's which suitable for your current operating system via that cli.");
            
            match self.os_type {
                OsType::Windows => println!("Your operating system type is Windows."),
                OsType::Linux => println!("Your operating system type is Linux."),
                OsType::Darwin => println!("Your operating system type is MacOs"),
            }

            println!("Your operating system release is: {}", self.os_release);

            println!("This program currently supports to install Rust language, Node.js and Bun Runtimes on various Windows and Linux distros and releases.");
            println!("Support for other languages will be provided on future releases. If you have any suggestion or feature request you can create an issue on https://www.github.com/Necoo33/incli .");
        } else {
            println!("error, if command type is not help, then you shouldn't run that method.")
        }

        return Self {
            os_type: self.os_type.clone(),
            os_release: self.os_release.clone(),
            current_user: self.current_user.clone(),
            first_arg: self.first_arg.clone(),
            second_arg: self.second_arg.clone(),
            third_arg: self.third_arg.clone(),
            fourth_arg: self.fourth_arg.clone()
        };
    }

    fn log_help_error(&self) -> Self {
        println!("invalid second argument for help command.");

        return Self {
            os_type: self.os_type.clone(),
            os_release: self.os_release.clone(),
            current_user: self.current_user.clone(),
            first_arg: self.first_arg.clone(),
            second_arg: self.second_arg.clone(),
            third_arg: self.third_arg.clone(),
            fourth_arg: self.fourth_arg.clone()
        };
    }
    
    fn rust_help(&self) -> Self {
        println!("Rust is a low level, safety first, unique language which is many times more performant than c and c++.");
        println!("Language is inspired mostly from Node.js, Haskell and C++. It took it's package manager and many function names from Node.js, pattern matching and immutability by default from haskell and it's various low level features, such as it's range of pointers from c++.");
        println!("Rust started to be developed on 2007 and it's first stable release on 2015.");
        println!("Rust's Ecosystem is still in early stages but because of the rust's guarantee 'If it compiles, it'll work without bug' every rust package that you can found most probably will work, if there is no logical bug.");
        println!("Rust has very loyal developer community that for most of them Rust is a niche.");
        println!("It has more different synthax and programming style than other c inspired languages such as c++, java, javascript, php. Especially the borrow checker is most complicated things for programmers but also that thing makes rust is safe.");
        println!("Also, that program written with rust, because we don't want to deal with poor package management and build systems of java and c++.");

        return Self {
            os_type: self.os_type.clone(),
            os_release: self.os_release.clone(),
            current_user: self.current_user.clone(),
            first_arg: self.first_arg.clone(),
            second_arg: self.second_arg.clone(),
            third_arg: self.third_arg.clone(),
            fourth_arg: self.fourth_arg.clone()
        };
    }

    fn go_help(&self) -> Self {
        println!("Go is High level, compiling, very performant, practical language with garbage collector. It has easy synthax.");
        println!("It's invented by google on 2007 and released it's first stable release on 2012.");
        println!("Even if it's intended to be an alternative to c++, it became more like a java alternative for Back-End Development. It's also very popular for developing cloud tools.");
        println!("For now, Go's ecosystem is not mature enough to be a whole alternative of java but it has active community which supports and develops it.");

        return Self {
            os_type: self.os_type.clone(),
            os_release: self.os_release.clone(),
            current_user: self.current_user.clone(),
            first_arg: self.first_arg.clone(),
            second_arg: self.second_arg.clone(),
            third_arg: self.third_arg.clone(),
            fourth_arg: self.fourth_arg.clone()
        };
    }

    fn nodejs_help(&self) -> Self {
        println!("Node.js is a Runtime Environment For Javascript. It uses V8 engine which developed by google. It comes with a package manager called npm");
        println!("It's development started on 2009, an it's first stable release released on 2014.");
        println!("Node.js is popular among the developers which learned javascript as their first language. It offers better performance than other popular back-end solutions such as php, ruby, c#.");

        return Self {
            os_type: self.os_type.clone(),
            os_release: self.os_release.clone(),
            current_user: self.current_user.clone(),
            first_arg: self.first_arg.clone(),
            second_arg: self.second_arg.clone(),
            third_arg: self.third_arg.clone(),
            fourth_arg: self.fourth_arg.clone()
        };
    }

    fn bun_help(&self) -> Self {
        println!("Bun is a brand new Runtime Environment for Javascript, which aims better performance than node.js and also %100 compatibility to it.");
        println!("Bun also has built-in testing, linting, packaging support. It's written with brand new low level language Zig.");
        println!("Bun released it's first stable version on 2023. It currently supports only Mac Os and Linux.");
        println!("According to benchmarks, bun is 2 times faster than node.js. It's fastest web frameworks such as Elysia.js And Hono.js are competing with go and java web frameworks on performance.");

        return Self {
            os_type: self.os_type.clone(),
            os_release: self.os_release.clone(),
            current_user: self.current_user.clone(),
            first_arg: self.first_arg.clone(),
            second_arg: self.second_arg.clone(),
            third_arg: self.third_arg.clone(),
            fourth_arg: self.fourth_arg.clone()
        };
    }

    fn yarn_help(&self) -> Self {
        println!("Yarn is a package manager for Node.js that developed primarily by facebook.");
        println!("It started to be developed on 2016 and it's first stable release released on 2017.");
        println!("It's compatible with npm and it offers many features that don't available on npm such as high performance, package caching and so on.");
        return Self {
            os_type: self.os_type.clone(),
            os_release: self.os_release.clone(),
            current_user: self.current_user.clone(),
            first_arg: self.first_arg.clone(),
            second_arg: self.second_arg.clone(),
            third_arg: self.third_arg.clone(),
            fourth_arg: self.fourth_arg.clone()
        };
    }

    fn jdk_help(&self) -> Self {
        println!("Java is a high level, statically typed compiled and interpreted programming language that invented on 1995.");
        println!("Java codes first compiled via Java compiler to java bytecodes then interpreted via Java Virtual Machine.");
        println!("Because of that nature, java is faster than other interpreted languages, such as javascript, python, php and ruby.");
        println!("Java is intended to be a c++ replacement for Performance-Critic Back-End due to it's safety, platform agnosticism and especially easiness of tooling relative to c++ and achieved very good success about it. It's more slower than c++ but way more easy to setup, develop and deploy all kind of apps.");        
        println!("Later than it published, in a few years, it achieved to be de facto standard on Back-End development for performance-critic systems for decades. In current years, it's in a slow decline especially because of rise of go and rust.");
        println!("JDK, which means Java Development Kit, includes Java Runtime Environment, Java Virtual Machine And Java Api's. It's main tooling for anything a java app needs.");
       
        return Self {
            os_type: self.os_type.clone(),
            os_release: self.os_release.clone(),
            current_user: self.current_user.clone(),
            first_arg: self.first_arg.clone(),
            second_arg: self.second_arg.clone(),
            third_arg: self.third_arg.clone(),
            fourth_arg: self.fourth_arg.clone()
        };
    }

    fn gradle_help(&self) -> Self {
        println!("Gradle is an open source build, testing, automation tool and package manager that commonly used with Java Virtual Machine.");
        println!("It founded on 2009 and released it's stable version on 2012.");
        println!("It's the main tool for android development.");

        return Self {
            os_type: self.os_type.clone(),
            os_release: self.os_release.clone(),
            current_user: self.current_user.clone(),
            first_arg: self.first_arg.clone(),
            second_arg: self.second_arg.clone(),
            third_arg: self.third_arg.clone(),
            fourth_arg: self.fourth_arg.clone()
        };
    }

    fn maven_help(&self) -> Self {
        println!("Maven is a compiler, tester, package manager for Java.");
        println!("It started to be developed on 2001 by Apache Ant and it released it's first stable version on 2004.");
        return Self {
            os_type: self.os_type.clone(),
            os_release: self.os_release.clone(),
            current_user: self.current_user.clone(),
            first_arg: self.first_arg.clone(),
            second_arg: self.second_arg.clone(),
            third_arg: self.third_arg.clone(),
            fourth_arg: self.fourth_arg.clone()
        };
    }

    fn python_help(&self) -> Self {
        println!("Python is a high level, multipurpose, interpreted programming language.");
        println!("It's development started on 1990 and it's first stable version released on 1994.");
        println!("It's commonly used for machine learning with packages that uses c and c++ under the hood.");
        println!("Because it's slowness and lacking of curly braces, most of the time it is preferred on basic things such as small web services and mini desktop apps.");

        return Self {
            os_type: self.os_type.clone(),
            os_release: self.os_release.clone(),
            current_user: self.current_user.clone(),
            first_arg: self.first_arg.clone(),
            second_arg: self.second_arg.clone(),
            third_arg: self.third_arg.clone(),
            fourth_arg: self.fourth_arg.clone()
        };
    }
}

impl Version for UserAction {
    fn log_version(&self) -> Self {
        println!("You're using v0.10.0 of incli.");

        return Self {
            os_type: self.os_type.clone(),
            os_release: self.os_release.clone(),
            current_user: self.current_user.clone(),
            first_arg: self.first_arg.clone(),
            second_arg: self.second_arg.clone(),
            third_arg: self.third_arg.clone(),
            fourth_arg: self.fourth_arg.clone()
        };
    }

    fn log_version_error(&self) -> Self {
        println!("invalid version command");
        
        return Self {
            os_type: self.os_type.clone(),
            os_release: self.os_release.clone(),
            current_user: self.current_user.clone(),
            first_arg: self.first_arg.clone(),
            second_arg: self.second_arg.clone(),
            third_arg: self.third_arg.clone(),
            fourth_arg: self.fourth_arg.clone()
        };
    }

    fn rust_version(&self) -> Self {
        rust::log_rust_version();

        return Self {
            os_type: self.os_type.clone(),
            os_release: self.os_release.clone(),
            current_user: self.current_user.clone(),
            first_arg: self.first_arg.clone(),
            second_arg: self.second_arg.clone(),
            third_arg: self.third_arg.clone(),
            fourth_arg: self.fourth_arg.clone()
        };
    }

    fn go_version(&self) -> Self {
        go::log_go_version();
        
        return Self {
            os_type: self.os_type.clone(),
            os_release: self.os_release.clone(),
            current_user: self.current_user.clone(),
            first_arg: self.first_arg.clone(),
            second_arg: self.second_arg.clone(),
            third_arg: self.third_arg.clone(),
            fourth_arg: self.fourth_arg.clone()
        };
    }

    fn nodejs_version(&self) -> Self {
        node::log_node_and_npm_version();

        return Self {
            os_type: self.os_type.clone(),
            os_release: self.os_release.clone(),
            current_user: self.current_user.clone(),
            first_arg: self.first_arg.clone(),
            second_arg: self.second_arg.clone(),
            third_arg: self.third_arg.clone(),
            fourth_arg: self.fourth_arg.clone()
        };
    }

    fn bun_version(&self) -> Self {
        bun::log_bun_version();
        
        return Self {
            os_type: self.os_type.clone(),
            os_release: self.os_release.clone(),
            current_user: self.current_user.clone(),
            first_arg: self.first_arg.clone(),
            second_arg: self.second_arg.clone(),
            third_arg: self.third_arg.clone(),
            fourth_arg: self.fourth_arg.clone()
        };
    }

    fn yarn_version(&self) -> Self {
        return Self {
            os_type: self.os_type.clone(),
            os_release: self.os_release.clone(),
            current_user: self.current_user.clone(),
            first_arg: self.first_arg.clone(),
            second_arg: self.second_arg.clone(),
            third_arg: self.third_arg.clone(),
            fourth_arg: self.fourth_arg.clone()
        };
    }

    fn jdk_version(&self) -> Self {
        return Self {
            os_type: self.os_type.clone(),
            os_release: self.os_release.clone(),
            current_user: self.current_user.clone(),
            first_arg: self.first_arg.clone(),
            second_arg: self.second_arg.clone(),
            third_arg: self.third_arg.clone(),
            fourth_arg: self.fourth_arg.clone()
        };
    }

    fn gradle_version(&self) -> Self {
        return Self {
            os_type: self.os_type.clone(),
            os_release: self.os_release.clone(),
            current_user: self.current_user.clone(),
            first_arg: self.first_arg.clone(),
            second_arg: self.second_arg.clone(),
            third_arg: self.third_arg.clone(),
            fourth_arg: self.fourth_arg.clone()
        };
    }

    fn maven_version(&self) -> Self {
        return Self {
            os_type: self.os_type.clone(),
            os_release: self.os_release.clone(),
            current_user: self.current_user.clone(),
            first_arg: self.first_arg.clone(),
            second_arg: self.second_arg.clone(),
            third_arg: self.third_arg.clone(),
            fourth_arg: self.fourth_arg.clone()
        };
    }

    fn python_version(&self) -> Self {
        return Self {
            os_type: self.os_type.clone(),
            os_release: self.os_release.clone(),
            current_user: self.current_user.clone(),
            first_arg: self.first_arg.clone(),
            second_arg: self.second_arg.clone(),
            third_arg: self.third_arg.clone(),
            fourth_arg: self.fourth_arg.clone()
        };
    }
}
