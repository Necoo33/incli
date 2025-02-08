use std::process::{exit, Command, ExitStatus, Output};
use std::fs::{File, self};
use std::io::{self, BufRead, BufReader, Error, Read};
use zip::read::ZipArchive;
use sys_info_extended;

use crate::models::{EnvConfiguration, EnvConfigurationOpts, ShellType};

pub fn return_linux_dist_etc_os_release<'a>() -> &'a str {
    if let Ok(file) = File::open("/etc/os-release") {
        let reader = io::BufReader::new(file);
        for line in reader.lines() {
            if let Ok(line) = line {
                if line.contains("PRETTY_NAME") {
                    if line.contains("Ubuntu") {
                        return "ubuntu"
                    }

                    if line.contains("Debian") {
                        return "debian"
                    }

                    if line.contains("Arch Linux") {
                        return "arch wsl"
                    }

                    if line.contains("Kali") {
                        return "kali linux"
                    }

                    if line.contains("Fedora") {
                        return "fedora"
                    }

                    if line.contains("CentOS") {
                        return "centos"                        
                    }

                    if line.contains("Rocky") {
                        return "rocky"                        
                    }

                    if line.contains("Pardus") {
                        return "pardus"
                    }

                    if line.contains("Alma") {
                        return "alma linux"
                    }

                    if line.contains("Alpine Linux") {
                        return "alpine"
                    }
                }
            }
        }
    }

    return "another"
}

pub fn check_if_linux_dist_is_arch_linux() -> bool {
    if let Ok(file) = File::open("/etc/arch-release") {
        let reader = io::BufReader::new(file);
        for line in reader.lines() {
            if let Ok(line) = line {
                if line.contains("Arch Linux") {
                    return true;
                }
            }
        }
    }

    return false;
}

