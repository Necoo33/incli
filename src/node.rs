use crate::models::EnvConfiguration;
use crate::models::EnvConfigurationError;
use crate::utils;
use crate::models;
use crate::env_conf;
use std::process::{Command, exit};
use std::fs;
use std::io;
use std::path::Path;
use sys_info_extended::get_current_user;

// Node.js Functions ----------------------------------------------------------

// url: "https://nodejs.org/dist/v20.10.0/node-v20.10.0-linux-x64.tar.xz"
// tar.gz: node-v20.10.0-linux-x64.tar.xz

// i don't know why but in arch linux we can't give 755 permissions.

pub fn install_nodejs_on_debian_based_distros(env_confs: &models::EnvConfiguration, url: &str, file_name: &str) {
    println!("Welcome to incli. Your request to install Node.js on Linux Reached.");
    println!("Be sure you have wget and xz-utils installed if you use debian and kali linux, otherwise this installation won't work.");

    let current_user = get_current_user();

    let slice_of_file_name = &file_name[0..file_name.len() - 7];

    let install_nodejs = Command::new("wget")
                                    .arg(url)
                                    .arg("-O")
                                    .arg(file_name)
                                    .output()
                                    .expect("Some Error Happened");

    let mut env_path;

    match current_user.as_str() {
        "root" => {
            if !install_nodejs.status.success() {
                println!("Couldn't install Node.js Source Files Because Of Whatever reason.");
                exit(1);
            }
    
            let get_current_file_command = Command::new("pwd").output().unwrap();
    
            let file_for_moving = format!("{}/{}", std::str::from_utf8(&get_current_file_command.stdout).unwrap().trim(), file_name);
    
            Command::new("mv").arg(file_for_moving).arg("/root").output().unwrap();
    
            let file_path = format!("/root/{}", file_name);
    
            Command::new("sudo")
                        .arg("chmod")
                        .arg("755")
                        .arg(&file_path)
                        .output()
                        .expect("couldn't give 755 permission to source code.");
    
            println!("Source Files Downloaded Successfully");
    
            let extract_tar_file = Command::new("sudo")
                                                .arg("tar")
                                                .arg("xvf")
                                                .arg(&file_path)
                                                .output();
    
            match extract_tar_file {
                Ok(_) => println!("source files successfully extracted, trying to add it on env's..."),
                Err(error) => {
                    eprintln!("Cannot extracted source files for this reason: {}", error);
                    exit(1)
                }
            }

            Command::new("sudo")
                        .arg("rm")
                        .arg("rf")
                        .arg(file_path)
                        .output()
                        .expect("cannot delete archive");
    
            let current_folder_again = Command::new("pwd").output().unwrap();
    
            let format_current_folder_again = format!("{}/{}", std::str::from_utf8(&current_folder_again.stdout).unwrap().trim(), slice_of_file_name);
    
            Command::new("sudo")
                        .arg("mv")
                        .arg(format_current_folder_again)
                        .arg("/root")
                        .output()
                        .unwrap();
    
            let new_path = format!("/root/{}", slice_of_file_name);
    
            env_path = format!("{}/bin", new_path);
        },
        &_ => {
            if !install_nodejs.status.success() {
                println!("Couldn't download Node.js Source Files Because Of Whatever reason.");
                exit(1);
            }
    
            let get_current_file_command = Command::new("pwd").output().unwrap();
    
            let file_for_moving = format!("{}/{}", std::str::from_utf8(&get_current_file_command.stdout).unwrap().trim(), file_name);
    
            Command::new("mv").arg(&file_for_moving).arg(env_confs.home_dir.to_string()).output().unwrap();
    
            let file_path = format!("{}/{}", env_confs.home_dir, file_name);
    
            Command::new("sudo")
                        .arg("chmod")
                        .arg("777")
                        .arg(&file_path)
                        .output()
                        .expect("couldn't give 755 permission to source code.");
    
            println!("Source Files Downloaded Successfully");
    
            println!("your file_for_moving: {}", file_for_moving);
            println!("your file_path: {}", file_path);
    
            let extract_tar_file = Command::new("sudo")
                                                .arg("tar")
                                                .arg("xvf")
                                                .arg(&file_path)
                                                .output();
    
            match extract_tar_file {
                Ok(_) => println!("source files successfully extracted, trying to add it on env's..."),
                Err(error) => {
                    eprintln!("Cannot extracted source files for this reason: {}", error);
                    exit(1)
                }
            }

            Command::new("sudo")
                        .arg("rm")
                        .arg("-rf")
                        .arg(file_path)
                        .output()
                        .expect("cannot delete archive");
    
            let current_folder_again = Command::new("pwd").output().unwrap();
    
            let format_current_folder_again = format!("{}/{}", std::str::from_utf8(&current_folder_again.stdout).unwrap().trim(), slice_of_file_name);
    
            Command::new("sudo")
                        .arg("chmod")
                        .arg("777")
                        .arg("-R")
                        .arg(&format_current_folder_again)
                        .output()
                        .unwrap();

            Command::new("sudo")
                        .arg("mv")
                        .arg(format_current_folder_again)
                        .arg(env_confs.home_dir.to_string())
                        .output()
                        .unwrap();
    
            let new_path = format!("{}/{}", env_confs.home_dir, slice_of_file_name);
    
            env_path = format!("{}/bin", new_path);
        }
    }

    match env_confs.configure_debian_path_var(&env_path) {
        Ok(_) => {
            println!("Envs's successfully configured for node.js.");
            
            exit(0)
        },
        Err(error) => {
            match error {
                EnvConfigurationError::UnableToOpenUserShellFile(err) => println!("That error occured when we try to set your env's: {}", err),
                EnvConfigurationError::UnableToWriteUserShellFile(err) => println!("That error occured when we try to set your env's: {}", err),
                EnvConfigurationError::UnableToOpenSystemShellFile(err) => println!("That error occured when we try to set your env's: {}", err),
                EnvConfigurationError::UnableToWriteSystemShellFile(err) => println!("That error occured when we try to set your env's: {}", err),
                _ => println!("There is a bug!")
            }

            exit(1)
        }
    }
}

