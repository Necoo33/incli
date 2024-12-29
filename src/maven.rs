use crate::utils;
use crate::models;
use std::process::{Command, exit};
use std::fs;
use std::io;
use std::path::Path;
use sys_info_extended::{get_current_user, get_user_env_var, get_system_env_var, set_env, EnvOptions};

pub fn install_maven_on_windows(env_confs: &models::EnvConfiguration, url: &str, file_name: &str, version: &str) {
    println!("Welcome to incli, your request to install Maven on Windows reached. Please wait until it finish...");
    println!("Keep pressing any of your keys when you focused on your terminal in regular time period, otherwise your installation may not run correctly.");

    let current_user = get_current_user();

    let get_downloads_path = format!("C:\\Users\\{}\\Downloads\\{}", current_user, file_name);

    let download_maven = Command::new("powershell")
                                                .arg("Invoke-WebRequest")
                                                .arg("-Uri")
                                                .arg(&url)
                                                .arg("-OutFile")
                                                .arg(&get_downloads_path)
                                                .output();

    match download_maven {
        Ok(_) => println!("Maven source files successfully downloaded"),
        Err(error) => {
            println!("Couldn't Maven source files for that reason: {}", error);
            exit(1)
        }
    }

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

    let mut maven_home_env_path = String::new();
    let mut bin_env_path = String::new();

    match version.starts_with("1") {
        true => match version {
            "1" => { 
                maven_home_env_path = "C:\\Incli Downloads\\maven-1.0".to_string();
                bin_env_path = "C:\\Incli Downloads\\maven-1.0\\bin".to_string();
            },
            "1.0.0" => {
                maven_home_env_path = "C:\\Incli Downloads\\maven-1.0".to_string();
                bin_env_path = "C:\\Incli Downloads\\maven-1.0\\bin".to_string();
            },
            "1.1.0" => {
                maven_home_env_path = "C:\\Incli Downloads\\maven-1.1".to_string();
                bin_env_path = "C:\\Incli Downloads\\maven-1.1\\bin".to_string();
            },
            _ => {
                maven_home_env_path = format!("C:\\Incli Downloads\\maven-{}", version);
                bin_env_path = format!("C:\\Incli Downloads\\maven-{}\\bin", version);
            }
        },
        false => match version.starts_with("2") {
            true => match version {
                "2" => { 
                    maven_home_env_path = "C:\\Incli Downloads\\apache-maven-2.0".to_string();
                    bin_env_path = "C:\\Incli Downloads\\apache-maven-2.0\\bin".to_string();
                },
                "2.0.0" => {
                    maven_home_env_path = "C:\\Incli Downloads\\apache-maven-2.0".to_string();
                    bin_env_path = "C:\\Incli Downloads\\apache-maven-2.0\\bin".to_string();
                },
                "2.1" => {
                    maven_home_env_path = "C:\\Incli Downloads\\apache-maven-2.1.0".to_string();
                    bin_env_path = "C:\\Incli Downloads\\apache-maven-2.1.0\\bin".to_string();
                },
                "2.2" => {
                    maven_home_env_path = "C:\\Incli Downloads\\apache-maven-2.2.0".to_string();
                    bin_env_path = "C:\\Incli Downloads\\apache-maven-2.2.0\\bin".to_string();
                },
                _ => {
                    maven_home_env_path = format!("C:\\Incli Downloads\\apache-maven-{}", version);
                    bin_env_path = format!("C:\\Incli Downloads\\apache-maven-{}\\bin", version);
                }
            },
            false => match version {
                "3" => {
                    maven_home_env_path = "C:\\Incli Downloads\\apache-maven-3.0".to_string();
                    bin_env_path = "C:\\Incli Downloads\\apache-maven-3.0\\bin".to_string();
                },
                "3.0.0" => {
                    maven_home_env_path = "C:\\Incli Downloads\\apache-maven-3.0".to_string();
                    bin_env_path = "C:\\Incli Downloads\\apache-maven-3.0\\bin".to_string();
                },
                "3.1" => {
                    maven_home_env_path = "C:\\Incli Downloads\\apache-maven-3.1.0".to_string();
                    bin_env_path = "C:\\Incli Downloads\\apache-maven-3.1.0\\bin".to_string();
                },
                "3.2" => {
                    maven_home_env_path = "C:\\Incli Downloads\\apache-maven-3.2.0".to_string();
                    bin_env_path = "C:\\Incli Downloads\\apache-maven-3.2.0\\bin".to_string();
                },
                _ => {
                    maven_home_env_path = format!("C:\\Incli Downloads\\apache-maven-{}", version);
                    bin_env_path = format!("C:\\Incli Downloads\\apache-maven-{}\\bin", version);
                }
            }
        }
    }

    let env_options = EnvOptions {
        name: "MAVEN_HOME".to_string(),
        value: maven_home_env_path,
        level: sys_info_extended::EnvLevel::Machine
    };

    match set_env(env_options) {
        Ok(_) => println!("MAVEN_HOME variable added to the system Successfully."),
        Err(error) => println!("We cannot add MAVEN_HOME variable for that reason: {}", error)
    }

    utils::append_env_to_system_path_on_windows(&bin_env_path);

    match get_system_env_var("JAVA_HOME") {
        Ok(_)=> (),
        Err(_) => match get_user_env_var("JAVA_HOME") {
            Ok(_) => (),
            Err(_) => {
                println!("Warning: it seems java does not installed yet or just env's don't set properly. You should either install java or if you don't set, set the JAVA_HOME variable properly.");
                println!("If you don't know how to do it properly, you can use incli to properly install java with your demanded version, such as: './incli install java 11'")
            }
        }
    }

    println!("Maven {} successfully installed on your computer.", version);

    exit(0)
}