pub fn configure_incli_envs_file(env_confs: &EnvConfiguration, user: &String, run_commands_as_root: bool){
    let incli_paths_path = format!("{}/INCLI_PATHS", env_confs.home_dir);

    let create_incli_paths_folder = match run_commands_as_root {
        true => Command::new("sudo")
                            .arg("mkdir")
                            .arg(&incli_paths_path)
                            .output(),
        false => Command::new("mkdir")
                            .arg(&incli_paths_path)
                            .output()
    };

    match create_incli_paths_folder {
        Ok(_) => (),
        Err(err) => {
            eprintln!("an error occured when INCLI_PATHS folder about to create: {}", err);
            return;
        }
    }

    let incli_envs_path = format!("{}/incli-envs.sh", incli_paths_path);

    let create_incli_envs_file = match run_commands_as_root {
        true => Command::new("sudo")
                            .arg("touch")
                            .arg(&incli_envs_path)
                            .output(),
        false => Command::new("touch")
                            .arg(&incli_envs_path)
                            .output() 
    };

    match create_incli_envs_file {
        Ok(_) => (),
        Err(err) => {
            eprintln!("an error occured when creating incli-envs.sh file: {}", err);
            return;
        }
    }

    let give_permission_for_incli_paths = match run_commands_as_root {
        true => Command::new("sudo")
                            .arg("chmod")
                            .arg("777")
                            .arg("-R")
                            .arg(incli_paths_path)
                            .output(),
        false => Command::new("chmod")
                            .arg("777")
                            .arg("-R")
                            .arg(incli_paths_path)
                            .output()
    };

    match give_permission_for_incli_paths {
        Ok(_) => (),
        Err(err) => {
            eprintln!("an error occured when try to give permission for INCLI_PATHS folder: {}", err);
            return;
        }
    }

    let open_and_type_incli_envs_file = fs::OpenOptions::new().append(true).read(true).open(&incli_envs_path);

    match open_and_type_incli_envs_file {
        Ok(mut file) => {
            let mut buffer = vec![];
            file.read_to_end(&mut buffer).unwrap();
            let quotes = String::from_utf8_lossy(&buffer);

            if !quotes.contains("# Hello from incli-envs.sh file.") {
                let incli_envs_greeting_quote1 = "# Hello from incli-envs.sh file. This file contains environment variables that added by Incli program\n";
                let incli_envs_greeting_quote2 = "# If you don't know what that program is, you can learn it via that addresses:\n";
                let incli_envs_greeting_quote3 = "# github.com repo: https://github.com/Necoo33/incli\n";
                let incli_envs_greeting_quote4 = "# crates.io page: https://crates.io/crates/incli\n";
    
                io::Write::write_all(&mut file, incli_envs_greeting_quote1.as_bytes()).unwrap();
                io::Write::write_all(&mut file, incli_envs_greeting_quote2.as_bytes()).unwrap();
                io::Write::write_all(&mut file, incli_envs_greeting_quote3.as_bytes()).unwrap();
                io::Write::write_all(&mut file, incli_envs_greeting_quote4.as_bytes()).unwrap();
            }
        },
        Err(error) => {
            eprintln!("cannot open incli-envs.sh file for that reason: {}", error);
        }
    }

    let rc_path = match &env_confs.shell {
        EnvConfigurationOpts::Shell(shell) => match shell {
            ShellType::Bash => format!("{}/.bashrc", env_confs.home_dir),
            ShellType::Zsh => format!("{}/.zshrc", env_confs.home_dir),
            _ => {
                println!("Error when we try to open user's bash profile page, your shell is not supported yet.");

                exit(1)
            }
        },
        _ => panic!("You cannot send any env configuration option other then shell to configure_incli_envs.sh file, panicking")
    };

    let give_permission_to_bashrc = match run_commands_as_root {
        true => Command::new("sudo")
                            .arg("chmod")
                            .arg("777")
                            .arg(&rc_path)
                            .output()
                            .expect("cannot give permission to .bashrc file"),
        false => Command::new("chmod")
                            .arg("777")
                            .arg(&rc_path)
                            .output()
                            .expect("cannot give permission to .bashrc file")
    };

    if !give_permission_to_bashrc.status.success() {
        println!("Cannot give required permissions for {}, you have to add incli-envs.sh file's path on that file via that synthax for adding your program on your user's env's: \". \"$HOME/INCLI_PATHS/incli-envs.sh\"\"", rc_path)
    }

    let bashrc_file = fs::OpenOptions::new().append(true).read(true).open(rc_path);

    match bashrc_file {
        Ok(mut file) => {
            let mut buffer = vec![];
            file.read_to_end(&mut buffer).unwrap();
            let quotes = String::from_utf8_lossy(&buffer);

            if !quotes.contains("incli-envs.sh") {
                let incli_envs = incli_envs_path;

                let format_incli_envs_bytes = match &env_confs.shell {
                    EnvConfigurationOpts::Shell(shell) => match shell {
                        ShellType::Bash => format!(". \"{}\"", incli_envs),
                        ShellType::Zsh => format!("\nif [ -f \"{}\" ]; then\n   source \"{}\"\nfi\n", incli_envs, incli_envs),
                        _ => {
                            println!("Error when we try to add incli_envs.sh file to rc file, your shell is not supported yet.");

                            exit(1)
                        }
                    },
                    _ => panic!("You cannot send any env configuration option other then shell to configure_incli_envs.sh file, panicking")
                };
    
                match io::Write::write_all(&mut file, format_incli_envs_bytes.as_bytes()) {
                    Ok(_) => println!("incli-envs.sh file successfully added on .bashrc file."),
                    Err(err) => eprintln!("An Error occured when incli-envs.sh file about to write on .bashrc file: {}", err)
                }
            }
        },
        Err(err) => {
            eprintln!("cannot find .bashrc file in this folder, you have to set env's manually.");
            eprintln!("an error occured: {}", err)
        }
    }
}

pub fn append_env_to_system_path_on_windows(new_path: &str) {
    let system_path = sys_info_extended::get_system_env_var("PATH").unwrap();

    let appended_path = format!("{};{}", system_path, new_path);

    let format_the_command = format!("[System.Environment]::SetEnvironmentVariable('Path', '{}', 'Machine')", appended_path);

    let execute_appending = Command::new("powershell.exe")
                                                        .arg("-Command")
                                                        .arg(format_the_command)
                                                        .output();

    match execute_appending {
        Ok(_) => println!("System's PATH env successfully updated."),
        Err(error) => println!("That Error Occured When we updating the System's PATH Env: {}", error)
    }
}

pub fn unzip(zip_path: &str, dest: &str) -> zip::result::ZipResult<()> {
    let zip_file = File::open(zip_path)?;
    let mut archive = ZipArchive::new(BufReader::new(zip_file))?;

    for i in 0..archive.len() {
        let mut file = archive.by_index(i)?;
        let outpath = std::path::Path::new(dest).join(file.name());

        if file.name().ends_with('/') {
            std::fs::create_dir_all(&outpath)?;
        } else {
            if let Some(p) = outpath.parent() {
                if !p.exists() {
                    std::fs::create_dir_all(p)?;
                }
            }
            let mut outfile = File::create(&outpath)?;
            std::io::copy(&mut file, &mut outfile)?;
        }
    }
    Ok(())
}
