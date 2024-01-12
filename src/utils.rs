// there is some platform-specific apis on that page:

#[cfg(target_os = "windows")]
use std::os::windows::process::ExitStatusExt;

#[cfg(target_os = "linux")]
use std::os::unix::process::ExitStatusExt;

use std::process::{Command, exit, Output, ExitStatus};
use std::fs::{File, self};
use std::io::{self, BufRead};
use std::path::Path;

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

pub fn install_rust_on_debian_based_distros(){
    println!("Welcome to incli. Your request to install rust on a Debian based os reached. Please wait until it finish...");

    let install_rust = Command::new("sudo")
                                .arg("apt")
                                .arg("install")
                                .arg("-y")
                                .arg("rustc")
                                .arg("cargo")
                                .output()
                                .expect("Installation failed.");

    if !install_rust.status.success() {
        eprintln!("Rust installation failed when installing rustc and cargo. Exiting.");
        exit(1);
    }

    println!("Installation continues...");

    let check_rustc = Command::new("rustc")
                                .arg("--version")
                                .output()
                                .expect("Failed to check Rust version.");

    if check_rustc.status.success() {
        println!("Rustc installed.");
    }

    let check_cargo = Command::new("cargo")
            .arg("--version")
            .output()
            .expect("Failed to check Cargo version.");

    if check_cargo.status.success() {
        println!("Cargo installed. Installation process finished.")
    }
}

pub fn install_rust_on_arch_wsl_and_kali(){
    println!("Welcome to incli. Your request to install rust reached. Please wait until it finish...");

    let install_rust = Command::new("wget")
                                        .arg("https://sh.rustup.rs")
                                        .arg("-O")
                                        .arg("rustup-init.sh")
                                        .output()
                                        .expect("failed to install rust");

    if !install_rust.status.success() {
        eprintln!("failed to install rustup-init.sh");
    }

    println!("Installation continues...");

    let give_777_permission_to_rustup_init = Command::new("chmod")
                                                                .arg("755")
                                                                .arg("rustup-init.sh")
                                                                .output()
                                                                .expect("failed to give permissions for rust-init.sh");

    if !give_777_permission_to_rustup_init.status.success(){
        eprintln!("failed to give 755 permission to rustup-init.sh")
    };

    let run_rustup_init = Command::new("./rustup-init.sh")
                                    .arg("-y")
                                    .output()
                                    .expect("failed to run rustup-init.sh");

    if !run_rustup_init.status.success() {
        eprintln!("failed to run rustup-init.sh")
    };

    // you have to give permissions to run env script

    Command::new("chmod")
                .arg("755")
                .arg("'$HOME/.cargo/env'")
                .output()
                .expect("Cannot give 755 permission for env script");

    // if there is any answer from kernel for a terminal command, by defalt compiler will throw error, even if doens't mean
    // that command were failed. Because of that, we need to make a way around like that:

    let add_cargo_to_envs = Command::new("source").arg("'$HOME/.cargo/env'").output().unwrap_or_else(|_| {
        println!("Restart the pc and run 'cargo --version' command. If you see a version number, that means installation is successfull.");

        return Output {
            status: ExitStatus::from_raw(0),
            stdout: Vec::new(),
            stderr: b"".to_vec()
        };
    });

    if add_cargo_to_envs.status.success() {
        println!("Cargo is successfully added on env's. You can check by running: 'cargo --version' command on terminal")
    };
}

