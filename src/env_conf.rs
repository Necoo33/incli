use crate::models;
use crate::utils;
use std::{fs::OpenOptions, io::{Read, Write}, path::Path, process::exit};
use sys_info_extended::{get_home_dir_and_shell, os_type, EnvOptions};

use models::{ShellType, ShellError, EnvConfiguration, EnvConfigurationError, EnvConfigurationOpts, OsType};
use utils::configure_incli_envs_file;

pub fn detect_shell(user: &str) -> std::result::Result<ShellType, ShellError> {
    let etc_passwd = Path::new("/etc/passwd");

    match etc_passwd.exists() {
        false => Err(ShellError::PasswdNotExist),
        true => {
            let open_passwd = OpenOptions::new().read(true).open(etc_passwd);

            match open_passwd {
                Ok(mut passwd) => {
                    let buffer = &mut Default::default();

                    match passwd.read_to_string(buffer) {
                        Ok(_) => {
                            for line in buffer.lines() {
                                if line.starts_with(user) {
                                    let mut split_the_line = line.split(":");
        
                                    return match split_the_line.nth(6) {
                                        Some(shell_path) => match shell_path {
                                            "/bin/bash" => Ok(ShellType::Bash),
                                            "/usr/bin/bash" => Ok(ShellType::Bash),
                                            "/bin/zsh" => Ok(ShellType::Zsh),
                                            "/usr/bin/zsh" => Ok(ShellType::Zsh),
                                            "/bin/fish" => Ok(ShellType::Fish),
                                            "/usr/bin/fish" => Ok(ShellType::Fish),
                                            "/usr/local/bin/fish" => Ok(ShellType::Fish),
                                            "/bin/sh" => Ok(ShellType::Sh),
                                            "/usr/bin/sh" => Ok(ShellType::Sh),
                                            "/usr/bin/ksh" => Ok(ShellType::Ksh),
                                            "/bin/ksh" => Ok(ShellType::Ksh),
                                            "/usr/bin/csh" => Ok(ShellType::Csh),
                                            "/bin/csh" => Ok(ShellType::Csh),
                                            "/usr/bin/tcsh" => Ok(ShellType::Tcsh),
                                            "/bin/tcsh" => Ok(ShellType::Tcsh),
                                            _ => Err(ShellError::UnknownShell)
                                        },
                                        None => Err(ShellError::FileMalformed)
                                    };
                                };
        
                                continue;
                            }
                        },
                        Err(error) => return Err(ShellError::UnableToReadPasswd(error.to_string()))
                    }

                    Err(ShellError::UserNotFound)
                },
                Err(error) => Err(ShellError::UnableToOpenPasswd(error.to_string()))
            }
        }
    }
}



impl EnvConfiguration {
    pub fn init(username: &String, os_type: OsType) -> Self {
        let home_dir = match os_type {
            OsType::Windows => EnvConfigurationOpts::Initial,
            _ => match get_home_dir_and_shell(&username) {
                Ok(confs) => EnvConfigurationOpts::Specific(confs.home_dir),
                Err(_) => panic!("Your env configurations either wrong or unusual, panicking.")
            } 
        };

        Self {
            os: os_type,
            home_dir,
            shell: match detect_shell(&username) {
                Ok(shell) => EnvConfigurationOpts::Shell(shell),
                Err(error) => panic!("Shell error: {}", error)
            }
        }
    }

    pub fn windows_configure_path_var(&self, env_opts: EnvOptions) {
        match sys_info_extended::append_env(env_opts) {
            Ok(_) => println!("You successfully append your value to path env on windows."),
            Err(error) => {
                println!("That error occured when we try to append your env: {}", error);

                exit(1)
            }
        }
    }

    pub fn windows_configure_another_env(&self, env_opts: EnvOptions) {
        match sys_info_extended::set_env(env_opts) {
            Ok(_) => println!("You successfully set your env."),
            Err(error) => {
                println!("That error occured when we try to append your env: {}", error);

                exit(1)
            }
        }
    }

