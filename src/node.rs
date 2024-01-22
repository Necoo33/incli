use crate::utils;
use std::process::{Command, exit};
use std::fs;
use std::io;
use std::path::Path;

// Node.js Functions ----------------------------------------------------------

// url: "https://nodejs.org/dist/v20.10.0/node-v20.10.0-linux-x64.tar.xz"
// tar.gz: node-v20.10.0-linux-x64.tar.xz

// i don't know why but in arch linux we can't give 755 permissions.

pub fn install_nodejs_on_debian_based_distros(url: &str, file_name: &str) {
    println!("Welcome to incli. Your request to install Node.js on Linux Reached.");
    println!("Be sure you have wget and xz-utils installed if you use debian and kali linux, otherwise this installation won't work.");
    println!("Be sure you're running that installation on your user's root directory, otherwise you have to set your env's manually.");

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

    println!("Source Files Downloaded Successfully");

    let extract_tar_file = Command::new("sudo")
                                                    .arg("tar")
                                                    .arg("xvf")
                                                    .arg(file_name)
                                                    .output()
                                                    .expect("cannot extract source file");

    if extract_tar_file.status.success() {
        println!("source files successfully extracted, trying to add it on env's...")
    }

    let slice_of_file_name = &file_name[0..file_name.len() - 7];

    let env_path = format!("$HOME/{}/bin", slice_of_file_name);

    let line_for_append = format!("export PATH=\"{}:$PATH\"", env_path);

    let line_for_append = line_for_append.as_bytes();
    
    let bashrc_file = fs::OpenOptions::new().append(true).open(".bashrc");

    match bashrc_file {
        Ok(mut file) => {
            let add_env = io::Write::write_all(&mut file, line_for_append);

            match add_env {
                Ok(_) => println!("Node.js successfully added on env's. You can try it by restarting your computer and typing 'node --version' on command line."),
                Err(error) => println!("And error occured: {}", error)
            }
        },
        Err(_) => println!("there is no .bashrc file on current folder, we cannot set env's. You can do it manually, Node.js installed on: {}", env_path)
    }
}

pub fn install_nodejs_on_arch_linux(url: &str, file_name: &str){
    println!("Welcome to incli. Your request to install Node.js on Arch Linux Reached.");
    println!("Be sure you're running that installation on your user's root directory, otherwise you have to set your env's manually.");

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

    println!("Source Files Downloaded Successfully");

    let extract_tar_file = Command::new("sudo")
                                                    .arg("tar")
                                                    .arg("xvf")
                                                    .arg(file_name)
                                                    .output()
                                                    .expect("cannot extract source file");

    if extract_tar_file.status.success() {
        println!("source files successfully extracted, trying to add it on env's...")
    }

    let check_if_incli_paths_exist = Command::new("cd").arg("INCLI_PATHS").output();

    match check_if_incli_paths_exist {
        Ok(_) => (),
        Err(_) => {
            println!("You don't have incli_envs.sh file yet. We're configuring it...");
            utils::configure_incli_envs_file()
        }
    }

    let incli_envs_file = fs::OpenOptions::new().append(true).open("./INCLI_PATHS/incli-envs.sh");

    match incli_envs_file {
        Ok(mut file) => {
            let slice_of_file_name = &file_name[0..file_name.len() - 7];

            let env_path = format!("$HOME/{}/bin", slice_of_file_name);
        
            let line_for_append = format!("export PATH=$PATH:{}", env_path);
            let line_for_append = line_for_append.as_bytes();
        
            let add_env_file_dest = io::Write::write_all(&mut file, line_for_append);

            match add_env_file_dest {
                Ok(_) => println!("envs successfully added on your user."),
                Err(err) => eprintln!("This error occured: {}", err)
            }
        },
        Err(err) => {
            eprintln!("Cannot open incli_envs.sh file for that reason: {}", err)
        }
    }
    
}

// bu dosyada "configure_incli_envs_file" ufulesini kullanarak bu dosyayı ekleyebilir ve env'leri bu vasıtayla ekleyebilirsin.
// mesela incli-envs.sh dosyasının içine şunu yazarsan: PATH="$HOME/incli_path:$PATH" home directory'deki "incli_path" dosyasını
// env'lere ekleyecekdir. 