pub fn install_rust_on_fedora_and_centos(){
    println!("Welcome to incli. Your request to install rust on red hat based os reached. Please wait until it finish...");

    let install_rust = Command::new("wget")
                                        .arg("https://sh.rustup.rs")
                                        .arg("-O")
                                        .arg("rustup-init.sh")
                                        .output()
                                        .expect("failed to install rust");

    if !install_rust.status.success() {
        eprintln!("failed to install rustup-init.sh");
    }

    println!("Installation continues...");

    let give_777_permission_to_rustup_init = Command::new("chmod")
                                                                .arg("755")
                                                                .arg("rustup-init.sh")
                                                                .output()
                                                                .expect("failed to give permissions for rust-init.sh");

    if !give_777_permission_to_rustup_init.status.success(){
        eprintln!("failed to give 755 permission to rustup-init.sh")
    };

    let run_rustup_init = Command::new("./rustup-init.sh")
                                    .arg("-y")
                                    .output()
                                    .expect("failed to run rustup-init.sh");

    if !run_rustup_init.status.success() {
        eprintln!("failed to run rustup-init.sh")
    };

    Command::new("chmod")
                .arg("755")
                .arg("'$HOME/.cargo/env'")
                .output()
                .expect("Cannot give 755 permission for env script");

    // if there is any answer from kernel for a terminal command, by defalt compiler will throw error, even if doens't mean
    // that command were failed. Because of that, we need to make a way around like that:

    let add_cargo_to_envs = Command::new("source").arg("'$HOME/.cargo/env'").output().unwrap_or_else(|_| {
        println!("Restart the pc and run 'cargo --version' command. If you see a version number, that means installation is successfull.");

        return Output {
            status: ExitStatus::from_raw(0),
            stdout: Vec::new(),
            stderr: b"".to_vec()
        };
    });

    if add_cargo_to_envs.status.success() {
        println!("Cargo is successfully added on env's. You can check by running: 'cargo --version' command on terminal")
    };
}

pub fn install_rust_on_alpine_linux(){
    println!("Welcome to incli. Your request to install rust on Alpine Linux reached. Please wait until it finish...");
    println!("Warning, because you're using alpine linux, you have to install build-base and gcompat packages for running rust. We'll install them first");
    println!("Be sure you're running this app with root user and you're installed sudo, otherwise this program cant't download necessary things. And installation will be aborted.");

    let install_build_base = Command::new("sudo").arg("apk").arg("add").arg("build-base").output().unwrap_or_else(|_| {
        println!("Looks like you're don't have sudo, program will exit now");

        return Output {
            status: ExitStatus::from_raw(1),
            stdout: b"".to_vec(),
            stderr: b"".to_vec()
        };
    });

    if !install_build_base.status.success() {
        println!("installation aborted.");

        return;
    }

    println!("You're installed build-base, installing gcompat now.");

    let install_gcompat = Command::new("sudo")
                                            .arg("apk")
                                            .arg("add")
                                            .arg("gcompat")
                                            .output()
                                            .expect("cannot install gcompat");

    if install_gcompat.status.success() {
       println!("You're installed gcompat. We'll start to download rust now.");
    }

    let install_rust = Command::new("wget")
                                        .arg("https://sh.rustup.rs")
                                        .arg("-O")
                                        .arg("rustup-init.sh")
                                        .output()
                                        .expect("failed to install rust");

    if !install_rust.status.success() {
        eprintln!("failed to install rustup-init.sh");
    }

    println!("Installation continues...");

    let give_777_permission_to_rustup_init = Command::new("chmod")
                                                                .arg("755")
                                                                .arg("rustup-init.sh")
                                                                .output()
                                                                .expect("failed to give permissions for rust-init.sh");

    if !give_777_permission_to_rustup_init.status.success(){
        eprintln!("failed to give 755 permission to rustup-init.sh")
    };

    let run_rustup_init = Command::new("./rustup-init.sh")
                                    .arg("-y")
                                    .output()
                                    .expect("failed to run rustup-init.sh");

    if !run_rustup_init.status.success() {
        eprintln!("failed to run rustup-init.sh")
    };

    Command::new("chmod")
                .arg("755")
                .arg("'$HOME/.cargo/env'")
                .output()
                .expect("Cannot give 755 permission for env script");

    // if there is any answer from kernel for a terminal command, by defalt compiler will throw error, even if doens't mean
    // that command were failed. Because of that, we need to make a way around like that:

    let add_cargo_to_envs = Command::new("source").arg("'$HOME/.cargo/env'").output().unwrap_or_else(|_| {
        println!("Restart the pc and run 'cargo --version' command. If you see a version number, that means installation is successfull.");

        return Output {
            status: ExitStatus::from_raw(0),
            stdout: Vec::new(),
            stderr: b"".to_vec()
        };
    });

    if add_cargo_to_envs.status.success() {
        println!("Cargo is successfully added on env's. You can check by running: 'cargo --version' command on terminal")
    };
}