pub fn install_nodejs_on_arch_linux(env_confs: &models::EnvConfiguration, url: &str, file_name: &str){
    println!("Welcome to incli. Your request to install Node.js on Arch Linux Reached.");

    let current_user = get_current_user();
    let env_path;

    let slice_of_file_name = &file_name[0..file_name.len() - 7];

    let install_nodejs = Command::new("wget")
                                                .arg(url)
                                                .arg("-O")
                                                .arg(file_name)
                                                .output()
                                                .expect("Some Error Happened");

    if !install_nodejs.status.success() {
        println!("Couldn't install Node.js Source Files Because Of Whatever reason.");
        exit(1);
    }

    Command::new("sudo")
                .arg("chmod")
                .arg("755")
                .arg(file_name)
                .output()
                .expect("couldn't give 755 permission to source code.");

    match current_user.as_str() {
        "root" => {
            let current_folder_path = Command::new("pwd").output().unwrap();
            let current_folder_path = std::str::from_utf8(&current_folder_path.stdout).unwrap().trim();

            let format_the_whole_file_path = format!("{}/{}", current_folder_path, file_name);

            Command::new("sudo")
                        .arg("chmod")
                        .arg("777")
                        .arg(&format_the_whole_file_path)
                        .output()
                        .unwrap();

            let extract_the_archive = Command::new("sudo")
                                                            .arg("tar")
                                                            .arg("xvf")
                                                            .arg(&format_the_whole_file_path)
                                                            .output();

            match extract_the_archive {
                Ok(_) => println!("archive file successfully extracted"),
                Err(error) => {
                    println!("that error occured when extracting archive file: {}", error);
                    exit(1)
                }
            }

            Command::new("sudo")
                        .arg("rm")
                        .arg("-rf")
                        .arg(format_the_whole_file_path)
                        .output()
                        .unwrap();

            let format_the_source_files_path = format!("{}/{}", current_folder_path, slice_of_file_name);

            let move_the_source_files_to_root = Command::new("sudo")
                                                                                    .arg("mv")
                                                                                    .arg(format_the_source_files_path)
                                                                                    .arg("/root")
                                                                                    .output();

            match move_the_source_files_to_root {
                Ok(_) => (),
                Err(error) => {
                    eprintln!("That error occured when moving source files: {}", error);
                    exit(1)
                }
            }

            env_path = format!("/root/{}/bin", slice_of_file_name)
        },
        &_ => {
            let current_folder_path = Command::new("pwd").output().unwrap();
            let current_folder_path = std::str::from_utf8(&current_folder_path.stdout).unwrap().trim();

            let format_the_whole_file_path = format!("{}/{}", current_folder_path, file_name);

            println!("your whole file path: {}", format_the_whole_file_path);

            Command::new("sudo")
                        .arg("chmod")
                        .arg("777")
                        .arg(&format_the_whole_file_path)
                        .output()
                        .unwrap();

            let extract_the_archive = Command::new("sudo")
                                                            .arg("tar")
                                                            .arg("xvf")
                                                            .arg(&format_the_whole_file_path)
                                                            .output();

            match extract_the_archive {
                Ok(_) => println!("archive file successfully extracted"),
                Err(error) => {
                    println!("that error occured when extracting archive file: {}", error);
                    exit(1)
                }
            }

            Command::new("sudo")
                        .arg("rm")
                        .arg("-rf")
                        .arg(format_the_whole_file_path)
                        .output()
                        .unwrap();

            let format_the_source_files_path = format!("{}/{}", current_folder_path, slice_of_file_name);

            println!("source files path: {}", format_the_source_files_path);

            let give_permissions_to_source_files = Command::new("sudo")
                                                                                    .arg("chmod")
                                                                                    .arg("777")
                                                                                    .arg("-R")
                                                                                    .arg(&format_the_source_files_path)
                                                                                    .output();

            match give_permissions_to_source_files {
                Ok(_) => (),
                Err(error) => {
                    println!("that error occured when giving permission to source files: {}", error);
                    exit(1)
                }
            }

            let move_the_source_files_to_root = Command::new("sudo")
                                                                                    .arg("mv")
                                                                                    .arg(format_the_source_files_path)
                                                                                    .arg(&env_confs.home_dir.to_string())
                                                                                    .output();

            match move_the_source_files_to_root {
                Ok(_) => (),
                Err(error) => {
                    eprintln!("That error occured when moving source files: {}", error);
                    exit(1)
                }
            }

            env_path = format!("{}/{}/bin", env_confs.home_dir.to_string(), slice_of_file_name)
        }
    }

    println!("Source Files Downloaded Successfully");

    match env_confs.configure_arch_linux_path_var(&current_user, &env_path) {
        Ok(_) => {
            println!("Envs's successfully configured for node.js.");
            
            exit(0)
        },
        Err(error) => {
            match error {
                EnvConfigurationError::UnableToOpenUserShellFile(err) => println!("That error occured when we try to set your env's: {}", err),
                EnvConfigurationError::UnableToWriteUserShellFile(err) => println!("That error occured when we try to set your env's: {}", err),
                EnvConfigurationError::UnableToOpenSystemShellFile(err) => println!("That error occured when we try to set your env's: {}", err),
                EnvConfigurationError::UnableToWriteSystemShellFile(err) => println!("That error occured when we try to set your env's: {}", err),
                _ => println!("There is a bug!")
            }

            exit(1)
        }
    }    
}

