use std::process::{Command, Output, ExitStatus, exit};
use sys_info_extended::get_current_user;

// there is some platform-specific apis on that page:

#[cfg(target_os = "windows")]
use std::os::windows::process::ExitStatusExt;

#[cfg(target_os = "linux")]
use std::os::unix::process::ExitStatusExt;

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
    println!("Welcome to incli, your request to install Rust on Windows reached. Please wait until it finish...");
    println!("Keep pressing any of your keys when you focused on your terminal in regular time period, otherwise your installation may not run correctly.");

    let current_user = get_current_user();

    let format_the_download_path = format!("C:\\Users\\{}\\Downloads\\rustup-init.exe", current_user);

    let download_command = Command::new("powershell")
                                    .arg("Invoke-WebRequest")
                                    .arg("-Uri")
                                    .arg("https://win.rustup.rs")
                                    .arg("-OutFile")
                                    .arg(&format_the_download_path)
                                    .output()
                                    .expect("Download failed.");

    if !download_command.status.success() {
        eprintln!("Failed to download rustup-init.exe for whatever reason. Exiting.");
        exit(1);
    }

    println!("Installation continues...");

    // rustup'u yükle
    let install_command = Command::new(format_the_download_path)
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