    pub fn configure_debian_path_var(&self, value: &str) -> Result<(), EnvConfigurationError> {
        match &self.shell {
            EnvConfigurationOpts::Shell(shell) => match shell {
                ShellType::Bash => {
                    match OpenOptions::new().append(true).open(format!("{}/.bashrc", self.home_dir)) {
                        Ok(mut bashrc_file) => {
                            let format_value = format!("\nexport PATH=\"{}:$PATH\"", value);

                            let add_env = Write::write_all(&mut bashrc_file, format_value.as_bytes());
                    
                            match add_env {
                                Ok(_) => Ok(()),
                                Err(error) => Err(EnvConfigurationError::UnableToWriteUserShellFile(error.to_string()))
                            }
                        },
                        Err(error) => Err(EnvConfigurationError::UnableToOpenUserShellFile(error.to_string()))
                    }
                },
                ShellType::Zsh => {
                    match OpenOptions::new().append(true).open(format!("{}/.zshrc", self.home_dir)) {
                        Ok(mut bashrc_file) => {
                            let format_value = format!("\nexport PATH=\"{}:$PATH\"", value);

                            let add_env = Write::write_all(&mut bashrc_file, format_value.as_bytes());
                    
                            match add_env {
                                Ok(_) => Ok(()),
                                Err(error) => Err(EnvConfigurationError::UnableToWriteUserShellFile(error.to_string()))
                            }
                        },
                        Err(error) => Err(EnvConfigurationError::UnableToOpenUserShellFile(error.to_string()))
                    }
                },
                _ => Err(EnvConfigurationError::NotConfigured)
            },
            EnvConfigurationOpts::Initial => Err(EnvConfigurationError::InvalidValueToPass),
            EnvConfigurationOpts::Specific(_) => Err(EnvConfigurationError::InvalidValueToPass)

        }
    }

    pub fn add_debian_env_var(&self, name: &str, value: &str) -> Result<(), EnvConfigurationError> {
        match &self.shell {
            EnvConfigurationOpts::Shell(shell) => match shell {
                ShellType::Bash => {
                    match OpenOptions::new().append(true).open(format!("{}/.bashrc", self.home_dir)) {
                        Ok(mut bashrc_file) => {
                            let format_value = format!("\nexport {}=\"{}\"", name, value);

                            let add_env = Write::write_all(&mut bashrc_file, format_value.as_bytes());
                    
                            match add_env {
                                Ok(_) => Ok(()),
                                Err(error) => Err(EnvConfigurationError::UnableToWriteUserShellFile(error.to_string()))
                            }
                        },
                        Err(error) => Err(EnvConfigurationError::UnableToOpenUserShellFile(error.to_string()))
                    }
                },
                ShellType::Zsh => {
                    match OpenOptions::new().append(true).open(format!("{}/.zshrc", self.home_dir)) {
                        Ok(mut rc_file) => {
                            let format_value = format!("\nexport {}=\"{}\"", name, value);

                            let add_env = Write::write_all(&mut rc_file, format_value.as_bytes());
                    
                            match add_env {
                                Ok(_) => Ok(()),
                                Err(error) => Err(EnvConfigurationError::UnableToWriteUserShellFile(error.to_string()))
                            }
                        },
                        Err(error) => Err(EnvConfigurationError::UnableToOpenUserShellFile(error.to_string()))
                    }
                },
                _ => Err(EnvConfigurationError::NotConfigured)
            },
            EnvConfigurationOpts::Initial => Err(EnvConfigurationError::InvalidValueToPass),
            EnvConfigurationOpts::Specific(_) => Err(EnvConfigurationError::InvalidValueToPass)

        }
    }

