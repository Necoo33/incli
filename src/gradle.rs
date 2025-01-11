use crate::models::EnvConfigurationError;
use crate::utils;
use crate::models;
use std::fmt::format;
use std::process::{Command, exit};
use std::fs;
use std::io;
use std::path::Path;
use sys_info_extended::get_current_user;

// indirme link şablonu şunlar:

// "https://services.gradle.org/distributions/gradle-8.10.2-bin.zip"
// "C:\Users\necdet\Downloads\gradle-8.10.2.zip"

pub fn install_gradle_on_windows(env_confs: &models::EnvConfiguration, version: &str) {
    println!("Welcome to incli, your request to install gradle on Windows reached. Please wait until it finish...");
    println!("Keep pressing any of your keys when you focused on your terminal in regular time period, otherwise your installation may not run correctly.");

    let version_input: String;
    if version.len() == 5 && version.ends_with("0") {
        let length_of_the_version = version.len();

        version_input = version.to_string().drain(length_of_the_version - 2..).as_str().to_string();
    } else if version.len() == 1 {
        version_input = format!("{}.0", version)
    } else {
        version_input = version.to_string()
    }

    let current_user = get_current_user();

    let url = format!("https://services.gradle.org/distributions/gradle-{}-bin.zip", version_input);
    let file_name = format!("gradle-{}", version_input);

    let get_downloads_path = format!("C:\\Users\\{}\\Downloads\\{}.zip", current_user, file_name);

    let download_gradle = Command::new("powershell")
                                                .arg("Invoke-WebRequest")
                                                .arg("-Uri")
                                                .arg(&url)
                                                .arg("-OutFile")
                                                .arg(&get_downloads_path)
                                                .output();
    
    match download_gradle {
        Ok(_) => {
            let zip_move_path = "C:\\Gradle";

            let format_the_command = format!("Expand-Archive -Path '{}' -DestinationPath '{}'", get_downloads_path, zip_move_path);

            let extract_the_zip_file = Command::new("powershell.exe")
                                                                        .arg("-Command")
                                                                        .arg(format_the_command)
                                                                        .output();

            match extract_the_zip_file {
                Ok(_) => {
                    println!("We're successfully extracted and moved the archive");

                    let format_remove_command = format!("Remove-Item -Path 'C:\\Users\\{}\\Downloads\\{}.zip' -Force", current_user, file_name);

                    std::process::Command::new("powershell.exe")
                                            .arg(format_remove_command)
                                            .output()
                                            .unwrap();
                },
                Err(error) => {
                    println!("This error occured when we extracting zip file: {}", error);
                    exit(1)
                }
            }
        },
        Err(error) => {
            println!("Either you typed a wrong version for gradle or your internet connection isn't exist.");
            println!("{}", error);
            exit(1)
        }
    }

    let env_path = format!("C:\\Gradle\\{}\\bin", file_name);

    utils::append_env_to_system_path_on_windows(&env_path);

    println!("Gradle {} successfully installed on your computer.", version);
    println!("Check it later than use");
    exit(0)
}