pub fn install_maven_on_debian_based_distros(env_confs: &models::EnvConfiguration, url: &str, file_name: &str, version: &str) {
    println!("Welcome to incli. Your request to install Maven on a debian based distro reached.");
    println!("Be sure you have installed wget on your pc, otherwise installation won't work.");

    let check_if_java_installed = std::process::Command::new("java")
                                                                                .arg("-version")
                                                                                .output();

    match check_if_java_installed {
        Ok(_) => println!("It seems you already installed java. Be sure you added 'JAVA_HOME' env added successfully."),
        Err(_) => ()
    }

    let current_user = get_current_user();

    let install_maven = Command::new("wget")
                                       .arg(url)
                                       .arg("-O")
                                       .arg(file_name)
                                       .output();

    match install_maven {
        Ok(_) => (),
        Err(error) => {
            println!("Couldn't install Maven Source Files Because Of That reason: {}", error);
            exit(1);
        }
    }

    let format_maven_folder_name = match file_name.contains("bin") {
        true => &file_name[..file_name.len() - 11],
        false => &file_name[..file_name.len() - 7]
    };

    let user_path;

    match current_user.as_str() {
        "root" => {
            user_path = "/root".to_string();

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
    
            let format_current_folder_again = format!("{}/{}", std::str::from_utf8(&current_folder_again.stdout).unwrap().trim(), format_maven_folder_name);
    
            Command::new("sudo")
                        .arg("mv")
                        .arg(format_current_folder_again)
                        .arg("/root")
                        .output()
                        .unwrap();
        },
        _ => {
            user_path = format!("/home/{}", current_user);

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
    
            let format_current_folder_again = format!("{}/{}", std::str::from_utf8(&current_folder_again.stdout).unwrap().trim(), format_maven_folder_name);

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
        }
    }

    let maven_home = format!("{}/{}", user_path, format_maven_folder_name);

    let line_for_append_1 = format!("\nexport MAVEN_HOME=\"{}\"\n", maven_home);
    let line_for_append_1 = line_for_append_1.as_bytes();

    let line_for_append_2 = format!("\nexport PATH=\"{}/bin:$PATH\"\n", maven_home);
    let line_for_append_2 = line_for_append_2.as_bytes();

    let bashrc_path = format!("{}/.bashrc", user_path);

    let bashrc_file = fs::OpenOptions::new().append(true).open(&bashrc_path);
                            
    match bashrc_file {
        Ok(mut file) => {
            let add_env = io::Write::write_all(&mut file, line_for_append_1);

            match add_env {
                Ok(_) => println!("MAVEN_HOME env successfully added on your user's envs."),
                Err(error) => println!("An error ocured when we try to set MAVEN_HOME env: {}", error)
            }

            let add_env = io::Write::write_all(&mut file, line_for_append_2);
            
            match add_env {
                Ok(_) => println!("Maven successfully added on env's. You can try it by restarting your computer and typing 'mvn -v' on command line."),
                Err(error) => println!("And error occured: {}", error)
            }
        },
        Err(error) => println!("couldn't open {} file for that reason: {}", bashrc_path, error)
    }

    // sanırım buraya kadarki kısmı doğru bir şekilde ayarlayabildim, artık gerisine evde bak.
    // env'lerin doğru bir şekilde ayarlanması lazım. 2.0.7'ye kadarki versiyonlar "maven-(versiyon numarası)" iken
    // ondan sonraki versiyonlar "apache-maven-(versiyon numarası)" şeklinde formatlanıyor.
}

