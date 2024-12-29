use crate::utils;
use crate::models;
use std::process::{Command, exit};
use std::fs;
use std::io;
use std::path::Path;
use sys_info_extended::{get_current_user, set_env, EnvOptions};

pub fn install_java_on_windows(env_confs: &models::EnvConfiguration, url: &str, file_name: &str, version: &str) {
    println!("Welcome to incli, your request to install Java on Windows reached. Please wait until it finish...");
    println!("Keep pressing any of your keys when you focused on your terminal in regular time period, otherwise your installation may not run correctly.");

    let current_user = get_current_user();

    let get_downloads_path = format!("C:\\Users\\{}\\Downloads\\{}", current_user, file_name);

    let download_jvm = Command::new("powershell")
                                                .arg("Invoke-WebRequest")
                                                .arg("-Uri")
                                                .arg(&url)
                                                .arg("-OutFile")
                                                .arg(&get_downloads_path)
                                                .output();

    if version == "8" {
        match download_jvm {
            Ok(_) => println!("Setup file successfully downloaded on pc, running it..."),
            Err(error) => {
                eprintln!("cannot download jvm setup file for that reason: {}", error);
                exit(1)
            }
        }
    
        let run_jvm_installer = Command::new("powershell")
                                                                .arg(&get_downloads_path)
                                                                .output();

        match run_jvm_installer {
            Ok(_) => {
                if file_name.contains("jdk-8") {
                    println!("Jdk installer runned, you can continue installation on that.");
                    println!("This version only includes jdk 8, you also have to install jre 8 for working with it. Later than that installation, we suggest you to run './incli install java 8 jre' command to install jre 8.");
                    exit(0)
                } else{
                    println!("Jre installer runned, you can continue installation on that.");
                    println!("This version only includes jre 8, you also have to install jdk-8 for working with it. Later than that installation, we suggest you to run './incli install java 8 jdk' command to install jdk 8.");
                    exit(0)
                }
            },
            Err(error) => {
                eprintln!("Cannot run jvm installer because of that reason: {}", error);
                exit(1)
            }
        }
    } 
    
    if version == "9" || version == "10" {
        println!("Unfortunately, we're currently not support jvm {} download for windows.", version);
        exit(0)
    }

    if version == "11" || 
       version == "12" || 
       version == "13" || 
       version == "14" || 
       version == "15" || 
       version == "16" ||
       version == "17" ||
       version == "18" ||
       version == "19" ||
       version == "20" ||
       version == "21" ||
       version == "22" ||
       version == "23" ||
       version == "24" {
            let zip_move_path = "C:\\Incli Downloads";

            let format_the_command = format!("Expand-Archive -Path '{}' -DestinationPath '{}'", get_downloads_path, zip_move_path);
            let extract_the_zip_file = Command::new("powershell.exe")
                                                                .arg("-Command")
                                                                .arg(format_the_command)
                                                                .output();

            match extract_the_zip_file {
                Ok(_) => {
                    println!("We're successfully extracted and moved the archive");

                    let format_remove_command = format!("Remove-Item -Path 'C:\\Users\\{}\\Downloads\\{}' -Force", current_user, file_name);

                    std::process::Command::new("powershell.exe")
                                .arg(format_remove_command)
                                .output()
                                .unwrap();
                },
                Err(error) => println!("That error occured when we extracting the zip file: {}", error)
            }

            let mut java_home_env_path = String::new();
            let mut bin_env_path = String::new();

            match version {
                "11" => {
                    java_home_env_path = "C:\\Incli Downloads\\jdk-11.0.2".to_string();
                    bin_env_path = "C:\\Incli Downloads\\jdk-11.0.2\\bin".to_string();

                },
                "12" => {
                    java_home_env_path = "C:\\Incli Downloads\\jdk-12.0.2".to_string();
                    bin_env_path = "C:\\Incli Downloads\\jdk-12.0.2\\bin".to_string(); 
                }
                "13" => {
                    java_home_env_path = "C:\\Incli Downloads\\jdk-13.0.2".to_string();
                    bin_env_path = "C:\\Incli Downloads\\jdk-13.0.2\\bin".to_string();
                },
                "14" => {
                    java_home_env_path = "C:\\Incli Downloads\\jdk-14.0.2".to_string();
                    bin_env_path = "C:\\Incli Downloads\\jdk-14.0.2\\bin".to_string();
                },
                "15" => {
                    java_home_env_path = "C:\\Incli Downloads\\jdk-15.0.2".to_string();
                    bin_env_path = "C:\\Incli Downloads\\jdk-15.0.2\\bin".to_string();
                },
                "16" => {
                    java_home_env_path = "C:\\Incli Downloads\\jdk-16.0.2".to_string();
                    bin_env_path = "C:\\Incli Downloads\\jdk-16.0.2\\bin".to_string();
                },
                "17" => {
                    java_home_env_path = "C:\\Incli Downloads\\jdk-17.0.2".to_string();
                    bin_env_path = "C:\\Incli Downloads\\jdk-17.0.2\\bin".to_string();
                },
                "18" => {
                    java_home_env_path = "C:\\Incli Downloads\\jdk-18.0.2".to_string();
                    bin_env_path = "C:\\Incli Downloads\\jdk-18.0.2\\bin".to_string();
                },
                "19" => {
                    java_home_env_path = "C:\\Incli Downloads\\jdk-19.0.1".to_string();
                    bin_env_path = "C:\\Incli Downloads\\jdk-19.0.1\\bin".to_string();
                },
                "20" => {
                    java_home_env_path = "C:\\Incli Downloads\\jdk-20.0.2".to_string();
                    bin_env_path = "C:\\Incli Downloads\\jdk-20.0.2\\bin".to_string();
                },
                "21" => {
                    java_home_env_path = "C:\\Incli Downloads\\jdk-21.0.2".to_string();
                    bin_env_path = "C:\\Incli Downloads\\jdk-21.0.2\\bin".to_string();
                },
                "22" => {
                    java_home_env_path = "C:\\Incli Downloads\\jdk-22.0.1".to_string();
                    bin_env_path = "C:\\Incli Downloads\\jdk-22.0.1\\bin".to_string();
                },
                "23" => {
                    java_home_env_path = "C:\\Incli Downloads\\jdk-23".to_string();
                    bin_env_path = "C:\\Incli Downloads\\jdk-23\\bin".to_string();
                }
                "24" => {
                    java_home_env_path = "C:\\Incli Downloads\\jdk-24".to_string();
                    bin_env_path = "C:\\Incli Downloads\\jdk-24\\bin".to_string();
                },
                &_ => ()
            }

            let env_options = EnvOptions {
                name: "JAVA_HOME".to_string(),
                value: java_home_env_path,
                level: sys_info_extended::EnvLevel::Machine
            };

            match set_env(env_options) {
                Ok(_) => println!("JAVA_HOME variable added to the system Successfully."),
                Err(error) => println!("We cannot add JAVA_HOME variable for that reason: {}", error)
            }

            utils::append_env_to_system_path_on_windows(&bin_env_path);

            println!("Jvm {} successfully installed on your computer.", version);

            exit(0)
    }

    println!("Unfortunately, no other version of java available for now.");
    exit(1)
}