pub fn install_gradle_on_debian_based_distros(env_confs: &models::EnvConfiguration, version: &str) {
    println!("Welcome to incli. Your request to install Gradle on a debian based distro reached.");
    println!("Be sure you have installed wget on your pc, otherwise installation won't work.");

    let version_input: String;
    if version.len() == 5 && version.ends_with("0") {
        let length_of_the_version = version.len();

        version_input = version.to_string().drain(length_of_the_version - 2..).as_str().to_string();
    } else if version.len() == 1 {
        version_input = format!("{}.0", version)
    } else {
        version_input = version.to_string()
    }

    let url = format!("https://services.gradle.org/distributions/gradle-{}-bin.zip", version_input);
    let file_name = format!("gradle-{}", version_input);

    let current_user = get_current_user();

    let install_gradle = Command::new("wget")
                                    .arg(url)
                                    .arg("-O")
                                    .arg(format!("{}.zip", file_name))
                                    .output()
                                    .expect("Some Error Happened");

    if !install_gradle.status.success() {
        println!("Couldn't download Gradle Source Files Because Of Whatever reason.");
        exit(1);
    }

    let env_path;

    match current_user.as_str() {
        "root" => {
            let get_current_file_command = Command::new("pwd").output().unwrap();
    
            let file_for_moving = format!("{}/{}.zip", std::str::from_utf8(&get_current_file_command.stdout).unwrap().trim(), file_name);
    
            Command::new("mv").arg(file_for_moving).arg("/root").output().unwrap();
    
            let file_path = format!("/root/{}.zip", file_name);
    
            Command::new("sudo")
                        .arg("chmod")
                        .arg("755")
                        .arg(&file_path)
                        .output()
                        .expect("couldn't give 755 permission to source code.");
    
            println!("Source Files Downloaded Successfully");
    
            match utils::unzip(&file_path, "/root") {
                Ok(_) => {
                    println!("gradle-{}.zip file extracted successfully!", version)
                },
                Err(error) => {
                    println!("This error occured when extracting gradle-{}.zip file: {}", version, error);

                    exit(1)
                }
            }

            Command::new("sudo")
                        .arg("rm")
                        .arg("-rf")
                        .arg(file_path)
                        .output()
                        .expect("cannot delete archive");
    
            let mut estimated_path = format!("/root/{}", file_name);

            for entity in fs::read_dir(estimated_path.clone()).unwrap() {
                if entity.unwrap().path().file_name().unwrap().to_string_lossy() == file_name {
                    estimated_path = format!("{}/{}", estimated_path, file_name)
                }
            }

            env_path = estimated_path;
        },
        &_ => {
            let get_current_file_command = Command::new("pwd").output().unwrap();
    
            let file_for_moving = format!("{}/{}.zip", std::str::from_utf8(&get_current_file_command.stdout).unwrap().trim(), file_name);
    
            Command::new("mv").arg(&file_for_moving).arg(&env_confs.home_dir.to_string()).output().unwrap();
    
            let file_path = format!("{}/{}.zip", env_confs.home_dir.to_string(), file_name);
    
            Command::new("sudo")
                        .arg("chmod")
                        .arg("777")
                        .arg(&file_path)
                        .output()
                        .expect("couldn't give 755 permission to source code.");
    
            match utils::unzip(&file_path, &format!("{}/{}",env_confs.home_dir.to_string(), file_name)) {
                Ok(_) => {
                    println!("gradle-{}.zip file extracted successfully!", version)
                },
                Err(error) => {
                    println!("This error occured when extracting gradle-{}.zip file: {}", version, error);
            
                    exit(1)
                }
            }

            Command::new("sudo")
                        .arg("rm")
                        .arg("-rf")
                        .arg(file_path)
                        .output()
                        .expect("cannot delete archive");
    
            let mut estimated_path = format!("{}/{}", env_confs.home_dir.to_string(), file_name);

            for entity in fs::read_dir(estimated_path.clone()).unwrap() {
                if entity.unwrap().path().file_name().unwrap().to_string_lossy() == file_name {
                    estimated_path = format!("{}/{}", estimated_path, file_name);
                    break;
                }
            }
            
            env_path = estimated_path;
        }
    }

    match env_confs.add_debian_env_var("GRADLE_HOME", &env_path) {
        Ok(_) => println!("GRADLE_HOME env successfully added."),
        Err(error) => {
            match error {
                EnvConfigurationError::UnableToOpenUserShellFile(err) => println!("That error occured when we try to set your env's: {}", err),
                EnvConfigurationError::UnableToWriteUserShellFile(err) => println!("That error occured when we try to set your env's: {}", err),
                EnvConfigurationError::UnableToOpenSystemShellFile(err) => println!("That error occured when we try to set your env's: {}", err),
                EnvConfigurationError::UnableToWriteSystemShellFile(err) => println!("That error occured when we try to set your env's: {}", err),
                _ => println!("There is a bug!")
            }
        }
    }

    match env_confs.configure_debian_path_var(&format!("{}/bin", env_path)) {
        Ok(_) => {
            println!("PATH env successfully configured for Gradle.");
            
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

    /*let line_for_append_1 = format!("export GRADLE_HOME=\"{}\"\n",env_path);
    let line_for_append_2 = format!("export PATH=\"$GRADLE_HOME/bin:$PATH\"\n");
            
    let line_for_append_1 = line_for_append_1.as_bytes();
    let line_for_append_2 = line_for_append_2.as_bytes();
                
    let bashrc_file = fs::OpenOptions::new().append(true).open("/root/.bashrc");
            
    match bashrc_file {
        Ok(mut file) => {
            let add_env_1 = io::Write::write_all(&mut file, line_for_append_1);
            
            match add_env_1 {
                Ok(_) => println!("GRADLE_HOME env added successfully, adding it to PATH env..."),
                Err(error) => println!("And error occured when we try to add GRADLE_HOME env: {}", error)
            }

            let add_env_2 = io::Write::write_all(&mut file, line_for_append_2);
            
            match add_env_2 {
                Ok(_) => {
                    println!("PATH env updated successfully, Check it out by typing 'gradle -v' on terminal later than reopen it.")
                },
                Err(error) => println!("And error occured: {}", error)
            }
        },
        Err(_) => println!("cannot installed gradle for that reason: {}", env_path)
    }*/
}

pub fn install_gradle_on_arch_linux(env_confs: &models::EnvConfiguration, version: &str) {
    println!("Welcome to incli. Your request to Install Gradle on Arch Linux Reached. Please wait until installation finish.");
    
    let version_input: String;
    if version.len() == 5 && version.ends_with("0") {
        let length_of_the_version = version.len();

        version_input = version.to_string().drain(length_of_the_version - 2..).as_str().to_string();
    } else if version.len() == 1 {
        version_input = format!("{}.0", version)
    } else {
        version_input = version.to_string()
    }

    let url = format!("https://services.gradle.org/distributions/gradle-{}-bin.zip", version_input);
    let file_name = format!("gradle-{}", version_input);

    let current_user = get_current_user();

    let install_gradle = Command::new("wget")
                                    .arg(url)
                                    .arg("-O")
                                    .arg(format!("{}.zip", file_name))
                                    .output()
                                    .expect("Some Error Happened");

    if !install_gradle.status.success() {
        println!("Couldn't download Gradle Source Files Because Of Whatever reason.");
        exit(1);
    }

    Command::new("sudo")
                .arg("chmod")
                .arg("755")
                .arg(format!("{}.zip", file_name))
                .output()
                .expect("couldn't give 755 permission to source code.");

    let env_path;
    match current_user.as_str() {
        "root" => {
            let current_folder_path = Command::new("pwd").output().unwrap();
            let current_folder_path = std::str::from_utf8(&current_folder_path.stdout).unwrap().trim();

            let format_the_whole_file_path = format!("{}/{}.zip", current_folder_path, file_name);

            Command::new("sudo")
                        .arg("chmod")
                        .arg("777")
                        .arg(&format_the_whole_file_path)
                        .output()
                        .unwrap();

            match utils::unzip(&format_the_whole_file_path, "/root") {
                Ok(_) => {
                    println!("gradle-{}.zip file extracted successfully!", version)
                },
                Err(error) => {
                     println!("This error occured when extracting gradle-{}.zip file: {}", version, error);
            
                    exit(1)
                }
            }

            Command::new("sudo")
                        .arg("rm")
                        .arg("-rf")
                        .arg(format_the_whole_file_path)
                        .output()
                        .unwrap();

            let mut estimated_path = format!("/root/{}", file_name);

            for entity in fs::read_dir(estimated_path.clone()).unwrap() {
                if entity.unwrap().path().file_name().unwrap().to_string_lossy() == file_name {
                    estimated_path = format!("{}/{}", estimated_path, file_name)
                }
            }

            env_path = estimated_path
        },
        &_ => {
            let current_folder_path = Command::new("pwd").output().unwrap();
            let current_folder_path = std::str::from_utf8(&current_folder_path.stdout).unwrap().trim();

            let format_the_whole_file_path = format!("{}/{}.zip", current_folder_path, file_name);

            println!("your whole file path: {}", format_the_whole_file_path);

            Command::new("sudo")
                        .arg("chmod")
                        .arg("777")
                        .arg(&format_the_whole_file_path)
                        .output()
                        .unwrap();

            match utils::unzip(&format_the_whole_file_path, &env_confs.home_dir.to_string()) {
                Ok(_) => {
                    println!("gradle-{}.zip file extracted successfully!", version)
                },
                Err(error) => {
                    println!("This error occured when extracting gradle-{}.zip file: {}", version, error);
                        
                    exit(1)
                }
            }

            Command::new("sudo")
                        .arg("rm")
                        .arg("-rf")
                        .arg(format_the_whole_file_path)
                        .output()
                        .unwrap();

            let mut estimated_path = format!("{}/{}", env_confs.home_dir.to_string(), file_name);

            for entity in fs::read_dir(estimated_path.clone()).unwrap() {
                if entity.unwrap().path().file_name().unwrap().to_string_lossy() == file_name {
                    estimated_path = format!("{}/{}", estimated_path, file_name);
                    break;
                }
            }

            env_path = estimated_path
        }
    }

    Command::new("sudo")
                .arg("chmod")
                .arg("777")
                .arg(format!("{}/bin/gradle", env_path))
                .output()
                .expect("Cannot give 777 permission to gradle executable.");

    println!("Source Files Downloaded Successfully");

    match env_confs.add_arch_linux_env_var("GRADLE_HOME", &env_path) {
        Ok(_) => println!("GRADLE_HOME env successfully added."),
        Err(error) => {
            match error {
                EnvConfigurationError::UnableToOpenUserShellFile(err) => println!("That error occured when we try to set your env's: {}", err),
                EnvConfigurationError::UnableToWriteUserShellFile(err) => println!("That error occured when we try to set your env's: {}", err),
                EnvConfigurationError::UnableToOpenSystemShellFile(err) => println!("That error occured when we try to set your env's: {}", err),
                EnvConfigurationError::UnableToWriteSystemShellFile(err) => println!("That error occured when we try to set your env's: {}", err),
                _ => println!("There is a bug!")
            }
        }
    }

    match env_confs.configure_arch_linux_path_var(&current_user, &format!("{}/bin", env_path)) {
        Ok(_) => {
            println!("PATH env successfully configured for Gradle.");
            
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

    let check_if_incli_paths_exist = Path::new(&get_incli_paths_path);

    if !check_if_incli_paths_exist.exists() {
        println!("You don't have incli_envs.sh file yet. We're configuring it...");
        utils::configure_incli_envs_file(env_confs, &current_user, true)
    }

    let get_incli_envs_path = format!("{}/incli-envs.sh", get_incli_paths_path);

    let incli_envs_file = fs::OpenOptions::new().append(true).open(get_incli_envs_path);

    match incli_envs_file {
        Ok(mut file) => {
            let line_for_append_1 = format!("export GRADLE_HOME={}\n", env_path);
            let line_for_append_2 = format!("export PATH=$PATH:$GRADLE_HOME/bin\n");
                    
            let line_for_append_1 = line_for_append_1.as_bytes();
            let line_for_append_2 = line_for_append_2.as_bytes();
        
            let add_env_1 = io::Write::write_all(&mut file, line_for_append_1);

            match add_env_1 {
                Ok(_) => {
                    println!("GRADLE_HOME env successfully added on your user.");
                },
                Err(err) => eprintln!("This error occured: {}", err)
            }

            let add_env_2 = io::Write::write_all(&mut file, line_for_append_2);

            match add_env_2 {
                Ok(_) => {
                    println!("envs successfully added on your user.");
                    println!("Your installation of Gradle on arch linux ended successfully. You can check it via typing 'gradle -v' later than close current terminal. If you can't see any answer, try to restart the computer and check it again.");
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

pub fn install_gradle_on_alma_linux(env_confs: &models::EnvConfiguration, version: &str) {
    println!("Welcome to incli. Your request to install Gradle on Alma Linux Reached.");

    let version_input: String;
    if version.len() == 5 && version.ends_with("0") {
        let length_of_the_version = version.len();

        version_input = version.to_string().drain(length_of_the_version - 2..).as_str().to_string();
    } else if version.len() == 1 {
        version_input = format!("{}.0", version)
    } else {
        version_input = version.to_string()
    }

    let url = format!("https://services.gradle.org/distributions/gradle-{}-bin.zip", version_input);
    let file_name = format!("gradle-{}", version_input);

    let current_user = get_current_user();

    let install_gradle = Command::new("wget")
                                    .arg(url)
                                    .arg("-O")
                                    .arg(format!("{}.zip", file_name))
                                    .output()
                                    .expect("Some Error Happened");

    if !install_gradle.status.success() {
        println!("Couldn't download Gradle Source Files Because Of Whatever reason.");
        exit(1);
    }

    let env_path;

    match current_user.as_str() {
        "root" => {
            let current_folder_path = Command::new("pwd").output().unwrap();
            let current_folder_path = std::str::from_utf8(&current_folder_path.stdout).unwrap().trim();

            let format_the_whole_file_path = format!("{}/{}.zip", current_folder_path, file_name);

            Command::new("sudo")
                        .arg("chmod")
                        .arg("755")
                        .arg(&format_the_whole_file_path)
                        .output()
                        .expect("couldn't give 755 permission to source code.");

            match utils::unzip(&format_the_whole_file_path, &env_confs.home_dir.to_string()) {
                Ok(_) => {
                    println!("gradle-{}.zip file extracted successfully!", version)
                },
                Err(error) => {
                    println!("This error occured when extracting gradle-{}.zip file: {}", version, error);
                                    
                    exit(1)
                }
            }

            Command::new("sudo")
                        .arg("rm")
                        .arg("-rf")
                        .arg(format_the_whole_file_path)
                        .output()
                        .unwrap();

            let mut estimated_path = format!("/root/{}", file_name);

            for entity in fs::read_dir(estimated_path.clone()).unwrap() {
                if entity.unwrap().path().file_name().unwrap().to_string_lossy() == file_name {
                    estimated_path = format!("{}/{}", estimated_path, file_name);
                    break;
                }
            }

            env_path = estimated_path
        },
        &_ => {
            let current_folder_path = Command::new("pwd").output().unwrap();
            let current_folder_path = std::str::from_utf8(&current_folder_path.stdout).unwrap().trim();

            let format_the_whole_file_path = format!("{}/{}.zip", current_folder_path, file_name);

            Command::new("sudo")
                        .arg("chmod")
                        .arg("755")
                        .arg(&format_the_whole_file_path)
                        .output()
                        .expect("couldn't give 755 permission to source code.");

            match utils::unzip(&format_the_whole_file_path, &env_confs.home_dir.to_string()) {
                Ok(_) => {
                    println!("gradle-{}.zip file extracted successfully!", version)
                },
                Err(error) => {
                    println!("This error occured when extracting gradle-{}.zip file: {}", version, error);
                                    
                    exit(1)
                }
            }

            Command::new("sudo")
                        .arg("rm")
                        .arg("-rf")
                        .arg(format_the_whole_file_path)
                        .output()
                        .unwrap();

            let mut estimated_path = format!("{}/{}", env_confs.home_dir.to_string(), file_name);

            for entity in fs::read_dir(estimated_path.clone()).unwrap() {
                if entity.unwrap().path().file_name().unwrap().to_string_lossy() == file_name {
                    estimated_path = format!("{}/{}", estimated_path, file_name);
                    break
                }
            }
            
            env_path = estimated_path
        }
    }

    println!("Source Files Downloaded Successfully");

    Command::new("sudo")
                .arg("chmod")
                .arg("755")
                .arg(format!("{}/bin/gradle", env_path))
                .output()
                .expect("Cannot give 755 permission to gradle executable.");

    match env_confs.add_alma_linux_env_var("GRADLE_HOME", &env_path) {
        Ok(_) => println!("GRADLE_HOME env successfully added."),
        Err(error) => {
            match error {
                EnvConfigurationError::UnableToOpenUserShellFile(err) => println!("That error occured when we try to set your env's: {}", err),
                EnvConfigurationError::UnableToWriteUserShellFile(err) => println!("That error occured when we try to set your env's: {}", err),
                EnvConfigurationError::UnableToOpenSystemShellFile(err) => println!("That error occured when we try to set your env's: {}", err),
                EnvConfigurationError::UnableToWriteSystemShellFile(err) => println!("That error occured when we try to set your env's: {}", err),
                _ => println!("There is a bug!")
            }
        }
    }
                
    match env_confs.configure_alma_linux_path_var(&current_user, &format!("{}/bin", env_path)) {
        Ok(_) => {
        println!("PATH env successfully configured for Gradle.");
                            
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
            let line_for_append_1 = format!("\nGRADLE_HOME=\"{}\"\n", env_path);
            let line_for_append_2 = "\nPATH=\"$GRADLE_HOME/bin:$PATH\"\n";
                    
            let line_for_append_1 = line_for_append_1.as_bytes();
            let line_for_append_2 = line_for_append_2.as_bytes();
        
            let line_for_append_1 = io::Write::write_all(&mut file, line_for_append_1);

            match line_for_append_1 {
                Ok(_) => {
                    println!("envs successfully added on your user.");
                    println!("You're successfully installed gradle. You can check it via typing 'gradle -v' later than open a new terminal. If it doesn't work, try it later than restart your computer.")
                },
                Err(err) => eprintln!("This error occured: {}", err)
            }

            let line_for_append_2 = io::Write::write_all(&mut file, line_for_append_2);

            match line_for_append_2 {
                Ok(_) => {
                    println!("envs successfully added on your user.");
                    println!("You're successfully installed gradle. You can check it via typing 'gradle -v' later than open a new terminal. If it doesn't work, try it later than restart your computer.")
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

pub fn install_gradle_on_centos_and_fedora(env_confs: &models::EnvConfiguration, version: &str) {
    println!("Welcome to incli. Your request to install Gradle on a Red Hat Based Distro Reached.");

    let version_input: String;
    if version.len() == 5 && version.ends_with("0") {
        let length_of_the_version = version.len();

        version_input = version.to_string().drain(length_of_the_version - 2..).as_str().to_string();
    } else if version.len() == 1 {
        version_input = format!("{}.0", version)
    } else {
        version_input = version.to_string()
    }

    let url = format!("https://services.gradle.org/distributions/gradle-{}-bin.zip", version_input);
    let file_name = format!("gradle-{}", version_input);

    let current_user = get_current_user();

    let install_gradle = Command::new("wget")
                                    .arg(url)
                                    .arg("-O")
                                    .arg(format!("{}.zip", file_name))
                                    .output()
                                    .expect("Some Error Happened");

    if !install_gradle.status.success() {
        println!("Couldn't download Gradle Source Files Because Of Whatever reason.");
        exit(1);
    }

    let env_path;
    match current_user.as_str() {
        "root" => {
            let current_folder_path = Command::new("pwd").output().unwrap();
            let current_folder_path = std::str::from_utf8(&current_folder_path.stdout).unwrap().trim();

            let format_the_whole_file_path = format!("{}/{}.zip", current_folder_path, file_name);

            Command::new("sudo")
                        .arg("chmod")
                        .arg("755")
                        .arg(&format_the_whole_file_path)
                        .output()
                        .expect("couldn't give 755 permission to source code.");

            match utils::unzip(&format_the_whole_file_path, &env_confs.home_dir.to_string()) {
                Ok(_) => {
                    println!("gradle-{}.zip file extracted successfully!", version)
                },
                Err(error) => {
                    println!("This error occured when extracting gradle-{}.zip file: {}", version, error);
                                                
                    exit(1)
                }
            }
                                                            
            Command::new("sudo")
                        .arg("rm")
                        .arg("-rf")
                        .arg(format_the_whole_file_path)
                        .output()
                        .unwrap();

            let mut estimated_path = format!("/root/{}", file_name);

            for entity in fs::read_dir(estimated_path.clone()).unwrap() {
                if entity.unwrap().path().file_name().unwrap().to_string_lossy() == file_name {
                    estimated_path = format!("{}/{}", estimated_path, file_name);
                    break
                }
            }
            
            env_path = estimated_path
        },
        &_ => {
            let current_folder_path = Command::new("pwd").output().unwrap();
            let current_folder_path = std::str::from_utf8(&current_folder_path.stdout).unwrap().trim();

            let format_the_whole_file_path = format!("{}/{}.zip", current_folder_path, file_name);

            Command::new("sudo")
                        .arg("chmod")
                        .arg("755")
                        .arg(&format_the_whole_file_path)
                        .output()
                        .expect("couldn't give 755 permission to source code.");

            match utils::unzip(&format_the_whole_file_path, &env_confs.home_dir.to_string()) {
                Ok(_) => {
                    println!("gradle-{}.zip file extracted successfully!", version)
                },
                Err(error) => {
                    println!("This error occured when extracting gradle-{}.zip file: {}", version, error);
                                                            
                    exit(1)
                }
            }
                                                            
            Command::new("sudo")
                        .arg("rm")
                        .arg("-rf")
                        .arg(format_the_whole_file_path)
                        .output()
                        .unwrap();

            let mut estimated_path = format!("{}/{}", env_confs.home_dir.to_string(), file_name);

            for entity in fs::read_dir(estimated_path.clone()).unwrap() {
                if entity.unwrap().path().file_name().unwrap().to_string_lossy() == file_name {
                    estimated_path = format!("{}/{}", estimated_path, file_name);
                    break
                }
            }
            
            env_path = estimated_path
        }
    }

    Command::new("sudo")
            .arg("chmod")
            .arg("755")
            .arg(format!("{}", env_path))
            .output()
            .expect("couldn't give 755 permission to gradle main folder.");

    Command::new("sudo")
            .arg("chmod")
            .arg("755")
            .arg(format!("{}/bin", env_path))
            .output()
            .expect("couldn't give 755 permission to gradle bin folder.");

    Command::new("sudo")
                .arg("chmod")
                .arg("755")
                .arg(format!("{}/bin/gradle", env_path))
                .output()
                .expect("couldn't give 755 permission to gradle executable.");

    match env_confs.add_alma_linux_env_var("GRADLE_HOME", &env_path) {
        Ok(_) => println!("GRADLE_HOME env successfully added."),
        Err(error) => {
            match error {
                EnvConfigurationError::UnableToOpenUserShellFile(err) => println!("That error occured when we try to set your env's: {}", err),
                EnvConfigurationError::UnableToWriteUserShellFile(err) => println!("That error occured when we try to set your env's: {}", err),
                EnvConfigurationError::UnableToOpenSystemShellFile(err) => println!("That error occured when we try to set your env's: {}", err),
                EnvConfigurationError::UnableToWriteSystemShellFile(err) => println!("That error occured when we try to set your env's: {}", err),
                _ => println!("There is a bug!")
            }
        }
    }
                
    match env_confs.configure_alma_linux_path_var(&current_user, &format!("{}/bin", env_path)) {
        Ok(_) => {
            println!("PATH env successfully configured for Gradle.");
                            
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
            let line_for_append_1 = format!("\nexport GRADLE_HOME=\"{}\"\n", env_path);
            let line_for_append_2 = format!("\nexport PATH=\"$GRADLE_HOME/bin:$PATH\"\n");
            
            let line_for_append_1 = line_for_append_1.as_bytes();
            let line_for_append_2 = line_for_append_2.as_bytes();
        
            let line_for_append_1 = io::Write::write_all(&mut file, line_for_append_1);

            match line_for_append_1 {
                Ok(_) => {
                    println!("envs successfully added on your user.");
                    println!("You're successfully installed Go on a Red Hat Based Distro. You can check it via typing 'go version' later than open a new terminal. If it doesn't work, restart your computer and type it again.");
                },
                Err(err) => eprintln!("This error occured: {}", err)
            }

            let line_for_append_2 = io::Write::write_all(&mut file, line_for_append_2);

            match line_for_append_2 {
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

pub fn install_gradle_on_rocky_linux(env_confs: &models::EnvConfiguration, version: &str) {
    println!("Welcome to incli. Your request to install Go on Rocky Linux Reached.");

    let version_input: String;
    if version.len() == 5 && version.ends_with("0") {
        let length_of_the_version = version.len();

        version_input = version.to_string().drain(length_of_the_version - 2..).as_str().to_string();
    } else if version.len() == 1 {
        version_input = format!("{}.0", version)
    } else {
        version_input = version.to_string()
    }

    let url = format!("https://services.gradle.org/distributions/gradle-{}-bin.zip", version_input);
    let file_name = format!("gradle-{}", version_input);

    let current_user = get_current_user();

    let install_gradle = Command::new("wget")
                                    .arg(url)
                                    .arg("-O")
                                    .arg(format!("{}.zip", file_name))
                                    .output()
                                    .expect("Some Error Happened");

    if !install_gradle.status.success() {
        println!("Couldn't download Gradle Source Files Because Of Whatever reason.");
        exit(1);
    }

    let env_path;

    match current_user.as_str() {
        "root" => {
            let current_folder_path = Command::new("pwd").output().unwrap();
            let current_folder_path = std::str::from_utf8(&current_folder_path.stdout).unwrap().trim();

            let format_the_whole_file_path = format!("{}/{}.zip", current_folder_path, file_name);

            Command::new("chmod")
                        .arg("755")
                        .arg(&format_the_whole_file_path)
                        .output()
                        .expect("couldn't give 755 permission to source code.");

            match utils::unzip(&format_the_whole_file_path, &env_confs.home_dir.to_string()) {
                Ok(_) => {
                    println!("gradle-{}.zip file extracted successfully!", version)
                },
                Err(error) => {
                    println!("This error occured when extracting gradle-{}.zip file: {}", version, error);
                                                            
                    exit(1)
                }
            }
                                                            
            Command::new("rm")
                        .arg("-rf")
                        .arg(format_the_whole_file_path)
                        .output()
                        .unwrap();

            let mut estimated_path = format!("/root/{}", file_name);

            for entity in fs::read_dir(estimated_path.clone()).unwrap() {
                if entity.unwrap().path().file_name().unwrap().to_string_lossy() == file_name {
                    estimated_path = format!("{}/{}", estimated_path, file_name);
                    break
                }
            }
            
            env_path = estimated_path
        },
        &_ => {
            let current_folder_path = Command::new("pwd").output().unwrap();
            let current_folder_path = std::str::from_utf8(&current_folder_path.stdout).unwrap().trim();

            let format_the_whole_file_path = format!("{}/{}.zip", current_folder_path, file_name);

            Command::new("chmod")
                        .arg("755")
                        .arg(&format_the_whole_file_path)
                        .output()
                        .expect("couldn't give 755 permission to source code.");

            match utils::unzip(&format_the_whole_file_path, &env_confs.home_dir.to_string()) {
                Ok(_) => {
                    println!("gradle-{}.zip file extracted successfully!", version)
                },
                Err(error) => {
                    println!("This error occured when extracting gradle-{}.zip file: {}", version, error);
                                                            
                    exit(1)
                }
            }
                                                            
            Command::new("rm")
                        .arg("-rf")
                        .arg(format_the_whole_file_path)
                        .output()
                        .unwrap();

            let mut estimated_path = format!("{}/{}", env_confs.home_dir.to_string(), file_name);

            for entity in fs::read_dir(estimated_path.clone()).unwrap() {
                if entity.unwrap().path().file_name().unwrap().to_string_lossy() == file_name {
                    estimated_path = format!("{}/{}", estimated_path, file_name);
                    break
                }
            }
            
            env_path = estimated_path
        }
    }

    Command::new("chmod")
                .arg("755")
                .arg(format!("{}/bin/gradle", env_path))
                .output()
                .expect("couldn't give 755 permission to gradle executable.");

    match env_confs.add_rocky_linux_env_var("GRADLE_HOME", &env_path) {
        Ok(_) => println!("GRADLE_HOME successfully added to your env's."),
        Err(error) => {
            match error {
                EnvConfigurationError::UnableToOpenUserShellFile(err) => println!("That error occured when we try to set your env's: {}", err),
                EnvConfigurationError::UnableToWriteUserShellFile(err) => println!("That error occured when we try to set your env's: {}", err),
                EnvConfigurationError::UnableToOpenSystemShellFile(err) => println!("That error occured when we try to set your env's: {}", err),
                EnvConfigurationError::UnableToWriteSystemShellFile(err) => println!("That error occured when we try to set your env's: {}", err),
                _ => println!("There is a bug!")
            }
        }
    }
    
    match env_confs.configure_rocky_linux_path_var(&current_user, &format!("{}/bin",env_path)) {
        Ok(_) => {
            println!("Envs's successfully configured for Gradle.");
                        
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
            let line_for_append_1 = format!("\nexport GRADLE_HOME=\"{}\"\n", env_path);
            let line_for_append_2 = format!("\nexport PATH=\"$GRADLE_HOME/bin:$PATH\"\n");
            
            let line_for_append_1 = line_for_append_1.as_bytes();
            let line_for_append_2 = line_for_append_2.as_bytes();
        
            let line_for_append_1 = io::Write::write_all(&mut file, line_for_append_1);

            match line_for_append_1 {
                Ok(_) => {
                    println!("envs successfully added on your user.");
                    println!("You're successfully installed Go on a Red Hat Based Distro. You can check it via typing 'go version' later than open a new terminal. If it doesn't work, restart your computer and type it again.");
                },
                Err(err) => eprintln!("This error occured: {}", err)
            }

            let line_for_append_2 = io::Write::write_all(&mut file, line_for_append_2);

            match line_for_append_2 {
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

// 982 satır