pub fn install_maven_on_arch_linux(env_confs: &models::EnvConfiguration, url: &str, file_name: &str, version: &str) {
    println!("Welcome to incli. Your request to Install Maven on Arch Linux Reached. Please wait until installation finish.");

    let current_user = get_current_user();

    let install_maven = Command::new("wget")
                                       .arg(url)
                                       .arg("-O")
                                       .arg(file_name)
                                       .output();

    Command::new("sudo")
                                       .arg("chmod")
                                       .arg("755")
                                       .arg(file_name)
                                       .output()
                                       .expect("couldn't give 755 permission to source code.");

    match install_maven {
        Ok(_) => (),
        Err(error) => {
            println!("Couldn't install Maven Source Files Because Of That reason: {}", error);
            exit(1);
        }
    }

    let format_maven_folder_name = match file_name.contains("bin") {
        true => &file_name[..file_name.len() - 11],
        false => &file_name[..file_name.len() - 7]
    };

    let user_path;

    let maven_home;

    match current_user.as_str() {
        "root" => {
            user_path = "/root".to_string();

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

            let format_the_source_files_path = format!("{}/{}", current_folder_path, format_maven_folder_name);

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

            maven_home = format!("/root/{}", format_maven_folder_name)
        },
        &_ => {
            user_path = format!("/home/{}", current_user);

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

            let format_the_source_files_path = format!("{}/{}", current_folder_path, format_maven_folder_name);

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
                                                                                    .arg(&user_path)
                                                                                    .output();

            match move_the_source_files_to_root {
                Ok(_) => (),
                Err(error) => {
                    eprintln!("That error occured when moving source files: {}", error);
                    exit(1)
                }
            }

            maven_home = format!("{}/{}", user_path, format_maven_folder_name)
        }
    }

    println!("Source Files Downloaded Successfully");

    let get_incli_paths_path = format!("{}/INCLI_PATHS", user_path);

    let check_if_incli_paths_exist = /*Command::new("cd").arg(&get_incli_paths_path).output()*/Path::new(&get_incli_paths_path);

    if !check_if_incli_paths_exist.exists() {
        println!("You don't have incli_envs.sh file yet. We're configuring it...");
        utils::configure_incli_envs_file(env_confs, &current_user, true)
    }

    let get_incli_envs_path = format!("{}/incli-envs.sh", get_incli_paths_path);

    let incli_envs_file = fs::OpenOptions::new().append(true).open(get_incli_envs_path);

    match incli_envs_file {
        Ok(mut file) => {
            let line_for_append_1 = format!("\nexport MAVEN_HOME={}\n", maven_home);
            let line_for_append_1 = line_for_append_1.as_bytes();

            let add_maven_home_env_file_dest = io::Write::write_all(&mut file, line_for_append_1);

            match add_maven_home_env_file_dest {
                Ok(_) => println!("MAVEN_HOME env successfully installed on your env's"),
                Err(error) => println!("An error occured when we try to add MAVEN_HOME env to your user's env's: {}", error)
            }

            let line_for_append_2 = format!("\nexport PATH=$PATH:{}/bin\n", maven_home);
            let line_for_append_2 = line_for_append_2.as_bytes();
        
            let add_env_file_dest = io::Write::write_all(&mut file, line_for_append_2);

            match add_env_file_dest {
                Ok(_) => {
                    println!("envs successfully added on your user.");
                    println!("Your installation of Maven on arch linux ended successfully. You can check it via typing 'mvn -v' later than close current terminal. If you can't see any answer, try to restart the computer and check it again.");
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

pub fn install_maven_on_alma_linux(env_confs: &models::EnvConfiguration, url: &str, file_name: &str, version: &str) {
    println!("Welcome to incli. Your request to install Maven on Alma Linux Reached.");
        
    let current_user = get_current_user();
        
    let install_maven = Command::new("wget")
                                            .arg(url)
                                            .arg("-O")
                                            .arg(file_name)
                                            .output();
        
    match install_maven {
        Ok(_) => (),
        Err(error) => {
            println!("Couldn't install Maven Source Files Because Of That reason: {}", error);
            exit(1);
        }
    }

    let format_maven_folder_name = match file_name.contains("bin") {
        true => &file_name[..file_name.len() - 11],
        false => &file_name[..file_name.len() - 7]
    };

    let user_path;

    let maven_home;
        
    match current_user.as_str() {
        "root" => {
            user_path = "/root".to_string();
        
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
        
            let source_files_path = format!("{}/{}", current_folder_path, format_maven_folder_name);
        
            let move_the_source_files = Command::new("sudo")
                                                                                .arg("mv")
                                                                                .arg(source_files_path)
                                                                                .arg(&user_path)
                                                                                .output();
        
            match move_the_source_files {
                Ok(_) => (),
                Err(error) => {
                    eprintln!("cannot move the source file for this reason: {}", error);
                    exit(1)
                }
            }
        
            maven_home = format!("/root/{}", format_maven_folder_name);
        },
        &_ => {
            user_path = format!("/home/{}", current_user);
        
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
        
            let source_files_path = format!("{}/{}", current_folder_path, format_maven_folder_name);
        
            let move_the_source_files = Command::new("sudo")
                                                                                .arg("mv")
                                                                                .arg(source_files_path)
                                                                                .arg(&user_path)
                                                                                .output();
        
            match move_the_source_files {
                Ok(_) => (),
                Err(error) => {
                    eprintln!("cannot move the source file for this reason: {}", error);
                    exit(1)
                }
            }
        
            maven_home = format!("{}/{}", user_path, format_maven_folder_name)
        }
    }
        
    println!("Source Files Downloaded Successfully");
        
    // in alma linux 9, terminal commands for checking existence of a folder always return success value.
    // because of that, we have to use std::path::PATH api.
        
    let incli_paths_path = format!("{}/INCLI_PATHS", user_path);
        
    let check_if_incli_paths_exist = Path::new(&incli_paths_path);
        
    if !check_if_incli_paths_exist.exists() {
        utils::configure_incli_envs_file(env_confs, &current_user, true)
    }
        
    let incli_envs_path = format!("{}/incli-envs.sh", incli_paths_path);
        
    let incli_envs_file = fs::OpenOptions::new().append(true).open(incli_envs_path);
        
    match incli_envs_file {
        Ok(mut file) => {
            let line_for_append_1 = format!("\nMAVEN_HOME=\"{}\"\n", maven_home);
            let line_for_append_1 = line_for_append_1.as_bytes();

            let line_for_append_2 = format!("\nPATH=\"{}/bin:$PATH\"\n", maven_home);
            let line_for_append_2 = line_for_append_2.as_bytes();

            let add_env_file_dest = io::Write::write_all(&mut file, line_for_append_1);
        
            match add_env_file_dest {
                Ok(_) => println!("MAVEN_HOME env successfully added on your user."),
                Err(err) => eprintln!("This error occured when we try to set MAVEN_HOME env: {}", err)
            }
                
            let add_env_file_dest = io::Write::write_all(&mut file, line_for_append_2);
        
            match add_env_file_dest {
                Ok(_) => {
                    println!("envs successfully added on your user.");
                    println!("You're successfully installed Maven {}. You can check it via typing 'mvn -v' later than open a new terminal. If it doesn't work, try it later than restart your computer.", version)
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

pub fn install_maven_on_centos_and_fedora(env_confs: &models::EnvConfiguration, url: &str, file_name: &str, version: &str) {
    println!("Welcome to incli. Your request to install Maven on a Red Hat Based Distro Reached.");

    let check_if_maven_installed = std::process::Command::new("mvn")
                                                                                .arg("-v")
                                                                                .output();

    match check_if_maven_installed {
        Ok(_) => {
            println!("It seems you already installed maven. Before you proceed, please delete it from your system.");
            println!("If you don't installed it by yourself, probably it's preinstalled by dnf.");
            println!("you can check if you have any of the maven version installed by dnf.");
            return;
        },
        Err(_) => ()
    }

    let format_maven_folder_name = match file_name.contains("bin") {
        true => &file_name[..file_name.len() - 11],
        false => &file_name[..file_name.len() - 7]
    };

    let user_path;

    let maven_home;

    let current_user = get_current_user();
        
    let install_maven = Command::new("wget")
                                                        .arg(url)
                                                        .arg("-O")
                                                        .arg(file_name)
                                                        .output();
        
    match install_maven {
        Ok(_) => (),
        Err(error) => {
            println!("Couldn't install Maven Source Files Because Of That Reason: {}", error);
            exit(1);
        }
    }
        
    match current_user.as_str() {
        "root" => {
            user_path = "root".to_string();
        
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
        
            let source_files_path = format!("{}/{}", current_folder_path, format_maven_folder_name);
        
            let move_the_source_files = Command::new("sudo")
                                                                                .arg("mv")
                                                                                .arg(source_files_path)
                                                                                .arg(&user_path)
                                                                                .output();
        
            match move_the_source_files {
                Ok(_) => (),
                Err(error) => {
                    eprintln!("cannot move the source file for this reason: {}", error);
                    exit(1)
                }
            }
        
            maven_home = format!("/root/{}", format_maven_folder_name)
        },
        &_ => {
            user_path = format!("/home/{}", current_user);
        
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
        
            let source_files_path = format!("{}/{}", current_folder_path, format_maven_folder_name);
        
            let move_the_source_files = Command::new("sudo")
                                                                                .arg("mv")
                                                                                .arg(source_files_path)
                                                                                .arg(&user_path)
                                                                                .output();
        
            match move_the_source_files {
                Ok(_) => (),
                Err(error) => {
                    eprintln!("cannot move the source file for this reason: {}", error);
                    exit(1)
                }
            }
        
            maven_home = format!("{}/{}", user_path, format_maven_folder_name);
        }
    }
        
    let incli_paths_path = format!("{}/INCLI_PATHS", user_path);
        
    let check_if_incli_paths_exist = Path::new(&incli_paths_path);
        
    if !check_if_incli_paths_exist.exists() {
        utils::configure_incli_envs_file(env_confs, &current_user, true)
    }
        
    let incli_envs_path = format!("{}/incli-envs.sh", incli_paths_path);
        
    let incli_envs_file = fs::OpenOptions::new().append(true).open(incli_envs_path);
        
    match incli_envs_file {
        Ok(mut file) => {
            let line_for_append_1 = format!("\nexport MAVEN_HOME=\"{}\"\n", maven_home);
            let line_for_append_1 = line_for_append_1.as_bytes();

            let add_env_file_dest = io::Write::write_all(&mut file, line_for_append_1);
        
            match add_env_file_dest {
                Ok(_) => println!("MAVEN_HOME env successfully added on your user's env's."),
                Err(err) => eprintln!("This error occured when we try to set MAVEN_HOME env's: {}", err)
            }

            let line_for_append_2 = format!("\nexport PATH=\"{}/bin:$PATH\"\n", maven_home);
            let line_for_append_2 = line_for_append_2.as_bytes();
                
            let add_env_file_dest = io::Write::write_all(&mut file, line_for_append_2);
        
            match add_env_file_dest {
                Ok(_) => {
                    println!("envs successfully added on your user.");
                    println!("You're successfully installed Maven on a Red Hat Based Distro. You can check it via typing 'mvn -v' later than open a new terminal. If it doesn't work, restart your computer and type it again.");
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

pub fn install_maven_on_rocky_linux(env_confs: &models::EnvConfiguration, url: &str, file_name: &str, version: &str) {
    println!("Welcome to incli. Your request to install Maven on Rocky Linux Reached.");

    let check_if_maven_installed = std::process::Command::new("mvn")
                                                                                    .arg("-v")
                                                                                    .output();

    match check_if_maven_installed {
        Ok(_) => {
            println!("It seems you already installed maven. Before you proceed, please delete it from your system.");
            println!("If you don't installed it by yourself, probably it's preinstalled by dnf.");
            println!("you can check if you have any of the maven version installed by dnf.");
            return;
        },
        Err(_) => ()
    }

    let format_maven_folder_name = match file_name.contains("bin") {
        true => &file_name[..file_name.len() - 11],
        false => &file_name[..file_name.len() - 7]
    };

    let user_path;

    let maven_home;

    let current_user = get_current_user();
        
    let install_maven = Command::new("wget")
                                                        .arg(url)
                                                        .arg("-O")
                                                        .arg(file_name)
                                                        .output();
        
    match install_maven {
        Ok(_) => (),
        Err(error) => {
            println!("Couldn't install Maven Source Files Because Of That Reason: {}", error);
            exit(1);
        }
    }
        
    match current_user.as_str() {
        "root" => {
            user_path = "/root".to_string();
        
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

            let source_files_path = format!("{}/{}", current_folder_path, format_maven_folder_name);
        
            let move_the_source_files = Command::new("mv")
                                                                                    .arg(source_files_path)
                                                                                    .arg(&user_path)
                                                                                    .output();
                    
            match move_the_source_files {
                Ok(_) => (),
                Err(error) => {
                    eprintln!("cannot move the source file for this reason: {}", error);
                    exit(1)
                }
            }
        
            maven_home = format!("/root/{}", format_maven_folder_name);
        },
        &_ => {
            user_path = format!("/home/{}", current_user);
        
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
        
            let source_files_path = format!("{}/{}", current_folder_path, format_maven_folder_name);
        
            let move_the_source_files = Command::new("mv")
                                                                                .arg(source_files_path)
                                                                                .arg(&user_path)
                                                                                .output();
        
            match move_the_source_files {
                Ok(_) => (),
                Err(error) => {
                    eprintln!("cannot move the source file for this reason: {}", error);
                    exit(1)
                }
            }
        
            maven_home = format!("{}/{}", user_path, format_maven_folder_name);
        }
    }
        
    let incli_paths_path = format!("{}/INCLI_PATHS", user_path);
        
    let check_if_incli_paths_exist = Path::new(&incli_paths_path);
        
    if !check_if_incli_paths_exist.exists() {
        utils::configure_incli_envs_file(env_confs, &current_user, true)
    }
        
    let incli_envs_path = format!("{}/incli-envs.sh", incli_paths_path);
        
    let incli_envs_file = fs::OpenOptions::new().append(true).open(incli_envs_path);
        
    match incli_envs_file {
        Ok(mut file) => {
            let line_for_append_1 = format!("\nexport MAVEN_HOME=\"{}\"", maven_home);
            let line_for_append_1 = line_for_append_1.as_bytes();

            let add_env_file_dest = io::Write::write_all(&mut file, line_for_append_1);
        
            match add_env_file_dest {
                Ok(_) => println!("MAVEN_HOME env successfully added on your user's env's."),
                Err(err) => eprintln!("This error occured when we try to set MAVEN_HOME env: {}", err)
            }

            let line_for_append_2 = format!("\nexport PATH=\"{}/bin:$PATH\"\n", maven_home);
            let line_for_append_2 = line_for_append_2.as_bytes();
                
            let add_env_file_dest = io::Write::write_all(&mut file, line_for_append_2);
        
            match add_env_file_dest {
                Ok(_) => {
                    println!("envs successfully added on your user.");
                    println!("You're successfully installed Maven on Rocky Linux. You can check it via typing 'mvn -v' later than open a new terminal. If it doesn't work, restart your computer and type it again.");
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