    pub fn configure_arch_linux_path_var(&self, name: &str, value: &str) -> Result<(), EnvConfigurationError> {
        let incli_paths_path = format!("{}/INCLI_PATHS", self.home_dir);

        let check_if_incli_paths_exist = Path::new(&incli_paths_path);
        
        match &self.shell {
            EnvConfigurationOpts::Shell(shell) => {
                if matches!(shell, ShellType::Bash | ShellType::Zsh) {
                    if !check_if_incli_paths_exist.exists() {
                        utils::configure_incli_envs_file(self, &name.to_string(), true)
                    }
            
                    let incli_envs_path = format!("{}/incli-envs.sh", incli_paths_path);
            
                    match OpenOptions::new().append(true).open(incli_envs_path) {
                        Ok(mut file) => {
                            let line_for_append = format!("\nexport PATH=\"{}:$PATH\"\n", value);
                            let line_for_append = line_for_append.as_bytes();
                        
                            let add_env_file_dest = Write::write_all(&mut file, line_for_append);
                
                            match add_env_file_dest {
                                Ok(_) => {
                                    println!("envs successfully added on your user.");

                                    Ok(())
                                },
                                Err(err) => {
                                    eprintln!("This error occured when we try to add your path value to incli-envs.sh file: {}", err);

                                    Err(EnvConfigurationError::UnableToWriteUserShellFile(err.to_string()))
                                }
                            }
                        },
                        Err(error) => {
                            println!("That error occured when we try to open 'incli_envs.sh' file: {}", error);

                            Err(EnvConfigurationError::UnableToOpenUserShellFile(error.to_string()))
                        }
                    }
                } else {
                    println!("The shell you were using is not supported.");

                    Err(EnvConfigurationError::NotConfigured)
                }
            },
            EnvConfigurationOpts::Initial => Err(EnvConfigurationError::InvalidValueToPass),
            EnvConfigurationOpts::Specific(_) => Err(EnvConfigurationError::InvalidValueToPass)
        }
    }

    pub fn add_arch_linux_env_var(&self, name: &str, value: &str) -> Result<(), EnvConfigurationError> {
        let incli_paths_path = format!("{}/INCLI_PATHS", self.home_dir);

        let check_if_incli_paths_exist = Path::new(&incli_paths_path);
        
        match &self.shell {
            EnvConfigurationOpts::Shell(shell) => {
                if matches!(shell, ShellType::Bash | ShellType::Zsh) {
                    if !check_if_incli_paths_exist.exists() {
                        utils::configure_incli_envs_file(self, &name.to_string(), true)
                    }
            
                    let incli_envs_path = format!("{}/incli-envs.sh", incli_paths_path);
            
                    match OpenOptions::new().append(true).open(incli_envs_path) {
                        Ok(mut file) => {
                            let line_for_append = format!("\nexport {}=\"{}\"\n", name, value);
                            let line_for_append = line_for_append.as_bytes();
                        
                            let add_env_file_dest = Write::write_all(&mut file, line_for_append);
                
                            match add_env_file_dest {
                                Ok(_) => {
                                    println!("envs successfully added on your user.");

                                    Ok(())
                                },
                                Err(err) => {
                                    eprintln!("This error occured when we try to add your path value to incli-envs.sh file: {}", err);

                                    Err(EnvConfigurationError::UnableToWriteUserShellFile(err.to_string()))
                                }
                            }
                        },
                        Err(error) => {
                            println!("That error occured when we try to open 'incli_envs.sh' file: {}", error);

                            Err(EnvConfigurationError::UnableToOpenUserShellFile(error.to_string()))
                        }
                    }
                } else {
                    println!("The shell you were using is not supported.");

                    Err(EnvConfigurationError::NotConfigured)
                }
            },
            EnvConfigurationOpts::Initial => Err(EnvConfigurationError::InvalidValueToPass),
            EnvConfigurationOpts::Specific(_) => Err(EnvConfigurationError::InvalidValueToPass)
        }
    }

