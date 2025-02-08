use std::process::Command;


pub fn install_yarn_on_windows() {
    println!("Welcome to incli, your request to install yarn on windows reached. Wait while we proccessing installation...");
    println!("Be sure you have installed node.js and npm. You can't use yarn without them.");
    
    let check_if_node_installed = Command::new("powershell")
                                                                    .arg("npm")
                                                                    .arg("--version")
                                                                    .output();

    match check_if_node_installed {
        Ok(_) => println!("npm exist in this computer, installation continues..."),
        Err(error) => {
            println!("That error occured when we check npm's version: {}", error);
            println!("Process exiting...");

            return;
        }
    }

    let install_yarn = Command::new("powershell")
                                                        .arg("npm")
                                                        .arg("install")
                                                        .arg("-g")
                                                        .arg("yarn")
                                                        .output();

    match install_yarn {
        Ok(_) => {
            println!("Yarn successfully installed on your computer. You can check it via 'yarn --version' command.");
            println!("If that command won't work, try to close your terminal and computer ordinarily, most probably it'll work.")
        },
        Err(error) => eprintln!("There is an error occured when installing yarn: {}", error)
    }

    
}

pub fn install_yarn_on_linux() {
    println!("Welcome to incli, your request to install yarn on linux reached. Wait while we proccessing installation...");
    println!("Be sure you have installed node.js and npm. You can't use yarn without them.");
    
    let check_if_node_installed = Command::new("npm")
                                                                    .arg("--version")
                                                                    .output();

    match check_if_node_installed {
        Ok(_) => println!("npm exist in this computer, installation continues..."),
        Err(error) => {
            println!("That error occured when we check npm's version: {}", error);
            println!("Process exiting...");

            return;
        }
    }

    let install_yarn = Command::new("npm")
                                                        .arg("install")
                                                        .arg("-g")
                                                        .arg("yarn")
                                                        .output();

    match install_yarn {
        Ok(_) => {
            println!("Yarn successfully installed on your computer. You can check it via 'yarn --version' command.");
            println!("If that command won't work, try to close your terminal and computer ordinarily, most probably it'll work.")
        },
        Err(error) => eprintln!("There is an error occured when installing yarn: {}", error)
    }

    
}