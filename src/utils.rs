use std::process::Command;
use std::fs::{File, self};
use std::io::{self, BufRead};

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

pub fn configure_incli_envs_file(user: &String){
    let user_path;

    if user != &"root" {
        user_path = format!("/home/{}", user)
    } else {
        user_path = "/root".to_string()
    }

    let incli_paths_path = format!("{}/INCLI_PATHS", user_path);

    let create_incli_paths_folder = Command::new("sudo")
                                            .arg("mkdir")
                                            .arg(&incli_paths_path)
                                            .output();

    match create_incli_paths_folder {
        Ok(_) => (),
        Err(err) => {
            eprintln!("an error occured when INCLI_PATHS folder about to create: {}", err);
            return;
        }
    }

    let incli_envs_path = format!("{}/incli-envs.sh", incli_paths_path);

    let create_incli_envs_file = Command::new("sudo")
                                            .arg("touch")
                                            .arg(&incli_envs_path)
                                            .output();

    match create_incli_envs_file {
        Ok(_) => (),
        Err(err) => {
            eprintln!("an error occured when creating incli-envs.sh file: {}", err);
            return;
        }
    }

    let give_permission_for_incli_paths = Command::new("sudo")
                                                                    .arg("chmod")
                                                                    .arg("777")
                                                                    .arg("-R")
                                                                    .arg(incli_paths_path)
                                                                    .output();

    match give_permission_for_incli_paths {
        Ok(_) => (),
        Err(err) => {
            eprintln!("an error occured when try to give permission for INCLI_PATHS folder: {}", err);
            return;
        }
    }

    let open_and_type_incli_envs_file = fs::OpenOptions::new().append(true).open(&incli_envs_path);

    match open_and_type_incli_envs_file {
        Ok(mut file) => {
            let incli_envs_greeting_quote1 = "# Hello from incli-envs.sh file. This file contains environment variables that added by Incli program\n";
            let incli_envs_greeting_quote2 = "# If you don't know what that program is, you can learn it via that addresses:\n";
            let incli_envs_greeting_quote3 = "# github.com repo: https://github.com/Necoo33/incli\n";
            let incli_envs_greeting_quote4 = "# crates.io page: https://crates.io/crates/incli\n";

            io::Write::write_all(&mut file, incli_envs_greeting_quote1.as_bytes()).unwrap();
            io::Write::write_all(&mut file, incli_envs_greeting_quote2.as_bytes()).unwrap();
            io::Write::write_all(&mut file, incli_envs_greeting_quote3.as_bytes()).unwrap();
            io::Write::write_all(&mut file, incli_envs_greeting_quote4.as_bytes()).unwrap();
        },
        Err(error) => {
            eprintln!("cannot open incli-envs.sh file for that reason: {}", error);
        }
    }

    let bashrc_path = format!("{}/.bashrc", user_path);

    let give_permission_to_bashrc = Command::new("sudo")
                                                        .arg("chmod")
                                                        .arg("777")
                                                        .arg(&bashrc_path)
                                                        .output()
                                                        .expect("cannot give permission to .bashrc file");

    if !give_permission_to_bashrc.status.success() {
        println!("Cannot give required permissions for .bashrc, you have to add incli-envs.sh file's path on that file via that synthax for adding node.js on your user's env's: \". \"$HOME/INCLI_PATHS/incli-envs.sh\"\"")
    }

    let bashrc_file = fs::OpenOptions::new().append(true).open(bashrc_path);

    match bashrc_file {
        Ok(mut file) => {
            let incli_envs = incli_envs_path;
            let format_incli_envs_bytes = format!(". \"{}\"", incli_envs);

            let add_env_file_dest = io::Write::write_all(&mut file, format_incli_envs_bytes.as_bytes());

            match add_env_file_dest {
                Ok(_) => println!("incli-envs.sh file successfully added on .bashrc file."),
                Err(err) => eprintln!("An Error occured when incli-envs.sh file about to write on .bashrc file: {}", err)
            }
        },
        Err(err) => {
            eprintln!("cannot find .bashrc file in this folder, you have to set env's manually.");
            eprintln!("an error occured: {}", err)
        }
    }
}