pub fn install_java_on_debian_based_distros(env_confs: &models::EnvConfiguration, url: &str, file_name: &str, version: &str) {
    println!("Welcome to incli. Your request to install Java on a debian based distro reached.");
    println!("Be sure you have installed wget on your pc, otherwise installation won't work.");

    let check_if_java_installed = std::process::Command::new("java")
                                                                                .arg("-version")
                                                                                .output();

    match check_if_java_installed {
        Ok(_) => {
            println!("It seems you already installed java. Before you proceed, please delete it from your system.");
            println!("If you don't installed it by yourself, probably it's preinstalled by apt.");
            println!("you can check if you have any of the java version installed by apt with that command: 'apt list --installed'");
            return;
        },
        Err(_) => ()
    }


    if version == "8" {
        let install_openjdk_8 = std::process::Command::new("sudo")
                                                                                .arg("apt")
                                                                                .arg("install")
                                                                                .arg("-y")
                                                                                .arg("openjdk-8-jdk")
                                                                                .output();

        match install_openjdk_8 {
            Ok(_) => println!("Java 8 Jdk Successfully installed. Now You have to install Java 8 Jre."),
            Err(error) => {
                println!("That error occured when installing Java 8 Jdk: {}", error);
            }
        }

        let install_openjdk_8_jre = std::process::Command::new("sudo")
                                                                                .arg("apt")
                                                                                .arg("install")
                                                                                .arg("-y")
                                                                                .arg("openjdk-8-jre")
                                                                                .output();

        match install_openjdk_8_jre {
            Ok(_) => println!("Java 8 Jre Successfully installed."),
            Err(error) => {
                println!("That error occured when installing Java 8 Jdk: {}", error);
                exit(1);
            }
        }
    }

    if version == "9" ||
       version == "10" || 
       version == "11" || 
       version == "12" || 
       version == "13" || 
       version == "14" || 
       version == "15" || 
       version == "16" ||
       version == "17" ||
       version == "18" ||
       version == "19" ||
       version == "20" ||
       version == "21" ||
       version == "22" ||
       version == "23" ||
       version == "24" {
            let current_user = get_current_user();

            let user_path = format!("/home/{}", current_user);
        
            let install_java = Command::new("wget")
                                            .arg(url)
                                            .arg("-O")
                                            .arg(file_name)
                                            .output()
                                            .expect("Some Error Happened");

            let format_jvm_folder_name = match version {
                "9" => "jdk-9.0.4",
                "10" => "jdk-10.0.2",
                "11" => "jdk-11.0.2",
                "12" => "jdk-12.0.2",
                "13" => "jdk-13.0.2",
                "14" => "jdk-14.0.2",
                "15" => "jdk-15.0.2",
                "16" => "jdk-16.0.2",
                "17" => "jdk-17.0.2",
                "18" => "jdk-18.0.2",
                "19" => "jdk-19.0.1",
                "20" => "jdk-20.0.2",
                "21" => "jdk-21.0.2",
                "22" => "jdk-22.0.1",
                "23" => "jdk-23",
                "24" => "jdk-24",
                &_ => ""
            };
        
            match current_user.as_str() {
                "root" => {
                    if !install_java.status.success() {
                        println!("Couldn't install Java Source Files Because Of Whatever reason.");
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
            
                    let format_current_folder_again = format!("{}/{}", std::str::from_utf8(&current_folder_again.stdout).unwrap().trim(), format_jvm_folder_name);
            
                    Command::new("sudo")
                                .arg("mv")
                                .arg(format_current_folder_again)
                                .arg("/root")
                                .output()
                                .unwrap();
            
                    let env_path = format!("/root/{}", format_jvm_folder_name);
            
                    let line_for_append_1 = format!("\nexport PATH=\"{}/bin:$PATH\"\n", env_path);
                    let line_for_append_1 = line_for_append_1.as_bytes();

                    let line_for_append_2 = format!("\nexport JAVA_HOME=\"{}\"\n", env_path);
                    let line_for_append_2 = line_for_append_2.as_bytes();
                                
                    let bashrc_file = fs::OpenOptions::new().append(true).open("/root/.bashrc");
                            
                    match bashrc_file {
                        Ok(mut file) => {
                            let add_env = io::Write::write_all(&mut file, line_for_append_2);

                            match add_env {
                                Ok(_) => println!("JAVA_HOME env successfully added on your user's envs."),
                                Err(error) => println!("An error ocured when we try to set JAVA_HOME env: {}", error)
                            }

                            let add_env = io::Write::write_all(&mut file, line_for_append_1);
                            
                            match add_env {
                                Ok(_) => println!("Java successfully added on env's. You can try it by restarting your computer and typing 'java -version' on command line."),
                                Err(error) => println!("And error occured: {}", error)
                            }
                        },
                        Err(_) => println!("cannot installed go for that reason: {}", env_path)
                    }
                },
                &_ => {
                    if !install_java.status.success() {
                        println!("Couldn't download Java Source Files Because Of Whatever reason.");
                        exit(1);
                    }
            
                    let get_current_file_command = Command::new("pwd").output().unwrap();
            
                    let file_for_moving = format!("{}/{}", std::str::from_utf8(&get_current_file_command.stdout).unwrap().trim(), file_name);
            
                    Command::new("mv").arg(&file_for_moving).arg(&user_path).output().unwrap();
            
                    let file_path = format!("{}/{}", user_path, file_name);
            
                    Command::new("sudo")
                                .arg("chmod")
                                .arg("777")
                                .arg(&file_path)
                                .output()
                                .expect("couldn't give 755 permission to source code.");
            
                    /*println!("Source Files Downloaded Successfully");
            
                    println!("your file_for_moving: {}", file_for_moving);
                    println!("your file_path: {}", file_path);*/
            
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
            
                    let format_current_folder_again = format!("{}/{}", std::str::from_utf8(&current_folder_again.stdout).unwrap().trim(), format_jvm_folder_name);
        
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
                                .arg(&user_path)
                                .output()
                                .unwrap();
            
                    let env_path = format!("{}/{}", user_path, format_jvm_folder_name);
            
                    let line_for_append_1 = format!("\nexport PATH=\"{}/bin:$PATH\"\n", env_path);
                    let line_for_append_1 = line_for_append_1.as_bytes();

                    let line_for_append_2 = format!("\nexport JAVA_HOME=\"{}\"\n", env_path);
                    let line_for_append_2 = line_for_append_2.as_bytes();
            
                    let create_bashrc_path = format!("{}/.bashrc", user_path);
                                
                    let bashrc_file = fs::OpenOptions::new().append(true).open(create_bashrc_path);
                            
                    match bashrc_file {
                        Ok(mut file) => {
                            let add_env = io::Write::write_all(&mut file, line_for_append_2);

                            match add_env {
                                Ok(_) => println!("JAVA_HOME env successfully added on your user's envs."),
                                Err(error) => println!("An error ocured when we try to set JAVA_HOME env: {}", error)
                            }

                            let add_env = io::Write::write_all(&mut file, line_for_append_1);
                            
                            match add_env {
                                Ok(_) => println!("Java successfully added on env's. You can try it by restarting your computer and typing 'java -version' on command line."),
                                Err(error) => println!("And error occured: {}", error)
                            }
                        },
                        Err(_) => println!("cannot installed go for that reason: {}", env_path)
                    }
                }
            }
      }
}

pub fn install_java_on_arch_linux(env_confs: &models::EnvConfiguration, url: &str, file_name: &str, version: &str) {
    println!("Welcome to incli. Your request to Install Java on Arch Linux Reached. Please wait until installation finish.");
    
    let current_user = get_current_user();
    let env_path;
    let current_user_path;

    let format_jvm_folder_name = match version {
        "9" => "jdk-9.0.4",
        "10" => "jdk-10.0.2",
        "11" => "jdk-11.0.2",
        "12" => "jdk-12.0.2",
        "13" => "jdk-13.0.2",
        "14" => "jdk-14.0.2",
        "15" => "jdk-15.0.2",
        "16" => "jdk-16.0.2",
        "17" => "jdk-17.0.2",
        "18" => "jdk-18.0.2",
        "19" => "jdk-19.0.1",
        "20" => "jdk-20.0.2",
        "21" => "jdk-21.0.2",
        "22" => "jdk-22.0.1",
        "23" => "jdk-23",
        "24" => "jdk-24",
        &_ => ""
    };

    let install_nodejs = Command::new("wget")
                                                .arg(url)
                                                .arg("-O")
                                                .arg(file_name)
                                                .output()
                                                .expect("Some Error Happened");

    if !install_nodejs.status.success() {
        println!("Couldn't install Java Source Files Because Of Whatever reason.");
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
            current_user_path = "/root".to_string();

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

            let format_the_source_files_path = format!("{}/{}", current_folder_path, format_jvm_folder_name);

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

            env_path = format!("/root/{}", format_jvm_folder_name)
        },
        &_ => {
            current_user_path = format!("/home/{}", current_user);

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

            let format_the_source_files_path = format!("{}/{}", current_folder_path, format_jvm_folder_name);

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
                                                                                    .arg(&current_user_path)
                                                                                    .output();

            match move_the_source_files_to_root {
                Ok(_) => (),
                Err(error) => {
                    eprintln!("That error occured when moving source files: {}", error);
                    exit(1)
                }
            }

            env_path = format!("{}/{}", current_user_path, format_jvm_folder_name)
        }
    }

    println!("Source Files Downloaded Successfully");

    let get_incli_paths_path = format!("{}/INCLI_PATHS", current_user_path);

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
            let line_for_append_2 = format!("\nexport JAVA_HOME={}\n", env_path);
            let line_for_append_2 = line_for_append_2.as_bytes();

            let add_java_home_env_file_dest = io::Write::write_all(&mut file, line_for_append_2);

            match add_java_home_env_file_dest {
                Ok(_) => println!("JAVA_HOME env successfully installed on your env's"),
                Err(error) => println!("An error occured when we try to add JAVA_HOME env to your user's env's: {}", error)
            }

            let line_for_append_1 = format!("\nexport PATH=$PATH:{}/bin\n", env_path);
            let line_for_append_1 = line_for_append_1.as_bytes();
        
            let add_env_file_dest = io::Write::write_all(&mut file, line_for_append_1);

            match add_env_file_dest {
                Ok(_) => {
                    println!("envs successfully added on your user.");
                    println!("Your installation of Java on arch linux ended successfully. You can check it via typing 'java -version' later than close current terminal. If you can't see any answer, try to restart the computer and check it again.");
                },
                Err(err) => eprintln!("This error occured: {}", err)
            }
        },
        Err(err) => {
            eprintln!("Cannot open incli_envs.sh file for that reason: {}", err);
            println!("Because of that, we cannot set env's. You can set your env's manually.");
            exit(1)
        }
    }
}

