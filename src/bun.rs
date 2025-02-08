use crate::utils;

#[cfg(target_os = "linux")]
use std::os::unix::process::ExitStatusExt;

use std::{str::from_utf8, process::{Command, Output, ExitStatus, exit}};


pub fn install_bun_on_linux(){
    println!("Welcome To incli, your request to install bun on Linux. Please wait until it finish...");
    println!("If you are using debian, be sure you have installed curl, wget and unzip, otherwise installation won't work.");
    println!("If you're using arch linux, kali linux or alma linux, be sure you have installed unzip.");

    let install_bun_script = Command::new("wget")
                                                        .arg("https://bun.sh/install")
                                                        .arg("-O")
                                                        .arg("bun-installer.sh")
                                                        .output()
                                                        .expect("cannot installed bun-installer.sh");

    if install_bun_script.status.success() {
        println!("bun-installer.sh successfully installed, other steps remaining...");
    } else {
        println!("for some reason, bun-installer.sh couldn't be downloaded, exiting.");
        exit(1);
    }

    Command::new("sudo")
                .arg("chmod")
                .arg("777")
                .arg("bun-installer.sh")
                .output()
                .expect("cannot give 777 permission for bun-installer.sh");

    let run_bun_installer = Command::new("./bun-installer.sh")
                                                    .output()
                                                    .expect("cannot install bun");

    if run_bun_installer.status.success() {
        println!("Bun installed successfully, you can check it via running 'bun --version' command.")
    } else {
        println!("For some reason, bun couldn't installed. Here is the reason: {}", from_utf8(&run_bun_installer.stderr).unwrap());
        exit(1);
    }

    #[cfg(target_os = "linux")]
    Command::new("source").arg("/home/neco/.bashrc").output().unwrap_or_else(|_| {
        println!("Restart the pc and run 'bun --version' command. If you see a version number, that means installation is successfull.");

        return Output {
            status: ExitStatus::from_raw(0),
            stdout: Vec::new(),
            stderr: b"".to_vec()
        };
    });
}

pub fn install_bun_on_rocky_linux(){
    println!("Welcome To incli, your request to install bun on Linux. Please wait until it finish...");
    println!("If you are using debian, be sure you have installed curl, wget and unzip, otherwise installation won't work.");
    println!("If you're using arch linux, kali linux or alma linux, be sure you have installed unzip.");

    let install_bun_script = Command::new("wget")
                                                        .arg("https://bun.sh/install")
                                                        .arg("-O")
                                                        .arg("bun-installer.sh")
                                                        .output()
                                                        .expect("cannot installed bun-installer.sh");

    if install_bun_script.status.success() {
        println!("bun-installer.sh successfully installed, other steps remaining...");
    } else {
        println!("for some reason, bun-installer.sh couldn't be downloaded, exiting.");
        exit(1);
    }

    Command::new("chmod")
                .arg("777")
                .arg("bun-installer.sh")
                .output()
                .expect("cannot give 777 permission for bun-installer.sh");

    let run_bun_installer = Command::new("./bun-installer.sh")
                                                    .output()
                                                    .expect("cannot install bun");

    if run_bun_installer.status.success() {
        println!("Bun installed successfully, you can check it via running 'bun --version' command.")
    } else {
        println!("For some reason, bun couldn't installed. Here is the reason: {}", from_utf8(&run_bun_installer.stderr).unwrap());
        exit(1);
    }

    #[cfg(target_os = "linux")]
    Command::new("source").arg("/home/neco/.bashrc").output().unwrap_or_else(|_| {
        println!("Restart the pc and run 'bun --version' command. If you see a version number, that means installation is successfull.");

        return Output {
            status: ExitStatus::from_raw(0),
            stdout: Vec::new(),
            stderr: b"".to_vec()
        };
    });
}

pub fn log_bun_version(){
    let bun_version = Command::new("bun")
                                        .arg("--version")
                                        .output()
                                        .expect("cannot check bun version");

    if bun_version.status.success() {
        let get_bun_output = from_utf8(&bun_version.stdout).unwrap();

       println!("Your bun version is: {}", get_bun_output)
    } else {
        println!("Error, there is no bun installation or it isn't recorded in your PATH env.");
    }

}