pub fn install_nodejs_on_alma_linux(env_confs: &models::EnvConfiguration, url: &str, file_name: &str){
    println!("Welcome to incli. Your request to install Node.js on Alma Linux Reached.");
    println!("Be sure you have installed xz-utils in your pc, otherwise installation won't work.");

    let current_user = get_current_user();
    let env_path;

    let slice_of_file_name = &file_name[0..file_name.len() - 7];

    let install_nodejs = Command::new("wget")
                                    .arg(url)
                                    .arg("-O")
                                    .arg(file_name)
                                    .output()
                                    .expect("Some Error Happened");

    if !install_nodejs.status.success() {
        println!("Couldn't install Node.js Source Files Because Of Whatever reason.");
        exit(1);
    }

    match current_user.as_str() {
        "root" => {
            let current_folder_path = Command::new("pwd").output().unwrap();
            let current_folder_path = std::str::from_utf8(&current_folder_path.stdout).unwrap().trim();

            let format_the_whole_file_path = format!("{}/{}", current_folder_path, file_name);

            Command::new("sudo")
                        .arg("chmod")
                        .arg("755")
                        .arg(&format_the_whole_file_path)
                        .output()
                        .expect("couldn't give 755 permission to source code.");

            let extract_the_archive = Command::new("sudo")
                                                .arg("tar")
                                                .arg("xvf")
                                                .arg(&format_the_whole_file_path)
                                                .output();

            match extract_the_archive {
                Ok(_) => println!("archive file successfully extracted"),
                Err(error) => {
                    println!("that error occured when extracting archive file: {}", error);
                    exit(1)
                }
            }

            Command::new("sudo")
                        .arg("rm")
                        .arg("-rf")
                        .arg(format_the_whole_file_path)
                        .output()
                        .unwrap();

            let source_files_path = format!("{}/{}", current_folder_path, slice_of_file_name);

            let move_the_source_files = Command::new("sudo")
                                                                        .arg("mv")
                                                                        .arg(source_files_path)
                                                                        .arg(&env_confs.home_dir.to_string())
                                                                        .output();

            match move_the_source_files {
                Ok(_) => (),
                Err(error) => {
                    eprintln!("cannot move the source file for this reason: {}", error);
                    exit(1)
                }
            }

            env_path = format!("/root/{}/bin", slice_of_file_name);
        },
        &_ => {
            let current_folder_path = Command::new("pwd").output().unwrap();
            let current_folder_path = std::str::from_utf8(&current_folder_path.stdout).unwrap().trim();

            let format_the_whole_file_path = format!("{}/{}", current_folder_path, file_name);

            Command::new("sudo")
                        .arg("chmod")
                        .arg("755")
                        .arg(&format_the_whole_file_path)
                        .output()
                        .expect("couldn't give 755 permission to source code.");

            let extract_the_archive = Command::new("sudo")
                                                                        .arg("tar")
                                                                        .arg("xvf")
                                                                        .arg(&format_the_whole_file_path)
                                                                        .output();

            match extract_the_archive {
                Ok(_) => println!("archive file successfully extracted"),
                Err(error) => {
                    println!("that error occured when extracting archive file: {}", error);
                    exit(1)
                }
            }

            Command::new("sudo")
                        .arg("rm")
                        .arg("-rf")
                        .arg(format_the_whole_file_path)
                        .output()
                        .unwrap();

            let source_files_path = format!("{}/{}", current_folder_path, slice_of_file_name);

            let move_the_source_files = Command::new("sudo")
                                                                        .arg("mv")
                                                                        .arg(source_files_path)
                                                                        .arg(&env_confs.home_dir.to_string())
                                                                        .output();

            match move_the_source_files {
                Ok(_) => (),
                Err(error) => {
                    eprintln!("cannot move the source file for this reason: {}", error);
                    exit(1)
                }
            }

            env_path = format!("{}/{}/bin", env_confs.home_dir.to_string(), slice_of_file_name)
        }
    }

    println!("Source Files Downloaded Successfully");

    match env_confs.configure_alma_linux_path_var(&current_user, &env_path) {
        Ok(_) => {
            println!("Envs's successfully configured for node.js.");
            
            exit(0)
        },
        Err(error) => {
            match error {
                EnvConfigurationError::UnableToOpenUserShellFile(err) => println!("That error occured when we try to set your env's: {}", err),
                EnvConfigurationError::UnableToWriteUserShellFile(err) => println!("That error occured when we try to set your env's: {}", err),
                EnvConfigurationError::UnableToOpenSystemShellFile(err) => println!("That error occured when we try to set your env's: {}", err),
                EnvConfigurationError::UnableToWriteSystemShellFile(err) => println!("That error occured when we try to set your env's: {}", err),
                _ => println!("There is a bug!")
            }

            exit(1)
        }
    }
}