pub fn install_java_on_alma_linux(env_confs: &models::EnvConfiguration, url: &str, file_name: &str, version: &str) {
    println!("Welcome to incli. Your request to install Java on Alma Linux Reached.");

    if version == "8" {
        let install_java_8 = Command::new("sudo")
                                                    .arg("yum")
                                                    .arg("install")
                                                    .arg("java-1.8.0-openjdk")
                                                    .arg("-y")
                                                    .output();

        match install_java_8 {
            Ok(_) => {
                println!("You're successfully installed java 8, you can check it via typing 'java -version' on your terminal later than reopen it.");
                exit(0);
            },
            Err(error) => {
                println!("that error occured when installing Java 8: {}", error);
                exit(1);
            }
        }
    }

    if version == "9" ||
       version == "10" || 
       version == "11" || 
       version == "12" || 
       version == "13" || 
       version == "14" || 
       version == "15" || 
       version == "16" ||
       version == "17" ||
       version == "18" ||
       version == "19" ||
       version == "20" ||
       version == "21" ||
       version == "22" ||
       version == "23" ||
       version == "24" {
            let format_jvm_folder_name = match version {
                "9" => "jdk-9.0.4",
                "10" => "jdk-10.0.2",
                "11" => "jdk-11.0.2",
                "12" => "jdk-12.0.2",
                "13" => "jdk-13.0.2",
                "14" => "jdk-14.0.2",
                "15" => "jdk-15.0.2",
                "16" => "jdk-16.0.2",
                "17" => "jdk-17.0.2",
                "18" => "jdk-18.0.2",
                "19" => "jdk-19.0.1",
                "20" => "jdk-20.0.2",
                "21" => "jdk-21.0.2",
                "22" => "jdk-22.0.1",
                "23" => "jdk-23",
                "24" => "jdk-24",
                &_ => ""
            };
        
            let current_user = get_current_user();
            let env_path;
            let current_user_path;
        
            let install_java = Command::new("wget")
                                            .arg(url)
                                            .arg("-O")
                                            .arg(file_name)
                                            .output()
                                            .expect("Some Error Happened");
        
            if !install_java.status.success() {
                println!("Couldn't install Java Source Files Because Of Whatever reason.");
                exit(1);
            }
        
            match current_user.as_str() {
                "root" => {
                    current_user_path = "/root".to_string();
        
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
        
                    let source_files_path = format!("{}/{}", current_folder_path, format_jvm_folder_name);
        
                    let move_the_source_files = Command::new("sudo")
                                                                                .arg("mv")
                                                                                .arg(source_files_path)
                                                                                .arg(&current_user_path)
                                                                                .output();
        
                    match move_the_source_files {
                        Ok(_) => (),
                        Err(error) => {
                            eprintln!("cannot move the source file for this reason: {}", error);
                            exit(1)
                        }
                    }
        
                    env_path = format!("/root/{}", format_jvm_folder_name);
                },
                &_ => {
                    current_user_path = format!("/home/{}", current_user);
        
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
        
                    let source_files_path = format!("{}/{}", current_folder_path, format_jvm_folder_name);
        
                    let move_the_source_files = Command::new("sudo")
                                                                                .arg("mv")
                                                                                .arg(source_files_path)
                                                                                .arg(&current_user_path)
                                                                                .output();
        
                    match move_the_source_files {
                        Ok(_) => (),
                        Err(error) => {
                            eprintln!("cannot move the source file for this reason: {}", error);
                            exit(1)
                        }
                    }
        
                    env_path = format!("{}/{}", current_user_path, format_jvm_folder_name)
                }
            }
        
            println!("Source Files Downloaded Successfully");
        
            // in alma linux 9, terminal commands for checking existence of a folder always return success value.
            // because of that, we have to use std::path::PATH api.
        
            let incli_paths_path = format!("{}/INCLI_PATHS", current_user_path);
        
            let check_if_incli_paths_exist = Path::new(&incli_paths_path);
        
            if !check_if_incli_paths_exist.exists() {
                utils::configure_incli_envs_file(env_confs, &current_user, true)
            }
        
            let incli_envs_path = format!("{}/incli-envs.sh", incli_paths_path);
        
            let incli_envs_file = fs::OpenOptions::new().append(true).open(incli_envs_path);
        
            match incli_envs_file {
                Ok(mut file) => {
                    let line_for_append_2 = format!("\nJAVA_HOME=\"{}\"\n", env_path);
                    let line_for_append_2 = line_for_append_2.as_bytes();

                    let line_for_append_1 = format!("\nPATH=\"{}/bin:$PATH\"\n", env_path);
                    let line_for_append_1 = line_for_append_1.as_bytes();

                    let add_env_file_dest = io::Write::write_all(&mut file, line_for_append_2);
        
                    match add_env_file_dest {
                        Ok(_) => println!("JAVA_HOME env successfully added on your user."),
                        Err(err) => eprintln!("This error occured when we try to set JAVA_HOME env: {}", err)
                    }
                
                    let add_env_file_dest = io::Write::write_all(&mut file, line_for_append_1);
        
                    match add_env_file_dest {
                        Ok(_) => {
                            println!("envs successfully added on your user.");
                            println!("You're successfully installed Java {}. You can check it via typing 'java -version' later than open a new terminal. If it doesn't work, try it later than restart your computer.", version)
                        },
                        Err(err) => eprintln!("This error occured: {}", err)
                    }
                },
                Err(err) => {
                    eprintln!("Cannot open incli_envs.sh file for that reason: {}", err);
                    println!("Because of that we couldn't set env's. You can set your env's manually if you want.")
                }
            }
    }
}

