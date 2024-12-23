use crate::utils;
use std::{fs::OpenOptions, io::Read, path::Path};

pub enum ShellType {
    Sh, Bash, Zsh, Fish, Ksh, Csh, Tcsh, Ion, Nushell, Hush, Dash, Ash
}

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
        
                                    let _ = match split_the_line.nth(6) {
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
                                        None => return Err(ShellError::FileMalformed)
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

pub enum ShellError {
    PasswdNotExist, UnableToOpenPasswd(String), UnableToReadPasswd(String), FileMalformed, UnknownShell, UserNotFound
}