pub fn install_nodejs_on_centos_and_fedora(env_confs: &models::EnvConfiguration, url: &str, file_name: &str) {
    println!("Welcome to incli. Your request to install Node.js on a Red Hat Based Distro Reached.");

    let current_user = get_current_user();
    let env_path;

    let slice_of_file_name = &file_name[0..file_name.len() - 7];

    let install_nodejs = Command::new("wget")
                                                .arg(url)
                                                .arg("-O")
                                                .arg(file_name)
                                                .output()
                                                .expect("Some Error Happened");

    if !install_nodejs.status.success() {
        println!("Couldn't install Node.js Source Files Because Of Whatever reason.");
        exit(1);
    }

    match current_user.as_str() {
        "root" => {
            let current_folder_path = Command::new("pwd").output().unwrap();
            let current_folder_path = std::str::from_utf8(&current_folder_path.stdout).unwrap().trim();

            let format_the_whole_file_path = format!("{}/{}", current_folder_path, file_name);

            Command::new("sudo")
                        .arg("chmod")
                        .arg("755")
                        .arg(&format_the_whole_file_path)
                        .output()
                        .expect("couldn't give 755 permission to source code.");

            let extract_the_archive = Command::new("sudo")
                                                                        .arg("tar")
                                                                        .arg("xvf")
                                                                        .arg(&format_the_whole_file_path)
                                                                        .output();

            match extract_the_archive {
                Ok(_) => println!("archive file successfully extracted"),
                Err(error) => {
                    println!("that error occured when extracting archive file: {}", error);
                    exit(1)
                }
            }
                                                            
            Command::new("sudo")
                        .arg("rm")
                        .arg("-rf")
                        .arg(format_the_whole_file_path)
                        .output()
                        .unwrap();

            let source_files_path = format!("{}/{}", current_folder_path, slice_of_file_name);

            let move_the_source_files = Command::new("sudo")
                                                                        .arg("mv")
                                                                        .arg(source_files_path)
                                                                        .arg(&env_confs.home_dir.to_string())
                                                                        .output();

            match move_the_source_files {
                Ok(_) => (),
                Err(error) => {
                    eprintln!("cannot move the source file for this reason: {}", error);
                    exit(1)
                }
            }

            env_path = format!("/root/{}/bin", slice_of_file_name);
        },
        &_ => {
            let current_folder_path = Command::new("pwd").output().unwrap();
            let current_folder_path = std::str::from_utf8(&current_folder_path.stdout).unwrap().trim();

            let format_the_whole_file_path = format!("{}/{}", current_folder_path, file_name);

            Command::new("sudo")
                        .arg("chmod")
                        .arg("755")
                        .arg(&format_the_whole_file_path)
                        .output()
                        .expect("couldn't give 755 permission to source code.");

            let extract_the_archive = Command::new("sudo")
                                                                        .arg("tar")
                                                                        .arg("xvf")
                                                                        .arg(&format_the_whole_file_path)
                                                                        .output();

            match extract_the_archive {
                Ok(_) => println!("archive file successfully extracted"),
                Err(error) => {
                    println!("that error occured when extracting archive file: {}", error);
                    exit(1)
                }
            }
                                                            
            Command::new("sudo")
                        .arg("rm")
                        .arg("-rf")
                        .arg(format_the_whole_file_path)
                        .output()
                        .unwrap();

            let source_files_path = format!("{}/{}", current_folder_path, slice_of_file_name);

            let move_the_source_files = Command::new("sudo")
                                                                        .arg("mv")
                                                                        .arg(source_files_path)
                                                                        .arg(&env_confs.home_dir.to_string())
                                                                        .output();

            match move_the_source_files {
                Ok(_) => (),
                Err(error) => {
                    eprintln!("cannot move the source file for this reason: {}", error);
                    exit(1)
                }
            }

            env_path = format!("{}/{}/bin", env_confs.home_dir.to_string(), slice_of_file_name);
        }
    }

    match env_confs.configure_centos_and_fedora_path_var(&current_user, &env_path) {
        Ok(_) => {
            println!("Envs's successfully configured for node.js.");
            
            exit(0)
        },
        Err(error) => {
            match error {
                EnvConfigurationError::UnableToOpenUserShellFile(err) => println!("That error occured when we try to set your env's: {}", err),
                EnvConfigurationError::UnableToWriteUserShellFile(err) => println!("That error occured when we try to set your env's: {}", err),
                EnvConfigurationError::UnableToOpenSystemShellFile(err) => println!("That error occured when we try to set your env's: {}", err),
                EnvConfigurationError::UnableToWriteSystemShellFile(err) => println!("That error occured when we try to set your env's: {}", err),
                _ => println!("There is a bug!")
            }

            exit(1)
        }
    }
}

