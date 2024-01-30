mod models;
mod utils;
mod rust;
mod node;
mod bun;
mod yarn;

use models::{CreateAction, OsType, Execution, Version, Help, CommandType};
use sys_info_extended::{os_release, os_type, get_current_user};
use std::env::args;


// yapılacak şeyler:

// şu an sadece rust için yapıldı, bunu bir windows ve macos subsystem bulup orada dene, linux için bilgisayardaki
// debian ve arch linux'da deneyebilirsin, fedora'yı da yükleyerek dene.

// windows'da bir dosya çalıştırmak için şu usulü kullanabilirsin: 
// terminalde alakalı dosya'nın klasörüne git: ".\'dosya ismi.dosyaUzantisi'" suretiyle terminale talimat gir.
// ya da şöyle yap: C klasörüne git, ".\klasor1\klasor2\'dosya ismi.dosyaUzantisi'" suretiyle terminale talimat gir.

fn main() {
    let user_session = models::UserSession {
        os_type: os_type().unwrap(),
        os_release: os_release().unwrap(),
        current_user: get_current_user()
    };

    let args: Vec<String> = args().collect();

    let mut arg1 = "".to_string();
    let mut arg2 = "".to_string();
    let mut arg3 = "".to_string();

    for (index, argument) in args.into_iter().enumerate() {
        if index == 1 {
            arg1 = match argument.as_str() {
                "help" => "help".to_string(),
                "install" => "install".to_string(),
                "version" => "version".to_string(),
                &_ => panic!("Invalid argument, you have to type either 'help', 'install' or 'version'")
            }
        }

        if index == 2 {
            arg2 = match argument.as_str() {
                "rust" => "rust".to_string(),
                "java" => "java".to_string(),
                "gradle" => "gradle".to_string(),
                "maven" => "maven".to_string(),
                "go" => "go".to_string(),
                "node" => "node".to_string(),
                "bun" => "bun".to_string(),
                "yarn" => "yarn".to_string(),
                "python" => "python".to_string(),
                "" => "".to_string(),
                &_ => panic!("You're mistyped or tried to download unsupported programming language.")
            }
        }

        if index == 3 {
            arg3 = argument;
        }
    };

    let user_action = user_session.action(arg1, arg2, arg3); 

    match  user_action.first_arg {
        CommandType::Help => {
            match user_action.second_arg.as_str() {
                "rust" => user_action.rust_help(),
                "go" => user_action.go_help(),
                "node" => user_action.nodejs_help(),
                "bun" => user_action.bun_help(),
                "yarn" => user_action.yarn_help(),
                "gradle" => user_action.gradle_help(),
                "maven" => user_action.maven_help(),
                "python" => user_action.python_help(),
                "" => user_action.log_help(),
                &_ => user_action.log_help_error()
            };
        },
        CommandType::Version => {
            match user_action.second_arg.as_str() {
                "rust" => user_action.rust_version(),
                "go" => user_action.go_version(),
                "node" => user_action.nodejs_version(),
                "bun" => user_action.bun_version(),
                "yarn" => user_action.yarn_version(),
                "gradle" => user_action.gradle_version(),
                "maven" => user_action.maven_version(),
                "python" => user_action.python_version(),
                "" => user_action.log_version(),
                &_ => user_action.log_version_error()
            };
        },
        CommandType::Install => {
            match user_action.os_type {
                OsType::Windows => user_action.execute_windows(),
                OsType::Linux => user_action.execute_linux(),
                OsType::Darwin => user_action.execute_darwin()
            };
        }
    }
}