// be sure you have latest Visual C++ Redistributable for Visual Studio version installed on your computer.
// and, if you ran "powershell" program on windows, you can't add "-y" flag as argument to terminal command.

pub fn install_rust_on_windows(){
    println!("Welcome to incli, your request to install rust on windows reached. Please wait until it finish...");

    let download_command = Command::new("powershell")
                                    .arg("Invoke-WebRequest")
                                    .arg("-Uri")
                                    .arg("https://win.rustup.rs")
                                    .arg("-OutFile")
                                    .arg("rustup-init.exe")
                                    .output()
                                    .expect("Download failed.");

    if !download_command.status.success() {
        eprintln!("Failed to download rustup-init.exe for whatever reason. Exiting.");
        exit(1);
    }

    println!("Installation continues...");

    // rustup'u yükle
    let install_command = Command::new(".\\rustup-init.exe")
                                    .arg("-y")
                                    .output()
                                    .expect("Installation failed.");

    if !install_command.status.success() {
        eprintln!("Installation failed. Exiting.");
        exit(1);
    } else {
        println!("cargo and rustc installed, you can use them by 'cargo' and 'rustc' commands later than close that terminal.")
    }
}

pub fn _install_rust_on_mac_os(){
    // it's empty due to lack of MacOs Support
}

pub fn log_rust_version(){
    let cargo_version_command = Command::new("cargo")
                                                    .arg("--version")
                                                    .output()
                                                    .expect("");

    if cargo_version_command.status.success() {
        let get_answer_as_string = std::str::from_utf8(&cargo_version_command.stdout).unwrap().to_string();

        let mut split_the_answer= get_answer_as_string.split(" ");

        println!("Your rust version is: {}", split_the_answer.nth(1).unwrap())
    } else {
        println!("Rust isn't installed on your system.");
    }
}

// Node.js Functions ----------------------------------------------------------

// url: "https://nodejs.org/dist/v20.10.0/node-v20.10.0-linux-x64.tar.xz"
// tar.gz: node-v20.10.0-linux-x64.tar.xz
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
    
    let mut bashrc_file = fs::OpenOptions::new().append(true).open(".bashrc");

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
            configure_incli_envs_file()
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
        configure_incli_envs_file()
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
        configure_incli_envs_file()
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

// i don't know why but in arch linux we can't give 755 permissions.

pub fn configure_incli_envs_file(){
    let create_incli_paths_folder = Command::new("sudo")
                                            .arg("mkdir")
                                            .arg("INCLI_PATHS")
                                            .output();

    match create_incli_paths_folder {
        Ok(_) => (),
        Err(err) => {
            eprintln!("an error occured when INCLI_PATHS folder about to create: {}", err);
            return;
        }
    }

    let create_incli_envs_file = Command::new("sudo")
                                            .arg("touch")
                                            .arg("./INCLI_PATHS/incli-envs.sh")
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
                                                                    .arg("INCLI_PATHS")
                                                                    .output();

    match give_permission_for_incli_paths {
        Ok(_) => (),
        Err(err) => {
            eprintln!("an error occured when try to give permission for INCLI_PATHS folder: {}", err);
            return;
        }
    }

    let give_permission_to_bashrc = Command::new("sudo")
                                                        .arg("chmod")
                                                        .arg("777")
                                                        .arg(".bashrc")
                                                        .output()
                                                        .expect("cannot give permission to .bashrc file");

    if !give_permission_to_bashrc.status.success() {
        println!("Cannot give required permissions for .bashrc, you have to add incli-envs.sh file's path on that file via that synthax for adding node.js on your user's env's: \". \"$HOME/INCLI_PATHS/incli-envs.sh\"\"")
    }

    let bashrc_file = fs::OpenOptions::new().append(true).open("./.bashrc");

    match bashrc_file {
        Ok(mut file) => {
            let add_env_file_dest = io::Write::write_all(&mut file, ". \"$HOME/INCLI_PATHS/incli-envs.sh\"".as_bytes());

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