    pub fn configure_alma_linux_path_var(&self, name: &str, value: &str) -> Result<(), EnvConfigurationError> {
        let incli_paths_path = format!("{}/INCLI_PATHS", self.home_dir);

        let check_if_incli_paths_exist = Path::new(&incli_paths_path);
        
        match &self.shell {
            EnvConfigurationOpts::Shell(shell) => {
                if matches!(shell, ShellType::Bash | ShellType::Zsh) {
                    if !check_if_incli_paths_exist.exists() {
                        utils::configure_incli_envs_file(self, &name.to_string(), true)
                    }
            
                    let incli_envs_path = format!("{}/incli-envs.sh", incli_paths_path);
            
                    match OpenOptions::new().append(true).open(incli_envs_path) {
                        Ok(mut file) => {
                            let line_for_append = format!("\nPATH=\"{}:$PATH\"\n", value);
                            let line_for_append = line_for_append.as_bytes();
                        
                            let add_env_file_dest = Write::write_all(&mut file, line_for_append);
                
                            match add_env_file_dest {
                                Ok(_) => {
                                    println!("envs successfully added on your user.");

                                    Ok(())
                                },
                                Err(err) => {
                                    eprintln!("This error occured when we try to add your path value to incli-envs.sh file: {}", err);

                                    Err(EnvConfigurationError::UnableToWriteUserShellFile(err.to_string()))
                                }
                            }
                        },
                        Err(error) => {
                            println!("That error occured when we try to open 'incli_envs.sh' file: {}", error);

                            Err(EnvConfigurationError::UnableToOpenUserShellFile(error.to_string()))
                        }
                    }
                } else {
                    println!("The shell you were using is not supported.");

                    Err(EnvConfigurationError::NotConfigured)
                }
            },
            EnvConfigurationOpts::Initial => Err(EnvConfigurationError::InvalidValueToPass),
            EnvConfigurationOpts::Specific(_) => Err(EnvConfigurationError::InvalidValueToPass)
        }
    }

    pub fn add_alma_linux_env_var(&self, name: &str, value: &str) -> Result<(), EnvConfigurationError> {
        let incli_paths_path = format!("{}/INCLI_PATHS", self.home_dir);

        let check_if_incli_paths_exist = Path::new(&incli_paths_path);
        
        match &self.shell {
            EnvConfigurationOpts::Shell(shell) => {
                if matches!(shell, ShellType::Bash | ShellType::Zsh) {
                    if !check_if_incli_paths_exist.exists() {
                        utils::configure_incli_envs_file(self, &name.to_string(), true)
                    }
            
                    let incli_envs_path = format!("{}/incli-envs.sh", incli_paths_path);
            
                    match OpenOptions::new().append(true).open(incli_envs_path) {
                        Ok(mut file) => {
                            let line_for_append = format!("\n{}=\"{}\"\n", name, value);
                            let line_for_append = line_for_append.as_bytes();
                        
                            let add_env_file_dest = Write::write_all(&mut file, line_for_append);
                
                            match add_env_file_dest {
                                Ok(_) => {
                                    println!("envs successfully added on your user.");

                                    Ok(())
                                },
                                Err(err) => {
                                    eprintln!("This error occured when we try to add your path value to incli-envs.sh file: {}", err);

                                    Err(EnvConfigurationError::UnableToWriteUserShellFile(err.to_string()))
                                }
                            }
                        },
                        Err(error) => {
                            println!("That error occured when we try to open 'incli_envs.sh' file: {}", error);

                            Err(EnvConfigurationError::UnableToOpenUserShellFile(error.to_string()))
                        }
                    }
                } else {
                    println!("The shell you were using is not supported.");

                    Err(EnvConfigurationError::NotConfigured)
                }
            },
            EnvConfigurationOpts::Initial => Err(EnvConfigurationError::InvalidValueToPass),
            EnvConfigurationOpts::Specific(_) => Err(EnvConfigurationError::InvalidValueToPass)
        }
    }