pub fn install_java_on_centos_and_fedora(env_confs: &models::EnvConfiguration, url: &str, file_name: &str, version: &str) {
    println!("Welcome to incli. Your request to install Java on a Red Hat Based Distro Reached.");

    let check_if_java_installed = std::process::Command::new("java")
                                                                                .arg("-version")
                                                                                .output();

    match check_if_java_installed {
        Ok(_) => {
            println!("It seems you already installed java. Before you proceed, please delete it from your system.");
            println!("If you don't installed it by yourself, probably it's preinstalled by dnf.");
            println!("you can check if you have any of the java version installed by dnf.");
            return;
        },
        Err(_) => ()
    }

    if version == "8" {
        let install_java_8 = Command::new("sudo")
                                                    .arg("yum")
                                                    .arg("install")
                                                    .arg("java-1.8.0-openjdk")
                                                    .arg("-y")
                                                    .output();

        match install_java_8 {
            Ok(_) => {
                println!("You're successfully installed java 8, you can check it via typing 'java -version' on your terminal later than reopen it.");
                exit(0);
            },
            Err(error) => {
                println!("that error occured when installing Java 8: {}", error);
                exit(1);
            }
        }
    }

    if version == "9" ||
       version == "10" || 
       version == "11" || 
       version == "12" || 
       version == "13" || 
       version == "14" || 
       version == "15" || 
       version == "16" ||
       version == "17" ||
       version == "18" ||
       version == "19" ||
       version == "20" ||
       version == "21" ||
       version == "22" ||
       version == "23" ||
       version == "24" {
            let format_jvm_folder_name = match version {
                "9" => "jdk-9.0.4",
                "10" => "jdk-10.0.2",
                "11" => "jdk-11.0.2",
                "12" => "jdk-12.0.2",
                "13" => "jdk-13.0.2",
                "14" => "jdk-14.0.2",
                "15" => "jdk-15.0.2",
                "16" => "jdk-16.0.2",
                "17" => "jdk-17.0.2",
                "18" => "jdk-18.0.2",
                "19" => "jdk-19.0.1",
                "20" => "jdk-20.0.2",
                "21" => "jdk-21.0.2",
                "22" => "jdk-22.0.1",
                "23" => "jdk-23",
                "24" => "jdk-24",
                &_ => ""
            };

            let current_user = get_current_user();
            let env_path;
            let current_user_path;
        
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
                    current_user_path = "root".to_string();
        
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
        
                    let source_files_path = format!("{}/{}", current_folder_path, format_jvm_folder_name);
        
                    let move_the_source_files = Command::new("sudo")
                                                                                .arg("mv")
                                                                                .arg(source_files_path)
                                                                                .arg(&current_user_path)
                                                                                .output();
        
                    match move_the_source_files {
                        Ok(_) => (),
                        Err(error) => {
                            eprintln!("cannot move the source file for this reason: {}", error);
                            exit(1)
                        }
                    }
        
                    env_path = format!("/root/{}", format_jvm_folder_name)
                },
                &_ => {
                    current_user_path = format!("/home/{}", current_user);
        
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
        
                    let source_files_path = format!("{}/{}", current_folder_path, format_jvm_folder_name);
        
                    let move_the_source_files = Command::new("sudo")
                                                                                .arg("mv")
                                                                                .arg(source_files_path)
                                                                                .arg(&current_user_path)
                                                                                .output();
        
                    match move_the_source_files {
                        Ok(_) => (),
                        Err(error) => {
                            eprintln!("cannot move the source file for this reason: {}", error);
                            exit(1)
                        }
                    }
        
                    env_path = format!("{}/{}", current_user_path, format_jvm_folder_name);
                }
            }
        
            let incli_paths_path = format!("{}/INCLI_PATHS", current_user_path);
        
            let check_if_incli_paths_exist = Path::new(&incli_paths_path);
        
            if !check_if_incli_paths_exist.exists() {
                utils::configure_incli_envs_file(env_confs, &current_user, true)
            }
        
            let incli_envs_path = format!("{}/incli-envs.sh", incli_paths_path);
        
            let incli_envs_file = fs::OpenOptions::new().append(true).open(incli_envs_path);
        
            match incli_envs_file {
                Ok(mut file) => {
                    let line_for_append_2 = format!("\nexport JAVA_HOME=\"{}\"\n", env_path);
                    let line_for_append_2 = line_for_append_2.as_bytes();

                    let add_env_file_dest = io::Write::write_all(&mut file, line_for_append_2);
        
                    match add_env_file_dest {
                        Ok(_) => println!("JAVA_HOME env successfully added on your user's env's."),
                        Err(err) => eprintln!("This error occured when we try to set JAVA_HOME env's: {}", err)
                    }

                    let line_for_append_1 = format!("\nexport PATH=\"{}/bin:$PATH\"\n", env_path);
                    let line_for_append_1 = line_for_append_1.as_bytes();
                
                    let add_env_file_dest = io::Write::write_all(&mut file, line_for_append_1);
        
                    match add_env_file_dest {
                        Ok(_) => {
                            println!("envs successfully added on your user.");
                            println!("You're successfully installed Java on a Red Hat Based Distro. You can check it via typing 'java -version' later than open a new terminal. If it doesn't work, restart your computer and type it again.");
                        },
                        Err(err) => eprintln!("This error occured: {}", err)
                    }
                },
                Err(err) => {
                    eprintln!("Cannot open incli_envs.sh file for that reason: {}", err);
                    println!("We couldn't set env's due to the previously printed reason. You can set it manually.")
                }
            }
    }
}