pub fn install_nodejs_on_rocky_linux(env_confs: &models::EnvConfiguration, url: &str, file_name: &str) {
    println!("Welcome to incli. Your request to install Node.js on a Red Hat Based Distro Reached.");
    println!("Be sure you're running that installation on your user's root directory, otherwise you have to set your env's manually.");

    let current_user = get_current_user();
    let env_path;

    let slice_of_file_name = &file_name[0..file_name.len() - 7];

    let install_nodejs = Command::new("wget")
                                                .arg(url)
                                                .arg("-O")
                                                .arg(file_name)
                                                .output()
                                                .expect("Some Error Happened");

    if !install_nodejs.status.success() {
        println!("Couldn't install Node.js Source Files Because Of Whatever reason.");
        exit(1);
    }

    match current_user.as_str() {
        "root" => {
            let current_folder_path = Command::new("pwd").output().unwrap();
            let current_folder_path = std::str::from_utf8(&current_folder_path.stdout).unwrap().trim();

            let format_the_whole_file_path = format!("{}/{}", current_folder_path, file_name);

            Command::new("chmod")
                        .arg("755")
                        .arg(&format_the_whole_file_path)
                        .output()
                        .expect("couldn't give 755 permission to source code.");

            let extract_the_archive = Command::new("tar")
                                                                        .arg("xvf")
                                                                        .arg(&format_the_whole_file_path)
                                                                        .output();

            match extract_the_archive {
                Ok(_) => println!("archive file successfully extracted"),
                Err(error) => {
                    println!("that error occured when extracting archive file: {}", error);
                    exit(1)
                }
            }
                                                            
            Command::new("rm")
                        .arg("-rf")
                        .arg(format_the_whole_file_path)
                        .output()
                        .unwrap();

            let source_files_path = format!("{}/{}", current_folder_path, slice_of_file_name);

            let move_the_source_files = Command::new("mv")
                                                                        .arg(source_files_path)
                                                                        .arg(&env_confs.home_dir.to_string())
                                                                        .output();

            match move_the_source_files {
                Ok(_) => (),
                Err(error) => {
                    eprintln!("cannot move the source file for this reason: {}", error);
                    exit(1)
                }
            }

            env_path = format!("/root/{}/bin", slice_of_file_name);
        },
        &_ => {
            let current_folder_path = Command::new("pwd").output().unwrap();
            let current_folder_path = std::str::from_utf8(&current_folder_path.stdout).unwrap().trim();

            let format_the_whole_file_path = format!("{}/{}", current_folder_path, file_name);

            Command::new("chmod")
                        .arg("755")
                        .arg(&format_the_whole_file_path)
                        .output()
                        .expect("couldn't give 755 permission to source code.");

            let extract_the_archive = Command::new("tar")
                                                                        .arg("xvf")
                                                                        .arg(&format_the_whole_file_path)
                                                                        .output();

            match extract_the_archive {
                Ok(_) => println!("archive file successfully extracted"),
                Err(error) => {
                    println!("that error occured when extracting archive file: {}", error);
                    exit(1)
                }
            }
                                                            
            Command::new("rm")
                        .arg("-rf")
                        .arg(format_the_whole_file_path)
                        .output()
                        .unwrap();

            let source_files_path = format!("{}/{}", current_folder_path, slice_of_file_name);

            let move_the_source_files = Command::new("mv")
                                                                        .arg(source_files_path)
                                                                        .arg(&env_confs.home_dir.to_string())
                                                                        .output();

            match move_the_source_files {
                Ok(_) => (),
                Err(error) => {
                    eprintln!("cannot move the source file for this reason: {}", error);
                    exit(1)
                }
            }

            env_path = format!("{}/{}/bin", env_confs.home_dir.to_string(), slice_of_file_name);
        }
    }

    match env_confs.configure_rocky_linux_path_var(&current_user, &env_path) {
        Ok(_) => {
            println!("Envs's successfully configured for node.js.");
            
            exit(0)
        },
        Err(error) => {
            match error {
                EnvConfigurationError::UnableToOpenUserShellFile(err) => println!("That error occured when we try to set your env's: {}", err),
                EnvConfigurationError::UnableToWriteUserShellFile(err) => println!("That error occured when we try to set your env's: {}", err),
                EnvConfigurationError::UnableToOpenSystemShellFile(err) => println!("That error occured when we try to set your env's: {}", err),
                EnvConfigurationError::UnableToWriteSystemShellFile(err) => println!("That error occured when we try to set your env's: {}", err),
                _ => println!("There is a bug!")
            }

            exit(1)
        }
    }
}