pub fn install_nodejs_on_alma_linux(url: &str, file_name: &str){
    println!("Welcome to incli. Your request to install Node.js on Alma Linux Reached.");
    println!("Be sure you have installed xz-utils in your pc, otherwise installation won't work.");
    println!("Be sure you're running that installation on your user's root directory, otherwise you have to set your env's manually.");

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

    println!("Source Files Downloaded Successfully");

    let extract_tar_file = Command::new("sudo")
                                        .arg("tar")
                                        .arg("xvf")
                                        .arg(file_name)
                                        .output()
                                        .expect("cannot extract source file");

    if extract_tar_file.status.success() {
        println!("source files successfully extracted, trying to add it on env's...")
    }

    // in alma linux 9, terminal commands for checking existence of a folder always return success value.
    // because of that, we have to use std::path::PATH api.

    let check_if_incli_paths_exist = Path::new("INCLI_PATHS");

    if !check_if_incli_paths_exist.exists() {
        utils::configure_incli_envs_file()
    }

    let incli_envs_file = fs::OpenOptions::new().append(true).open("./INCLI_PATHS/incli-envs.sh");

    match incli_envs_file {
        Ok(mut file) => {
            let slice_of_file_name = &file_name[0..file_name.len() - 7];

            let env_path = format!("$HOME/{}/bin", slice_of_file_name);
        
            let line_for_append = format!("PATH=\"{}:$PATH\"", env_path);
            let line_for_append = line_for_append.as_bytes();
        
            let add_env_file_dest = io::Write::write_all(&mut file, line_for_append);

            match add_env_file_dest {
                Ok(_) => println!("envs successfully added on your user."),
                Err(err) => eprintln!("This error occured: {}", err)
            }
        },
        Err(err) => {
            eprintln!("Cannot open incli_envs.sh file for that reason: {}", err)
        }
    }
}

pub fn install_nodejs_on_centos_and_fedora(url: &str, file_name: &str) {
    println!("Welcome to incli. Your request to install Node.js on a Red Hat Based Distro Reached.");
    println!("Be sure you're running that installation on your user's root directory, otherwise you have to set your env's manually.");

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

    println!("Source Files Downloaded Successfully");

    let extract_tar_file = Command::new("sudo")
                                                    .arg("tar")
                                                    .arg("xvf")
                                                    .arg(file_name)
                                                    .output()
                                                    .expect("cannot extract source file");

    if extract_tar_file.status.success() {
        println!("source files successfully extracted, trying to add it on env's...")
    }

    let check_if_incli_paths_exist = Path::new("INCLI_PATHS");

    if !check_if_incli_paths_exist.exists() {
        utils::configure_incli_envs_file()
    }

    let incli_envs_file = fs::OpenOptions::new().append(true).open("./INCLI_PATHS/incli-envs.sh");

    match incli_envs_file {
        Ok(mut file) => {
            let slice_of_file_name = &file_name[0..file_name.len() - 7];

            let env_path = format!("$HOME/{}/bin", slice_of_file_name);
        
            let line_for_append = format!("export PATH=\"{}:$PATH\"", env_path);
            let line_for_append = line_for_append.as_bytes();
        
            let add_env_file_dest = io::Write::write_all(&mut file, line_for_append);

            match add_env_file_dest {
                Ok(_) => println!("envs successfully added on your user."),
                Err(err) => eprintln!("This error occured: {}", err)
            }
        },
        Err(err) => {
            eprintln!("Cannot open incli_envs.sh file for that reason: {}", err)
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

pub fn install_nodejs_on_windows(url: &str, exe_name: &str) {
    println!("Welcome to incli. Your request to install Node.js on Windows Reached. ");

    let download_command = Command::new("powershell")
                                        .arg("Invoke-WebRequest")
                                        .arg("-Uri")
                                        .arg(url)
                                        .arg("-OutFile")
                                        .arg(exe_name)
                                        .output()
                                        .expect("Download failed.");

    if !download_command.status.success() {
        eprintln!("Failed to download {} for whatever reason. Exiting.", exe_name);
        exit(1);
    }

    println!("Download completed, you can continue installing node.js through the pop up which will open.");

    let install_command = Command::new("powershell")
                                            .arg(format!(".\\{}", exe_name))
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