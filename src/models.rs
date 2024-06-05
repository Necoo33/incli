use crate::utils;
use crate::rust;
use crate::node;
use crate::bun;
use crate::yarn;
use crate::go;

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
}

pub trait CreateAction {
    fn action(&self, cmd_type: String, second_arg: String, third_arg: String) -> UserAction;
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
    fn action(&self, cmd_type: String, second_arg: String, third_arg: String) -> UserAction {
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
            third_arg: third_arg.to_string()
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
                    third_arg: self.third_arg.clone()
                };
            }

            _ => {
                return Self {
                    os_type: self.os_type.clone(),
                    os_release: self.os_release.clone(),
                    current_user: self.current_user.clone(),
                    first_arg: self.first_arg.clone(),
                    second_arg: self.second_arg.clone(),
                    third_arg: self.third_arg.clone()
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
                    third_arg: self.third_arg.clone()
                };
            },
            _ => {
                return Self {
                    os_type: self.os_type.clone(),
                    os_release: self.os_release.clone(),
                    current_user: self.current_user.clone(),
                    first_arg: self.first_arg.clone(),
                    second_arg: self.second_arg.clone(),
                    third_arg: self.third_arg.clone()
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
                    third_arg: self.third_arg.clone()
                };
            },
            _ => {
                return Self {
                    os_type: self.os_type.clone(),
                    os_release: self.os_release.clone(),
                    current_user: self.current_user.clone(),
                    first_arg: self.first_arg.clone(),
                    second_arg: self.second_arg.clone(),
                    third_arg: self.third_arg.clone()
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
            third_arg: self.third_arg.clone()
        };
    }

    fn install_go(&self) -> Self {
        if self.first_arg == CommandType::Install && self.second_arg == "go".to_string() {
            match self.os_type {
                OsType::Windows => {
                    match self.third_arg.as_str() {
                        "lts" => go::install_go_on_windows("https://go.dev/dl/go1.22.4.windows-amd64.msi", "go1.22.4.windows-amd64.msi"),
                    "" => go::install_go_on_windows("https://go.dev/dl/go1.22.4.windows-amd64.msi", "go1.22.4.windows-amd64.msi"),
                        "latest" => go::install_go_on_windows("https://go.dev/dl/go1.22rc2.windows-amd64.msi", "go1.22rc2.windows-amd64.msi"),
                        &_ => go::install_go_error()
                    };
                },
                OsType::Linux => {
                    let linux_dist = utils::return_linux_dist_etc_os_release();
                    let mut null_var = "".to_string();

                    match linux_dist {
                        "ubuntu" => {
                            match self.third_arg.as_str() {
                                "lts" => go::install_go_on_debian_based_distros("https://go.dev/dl/go1.22.4.linux-amd64.tar.gz", "go1.22.4.linux-amd64.tar.gz"),
                                "" => go::install_go_on_debian_based_distros("https://go.dev/dl/go1.22.4.linux-amd64.tar.gz", "go1.22.4.linux-amd64.tar.gz"),
                                "latest" => go::install_go_on_debian_based_distros("https://go.dev/dl/go1.22rc2.linux-amd64.tar.gz", "go1.22rc2.linux-amd64.tar.gz"),
                                &_ => go::install_go_error()
                            }
                        },
                        "debian" => {
                            match self.third_arg.as_str() {
                                "lts" => go::install_go_on_debian_based_distros("https://go.dev/dl/go1.22.4.linux-amd64.tar.gz", "go1.22.4.linux-amd64.tar.gz"),
                                "" => go::install_go_on_debian_based_distros("https://go.dev/dl/go1.22.4.linux-amd64.tar.gz", "go1.22.4.linux-amd64.tar.gz"),
                                "latest" => go::install_go_on_debian_based_distros("https://go.dev/dl/go1.22rc2.linux-amd64.tar.gz", "go1.22rc2.linux-amd64.tar.gz"),
                                &_ => go::install_go_error()
                            }
                        },
                        "pardus" => {
                            match self.third_arg.as_str() {
                                "lts" => go::install_go_on_debian_based_distros("https://go.dev/dl/go1.22.4.linux-amd64.tar.gz", "go1.22.4.linux-amd64.tar.gz"),
                                "" => go::install_go_on_debian_based_distros("https://go.dev/dl/go1.22.4.linux-amd64.tar.gz", "go1.22.4.linux-amd64.tar.gz"),
                                "latest" => go::install_go_on_debian_based_distros("https://go.dev/dl/go1.22rc2.linux-amd64.tar.gz", "go1.22rc2.linux-amd64.tar.gz"),
                                &_ => go::install_go_error()
                            }
                        },
                        "arch wsl" => {
                            match self.third_arg.as_str() {
                                "lts" => go::install_go_on_arch_linux("https://go.dev/dl/go1.22.4.linux-amd64.tar.gz", "go1.22.4.linux-amd64.tar.gz"),
                                "" => go::install_go_on_arch_linux("https://go.dev/dl/go1.22.4.linux-amd64.tar.gz", "go1.22.4.linux-amd64.tar.gz"),
                                "latest" => go::install_go_on_arch_linux("https://go.dev/dl/go1.22rc2.linux-amd64.tar.gz", "go1.22rc2.linux-amd64.tar.gz"),
                                &_ => go::install_go_error()
                            }
                        }
                        "kali linux" => {
                            match self.third_arg.as_str() {
                                "lts" => go::install_go_on_debian_based_distros("https://go.dev/dl/go1.22.4.linux-amd64.tar.gz", "go1.22.4.linux-amd64.tar.gz"),
                                "" => go::install_go_on_debian_based_distros("https://go.dev/dl/go1.22.4.linux-amd64.tar.gz", "go1.22.4.linux-amd64.tar.gz"),
                                "latest" => go::install_go_on_debian_based_distros("https://go.dev/dl/go1.22rc2.linux-amd64.tar.gz", "go1.22rc2.linux-amd64.tar.gz"),
                                &_ => go::install_go_error()
                            }
                        },
                        "fedora" => {
                            match self.third_arg.as_str() {
                                "lts" => go::install_go_on_centos_and_fedora("https://go.dev/dl/go1.22.4.linux-amd64.tar.gz", "go1.22.4.linux-amd64.tar.gz"),
                                "" => go::install_go_on_centos_and_fedora("https://go.dev/dl/go1.22.4.linux-amd64.tar.gz", "go1.22.4.linux-amd64.tar.gz"),
                                "latest" => go::install_go_on_centos_and_fedora("https://go.dev/dl/go1.22rc2.linux-amd64.tar.gz", "go1.22rc2.linux-amd64.tar.gz"),
                                &_ => go::install_go_error()
                            }
                        },
                        "centos" => {
                            match self.third_arg.as_str() {
                                "lts" => go::install_go_on_centos_and_fedora("https://go.dev/dl/go1.22.4.linux-amd64.tar.gz", "go1.22.4.linux-amd64.tar.gz"),
                                "" => go::install_go_on_centos_and_fedora("https://go.dev/dl/go1.22.4.linux-amd64.tar.gz", "go1.22.4.linux-amd64.tar.gz"),
                                "latest" => go::install_go_on_centos_and_fedora("https://go.dev/dl/go1.22rc2.linux-amd64.tar.gz", "go1.22rc2.linux-amd64.tar.gz"),
                                &_ => go::install_go_error()
                            }
                        },
                        "alma linux" => {
                            match self.third_arg.as_str() {
                                "lts" => go::install_go_on_alma_linux("https://go.dev/dl/go1.22.4.linux-amd64.tar.gz", "go1.22.4.linux-amd64.tar.gz"),
                                "" => go::install_go_on_alma_linux("https://go.dev/dl/go1.22.4.linux-amd64.tar.gz", "go1.22.4.linux-amd64.tar.gz"),
                                "latest" => go::install_go_on_alma_linux("https://go.dev/dl/go1.22rc2.linux-amd64.tar.gz", "go1.22rc2.linux-amd64.tar.gz"),
                                &_ => go::install_go_error()
                            }
                        },
                        "alpine" => go::install_go_on_alpine_linux("https://go.dev/dl/go1.22.4.linux-amd64.tar.gz", "go1.22.4.linux-amd64.tar.gz"),
                        &_ => null_var = "none".to_string()
                    }

                    if null_var == "none".to_string() {
                        if utils::check_if_linux_dist_is_arch_linux() {
                            match self.third_arg.as_str() {
                                "lts" => go::install_go_on_arch_linux("https://go.dev/dl/go1.22.4.linux-amd64.tar.gz", "go1.22.4.linux-amd64.tar.gz"),
                                "" => go::install_go_on_arch_linux("https://go.dev/dl/go1.22.4.linux-amd64.tar.gz", "go1.22.4.linux-amd64.tar.gz"),
                                "latest" => go::install_go_on_arch_linux("https://go.dev/dl/go1.22rc2.linux-amd64.tar.gz", "go1.22rc2.linux-amd64.tar.gz"),
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
            third_arg: self.third_arg.clone()
        };
    }

    fn install_jdk(&self) -> Self {
        if self.first_arg == CommandType::Install && self.second_arg == "java".to_string() {
            todo!()
        }

        return Self {
            os_type: self.os_type.clone(),
            os_release: self.os_release.clone(),
            current_user: self.current_user.clone(),
            first_arg: self.first_arg.clone(),
            second_arg: self.second_arg.clone(),
            third_arg: self.third_arg.clone()
        };
    }

    fn install_gradle(&self) -> Self {
        if self.first_arg == CommandType::Install && self.second_arg == "gradle".to_string() {
            todo!()
        }

        return Self {
            os_type: self.os_type.clone(),
            os_release: self.os_release.clone(),
            current_user: self.current_user.clone(),
            first_arg: self.first_arg.clone(),
            second_arg: self.second_arg.clone(),
            third_arg: self.third_arg.clone()
        };
    }

    fn install_maven(&self) -> Self {
        if self.first_arg == CommandType::Install && self.second_arg == "maven".to_string() {
            todo!()
        }

        return Self {
            os_type: self.os_type.clone(),
            os_release: self.os_release.clone(),
            current_user: self.current_user.clone(),
            first_arg: self.first_arg.clone(),
            second_arg: self.second_arg.clone(),
            third_arg: self.third_arg.clone()
        };
    }

    fn install_nodejs(&self) -> Self {
        if self.first_arg == CommandType::Install && self.second_arg == "node".to_string() {
            match self.os_type {
                OsType::Windows => {
                    match self.third_arg.as_str() {
                        "lts" => node::install_nodejs_on_windows("https://nodejs.org/dist/v20.14.0/node-v20.14.0-x64.msi", "node-v20.14.0-x64.msi"),
                        "" => node::install_nodejs_on_windows("https://nodejs.org/dist/v20.14.0/node-v20.14.0-x64.msi", "node-v20.14.0-x64.msi"),
                        "latest" => node::install_nodejs_on_windows("https://nodejs.org/dist/v22.2.0/node-v22.2.0-x64.msi", "node-v22.2.0-x64.msi"),
                        &_ => node::install_nodejs_error()
                    };
                },
                OsType::Linux => {
                    let linux_dist = utils::return_linux_dist_etc_os_release();
                    let mut null_var = "".to_string();

                    match linux_dist {
                        "ubuntu" => {
                            match self.third_arg.as_str() {
                                "lts" => node::install_nodejs_on_debian_based_distros("https://nodejs.org/dist/v20.14.0/node-v20.14.0-linux-x64.tar.xz", "node-v20.14.0-linux-x64.tar.xz"),
                                "" => node::install_nodejs_on_debian_based_distros("https://nodejs.org/dist/v20.14.0/node-v20.14.0-linux-x64.tar.xz", "node-v20.14.0-linux-x64.tar.xz"),
                                "latest" => node::install_nodejs_on_debian_based_distros("https://nodejs.org/dist/v22.2.0/node-v22.2.0-linux-x64.tar.xz", "node-v22.2.0-linux-x64.tar.xz"),
                                &_ => node::install_nodejs_error()
                            }
                        },
                        "debian" => {
                            match self.third_arg.as_str() {
                                "lts" => node::install_nodejs_on_debian_based_distros("https://nodejs.org/dist/v20.14.0/node-v20.14.0-linux-x64.tar.xz", "node-v20.14.0-linux-x64.tar.xz"),
                                "" => node::install_nodejs_on_debian_based_distros("https://nodejs.org/dist/v20.14.0/node-v20.14.0-linux-x64.tar.xz", "node-v20.14.0-linux-x64.tar.xz"),
                                "latest" => node::install_nodejs_on_debian_based_distros("https://nodejs.org/dist/v22.2.0/node-v22.2.0-linux-x64.tar.xz", "node-v22.2.0-linux-x64.tar.xz"),
                                &_ => node::install_nodejs_error()
                            }
                        },
                        "pardus" => {
                            match self.third_arg.as_str() {
                                "lts" => node::install_nodejs_on_debian_based_distros("https://nodejs.org/dist/v20.14.0/node-v20.14.0-linux-x64.tar.xz", "node-v20.14.0-linux-x64.tar.xz"),
                                "" => node::install_nodejs_on_debian_based_distros("https://nodejs.org/dist/v20.14.0/node-v20.14.0-linux-x64.tar.xz", "node-v20.14.0-linux-x64.tar.xz"),
                                "latest" => node::install_nodejs_on_debian_based_distros("https://nodejs.org/dist/v22.2.0/node-v22.2.0-linux-x64.tar.xz", "node-v22.2.0-linux-x64.tar.xz"),
                                &_ => node::install_nodejs_error()
                            }
                        },
                        "arch wsl" => {
                            match self.third_arg.as_str() {
                                "lts" => node::install_nodejs_on_arch_linux("https://nodejs.org/dist/v20.14.0/node-v20.14.0-linux-x64.tar.xz", "node-v20.14.0-linux-x64.tar.xz"),
                                "" => node::install_nodejs_on_arch_linux("https://nodejs.org/dist/v20.14.0/node-v20.14.0-linux-x64.tar.xz", "node-v20.14.0-linux-x64.tar.xz"),
                                "latest" => node::install_nodejs_on_arch_linux("https://nodejs.org/dist/v22.2.0/node-v22.2.0-linux-x64.tar.xz", "node-v22.2.0-linux-x64.tar.xz"),
                                &_ => node::install_nodejs_error()
                            }
                        }
                        "kali linux" => {
                            match self.third_arg.as_str() {
                                "lts" => node::install_nodejs_on_debian_based_distros("https://nodejs.org/dist/v20.14.0/node-v20.14.0-linux-x64.tar.xz", "node-v20.14.0-linux-x64.tar.xz"),
                                "" => node::install_nodejs_on_debian_based_distros("https://nodejs.org/dist/v20.14.0/node-v20.14.0-linux-x64.tar.xz", "node-v20.14.0-linux-x64.tar.xz"),
                                "latest" => node::install_nodejs_on_debian_based_distros("https://nodejs.org/dist/v22.2.0/node-v22.2.0-linux-x64.tar.xz", "node-v22.2.0-linux-x64.tar.xz"),
                                &_ => node::install_nodejs_error()
                            }
                        },
                        "fedora" => {
                            match self.third_arg.as_str() {
                                "lts" => node::install_nodejs_on_centos_and_fedora("https://nodejs.org/dist/v20.14.0/node-v20.14.0-linux-x64.tar.xz", "node-v20.14.0-linux-x64.tar.xz"),
                                "" => node::install_nodejs_on_centos_and_fedora("https://nodejs.org/dist/v20.14.0/node-v20.14.0-linux-x64.tar.xz", "node-v20.14.0-linux-x64.tar.xz"),
                                "latest" => node::install_nodejs_on_centos_and_fedora("https://nodejs.org/dist/v22.2.0/node-v22.2.0-linux-x64.tar.xz", "node-v22.2.0-linux-x64.tar.xz"),
                                &_ => node::install_nodejs_error()
                            }
                        },
                        "centos" => {
                            match self.third_arg.as_str() {
                                "lts" => node::install_nodejs_on_centos_and_fedora("https://nodejs.org/dist/v20.14.0/node-v20.14.0-linux-x64.tar.xz", "node-v20.14.0-linux-x64.tar.xz"),
                                "" => node::install_nodejs_on_centos_and_fedora("https://nodejs.org/dist/v20.14.0/node-v20.14.0-linux-x64.tar.xz", "node-v20.14.0-linux-x64.tar.xz"),
                                "latest" => node::install_nodejs_on_centos_and_fedora("https://nodejs.org/dist/v22.2.0/node-v22.2.0-linux-x64.tar.xz", "node-v22.2.0-linux-x64.tar.xz"),
                                &_ => node::install_nodejs_error()
                            }
                        },
                        "alma linux" => {
                            match self.third_arg.as_str() {
                                "lts" => node::install_nodejs_on_alma_linux("https://nodejs.org/dist/v20.14.0/node-v20.14.0-linux-x64.tar.xz", "node-v20.14.0-linux-x64.tar.xz"),
                                "" => node::install_nodejs_on_alma_linux("https://nodejs.org/dist/v20.14.0/node-v20.14.0-linux-x64.tar.xz", "node-v20.14.0-linux-x64.tar.xz"),
                                "latest" => node::install_nodejs_on_alma_linux("https://nodejs.org/dist/v22.2.0/node-v22.2.0-linux-x64.tar.xz", "node-v22.2.0-linux-x64.tar.xz"),
                                &_ => node::install_nodejs_error()
                            }
                        },
                        "alpine" => node::install_node_on_alpine_linux(),
                        &_ => null_var = "none".to_string()
                    }

                    if null_var == "none".to_string() {
                        if utils::check_if_linux_dist_is_arch_linux() {
                            match self.third_arg.as_str() {
                                "lts" => node::install_nodejs_on_arch_linux("https://nodejs.org/dist/v20.14.0/node-v20.14.0-linux-x64.tar.xz", "node-v20.14.0-linux-x64.tar.xz"),
                                "" => node::install_nodejs_on_arch_linux("https://nodejs.org/dist/v20.14.0/node-v20.14.0-linux-x64.tar.xz", "node-v20.14.0-linux-x64.tar.xz"),
                                "latest" => node::install_nodejs_on_arch_linux("https://nodejs.org/dist/v22.2.0/node-v22.2.0-linux-x64.tar.xz", "node-v22.2.0-linux-x64.tar.xz"),
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
            third_arg: self.third_arg.clone()
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
            third_arg: self.third_arg.clone()
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
            third_arg: self.third_arg.clone()
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
            third_arg: self.third_arg.clone()
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
            third_arg: self.third_arg.clone()
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
            third_arg: self.third_arg.clone()
        };
    }
    
    fn rust_help(&self) -> Self {
        println!("Rust is a low level, safety first, unique language which is sometimes more performant than c and c++.");
        println!("Language is inspired from Node.js, Haskell and C++");
        println!("Rust started to be develope on 2007 and it's first stable release on 2015.");
        println!("Rust's Ecosystem is still in early stages but because of the rust's guarantee 'If it compiles, it'll work without bug' every rust package that you can found most probably will work.");
        println!("Rust has very loyal developer community that for most of them Rust is a niche.");
        println!("It has more different synthax and programming style than other c inspired languages such as c++, java, javascript, php. Especially the borrow checker is most complicated things for programmers but also that thing makes rust is safe.");
        println!("Also, that program written with rust.");

        return Self {
            os_type: self.os_type.clone(),
            os_release: self.os_release.clone(),
            current_user: self.current_user.clone(),
            first_arg: self.first_arg.clone(),
            second_arg: self.second_arg.clone(),
            third_arg: self.third_arg.clone()
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
            third_arg: self.third_arg.clone()
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
            third_arg: self.third_arg.clone()
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
            third_arg: self.third_arg.clone()
        };
    }

    fn yarn_help(&self) -> Self {
        println!("Yarn is a package manager for Node.js that developed primarily by facebook.");
        println!("It started to be develope on 2016 and it's first stable release released on 2017.");
        println!("It's compatible with npm and it offers many features that don't available on npm such as high performance, package caching and so on.");
        return Self {
            os_type: self.os_type.clone(),
            os_release: self.os_release.clone(),
            current_user: self.current_user.clone(),
            first_arg: self.first_arg.clone(),
            second_arg: self.second_arg.clone(),
            third_arg: self.third_arg.clone()
        };
    }

    fn jdk_help(&self) -> Self {
        println!("Java is a high level, compiled and interpreted programming language that invented on 1995.");
        println!("Java codes first compiled via Java compiler to java bytecodes then interpreted via Java Virtual Machine.");
        println!("Because of that nature, java is faster than other interpreted languages, such as javascript, python, php and ruby.");
        println!("Java is intended to be a c++ replacement for Performance-Critic Back-End due to it's safety, platform agnosticism and especially easiness of tooling relative to c++ and achieved very good success about it. It's more slower than c++ but way more easy to setup, develop and deploy all kind of apps.");        
        println!("Later than it published, in a few years, it achieved to be de facto standard on Back-End development for performance-critic systems for decades. In current years, it's in a slow decline especially because of rise of go and rust, and maybe in future bun.");
        println!("JDK, which means Java Development Kit, includes Java Runtime Environment, Java Virtual Machine And Java Api's. It's main tooling for anything a java app needs.");
       
        return Self {
            os_type: self.os_type.clone(),
            os_release: self.os_release.clone(),
            current_user: self.current_user.clone(),
            first_arg: self.first_arg.clone(),
            second_arg: self.second_arg.clone(),
            third_arg: self.third_arg.clone()
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
            third_arg: self.third_arg.clone()
        };
    }

    fn maven_help(&self) -> Self {
        println!("Maven is a compiler, tester, package manager for Java.");
        println!("It started to be develope on 2001 by Apache Ant and it released it's first stable version on 2004.");
        return Self {
            os_type: self.os_type.clone(),
            os_release: self.os_release.clone(),
            current_user: self.current_user.clone(),
            first_arg: self.first_arg.clone(),
            second_arg: self.second_arg.clone(),
            third_arg: self.third_arg.clone()
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
            third_arg: self.third_arg.clone()
        };
    }
}

// aşşağıdaki ufulelerde nermefrazların yüklü olup olmadığını teftiş et, eğer yüklü iseler "--version" talimatı
// ile yüklü unsurların versiyonunu bul ve daha sonra mevcud en yeni versiyonlarının hangi versiyonları olduğunu 
// bul ve şu usulle ekrana yazdır: "Your ... version is ... , the newest ... version is ..."

impl Version for UserAction {
    fn log_version(&self) -> Self {
        println!("You're using v0.3.0 of incli.");

        return Self {
            os_type: self.os_type.clone(),
            os_release: self.os_release.clone(),
            current_user: self.current_user.clone(),
            first_arg: self.first_arg.clone(),
            second_arg: self.second_arg.clone(),
            third_arg: self.third_arg.clone()
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
            third_arg: self.third_arg.clone()
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
            third_arg: self.third_arg.clone()
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
            third_arg: self.third_arg.clone()
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
            third_arg: self.third_arg.clone()
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
            third_arg: self.third_arg.clone()
        };
    }

    fn yarn_version(&self) -> Self {
        return Self {
            os_type: self.os_type.clone(),
            os_release: self.os_release.clone(),
            current_user: self.current_user.clone(),
            first_arg: self.first_arg.clone(),
            second_arg: self.second_arg.clone(),
            third_arg: self.third_arg.clone()
        };
    }

    fn jdk_version(&self) -> Self {
        return Self {
            os_type: self.os_type.clone(),
            os_release: self.os_release.clone(),
            current_user: self.current_user.clone(),
            first_arg: self.first_arg.clone(),
            second_arg: self.second_arg.clone(),
            third_arg: self.third_arg.clone()
        };
    }

    fn gradle_version(&self) -> Self {
        return Self {
            os_type: self.os_type.clone(),
            os_release: self.os_release.clone(),
            current_user: self.current_user.clone(),
            first_arg: self.first_arg.clone(),
            second_arg: self.second_arg.clone(),
            third_arg: self.third_arg.clone()
        };
    }

    fn maven_version(&self) -> Self {
        return Self {
            os_type: self.os_type.clone(),
            os_release: self.os_release.clone(),
            current_user: self.current_user.clone(),
            first_arg: self.first_arg.clone(),
            second_arg: self.second_arg.clone(),
            third_arg: self.third_arg.clone()
        };
    }

    fn python_version(&self) -> Self {
        return Self {
            os_type: self.os_type.clone(),
            os_release: self.os_release.clone(),
            current_user: self.current_user.clone(),
            first_arg: self.first_arg.clone(),
            second_arg: self.second_arg.clone(),
            third_arg: self.third_arg.clone()
        };
    }
}