pub fn install_java_on_rocky_linux(env_confs: &models::EnvConfiguration, url: &str, file_name: &str, version: &str) {
    println!("Welcome to incli. Your request to install Java on Rocky Linux Reached.");
    println!("If you want to install Java 8 on Rocky Linux, you have to make it as root user, otherwise installation won't work.");

    let check_if_java_installed = std::process::Command::new("java")
                                                                                    .arg("-version")
                                                                                    .output();

    match check_if_java_installed {
        Ok(_) => {
            println!("It seems you already installed java. Before you proceed, please delete it from your system.");
            println!("If you don't installed it by yourself, probably it's preinstalled by dnf.");
            println!("you can check if you have any of the java version installed by dnf.");
            return;
        },
        Err(_) => ()
    }

    if version == "8" {
        let install_java_8 = Command::new("yum")
                                                                .arg("install")
                                                                .arg("java-1.8.0-openjdk")
                                                                .arg("-y")
                                                                .output();

        match install_java_8 {
            Ok(_) => {
                println!("You're successfully installed java 8, you can check it via typing 'java -version' on your terminal later than reopen it.");
                exit(0);
            },
            Err(error) => {
                println!("that error occured when installing Java 8: {}", error);
                exit(1);
            }
        }
    }

    if version == "9" ||
       version == "10" || 
       version == "11" || 
       version == "12" || 
       version == "13" || 
       version == "14" || 
       version == "15" || 
       version == "16" ||
       version == "17" ||
       version == "18" ||
       version == "19" ||
       version == "20" ||
       version == "21" ||
       version == "22" ||
       version == "23" ||
       version == "24" {
            let format_jvm_folder_name = match version {
                "9" => "jdk-9.0.4",
                "10" => "jdk-10.0.2",
                "11" => "jdk-11.0.2",
                "12" => "jdk-12.0.2",
                "13" => "jdk-13.0.2",
                "14" => "jdk-14.0.2",
                "15" => "jdk-15.0.2",
                "16" => "jdk-16.0.2",
                "17" => "jdk-17.0.2",
                "18" => "jdk-18.0.2",
                "19" => "jdk-19.0.1",
                "20" => "jdk-20.0.2",
                "21" => "jdk-21.0.2",
                "22" => "jdk-22.0.1",
                "23" => "jdk-23",
                "24" => "jdk-24",
                &_ => ""
            };

            let current_user = get_current_user();
            let env_path;
            let current_user_path;
        
            let install_java = Command::new("wget")
                                                        .arg(url)
                                                        .arg("-O")
                                                        .arg(file_name)
                                                        .output()
                                                        .expect("Some Error Happened");
        
            if !install_java.status.success() {
                println!("Couldn't install Node.js Source Files Because Of Whatever reason.");
                exit(1);
            }
        
            match current_user.as_str() {
                "root" => {
                    current_user_path = "/root".to_string();
        
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

                    let source_files_path = format!("{}/{}", current_folder_path, format_jvm_folder_name);
        
                    let move_the_source_files = Command::new("mv")
                                                                                    .arg(source_files_path)
                                                                                    .arg(&current_user_path)
                                                                                    .output();
                    
                    match move_the_source_files {
                        Ok(_) => (),
                        Err(error) => {
                            eprintln!("cannot move the source file for this reason: {}", error);
                            exit(1)
                        }
                    }
        
                    env_path = format!("/root/{}", format_jvm_folder_name);
                },
                &_ => {
                    current_user_path = format!("/home/{}", current_user);
        
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
        
                    let source_files_path = format!("{}/{}", current_folder_path, format_jvm_folder_name);
        
                    let move_the_source_files = Command::new("mv")
                                                                                .arg(source_files_path)
                                                                                .arg(&current_user_path)
                                                                                .output();
        
                    match move_the_source_files {
                        Ok(_) => (),
                        Err(error) => {
                            eprintln!("cannot move the source file for this reason: {}", error);
                            exit(1)
                        }
                    }
        
                    env_path = format!("{}/{}", current_user_path, format_jvm_folder_name);
                }
            }
        
            let incli_paths_path = format!("{}/INCLI_PATHS", current_user_path);
        
            let check_if_incli_paths_exist = Path::new(&incli_paths_path);
        
            if !check_if_incli_paths_exist.exists() {
                utils::configure_incli_envs_file(env_confs, &current_user, true)
            }
        
            let incli_envs_path = format!("{}/incli-envs.sh", incli_paths_path);
        
            let incli_envs_file = fs::OpenOptions::new().append(true).open(incli_envs_path);
        
            match incli_envs_file {
                Ok(mut file) => {
                    let line_for_append_2 = format!("\nexport JAVA_HOME=\"{}\"", env_path);
                    let line_for_append_2 = line_for_append_2.as_bytes();

                    let add_env_file_dest = io::Write::write_all(&mut file, line_for_append_2);
        
                    match add_env_file_dest {
                        Ok(_) => println!("JAVA_HOME env successfully added on your user's env's."),
                        Err(err) => eprintln!("This error occured when we try to set JAVA_HOME env: {}", err)
                    }

                    let line_for_append_1 = format!("\nexport PATH=\"{}/bin:$PATH\"\n", env_path);
                    let line_for_append_1 = line_for_append_1.as_bytes();
                
                    let add_env_file_dest = io::Write::write_all(&mut file, line_for_append_1);
        
                    match add_env_file_dest {
                        Ok(_) => {
                            println!("envs successfully added on your user.");
                            println!("You're successfully installed Java on Rocky Linux. You can check it via typing 'java -version' later than open a new terminal. If it doesn't work, restart your computer and type it again.");
                        },
                        Err(err) => eprintln!("This error occured: {}", err)
                    }
                },
                Err(err) => {
                    eprintln!("Cannot open incli_envs.sh file for that reason: {}", err);
                    println!("We couldn't set env's due to the previously printed reason. You can set it manually.")
                }
            }


    }
}