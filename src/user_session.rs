use crate::models;
use models::{UserSession, OsType, CommandType, Version, UserAction, EnvConfiguration};
use sys_info_extended::{get_current_user, get_home_dir_and_shell, os_release, os_type};

impl UserSession {
    pub fn create() -> Self {
        Self {
            os_type: Self::get_os_type().unwrap(),
            os_release: os_release().unwrap(),
            current_user: get_current_user()
        }
    }

    fn get_os_type() -> Result<OsType, ()> {
        let os_type = os_type();

        match os_type {
            Ok(os) => match os.as_str() {
                "Windows" => Ok(OsType::Windows),
                "Linux" => Ok(OsType::Linux),
                "Darwin" => Ok(OsType::Darwin),
                &_ => panic!("This program developed for working on either Windows, Linux Or Darwin, Panicking!")
            },
            Err(_) => Err(())
        }
    }

    pub fn action(&self, cmd_type: String, second_arg: String, third_arg: String, fourth_arg: String) -> UserAction {
        let command_type;

        match cmd_type.as_str() {
            "install" => command_type = CommandType::Install,
            "version" => command_type = CommandType::Version,
            "help" => command_type = CommandType::Help,
            &_ => panic!("unrecognized second argument. if you don't know how to use this tool, please type 'incli help' to learn it")
        }

        let s_arg = match second_arg.as_str() {
            "rust" => "rust".to_string(),
            "java" => "java".to_string(),
            "go" => "go".to_string(),
            "gradle" => "gradle".to_string(),
            "maven" => "maven".to_string(),
            "zig" => "zig".to_string(),
            "composer" => "composer".to_string(),
            "android" => "android".to_string(),
            "node" => "node".to_string(),
            "bun" => "bun".to_string(),
            "yarn" => "yarn".to_string(),
            "python" => "python".to_string(),
            &_ => "".to_string()
        };

        let os_type = Self::get_os_type().unwrap();

        return UserAction {
            os_release: self.os_release.clone(),
            os_type: os_type.clone(),
            env_confs: EnvConfiguration::init(&self.current_user, os_type),
            current_user: self.current_user.clone(),
            first_arg: command_type.clone(),
            second_arg: s_arg,
            third_arg: third_arg.to_string(),
            fourth_arg: fourth_arg.to_string(),
        }
    }
}