    pub fn configure_centos_and_fedora_path_var(&self, name: &str, value: &str) -> Result<(), EnvConfigurationError> {
        let incli_paths_path = format!("{}/INCLI_PATHS", self.home_dir);

        let check_if_incli_paths_exist = Path::new(&incli_paths_path);
        
        match &self.shell {
            EnvConfigurationOpts::Shell(shell) => {
                if matches!(shell, ShellType::Bash | ShellType::Zsh) {
                    if !check_if_incli_paths_exist.exists() {
                        utils::configure_incli_envs_file(self, &name.to_string(), true)
                    }
            
                    let incli_envs_path = format!("{}/incli-envs.sh", incli_paths_path);
            
                    match OpenOptions::new().append(true).open(incli_envs_path) {
                        Ok(mut file) => {
                            let line_for_append = format!("\nexport PATH=\"{}:$PATH\"\n", value);
                            let line_for_append = line_for_append.as_bytes();
                        
                            let add_env_file_dest = Write::write_all(&mut file, line_for_append);
                
                            match add_env_file_dest {
                                Ok(_) => {
                                    println!("envs successfully added on your user.");

                                    Ok(())
                                },
                                Err(err) => {
                                    eprintln!("This error occured when we try to add your path value to incli-envs.sh file: {}", err);

                                    Err(EnvConfigurationError::UnableToWriteUserShellFile(err.to_string()))
                                }
                            }
                        },
                        Err(error) => {
                            println!("That error occured when we try to open 'incli_envs.sh' file: {}", error);

                            Err(EnvConfigurationError::UnableToOpenUserShellFile(error.to_string()))
                        }
                    }
                } else {
                    println!("The shell you were using is not supported.");

                    Err(EnvConfigurationError::NotConfigured)
                }
            },
            EnvConfigurationOpts::Initial => Err(EnvConfigurationError::InvalidValueToPass),
            EnvConfigurationOpts::Specific(_) => Err(EnvConfigurationError::InvalidValueToPass)
        }
    }

    pub fn add_centos_and_fedora_env_var(&self, name: &str, value: &str) -> Result<(), EnvConfigurationError> {
        let incli_paths_path = format!("{}/INCLI_PATHS", self.home_dir);

        let check_if_incli_paths_exist = Path::new(&incli_paths_path);
        
        match &self.shell {
            EnvConfigurationOpts::Shell(shell) => {
                if matches!(shell, ShellType::Bash | ShellType::Zsh) {
                    if !check_if_incli_paths_exist.exists() {
                        utils::configure_incli_envs_file(self, &name.to_string(), true)
                    }
            
                    let incli_envs_path = format!("{}/incli-envs.sh", incli_paths_path);
            
                    match OpenOptions::new().append(true).open(incli_envs_path) {
                        Ok(mut file) => {
                            let line_for_append = format!("\nexport {}=\"{}\"\n", name, value);
                            let line_for_append = line_for_append.as_bytes();
                        
                            let add_env_file_dest = Write::write_all(&mut file, line_for_append);
                
                            match add_env_file_dest {
                                Ok(_) => {
                                    println!("envs successfully added on your user.");

                                    Ok(())
                                },
                                Err(err) => {
                                    eprintln!("This error occured when we try to add your path value to incli-envs.sh file: {}", err);

                                    Err(EnvConfigurationError::UnableToWriteUserShellFile(err.to_string()))
                                }
                            }
                        },
                        Err(error) => {
                            println!("That error occured when we try to open 'incli_envs.sh' file: {}", error);

                            Err(EnvConfigurationError::UnableToOpenUserShellFile(error.to_string()))
                        }
                    }
                } else {
                    println!("The shell you were using is not supported.");

                    Err(EnvConfigurationError::NotConfigured)
                }
            },
            EnvConfigurationOpts::Initial => Err(EnvConfigurationError::InvalidValueToPass),
            EnvConfigurationOpts::Specific(_) => Err(EnvConfigurationError::InvalidValueToPass)
        }
    }