pub fn install_node_on_alpine_linux() {
    println!("Welcome to the incli. Your request to install node.js on alpine linux reached.");
    println!("Be sure you have the root user or have sudo privileges. Otherwise Installation Won't Work.");
    
    let install_nodejs = Command::new("sudo")
                                            .arg("apk")
                                            .arg("add")
                                            .arg("--update")
                                            .arg("nodejs")
                                            .arg("npm").output().expect("cannot download node.js for some reason");

    if install_nodejs.status.success() {
        println!("You're successfully installed node.js")
    } else {
        eprintln!("You couldn'y installed node.js for that reason: {:#?}", std::str::from_utf8(&install_nodejs.stderr))
    }
}

pub fn install_nodejs_on_windows(env_confs: &models::EnvConfiguration, url: &str, exe_name: &str) {
    println!("Welcome to incli. Your request to install Node.js on Windows Reached.");
    println!("Keep pressing any of your keys when you focused on your terminal in regular time period, otherwise your installation may not run correctly.");

    let current_user = get_current_user();
    let get_downloads_path = format!("C:\\Users\\{}\\Downloads\\{}", current_user, exe_name);

    let download_command = Command::new("powershell")
                                        .arg("Invoke-WebRequest")
                                        .arg("-Uri")
                                        .arg(url)
                                        .arg("-OutFile")
                                        .arg(&get_downloads_path)
                                        .output()
                                        .expect("Download failed.");

    if !download_command.status.success() {
        eprintln!("Failed to download {} for whatever reason. Exiting.", exe_name);
        exit(1);
    }

    println!("Download completed, you can continue installing node.js through the pop up which will open.");

    let install_command = Command::new("powershell")
                                            .arg(get_downloads_path)
                                            .output()
                                            .expect("Installation failed.");

    if !install_command.status.success() {
        eprintln!("Installation failed. Exiting.");
        exit(1);
    } else {
        println!("Node.js installed, you can check it via opening a new terminal window and typing: 'node --version' command.")
    }


}

pub fn install_nodejs_error() {
    println!("Wrong third argument for installing node.js")
}

pub fn log_node_and_npm_version(){
    let node_version_command = Command::new("node")
                                                    .arg("--version")
                                                    .output()
                                                    .expect("");

    if !&node_version_command.status.success() {
        println!("Node isn't installed on your system.");
    } 

    let npm_version_command = Command::new("npm")
                                                .arg("--version")
                                                .output()
                                                .expect("");

    if !&npm_version_command.status.success() {
        println!("Npm isn't installed on your system.");
    } 

    let get_node_answer_as_string = std::str::from_utf8(&node_version_command.stdout).unwrap().to_string();

    println!("Your Node version is: {}", get_node_answer_as_string);

    let get_npm_answer_as_string = std::str::from_utf8(&npm_version_command.stdout).unwrap().to_string();

    println!("Your Npm version is: {}", get_npm_answer_as_string)
}