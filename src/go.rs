use crate::models::EnvConfigurationError;
use crate::utils;
use crate::models;
use std::process::{Command, exit};
use std::fs;
use std::io;
use std::path::Path;
use sys_info_extended::get_current_user;

pub fn install_go_on_windows(env_confs: &models::EnvConfiguration, url: &str, exe_name: &str) {
    println!("Welcome to incli, your request to install Go on Windows reached. Please wait until it finish...");
    println!("Keep pressing any of your keys when you focused on your terminal in regular time period, otherwise your installation may not run correctly.");

    let current_user = get_current_user();

    let get_downloads_path = format!("C:\\Users\\{}\\Downloads\\{}", current_user, exe_name);

    let download_go = Command::new("powershell")
                                                .arg("Invoke-WebRequest")
                                                .arg("-Uri")
                                                .arg(url)
                                                .arg("-OutFile")
                                                .arg(&get_downloads_path)
                                                .output();

    match download_go {
        Ok(_) => println!("go setup file successfully downloaded on pc, running it..."),
        Err(error) => {
            eprintln!("cannot download go setup file for that reason: {}", error);
            exit(1)
        }
    }

    let run_go_installer = Command::new("powershell")
                                                            .arg(get_downloads_path)
                                                            .output();

    match run_go_installer {
        Ok(_) => {
            println!("go installer runned, you can continue installation on that.")
        },
        Err(error) => {
            eprintln!("Cannot run go installer because of that reason: {}", error);
            exit(1)
        }
    }
}