    pub fn configure_rocky_linux_path_var(&self, name: &str, value: &str) -> Result<(), EnvConfigurationError> {
        let incli_paths_path = format!("{}/INCLI_PATHS", self.home_dir);

        let check_if_incli_paths_exist = Path::new(&incli_paths_path);
        
        match &self.shell {
            EnvConfigurationOpts::Shell(shell) => {
                if matches!(shell, ShellType::Bash | ShellType::Zsh) {
                    if !check_if_incli_paths_exist.exists() {
                        utils::configure_incli_envs_file(self, &name.to_string(), true)
                    }
            
                    let incli_envs_path = format!("{}/incli-envs.sh", incli_paths_path);
            
                    match OpenOptions::new().append(true).open(incli_envs_path) {
                        Ok(mut file) => {
                            let line_for_append = format!("\nexport PATH=\"$PATH:{}\"\n", value);
                            let line_for_append = line_for_append.as_bytes();
                        
                            let add_env_file_dest = Write::write_all(&mut file, line_for_append);
                
                            match add_env_file_dest {
                                Ok(_) => {
                                    println!("envs successfully added on your user.");

                                    Ok(())
                                },
                                Err(err) => {
                                    eprintln!("This error occured when we try to add your path value to incli-envs.sh file: {}", err);

                                    Err(EnvConfigurationError::UnableToWriteUserShellFile(err.to_string()))
                                }
                            }
                        },
                        Err(error) => {
                            println!("That error occured when we try to open 'incli_envs.sh' file: {}", error);

                            Err(EnvConfigurationError::UnableToOpenUserShellFile(error.to_string()))
                        }
                    }
                } else {
                    println!("The shell you were using is not supported.");

                    Err(EnvConfigurationError::NotConfigured)
                }
            },
            EnvConfigurationOpts::Initial => Err(EnvConfigurationError::InvalidValueToPass),
            EnvConfigurationOpts::Specific(_) => Err(EnvConfigurationError::InvalidValueToPass)
        }
    }

    pub fn add_rocky_linux_env_var(&self, name: &str, value: &str) -> Result<(), EnvConfigurationError> {
        let incli_paths_path = format!("{}/INCLI_PATHS", self.home_dir);

        let check_if_incli_paths_exist = Path::new(&incli_paths_path);
        
        match &self.shell {
            EnvConfigurationOpts::Shell(shell) => {
                if matches!(shell, ShellType::Bash | ShellType::Zsh) {
                    if !check_if_incli_paths_exist.exists() {
                        utils::configure_incli_envs_file(self, &name.to_string(), true)
                    }
            
                    let incli_envs_path = format!("{}/incli-envs.sh", incli_paths_path);
            
                    match OpenOptions::new().append(true).open(incli_envs_path) {
                        Ok(mut file) => {
                            let line_for_append = format!("\nexport {}=\"{}\"\n", name, value);
                            let line_for_append = line_for_append.as_bytes();
                        
                            let add_env_file_dest = Write::write_all(&mut file, line_for_append);
                
                            match add_env_file_dest {
                                Ok(_) => {
                                    println!("envs successfully added on your user.");

                                    Ok(())
                                },
                                Err(err) => {
                                    eprintln!("This error occured when we try to add your path value to incli-envs.sh file: {}", err);

                                    Err(EnvConfigurationError::UnableToWriteUserShellFile(err.to_string()))
                                }
                            }
                        },
                        Err(error) => {
                            println!("That error occured when we try to open 'incli_envs.sh' file: {}", error);

                            Err(EnvConfigurationError::UnableToOpenUserShellFile(error.to_string()))
                        }
                    }
                } else {
                    println!("The shell you were using is not supported.");

                    Err(EnvConfigurationError::NotConfigured)
                }
            },
            EnvConfigurationOpts::Initial => Err(EnvConfigurationError::InvalidValueToPass),
            EnvConfigurationOpts::Specific(_) => Err(EnvConfigurationError::InvalidValueToPass)
        }
    }
}