pub fn install_go_on_debian_based_distros(env_confs: &models::EnvConfiguration, url: &str, file_name: &str) {
    println!("Welcome to incli. Your request to install Go on a debian based distro reached.");
    println!("Be sure you have installed wget on your pc, otherwise installation won't work.");

    let current_user = get_current_user();

    let install_go = Command::new("wget")
                                    .arg(url)
                                    .arg("-O")
                                    .arg(file_name)
                                    .output()
                                    .expect("Some Error Happened");

    if !install_go.status.success() {
        println!("Couldn't download Go Source Files Because Of Whatever reason.");
        exit(1);
    }

    let mut env_path;

    match current_user.as_str() {
        "root" => {
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
                                                .arg("xzvf")
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
    
            let format_current_folder_again = format!("{}/go", std::str::from_utf8(&current_folder_again.stdout).unwrap().trim());
    
            Command::new("sudo")
                        .arg("mv")
                        .arg(format_current_folder_again)
                        .arg("/root")
                        .output()
                        .unwrap();
    
            env_path = format!("/root/go/bin");
        },
        &_ => {
            let get_current_file_command = Command::new("pwd").output().unwrap();
    
            let file_for_moving = format!("{}/{}", std::str::from_utf8(&get_current_file_command.stdout).unwrap().trim(), file_name);
    
            Command::new("mv").arg(&file_for_moving).arg(&env_confs.home_dir.to_string()).output().unwrap();
    
            let file_path = format!("{}/{}", env_confs.home_dir.to_string(), file_name);
    
            Command::new("sudo")
                        .arg("chmod")
                        .arg("777")
                        .arg(&file_path)
                        .output()
                        .expect("couldn't give 755 permission to source code.");
    
            let extract_tar_file = Command::new("sudo")
                                                .arg("tar")
                                                .arg("xzvf")
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
    
            let format_current_folder_again = format!("{}/go", std::str::from_utf8(&current_folder_again.stdout).unwrap().trim());

            println!("your format the current folder again: {}", format_current_folder_again);
    
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
                        .arg(&env_confs.home_dir.to_string())
                        .output()
                        .unwrap();
    
            env_path = format!("{}/go/bin", env_confs.home_dir.to_string());
        }
    }

    match env_confs.configure_debian_path_var(&env_path) {
        Ok(_) => {
            println!("Envs's successfully configured for go.");
            
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

    /*let line_for_append = format!("export PATH=\"{}:$PATH\"\n", env_path);
                    
    let line_for_append = line_for_append.as_bytes();
                
    let bashrc_file = fs::OpenOptions::new().append(true).open("/root/.bashrc");
            
    match bashrc_file {
        Ok(mut file) => {
            let add_env = io::Write::write_all(&mut file, line_for_append);
            
            match add_env {
                Ok(_) => println!("Go successfully added on env's. You can try it by restarting your computer and typing 'go version' on command line."),
                Err(error) => println!("And error occured: {}", error)
            }
        },
        Err(_) => println!("cannot installed go for that reason: {}", env_path)
    }*/
}

pub fn install_go_on_arch_linux(env_confs: &models::EnvConfiguration, url: &str, file_name: &str) {
    println!("Welcome to incli. Your request to Install Go on Arch Linux Reached. Please wait until installation finish.");
    
    let current_user = get_current_user();
    let env_path;

    let install_go = Command::new("wget")
                                                .arg(url)
                                                .arg("-O")
                                                .arg(file_name)
                                                .output()
                                                .expect("Some Error Happened");

    if !install_go.status.success() {
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
                                                            .arg("xzvf")
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

            let format_the_source_files_path = format!("{}/go", current_folder_path);

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

            env_path = format!("/root/go/bin")
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
                                                            .arg("xzvf")
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

            let format_the_source_files_path = format!("{}/go", current_folder_path);

            //println!("source files path: {}", format_the_source_files_path);

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

            env_path = format!("{}/go/bin", env_confs.home_dir.to_string())
        }
    }

    println!("Source Files Downloaded Successfully");

    match env_confs.configure_arch_linux_path_var(&current_user, &env_path) {
        Ok(_) => {
            println!("Envs's successfully configured for go.");
            
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

    /*let get_incli_paths_path = format!("{}/INCLI_PATHS", env_confs.home_dir.to_string());

    let check_if_incli_paths_exist = Command::new("cd").arg(&get_incli_paths_path).output();

    match check_if_incli_paths_exist {
        Ok(_) => (),
        Err(_) => {
            println!("You don't have incli_envs.sh file yet. We're configuring it...");
            utils::configure_incli_envs_file(env_confs, &current_user, true)
        }
    }

    let get_incli_envs_path = format!("{}/incli-envs.sh", get_incli_paths_path);

    let incli_envs_file = fs::OpenOptions::new().append(true).open(get_incli_envs_path);

    match incli_envs_file {
        Ok(mut file) => {
            let line_for_append = format!("export PATH=$PATH:{}\n", env_path);
            let line_for_append = line_for_append.as_bytes();
        
            let add_env_file_dest = io::Write::write_all(&mut file, line_for_append);

            match add_env_file_dest {
                Ok(_) => {
                    println!("envs successfully added on your user.");
                    println!("Your installation of Go on arch linux ended successfully. You can check it via typing 'go version' later than close current terminal. If you can't see any answer, try to restart the computer and check it again.");
                },
                Err(err) => eprintln!("This error occured: {}", err)
            }
        },
        Err(err) => {
            eprintln!("Cannot open incli_envs.sh file for that reason: {}", err);
            println!("Because of that, we cannot set env's. You can set your env's manually.");
            exit(1)
        }
    }*/
}

pub fn install_go_on_alma_linux(env_confs: &models::EnvConfiguration, url: &str, file_name: &str) {
    println!("Welcome to incli. Your request to install Go on Alma Linux Reached.");

    let current_user = get_current_user();
    let env_path;

    let install_go = Command::new("wget")
                                    .arg(url)
                                    .arg("-O")
                                    .arg(file_name)
                                    .output()
                                    .expect("Some Error Happened");

    if !install_go.status.success() {
        println!("Couldn't install Go Source Files Because Of Whatever reason.");
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
                                                .arg("xzvf")
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

            let source_files_path = format!("{}/go", current_folder_path);

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

            env_path = "/root/go/bin".to_string();
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
                                                                        .arg("xzvf")
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

            let source_files_path = format!("{}/go", current_folder_path);

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

            env_path = format!("{}/go/bin", env_confs.home_dir.to_string())
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

    // in alma linux 9, terminal commands for checking existence of a folder always return success value.
    // because of that, we have to use std::path::PATH api.

    /*let incli_paths_path = format!("{}/INCLI_PATHS", env_confs.home_dir.to_string());

    let check_if_incli_paths_exist = Path::new(&incli_paths_path);

    if !check_if_incli_paths_exist.exists() {
        utils::configure_incli_envs_file(env_confs, &current_user, true)
    }

    let incli_envs_path = format!("{}/incli-envs.sh", incli_paths_path);

    let incli_envs_file = fs::OpenOptions::new().append(true).open(incli_envs_path);

    match incli_envs_file {
        Ok(mut file) => {
            let line_for_append = format!("\nPATH=\"{}:$PATH\"\n", env_path);
            let line_for_append = line_for_append.as_bytes();
        
            let add_env_file_dest = io::Write::write_all(&mut file, line_for_append);

            match add_env_file_dest {
                Ok(_) => {
                    println!("envs successfully added on your user.");
                    println!("You're successfully installed go. You can check it via typing 'go version' later than open a new terminal. If it doesn't work, try it later than restart your computer.")
                },
                Err(err) => eprintln!("This error occured: {}", err)
            }
        },
        Err(err) => {
            eprintln!("Cannot open incli_envs.sh file for that reason: {}", err);
            println!("Because of that we couldn't set env's. You can set your env's manually if you want.")
        }
    }*/
}

pub fn install_go_on_centos_and_fedora(env_confs: &models::EnvConfiguration, url: &str, file_name: &str) {
    println!("Welcome to incli. Your request to install Go on a Red Hat Based Distro Reached.");

    let current_user = get_current_user();
    let env_path;

    let install_go = Command::new("wget")
                                                .arg(url)
                                                .arg("-O")
                                                .arg(file_name)
                                                .output()
                                                .expect("Some Error Happened");

    if !install_go.status.success() {
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
                                                                        .arg("xzvf")
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

            let source_files_path = format!("{}/go", current_folder_path);

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

            env_path = "/root/go/bin".to_string();
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
                                                                        .arg("xzvf")
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

            let source_files_path = format!("{}/go", current_folder_path);

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

            env_path = format!("{}/go/bin", env_confs.home_dir.to_string());
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

    /*let incli_paths_path = format!("{}/INCLI_PATHS", env_confs.home_dir.to_string());

    let check_if_incli_paths_exist = Path::new(&incli_paths_path);

    if !check_if_incli_paths_exist.exists() {
        utils::configure_incli_envs_file(env_confs, &current_user, true)
    }

    let incli_envs_path = format!("{}/incli-envs.sh", incli_paths_path);

    let incli_envs_file = fs::OpenOptions::new().append(true).open(incli_envs_path);

    match incli_envs_file {
        Ok(mut file) => {
            let line_for_append = format!("\nexport PATH=\"{}:$PATH\"\n", env_path);
            let line_for_append = line_for_append.as_bytes();
        
            let add_env_file_dest = io::Write::write_all(&mut file, line_for_append);

            match add_env_file_dest {
                Ok(_) => {
                    println!("envs successfully added on your user.");
                    println!("You're successfully installed Go on a Red Hat Based Distro. You can check it via typing 'go version' later than open a new terminal. If it doesn't work, restart your computer and type it again.");
                },
                Err(err) => eprintln!("This error occured: {}", err)
            }
        },
        Err(err) => {
            eprintln!("Cannot open incli_envs.sh file for that reason: {}", err);
            println!("We couldn't set env's due to the previously printed reason. You can set it manually.")
        }
    }*/
}

pub fn install_go_on_rocky_linux(env_confs: &models::EnvConfiguration, url: &str, file_name: &str) {
    println!("Welcome to incli. Your request to install Go on Rocky Linux Reached.");

    let current_user = get_current_user();
    let env_path;

    let install_go = Command::new("wget")
                                                .arg(url)
                                                .arg("-O")
                                                .arg(file_name)
                                                .output()
                                                .expect("Some Error Happened");

    if !install_go.status.success() {
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
                                                                        .arg("xzvf")
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

            env_path = "/root/go/bin".to_string();
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
                                                                        .arg("xzvf")
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

            let source_files_path = format!("{}/go", current_folder_path);

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

            env_path = format!("{}/go/bin", env_confs.home_dir.to_string());
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

    /*let incli_paths_path = format!("{}/INCLI_PATHS", env_confs.home_dir.to_string());

    let check_if_incli_paths_exist = Path::new(&incli_paths_path);

    if !check_if_incli_paths_exist.exists() {
        utils::configure_incli_envs_file(env_confs, &current_user, true)
    }

    let incli_envs_path = format!("{}/incli-envs.sh", incli_paths_path);

    let incli_envs_file = fs::OpenOptions::new().append(true).open(incli_envs_path);

    match incli_envs_file {
        Ok(mut file) => {
            let line_for_append = format!("\nexport PATH=\"{}:$PATH\"\n", env_path);
            let line_for_append = line_for_append.as_bytes();
        
            let add_env_file_dest = io::Write::write_all(&mut file, line_for_append);

            match add_env_file_dest {
                Ok(_) => {
                    println!("envs successfully added on your user.");
                    println!("You're successfully installed Go on a Red Hat Based Distro. You can check it via typing 'go version' later than open a new terminal. If it doesn't work, restart your computer and type it again.");
                },
                Err(err) => eprintln!("This error occured: {}", err)
            }
        },
        Err(err) => {
            eprintln!("Cannot open incli_envs.sh file for that reason: {}", err);
            println!("We couldn't set env's due to the previously printed reason. You can set it manually.")
        }
    }*/
}

pub fn install_go_on_alpine_linux(_url: &str, _file_name: &str) {
    println!("Incli doesn't support alpine linux downloads, exiting...");

    return;
}

pub fn log_go_version() {
    let get_go_version = Command::new("go").arg("version").output();

    match get_go_version {
        Ok(version) => {
            let format_the_version = std::str::from_utf8(&version.stdout).unwrap();

            for line in format_the_version.lines().into_iter() {
                if line.starts_with("go version") {
                    let split_the_go_version_output: Vec<&str> = line.split(" ").collect::<Vec<&str>>();

                    println!("Your go version is: {}", split_the_go_version_output[2].strip_prefix("go").unwrap());
                    println!("Your go's platform/architecture specification is: {}", split_the_go_version_output[3]);
                }
            }


        },
        Err(error) => eprintln!("cannot log go version because of that: {}", error)
    }
}

pub fn install_go_error() {
    println!("Wrong third argument for installing go")
}